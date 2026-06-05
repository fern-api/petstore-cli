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

