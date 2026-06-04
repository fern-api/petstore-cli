pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Pet {
    /// Unique identifier for the pet.
    #[serde(default)]
    pub id: String,
    /// Name of the pet.
    #[serde(default)]
    pub name: String,
    /// Optional tag for the pet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl Pet {
    pub fn builder() -> PetBuilder {
        <PetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PetBuilder {
    id: Option<String>,
    name: Option<String>,
    tag: Option<String>,
}

impl PetBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn tag(mut self, value: impl Into<String>) -> Self {
        self.tag = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Pet`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PetBuilder::id)
    /// - [`name`](PetBuilder::name)
    pub fn build(self) -> Result<Pet, BuildError> {
        Ok(Pet {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            tag: self.tag,
        })
    }
}
