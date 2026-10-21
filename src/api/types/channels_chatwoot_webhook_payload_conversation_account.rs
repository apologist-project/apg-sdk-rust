pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootWebhookPayloadConversationAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ChatwootWebhookPayloadConversationAccount {
    pub fn builder() -> ChatwootWebhookPayloadConversationAccountBuilder {
        <ChatwootWebhookPayloadConversationAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootWebhookPayloadConversationAccountBuilder {
    id: Option<i64>,
}

impl ChatwootWebhookPayloadConversationAccountBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatwootWebhookPayloadConversationAccount`].
    pub fn build(self) -> Result<ChatwootWebhookPayloadConversationAccount, BuildError> {
        Ok(ChatwootWebhookPayloadConversationAccount {
            id: self.id,
            extra: Default::default(),
        })
    }
}
