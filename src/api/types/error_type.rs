pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Error {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub errors: Vec<String>,
}

impl Error {
    pub fn builder() -> ErrorBuilder {
        <ErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorBuilder {
    success: Option<bool>,
    errors: Option<Vec<String>>,
}

impl ErrorBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Error`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](ErrorBuilder::success)
    /// - [`errors`](ErrorBuilder::errors)
    pub fn build(self) -> Result<Error, BuildError> {
        Ok(Error {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
        })
    }
}
