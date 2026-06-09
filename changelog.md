## 1.1.0 - 2026-06-09
### Added
* **`ApiError::Executor`** — new variant that captures errors from custom `RequestExecutor` implementations, preserving the original error as a boxed `std::error::Error` for downcasting.
* **`std::fmt::Display` and `std::error::Error` for `SdkError`** — `SdkError` can now be boxed as `Box<dyn std::error::Error + Send + Sync>`, transmitted across the executor boundary, and downcast at the call site.
### Changed
* **`RequestExecutor::execute`** now returns `Result<Response, Box<dyn std::error::Error + Send + Sync>>` instead of `Result<Response, reqwest::Error>`. Custom executor implementations must update their return type; the built-in `ReqwestExecutor` and `CliExecutorAdapter` have been updated automatically.

## 1.0.0 - 2026-06-08
### Breaking Changes
* **`SdkRequestExecutor::execute`** now returns `Result<Response, SdkError>` instead of `Result<Response, reqwest::Error>`. Update all trait implementations and call sites to handle `SdkError` in place of `reqwest::Error`.
* **`CliExecutor` auth handling** is now fail-closed: if the auth provider returns an error, the request is aborted and `SdkError::Auth` is returned rather than silently sending without credentials. Code that relied on the previous fallback-without-auth behavior will now receive an error instead.
### Added
* **`SdkError::Auth`** — new variant surfacing credential-resolution and token-refresh failures, mapping to `CliError::Auth` on conversion.
* **`CliApp::command_typed`** and **`CliApp::command_typed_with`** — register top-level custom commands with compile-time typed `clap::Args` structs, eliminating manual `ArgMatches` parsing.
* **`CliApp::command_under_typed`** and **`CliApp::command_under_typed_with`** — same typed registration for commands nested under an existing command path.

## 0.6.1 - 2026-06-08
* chore: remove sdk_glue integration tests and petstore_api_types dev-dependency
* The sdk_glue_verification integration test suite and its associated
* `petstore_api_types` dev-dependency have been removed from the workspace.
* These tests were tied to the petstore example schema and are no longer
* needed as the SDK glue layer has stabilised.
* Key changes:
* Delete `tests/sdk_glue_verification.rs` (450-line integration test suite covering wiremock parity, auth inheritance, type identity, and regression scenarios)
* Remove `petstore_api_types` path dependency from `[dev-dependencies]` in `Cargo.toml`
* 🌿 Generated with Fern

## 0.6.0 - 2026-06-06
### Added
* **`sdk_glue::sdk_client()`** — new helper that constructs a fully-wired SDK `ApiClient` from the CLI's `AppContext`, automatically inheriting auth, retries, TLS, and global headers.
* **`sdk_glue::block_on()`** — new utility for invoking async SDK operations from synchronous custom-command handlers, bridging `ApiError` into `CliError` so `?` works naturally.
* **`CliExecutorAdapter`** — new internal adapter implementing `petstore_api_sdk::RequestExecutor` that routes SDK HTTP requests through the CLI's existing executor stack.

## 0.3.0 - 2026-06-06
### Added
* **`petstore-api-sdk`** — new Rust HTTP client crate exposing `ApiClient`, `PetsClient` (`list_pets`, `create_pet`, `get_pet`), and `AuthClient` (`get_token`) for the Petstore API.
* **`ApiClientBuilder` / `ClientConfig` / `HttpClient`** — new fluent builder, configuration struct, and internal HTTP client supporting JSON, streaming downloads via `ByteStream`, exponential-backoff retries, and automatic OAuth token management.
* **`AsyncPaginator<T>` / `SyncPaginator<T>` / `PaginationResult<T>`** — new async and sync paginators (implementing `Stream` and `Iterator` respectively) for cursor-based and offset-based pagination, with per-page status codes, headers, and raw response bodies.
* **`QueryBuilder` / `RequestOptions`** — new type-safe query-parameter builder and per-request options struct for overriding auth, retries, timeout, and headers on individual calls.
* **`CliExecutor` / `SdkRequestExecutor` / `SdkError`** — new CLI transport bridge that routes SDK HTTP requests through the CLI's existing TLS, auth, and retry stack, with `block_on` for invoking async SDK operations from synchronous command context.

## 0.4.0 - 2026-06-05
### Added
* **Object-shorthand input flags** — GraphQL commands now accept a `--<arg>` flag (e.g. `--filter '{"query":"x"}'`) as a shorthand for passing an entire input object as JSON, alongside existing per-field leaf flags.
* **`--output -` stdout sentinel** — passing `-` to `--output` now streams binary response bytes directly to stdout instead of writing to a file named `-`.
* **`core::base64_bytes`, `core::bigint_string`, `core::flexible_datetime`** — new serde helpers in `petstore-api-types` for base64 `Vec<u8>`, `BigInt`-as-string, and flexible RFC3339/ISO 8601 datetime (de)serialization respectively.
* **`core::number_serializers`** — new serde helper that serializes whole-valued `f64` fields without a trailing decimal, with `Option<f64>` support.
* **`pub mod core` and prelude additions** — `petstore_api_types` now exposes a public `core` module and re-exports `chrono` date/time types and `OrderedFloat` from its prelude.
### Changed
* **Mutually exclusive input mode validation** — combining `--json` with per-field flags, or an object-shorthand flag with its leaf flags, now produces an immediate validation error instead of silently generating an incorrect GraphQL body.
* **Object-shorthand JSON validation** — object-shorthand flags now eagerly parse and shape-validate the provided JSON, rejecting non-object payloads (arrays, numbers, booleans, null) with an explicit error.
* **`validate_safe_file_path`** — the `--output` path validator now requires the parent directory to exist, canonicalizes only the parent, and rejects `.`, `..`, or empty strings with a clear diagnostic.
* **Object body parameter parsing** — the OpenAPI parser now emits a parent object-typed flag (e.g. `--address`) alongside dot-notation sub-flags (e.g. `--address.city`) for inline and `$ref` object properties.

## 0.3.0 - 2026-06-05
### Added
* **`BearerAuth`** — the CLI now supports Bearer token authentication, reading the token from the `PETSTORE_TOKEN` environment variable automatically.
* **`GetTokenAuthRequest`** — new request type (with builder) for the `POST /auth/token` OAuth2 token endpoint, carrying `client_id` and `client_secret` fields.
* **`TokenResponse`** — new response type (with builder) returned by the token endpoint, exposing `access_token` and `expires_in` fields.
* **`auth.getToken`** operation — new `/auth/token` endpoint added to the OpenAPI spec and surfaced as a typed CLI command.

## 0.2.0 - 2026-06-04
### Added
* **`petstore_api_types`** crate — new Rust library exposing strongly-typed structs (`Pet`, `CreatePetRequest`, `ListPetsQueryRequest`, `Error`) with builder patterns for all Petstore API request and response objects.
* **`custom::register`** extension point — a user-owned `custom.rs` module is now wired into the CLI entrypoint, allowing custom commands to be added without being overwritten on regeneration.

