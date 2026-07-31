pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CtaClickRequest {
    #[serde(default)]
    pub prompt_id: String,
}

impl CtaClickRequest {
    pub fn builder() -> CtaClickRequestBuilder {
        <CtaClickRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CtaClickRequestBuilder {
    prompt_id: Option<String>,
}

impl CtaClickRequestBuilder {
    pub fn prompt_id(mut self, value: impl Into<String>) -> Self {
        self.prompt_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CtaClickRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt_id`](CtaClickRequestBuilder::prompt_id)
    pub fn build(self) -> Result<CtaClickRequest, BuildError> {
        Ok(CtaClickRequest {
            prompt_id: self
                .prompt_id
                .ok_or_else(|| BuildError::missing_field("prompt_id"))?,
        })
    }
}
