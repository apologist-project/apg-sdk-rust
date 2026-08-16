pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResumeConversationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Conversation>,
}

impl ResumeConversationResponse {
    pub fn builder() -> ResumeConversationResponseBuilder {
        <ResumeConversationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeConversationResponseBuilder {
    data: Option<Conversation>,
}

impl ResumeConversationResponseBuilder {
    pub fn data(mut self, value: Conversation) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResumeConversationResponse`].
    pub fn build(self) -> Result<ResumeConversationResponse, BuildError> {
        Ok(ResumeConversationResponse { data: self.data })
    }
}
