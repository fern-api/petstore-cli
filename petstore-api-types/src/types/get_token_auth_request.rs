pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetTokenAuthRequest {
    /// OAuth2 client ID.
    #[serde(default)]
    pub client_id: String,
    /// OAuth2 client secret.
    #[serde(default)]
    pub client_secret: String,
}

impl GetTokenAuthRequest {
    pub fn builder() -> GetTokenAuthRequestBuilder {
        <GetTokenAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetTokenAuthRequestBuilder {
    client_id: Option<String>,
    client_secret: Option<String>,
}

impl GetTokenAuthRequestBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetTokenAuthRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](GetTokenAuthRequestBuilder::client_id)
    /// - [`client_secret`](GetTokenAuthRequestBuilder::client_secret)
    pub fn build(self) -> Result<GetTokenAuthRequest, BuildError> {
        Ok(GetTokenAuthRequest {
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
            client_secret: self.client_secret.ok_or_else(|| BuildError::missing_field("client_secret"))?,
        })
    }
}

