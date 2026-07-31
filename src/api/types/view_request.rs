pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ViewRequest {
    #[serde(default)]
    pub prompt_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl ViewRequest {
    pub fn builder() -> ViewRequestBuilder {
        <ViewRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ViewRequestBuilder {
    prompt_id: Option<String>,
    user_id: Option<String>,
}

impl ViewRequestBuilder {
    pub fn prompt_id(mut self, value: impl Into<String>) -> Self {
        self.prompt_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ViewRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt_id`](ViewRequestBuilder::prompt_id)
    pub fn build(self) -> Result<ViewRequest, BuildError> {
        Ok(ViewRequest {
            prompt_id: self
                .prompt_id
                .ok_or_else(|| BuildError::missing_field("prompt_id"))?,
            user_id: self.user_id,
        })
    }
}
