pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PauseConversationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Conversation>,
}

impl PauseConversationResponse {
    pub fn builder() -> PauseConversationResponseBuilder {
        <PauseConversationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseConversationResponseBuilder {
    data: Option<Conversation>,
}

impl PauseConversationResponseBuilder {
    pub fn data(mut self, value: Conversation) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PauseConversationResponse`].
    pub fn build(self) -> Result<PauseConversationResponse, BuildError> {
        Ok(PauseConversationResponse { data: self.data })
    }
}
