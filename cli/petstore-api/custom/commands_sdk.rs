// //! Custom commands implemented on the generated **`petstore` Rust SDK**.
// //!
// //! These are the SDK-backed twins of [`super::commands_native`]. Instead
// //! of routing through [`AppContext`]'s native executor, each handler
// //! constructs a [`PetstoreClient`] from the `petstore` crate (added as a
// //! git dependency in `Cargo.toml`) and calls its typed, async methods.
// //!
// //! To coexist with the native commands, these are registered with a
// //! `-sdk` suffix:
// //! * `adopt-sdk`       — top-level; chains `create_pet` + `get_pet`.
// //! * `pets find-sdk`   — grafted under the generated `pets` group.
// //! * `pets count-sdk`  — also under `pets`.
// //!
// //! `AppContext` is still threaded in — not for HTTP, but to resolve the
// //! API base URL (from `--base-url`, the `PETSTORE_API_BASE_URL` env var,
// //! or the spec's declared server) so SDK calls hit the same endpoint as
// //! the native commands.

// use clap::{Arg, ArgMatches, Command};
// use fern_cli_sdk::app::CliApp;
// use fern_cli_sdk::error::CliError;
// use fern_cli_sdk::formatter::{self, OutputFormat};
// use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};
// use petstore::prelude::*;

// /// Register the SDK-backed command set on the CLI app builder.
// pub fn register(app: CliApp) -> CliApp {
//     app
//         .command(adopt_command(), OpenApiBinding::handler(handle_adopt))
//         .command_under(&["pets"], find_command(), OpenApiBinding::handler(handle_find))
//         .command_under(&["pets"], count_command(), OpenApiBinding::handler(handle_count))
// }

// // ── Shared helpers ──────────────────────────────────────────────────

// /// Drive a `Future` to completion from a synchronous custom-command
// /// handler. Handlers run *inside* the CLI's tokio runtime, so a naive
// /// `block_on` would panic ("Cannot start a runtime from within a
// /// runtime"); `block_in_place` parks the current worker first.
// fn block_on<F: std::future::Future>(fut: F) -> F::Output {
//     tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(fut))
// }

// /// Build a [`PetstoreClient`] pointed at the same base URL the native
// /// commands would use: `--base-url` flag → `PETSTORE_API_BASE_URL` env
// /// var → the spec's declared server URL.
// fn make_client(matches: &ArgMatches, ctx: &AppContext) -> Result<PetstoreClient, CliError> {
//     let base_url = matches
//         .get_one::<String>("base-url")
//         .cloned()
//         .or_else(|| std::env::var("PETSTORE_API_BASE_URL").ok())
//         .filter(|s| !s.is_empty())
//         .unwrap_or_else(|| ctx.spec().root_url.clone());

//     let config = ClientConfig {
//         base_url,
//         ..Default::default()
//     };
//     PetstoreClient::new(config).map_err(|e| CliError::Other(e.into()))
// }

// // ── `adopt-sdk` ─────────────────────────────────────────────────────

// fn adopt_command() -> Command {
//     Command::new("adopt-sdk")
//         .about("Create a pet and immediately fetch the stored record (petstore SDK)")
//         .arg(
//             Arg::new("name")
//                 .required(true)
//                 .help("Name of the pet to adopt"),
//         )
//         .arg(
//             Arg::new("tag")
//                 .long("tag")
//                 .help("Optional tag/category for the pet"),
//         )
// }

// fn handle_adopt(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
//     let name = matches.get_one::<String>("name").expect("required arg");
//     let client = make_client(matches, ctx)?;

//     let request = CreatePetRequest {
//         name: name.clone(),
//         tag: matches.get_one::<String>("tag").cloned(),
//     };

//     // 1. POST /pets via the SDK.
//     let created = block_on(client.pets.create_pet(&request, None))
//         .map_err(|e| CliError::Other(e.into()))?;
//     eprintln!("Created pet '{}' (id {})", created.name, created.id);

//     // 2. GET /pets/{petId} via the SDK — confirm it round-trips.
//     let fetched = block_on(client.pets.get_pet(&created.id, None))
//         .map_err(|e| CliError::Other(e.into()))?;

//     println!(
//         "{}",
//         serde_json::to_string_pretty(&fetched).map_err(|e| CliError::Other(e.into()))?
//     );
//     Ok(())
// }

// // ── `pets find-sdk` ─────────────────────────────────────────────────

// fn find_command() -> Command {
//     Command::new("find-sdk")
//         .about("List pets and filter by a case-insensitive name substring (petstore SDK)")
//         .arg(
//             Arg::new("query")
//                 .required(true)
//                 .help("Substring to match against pet names"),
//         )
//         .arg(
//             Arg::new("limit")
//                 .long("limit")
//                 .value_parser(clap::value_parser!(i64))
//                 .help("Max pets to fetch from the API before filtering"),
//         )
// }

// fn handle_find(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
//     let query = matches
//         .get_one::<String>("query")
//         .expect("required arg")
//         .to_lowercase();
//     let client = make_client(matches, ctx)?;

//     let request = ListPetsQueryRequest {
//         limit: matches.get_one::<i64>("limit").copied(),
//     };

//     let pets = block_on(client.pets.list_pets(&request, None))
//         .map_err(|e| CliError::Other(e.into()))?;
//     let matched: Vec<&Pet> = pets
//         .iter()
//         .filter(|p| p.name.to_lowercase().contains(&query))
//         .collect();

//     println!(
//         "{}",
//         serde_json::to_string_pretty(&matched).map_err(|e| CliError::Other(e.into()))?
//     );
//     Ok(())
// }

// // ── `pets count-sdk` ────────────────────────────────────────────────

// fn count_command() -> Command {
//     Command::new("count-sdk").about("Count the total number of pets (petstore SDK)")
// }

// fn handle_count(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
//     let client = make_client(matches, ctx)?;

//     let pets = block_on(client.pets.list_pets(&ListPetsQueryRequest::default(), None))
//         .map_err(|e| CliError::Other(e.into()))?;

//     let format = matches
//         .get_one::<String>("format")
//         .map(|s| OutputFormat::from_str(s))
//         .unwrap_or_default();
//     let summary = serde_json::json!({ "count": pets.len() });
//     println!("{}", formatter::format_value(&summary, &format));
//     Ok(())
// }
