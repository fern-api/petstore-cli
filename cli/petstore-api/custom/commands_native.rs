//! Custom commands implemented on the CLI's **native runtime**.
//!
//! These handlers go through [`AppContext`], whose `invoke()` /
//! `execute()` methods reuse the generated CLI's own HTTP executor —
//! same auth, retries, base-URL resolution, and global headers as the
//! built-in commands. Request/response models come from the
//! `petstore_api_types` crate.
//!
//! Registered command surface:
//! * `adopt`       — top-level; chains create + get.
//! * `pets find`   — grafted under the generated `pets` group.
//! * `pets count`  — also under `pets`.
//!
//! The SDK-backed twins of these commands live in
//! [`super::commands_sdk`].

use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::formatter::{self, OutputFormat};
use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};
use petstore_api_types::*;

/// Register the native-runtime command set on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    app
        .command(adopt_command(), OpenApiBinding::handler(handle_adopt))
        .command_under(&["pets"], find_command(), OpenApiBinding::handler(handle_find))
        .command_under(&["pets"], count_command(), OpenApiBinding::handler(handle_count))
}

// ── `adopt` ─────────────────────────────────────────────────────────
//
// Chains POST /pets then GET /pets/{petId} via `invoke()` (which
// captures the JSON response instead of printing it, so one call's
// output can feed the next).

fn adopt_command() -> Command {
    Command::new("adopt")
        .about("Create a pet and immediately fetch the stored record (native runtime)")
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
// Grafted UNDER the `pets` group. Builds typed query parameters and
// post-processes a typed `Vec<Pet>` with a client-side name filter.

fn find_command() -> Command {
    Command::new("find")
        .about("List pets and filter by a case-insensitive name substring (native runtime)")
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
// Aggregates the list into a derived value and renders it with the
// CLI's own formatter, honoring the global `--format` flag.

fn count_command() -> Command {
    Command::new("count").about("Count the total number of pets (native runtime)")
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
