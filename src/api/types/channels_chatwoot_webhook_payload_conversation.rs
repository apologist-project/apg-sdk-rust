pub use crate::prelude::*;

/// Nested on message events. Conversation events send `id`, `status`, and `meta` at the top level instead.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootWebhookPayloadConversation {
    /// Conversation display id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// `pending`, `open`, `resolved`, or `snoozed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<ChatwootWebhookPayloadConversationAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ChatwootConversationMeta>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ChatwootWebhookPayloadConversation {
    pub fn builder() -> ChatwootWebhookPayloadConversationBuilder {
        <ChatwootWebhookPayloadConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootWebhookPayloadConversationBuilder {
    id: Option<i64>,
    status: Option<String>,
    account: Option<ChatwootWebhookPayloadConversationAccount>,
    meta: Option<ChatwootConversationMeta>,
}

impl ChatwootWebhookPayloadConversationBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn account(mut self, value: ChatwootWebhookPayloadConversationAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn meta(mut self, value: ChatwootConversationMeta) -> Self {
        self.meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatwootWebhookPayloadConversation`].
    pub fn build(self) -> Result<ChatwootWebhookPayloadConversation, BuildError> {
        Ok(ChatwootWebhookPayloadConversation {
            id: self.id,
            status: self.status,
            account: self.account,
            meta: self.meta,
            extra: Default::default(),
        })
    }
}
