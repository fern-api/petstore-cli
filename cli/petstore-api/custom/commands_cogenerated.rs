//! Custom commands implemented via the **co-generated SDK glue**.
//!
//! These are the SDK-backed twins of [`super::commands_native`]. Instead
//! of routing through [`AppContext`]'s native `invoke()` / `execute()`
//! methods, each handler uses the generated `sdk_glue` module to get a
//! fully-wired SDK client that inherits auth, retries, TLS, and global
//! headers from the CLI's own executor.
//!
//! To coexist with the native commands, these are registered with a
//! `-cogen` suffix:
//! * `adopt-cogen`       — top-level; chains `create_pet` + `get_pet`.
//! * `pets find-cogen`   — grafted under the generated `pets` group.
//! * `pets count-cogen`  — also under `pets`.

use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::formatter::{self, OutputFormat};
use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};

use petstore_api_sdk::prelude::*;

use super::super::sdk_glue;

/// Register the co-generated-SDK command set on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    app
        .command(adopt_command(), OpenApiBinding::handler(handle_adopt))
        .command_under(&["pets"], find_command(), OpenApiBinding::handler(handle_find))
        .command_under(&["pets"], count_command(), OpenApiBinding::handler(handle_count))
}

// ── `adopt-cogen` — chain create + get ──────────────────────────────

fn adopt_command() -> Command {
    Command::new("adopt-cogen")
        .about("Create a pet and immediately fetch the stored record (co-generated SDK)")
        .arg(Arg::new("name").required(true).help("Name of the pet to adopt"))
        .arg(Arg::new("tag").long("tag").help("Optional tag/category for the pet"))
}

fn handle_adopt(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let name = matches.get_one::<String>("name").expect("required arg");

    let client = sdk_glue::sdk_client(ctx);

    let request = CreatePetRequest {
        name: name.clone(),
        tag: matches.get_one::<String>("tag").cloned(),
    };

    let created: Pet = sdk_glue::block_on(client.pets.create_pet(&request, None))?;
    eprintln!("Created pet '{}' (id {})", created.name, created.id);

    let fetched: Pet = sdk_glue::block_on(client.pets.get_pet(&created.id, None))?;
    println!("{}", serde_json::to_string_pretty(&fetched).unwrap());
    Ok(())
}

// ── `pets find-cogen` — typed list + client-side filter ─────────────

fn find_command() -> Command {
    Command::new("find-cogen")
        .about("List pets and filter by a case-insensitive name substring (co-generated SDK)")
        .arg(Arg::new("query").required(true).help("Substring to match against pet names"))
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

    let client = sdk_glue::sdk_client(ctx);

    let request = ListPetsQueryRequest {
        limit: matches.get_one::<i64>("limit").copied(),
    };

    let pets: Vec<Pet> = sdk_glue::block_on(client.pets.list_pets(&request, None))?;
    let matched: Vec<&Pet> = pets
        .iter()
        .filter(|p| p.name.to_lowercase().contains(&query))
        .collect();

    println!("{}", serde_json::to_string_pretty(&matched).unwrap());
    Ok(())
}

// ── `pets count-cogen` — aggregate, honor --format ──────────────────

fn count_command() -> Command {
    Command::new("count-cogen").about("Count the total number of pets (co-generated SDK)")
}

fn handle_count(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = sdk_glue::sdk_client(ctx);

    let pets: Vec<Pet> = sdk_glue::block_on(client.pets.list_pets(&ListPetsQueryRequest::default(), None))?;

    let format = matches
        .get_one::<String>("format")
        .map(|s| OutputFormat::from_str(s))
        .unwrap_or_default();
    let summary = serde_json::json!({ "count": pets.len() });
    println!("{}", formatter::format_value(&summary, &format));
    Ok(())
}
