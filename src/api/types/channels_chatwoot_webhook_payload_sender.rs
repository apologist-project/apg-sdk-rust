pub use crate::prelude::*;

/// Present on message events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootWebhookPayloadSender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// `contact` for visitors, `user` for staff, `agent_bot` for this Agent. The Agent ignores its own outgoing messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ChatwootWebhookPayloadSender {
    pub fn builder() -> ChatwootWebhookPayloadSenderBuilder {
        <ChatwootWebhookPayloadSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootWebhookPayloadSenderBuilder {
    id: Option<i64>,
    name: Option<String>,
    r#type: Option<String>,
}

impl ChatwootWebhookPayloadSenderBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatwootWebhookPayloadSender`].
    pub fn build(self) -> Result<ChatwootWebhookPayloadSender, BuildError> {
        Ok(ChatwootWebhookPayloadSender {
            id: self.id,
            name: self.name,
            r#type: self.r#type,
            extra: Default::default(),
        })
    }
}
