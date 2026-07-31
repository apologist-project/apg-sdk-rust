pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FeedbackRequest {
    #[serde(default)]
    pub feedback: String,
}

impl FeedbackRequest {
    pub fn builder() -> FeedbackRequestBuilder {
        <FeedbackRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeedbackRequestBuilder {
    feedback: Option<String>,
}

impl FeedbackRequestBuilder {
    pub fn feedback(mut self, value: impl Into<String>) -> Self {
        self.feedback = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FeedbackRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`feedback`](FeedbackRequestBuilder::feedback)
    pub fn build(self) -> Result<FeedbackRequest, BuildError> {
        Ok(FeedbackRequest {
            feedback: self
                .feedback
                .ok_or_else(|| BuildError::missing_field("feedback"))?,
        })
    }
}
