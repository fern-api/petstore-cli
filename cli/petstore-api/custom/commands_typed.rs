//! Custom commands using **compile-time typed arguments**.
//!
//! These are the typed-args twins of [`super::commands_cogenerated`].
//! Instead of reading CLI arguments from `&ArgMatches` via string
//! literals (which panics at runtime on typos), each handler declares
//! a `#[derive(clap::Args)]` struct whose fields *are* the arguments.
//! A mismatched field name is a compile error, not a runtime panic.
//!
//! To coexist with the other command sets, these are registered with
//! a `-typed` suffix:
//! * `adopt-typed`       — top-level; chains `create_pet` + `get_pet`.
//! * `pets find-typed`   — grafted under the generated `pets` group.
//! * `pets count-typed`  — also under `pets`.

use clap::Command;
use fern_cli_sdk::app::CliApp;
use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};

use petstore_api_sdk::prelude::*;

use super::super::sdk_glue;

/// Register the typed-argument command set on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    app.command_typed(
        Command::new("adopt-typed")
            .about("Create a pet and immediately fetch the stored record (typed args)"),
        OpenApiBinding::typed_handler(handle_adopt),
    )
    .command_under_typed(
        &["pets"],
        Command::new("find-typed")
            .about("List pets and filter by a case-insensitive name substring (typed args)"),
        OpenApiBinding::typed_handler(handle_find),
    )
    .command_under_typed(
        &["pets"],
        Command::new("count-typed").about("Count the total number of pets (typed args)"),
        OpenApiBinding::typed_handler(handle_count),
    )
}

// ── `adopt-typed` — chain create + get ──────────────────────────────

#[derive(clap::Args)]
struct AdoptArgs {
    /// Name of the pet to adopt
    name: String,
    /// Optional tag/category for the pet
    #[arg(long)]
    tag: Option<String>,
}

fn handle_adopt(args: AdoptArgs, ctx: &AppContext) -> Result<(), CliError> {
    let client = sdk_glue::sdk_client(ctx);

    let request = CreatePetRequest {
        name: args.name.clone(),
        tag: args.tag,
    };

    let created: Pet = sdk_glue::block_on(client.pets.create_pet(&request, None))?;
    eprintln!("Created pet '{}' (id {})", created.name, created.id);

    let fetched: Pet = sdk_glue::block_on(client.pets.get_pet(&created.id, None))?;
    println!(
        "{}",
        serde_json::to_string_pretty(&fetched).map_err(|e| CliError::Other(e.into()))?
    );
    Ok(())
}

// ── `pets find-typed` — typed list + client-side filter ─────────────

#[derive(clap::Args)]
struct FindArgs {
    /// Substring to match against pet names
    query: String,
    /// Max pets to fetch from the API before filtering
    #[arg(long)]
    limit: Option<i64>,
}

fn handle_find(args: FindArgs, ctx: &AppContext) -> Result<(), CliError> {
    let query = args.query.to_lowercase();
    let client = sdk_glue::sdk_client(ctx);

    let request = ListPetsQueryRequest {
        limit: args.limit,
    };

    let pets: Vec<Pet> = sdk_glue::block_on(client.pets.list_pets(&request, None))?;
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

// ── `pets count-typed` — aggregate ──────────────────────────────────

#[derive(clap::Args)]
struct CountArgs {}

fn handle_count(_args: CountArgs, ctx: &AppContext) -> Result<(), CliError> {
    let client = sdk_glue::sdk_client(ctx);

    let pets: Vec<Pet> = sdk_glue::block_on(
        client
            .pets
            .list_pets(&ListPetsQueryRequest::default(), None),
    )?;

    let summary = serde_json::json!({ "count": pets.len() });
    println!(
        "{}",
        serde_json::to_string_pretty(&summary).map_err(|e| CliError::Other(e.into()))?
    );
    Ok(())
}
