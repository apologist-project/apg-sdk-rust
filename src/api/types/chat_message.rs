pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChatMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ChatMessageRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl ChatMessage {
    pub fn builder() -> ChatMessageBuilder {
        <ChatMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatMessageBuilder {
    role: Option<ChatMessageRole>,
    content: Option<String>,
}

impl ChatMessageBuilder {
    pub fn role(mut self, value: ChatMessageRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatMessage`].
    pub fn build(self) -> Result<ChatMessage, BuildError> {
        Ok(ChatMessage {
            role: self.role,
            content: self.content,
        })
    }
}
