//! Request and response types for the Petstore API
//!
//! This module contains all data structures used for API communication,
//! including request bodies, response types, and shared models.
//!
//! ## Type Categories
//!
//! - **Request/Response Types**: 4 types for API operations
//! - **Model Types**: 2 types for data representation

pub mod pet;
pub mod error_type;
pub mod token_response;
pub mod create_pet_request;
pub mod get_token_auth_request;
pub mod list_pets_query_request;

pub use pet::Pet;
pub use error_type::Error;
pub use token_response::TokenResponse;
pub use create_pet_request::CreatePetRequest;
pub use get_token_auth_request::GetTokenAuthRequest;
pub use list_pets_query_request::ListPetsQueryRequest;

