//! Custom command handlers.
//!
//! This file is yours to edit — it is listed in `.fernignore` so
//! `fern generate` will never overwrite your changes.
//!
//! The generated `main.rs` calls `custom::register(app)` at startup,
//! composing your commands into the CLI at compile time. This is the
//! same pattern used by other Fern generators (e.g. Ruby's
//! `requirePaths`) — the generated entrypoint references this
//! user-owned file, and `.fernignore` keeps it safe across
//! regenerations.
//!
//! Each handler receives an [`AppContext`] whose `invoke()` and
//! `execute()` methods use the CLI's native HTTP executor (same auth,
//! retries, base-URL resolution, and global headers as the generated
//! commands). Combine these with the typed structs from
//! `petstore_api_types` for strongly-typed request/response handling.
//!
//! ## What's in here
//!
//! Three worked examples against the Petstore API:
//!
//! * `adopt`            — top-level command that *chains* two calls
//!                        (create a pet, then fetch it back) using typed
//!                        request/response models.
//! * `pets find`        — grafted *under* the generated `pets` group;
//!                        lists pets and filters them client-side.
//! * `pets count`       — also under `pets`; aggregates the list into a
//!                        single count and honors the global `--format`.

use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::formatter::{self, OutputFormat};
use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};
use petstore_api_types::*;

/// Register custom commands on the CLI app builder.
///
/// Called from `main.rs` during startup. Add or remove commands here;
/// each is wrapped with [`OpenApiBinding::handler`] so the handler
/// receives a strongly-typed [`AppContext`] instead of a `&dyn Any`.
pub fn register(app: CliApp) -> CliApp {
    app
        // A brand-new top-level command.
        .command(adopt_command(), OpenApiBinding::handler(handle_adopt))
        // Grafted under the generated `pets` group, alongside
        // `create-pet`, `get-pet`, and `list-pets`.
        .command_under(&["pets"], find_command(), OpenApiBinding::handler(handle_find))
        .command_under(&["pets"], count_command(), OpenApiBinding::handler(handle_count))
}

// ── `adopt` ─────────────────────────────────────────────────────────
//
// Demonstrates chaining: POST /pets to create a pet, then GET
// /pets/{petId} to read it back — using typed request/response models
// and `invoke()` (which captures the JSON response instead of printing
// it, so the handler can feed one call's output into the next).

fn adopt_command() -> Command {
    Command::new("adopt")
        .about("Create a pet and immediately fetch the stored record (chained call)")
        .arg(
            Arg::new("name")
                .required(true)
                .help("Name of the pet to adopt"),
        )
        .arg(
            Arg::new("tag")
                .long("tag")
                .help("Optional tag/category for the pet"),
        )
}

fn handle_adopt(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let name = matches.get_one::<String>("name").expect("required arg");

    // Build a typed request body with the generated builder, then
    // serialize it to JSON for the executor.
    let mut builder = CreatePetRequest::builder().name(name.as_str());
    if let Some(tag) = matches.get_one::<String>("tag") {
        builder = builder.tag(tag.as_str());
    }
    let body = builder
        .build()
        .map_err(|e| CliError::Validation(e.to_string()))?;
    let body_json = serde_json::to_string(&body).map_err(|e| CliError::Other(e.into()))?;

    // 1. POST /pets
    let create_method = ctx.find_method("pets", "create-pet")?;
    let created = ctx.invoke(create_method, None, Some(&body_json), None)?;
    let created: Pet =
        serde_json::from_value(created).map_err(|e| CliError::Other(e.into()))?;
    eprintln!("Created pet '{}' (id {})", created.name, created.id);

    // 2. GET /pets/{petId} — confirm the record round-trips.
    let get_method = ctx.find_method("pets", "get-pet")?;
    let params = serde_json::json!({ "petId": created.id }).to_string();
    let fetched = ctx.invoke(get_method, Some(&params), None, None)?;
    let fetched: Pet =
        serde_json::from_value(fetched).map_err(|e| CliError::Other(e.into()))?;

    println!(
        "{}",
        serde_json::to_string_pretty(&fetched).map_err(|e| CliError::Other(e.into()))?
    );
    Ok(())
}

// ── `pets find` ─────────────────────────────────────────────────────
//
// Demonstrates grafting a command UNDER an existing group, building
// typed query parameters, and post-processing a typed response
// (`Vec<Pet>`) — here, a case-insensitive client-side name filter.

fn find_command() -> Command {
    Command::new("find")
        .about("List pets and filter by a case-insensitive name substring")
        .arg(
            Arg::new("query")
                .required(true)
                .help("Substring to match against pet names"),
        )
        .arg(
            Arg::new("limit")
                .long("limit")
                .value_parser(clap::value_parser!(i64))
                .help("Max pets to fetch from the API before filtering"),
        )
}

fn handle_find(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let query = matches
        .get_one::<String>("query")
        .expect("required arg")
        .to_lowercase();

    // Typed query parameters. `limit` is optional and skipped when
    // unset, so an absent flag serializes to `{}`.
    let mut q = ListPetsQueryRequest::builder();
    if let Some(limit) = matches.get_one::<i64>("limit") {
        q = q.limit(*limit);
    }
    let q = q.build().map_err(|e| CliError::Validation(e.to_string()))?;
    let params_json = serde_json::to_string(&q).map_err(|e| CliError::Other(e.into()))?;

    let list_method = ctx.find_method("pets", "list-pets")?;
    let response = ctx.invoke(list_method, Some(&params_json), None, None)?;

    let pets: Vec<Pet> =
        serde_json::from_value(response).map_err(|e| CliError::Other(e.into()))?;
    let matched: Vec<&Pet> = pets
        .iter()
        .filter(|p| p.name.to_lowercase().contains(&query))
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&matched).map_err(|e| CliError::Other(e.into()))?
    );
    Ok(())
}

// ── `pets count` ────────────────────────────────────────────────────
//
// Demonstrates aggregating a response into a derived value and
// rendering it with the CLI's own formatter, honoring the global
// `--format` flag (json / table / yaml / csv) that clap propagates
// down to every subcommand.

fn count_command() -> Command {
    Command::new("count").about("Count the total number of pets")
}

fn handle_count(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let list_method = ctx.find_method("pets", "list-pets")?;
    let response = ctx.invoke(list_method, None, None, None)?;
    let pets: Vec<Pet> =
        serde_json::from_value(response).map_err(|e| CliError::Other(e.into()))?;

    let format = matches
        .get_one::<String>("format")
        .map(|s| OutputFormat::from_str(s))
        .unwrap_or_default();
    let summary = serde_json::json!({ "count": pets.len() });
    println!("{}", formatter::format_value(&summary, &format));
    Ok(())
}
