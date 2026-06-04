// ─────────────────────────────────────────────────────────────────────────────
// COMMENTED OUT — reference design sketch only; NOT compiled and NOT registered.
//
// Illustrates the proposed "co-generated SDK" custom-command authoring experience.
// The APIs it uses (ctx.sdk_client(), ctx.block_on(...)) are the TARGET and do not
// exist yet. Kept in-tree purely as a worked example for the generator migration.
// Compare against commands_native.rs (native runtime) and commands_sdk.rs (external SDK).
// ─────────────────────────────────────────────────────────────────────────────

// //! DESIGN SKETCH — the proposed "co-generated SDK" authoring experience.
// //!
// //! ⚠️  NON-COMPILING / NOT REGISTERED. This file is intentionally left out of
// //! `custom.rs`'s `mod` list, so cargo never builds it. It exists only to show
// //! what custom commands would look like *after* the generator co-produces the
// //! Rust SDK into the CLI project and the command context hands back a client
// //! that shares the CLI's execution environment. The APIs used here
// //! (`ctx.sdk_client()`, `ctx.block_on(...)`) do not exist yet — they're the
// //! target.
// //!
// //! Compare the same three commands across the three files:
// //!   * commands_native.rs      — today's native runtime: stringly-typed
// //!                               `find_method("pets","create-pet")`, build a
// //!                               JSON string, get a `serde_json::Value` back.
// //!   * commands_sdk.rs         — today's external SDK: fully typed, but a
// //!                               SEPARATE stack you hand-wire (see `make_client`,
// //!                               which re-resolves the base URL and does NOT
// //!                               inherit the CLI's auth / global headers / TLS),
// //!                               pinned via a git dependency that can drift.
// //!   * commands_cogenerated.rs — THIS file: typed call sites AND the CLI's real
// //!                               execution environment, with zero client setup.
// //!
// //! What disappears vs. commands_sdk.rs:
// //!   - No `make_client`, no `ClientConfig`, no manual `--base-url`/env resolution.
// //!   - No external `petstore` git dependency — the SDK is co-generated locally
// //!     and regenerated in lockstep, so it can't drift from the CLI's spec.
// //!   - Auth, global headers, retries, timeouts, and TLS roots are INHERITED from
// //!     the CLI (shared execution), not re-derived into a second HTTP stack.
// //!   - SDK errors flow into `CliError` through `?` (the runtime provides the
// //!     bridge), so no per-call `.map_err(...)`.
// //!   - `Pet` / `CreatePetRequest` here are the SAME types the built-in `pets`
// //!     commands use — one canonical type identity, no `From`/`Into` shims.
// 
// use clap::{Arg, ArgMatches, Command};
// use fern_cli_sdk::app::CliApp;
// use fern_cli_sdk::error::CliError;
// use fern_cli_sdk::formatter::{self, OutputFormat};
// use fern_cli_sdk::openapi::{AppContext, OpenApiBinding};
// 
// // In the target, the co-generated SDK is a local crate emitted alongside the
// // CLI (same generation run, same spec). Its model/types are the canonical ones
// // the whole binary shares.
// use petstore::prelude::*;
// 
// /// Register the (illustrative) co-generated-SDK command set.
// ///
// /// Identical shape to the other modules — the difference is entirely in the
// /// handler bodies below.
// pub fn register(app: CliApp) -> CliApp {
//     app
//         .command(adopt_command(), OpenApiBinding::handler(handle_adopt))
//         .command_under(&["pets"], find_command(), OpenApiBinding::handler(handle_find))
//         .command_under(&["pets"], count_command(), OpenApiBinding::handler(handle_count))
// }
// 
// // ── `adopt` — chain create + get ────────────────────────────────────
// 
// fn handle_adopt(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
//     let name = matches.get_one::<String>("name").expect("required arg");
// 
//     // The whole "set up a client" step is one call. This client is already
//     // pointed at the CLI's resolved base URL and carries its auth, global
//     // headers, retries, and TLS — because it routes through the CLI's own
//     // executor. Nothing to configure, nothing that can drift.
//     let client = ctx.sdk_client();
// 
//     let request = CreatePetRequest {
//         name: name.clone(),
//         tag: matches.get_one::<String>("tag").cloned(),
//     };
// 
//     // Typed in, typed out. No find_method, no JSON round-trip. `ApiError`
//     // converts into `CliError` via the runtime's error bridge, so `?` works.
//     // `ctx.block_on` is the runtime-provided async→sync bridge (handlers are
//     // sync but run inside the CLI's tokio runtime).
//     let created: Pet = ctx.block_on(client.pets.create_pet(&request, None))?;
//     eprintln!("Created pet '{}' (id {})", created.name, created.id);
// 
//     let fetched: Pet = ctx.block_on(client.pets.get_pet(&created.id, None))?;
//     println!("{}", serde_json::to_string_pretty(&fetched).unwrap());
//     Ok(())
// }
// 
// // ── `pets find` — typed list + client-side filter ───────────────────
// 
// fn handle_find(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
//     let query = matches
//         .get_one::<String>("query")
//         .expect("required arg")
//         .to_lowercase();
// 
//     let client = ctx.sdk_client();
// 
//     let request = ListPetsQueryRequest {
//         limit: matches.get_one::<i64>("limit").copied(),
//     };
// 
//     let pets: Vec<Pet> = ctx.block_on(client.pets.list_pets(&request, None))?;
//     let matched: Vec<&Pet> = pets
//         .iter()
//         .filter(|p| p.name.to_lowercase().contains(&query))
//         .collect();
// 
//     println!("{}", serde_json::to_string_pretty(&matched).unwrap());
//     Ok(())
// }
// 
// // ── `pets count` — aggregate, honor --format ────────────────────────
// 
// fn handle_count(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
//     let client = ctx.sdk_client();
// 
//     let pets: Vec<Pet> = ctx.block_on(client.pets.list_pets(&ListPetsQueryRequest::default(), None))?;
// 
//     let format = matches
//         .get_one::<String>("format")
//         .map(|s| OutputFormat::from_str(s))
//         .unwrap_or_default();
//     let summary = serde_json::json!({ "count": pets.len() });
//     println!("{}", formatter::format_value(&summary, &format));
//     Ok(())
// }
// 
// // ── clap command definitions (unchanged from the other modules) ─────
// 
// fn adopt_command() -> Command {
//     Command::new("adopt")
//         .about("Create a pet and immediately fetch the stored record (co-generated SDK)")
//         .arg(Arg::new("name").required(true).help("Name of the pet to adopt"))
//         .arg(Arg::new("tag").long("tag").help("Optional tag/category for the pet"))
// }
// 
// fn find_command() -> Command {
//     Command::new("find")
//         .about("List pets and filter by a case-insensitive name substring (co-generated SDK)")
//         .arg(Arg::new("query").required(true).help("Substring to match against pet names"))
//         .arg(
//             Arg::new("limit")
//                 .long("limit")
//                 .value_parser(clap::value_parser!(i64))
//                 .help("Max pets to fetch from the API before filtering"),
//         )
// }
// 
// fn count_command() -> Command {
//     Command::new("count").about("Count the total number of pets (co-generated SDK)")
// }
