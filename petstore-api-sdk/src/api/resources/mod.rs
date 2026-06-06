//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Pets**
//! - **Auth**

use crate::{ApiError, ClientConfig};

pub mod auth;
pub mod pets;
pub struct ApiClient {
    pub config: ClientConfig,
    pub pets: PetsClient,
    pub auth: AuthClient,
}

impl ApiClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            pets: PetsClient::new(config.clone())?,
            auth: AuthClient::new(config.clone())?,
        })
    }
}

pub use auth::AuthClient;
pub use pets::PetsClient;
