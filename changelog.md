## 0.2.0 - 2026-06-04
### Added
* **`petstore_api_types`** crate — new Rust library exposing strongly-typed structs (`Pet`, `CreatePetRequest`, `ListPetsQueryRequest`, `Error`) with builder patterns for all Petstore API request and response objects.
* **`custom::register`** extension point — a user-owned `custom.rs` module is now wired into the CLI entrypoint, allowing custom commands to be added without being overwritten on regeneration.

