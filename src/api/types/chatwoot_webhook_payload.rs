pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootWebhookPayload {
    /// Chatwoot event name. Incoming visitor messages use `message_created`. Staff takeover and assignment use conversation events such as `conversation_opened`, `conversation_status_changed`, and `conversation_updated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    /// Message id on `message_*` events. Conversation display id on `conversation_*` events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Message text. Present on message events. Empty or private notes are ignored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Incoming visitor messages are `incoming` (or `0`). Staff replies are `outgoing` (or `1`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<String>,
    /// When true, the Agent ignores the message (Chatwoot private note).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<ChatwootWebhookPayloadAccount>,
    /// Present on message events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<ChatwootWebhookPayloadSender>,
    /// Nested on message events. Conversation events send `id`, `status`, and `meta` at the top level instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ChatwootWebhookPayloadConversation>,
    /// Top-level conversation status on conversation events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Top-level conversation meta on conversation events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<ChatwootConversationMeta>,
}

impl ChatwootWebhookPayload {
    pub fn builder() -> ChatwootWebhookPayloadBuilder {
        <ChatwootWebhookPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootWebhookPayloadBuilder {
    event: Option<String>,
    id: Option<i64>,
    content: Option<String>,
    message_type: Option<String>,
    private: Option<bool>,
    account: Option<ChatwootWebhookPayloadAccount>,
    sender: Option<ChatwootWebhookPayloadSender>,
    conversation: Option<ChatwootWebhookPayloadConversation>,
    status: Option<String>,
    meta: Option<ChatwootConversationMeta>,
}

impl ChatwootWebhookPayloadBuilder {
    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn message_type(mut self, value: impl Into<String>) -> Self {
        self.message_type = Some(value.into());
        self
    }

    pub fn private(mut self, value: bool) -> Self {
        self.private = Some(value);
        self
    }

    pub fn account(mut self, value: ChatwootWebhookPayloadAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn sender(mut self, value: ChatwootWebhookPayloadSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn conversation(mut self, value: ChatwootWebhookPayloadConversation) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn meta(mut self, value: ChatwootConversationMeta) -> Self {
        self.meta = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatwootWebhookPayload`].
    pub fn build(self) -> Result<ChatwootWebhookPayload, BuildError> {
        Ok(ChatwootWebhookPayload {
            event: self.event,
            id: self.id,
            content: self.content,
            message_type: self.message_type,
            private: self.private,
            account: self.account,
            sender: self.sender,
            conversation: self.conversation,
            status: self.status,
            meta: self.meta,
        })
    }
}
