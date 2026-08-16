pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetConversationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Conversation>,
}

impl GetConversationResponse {
    pub fn builder() -> GetConversationResponseBuilder {
        <GetConversationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetConversationResponseBuilder {
    data: Option<Conversation>,
}

impl GetConversationResponseBuilder {
    pub fn data(mut self, value: Conversation) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetConversationResponse`].
    pub fn build(self) -> Result<GetConversationResponse, BuildError> {
        Ok(GetConversationResponse { data: self.data })
    }
}
