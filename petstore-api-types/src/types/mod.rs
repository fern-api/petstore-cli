//! Request and response types for the Petstore API
//!
//! This module contains all data structures used for API communication,
//! including request bodies, response types, and shared models.
//!
//! ## Type Categories
//!
//! - **Request/Response Types**: 2 types for API operations
//! - **Model Types**: 2 types for data representation

pub mod pet;
pub mod error_type;
pub mod create_pet_request;
pub mod list_pets_query_request;

pub use pet::Pet;
pub use error_type::Error;
pub use create_pet_request::CreatePetRequest;
pub use list_pets_query_request::ListPetsQueryRequest;

