pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootWebhookPayloadAccount {
    /// Must match the channel Account ID, or the request is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ChatwootWebhookPayloadAccount {
    pub fn builder() -> ChatwootWebhookPayloadAccountBuilder {
        <ChatwootWebhookPayloadAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootWebhookPayloadAccountBuilder {
    id: Option<i64>,
}

impl ChatwootWebhookPayloadAccountBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatwootWebhookPayloadAccount`].
    pub fn build(self) -> Result<ChatwootWebhookPayloadAccount, BuildError> {
        Ok(ChatwootWebhookPayloadAccount {
            id: self.id,
            extra: Default::default(),
        })
    }
}
