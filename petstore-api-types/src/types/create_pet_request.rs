pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePetRequest {
    /// Name of the pet.
    #[serde(default)]
    pub name: String,
    /// Optional tag for the pet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl CreatePetRequest {
    pub fn builder() -> CreatePetRequestBuilder {
        <CreatePetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePetRequestBuilder {
    name: Option<String>,
    tag: Option<String>,
}

impl CreatePetRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePetRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreatePetRequestBuilder::name)
    pub fn build(self) -> Result<CreatePetRequest, BuildError> {
        Ok(CreatePetRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tag: self.tag,
        })
    }
}

