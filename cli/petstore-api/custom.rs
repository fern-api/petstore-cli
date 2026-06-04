//! Custom command **registration**.
//!
//! This file is yours to edit — it is listed in `.fernignore` so
//! `fern generate` will never overwrite your changes.
//!
//! The generated `main.rs` calls `custom::register(app)` at startup,
//! composing your commands into the CLI at compile time. This module
//! only wires commands in; the actual implementations live in sibling
//! modules so each approach stays self-contained:
//!
//! * [`commands_native`] — built on the CLI's native runtime
//!   ([`AppContext::invoke`](fern_cli_sdk::openapi::AppContext)), reusing
//!   the generated executor's auth, retries, and base-URL resolution.
//! * [`commands_sdk`] — built on the generated `petstore` Rust SDK
//!   (a git dependency declared in `Cargo.toml`), constructing a typed
//!   client and calling its async methods.
//!
//! Both register the same trio of commands (create+get chain, filtered
//! list, count); the SDK-backed set uses a `-sdk` suffix so the two
//! coexist side by side for comparison.

mod commands_native;
mod commands_sdk;

use fern_cli_sdk::app::CliApp;

/// Register all custom commands on the CLI app builder.
pub fn register(app: CliApp) -> CliApp {
    let app = commands_native::register(app);
    commands_sdk::register(app)
}
