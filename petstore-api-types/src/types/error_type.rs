pub use crate::prelude::*;
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Error {
    /// Error code.
    #[serde(default)]
    pub code: i64,
    /// Error message.
    #[serde(default)]
    pub message: String,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    code: Option<i64>,
    message: Option<String>,
}

impl ErrorBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ErrorBuilder::code)
    /// - [`message`](ErrorBuilder::message)
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
