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

