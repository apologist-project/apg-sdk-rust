pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootConversationMeta {
    /// `AgentBot` while the Agent owns the thread. `User` after staff take over.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<ChatwootConversationMetaSender>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ChatwootConversationMeta {
    pub fn builder() -> ChatwootConversationMetaBuilder {
        <ChatwootConversationMetaBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootConversationMetaBuilder {
    assignee_type: Option<String>,
    sender: Option<ChatwootConversationMetaSender>,
}

impl ChatwootConversationMetaBuilder {
    pub fn assignee_type(mut self, value: impl Into<String>) -> Self {
        self.assignee_type = Some(value.into());
        self
    }

    pub fn sender(mut self, value: ChatwootConversationMetaSender) -> Self {
        self.sender = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatwootConversationMeta`].
    pub fn build(self) -> Result<ChatwootConversationMeta, BuildError> {
        Ok(ChatwootConversationMeta {
            assignee_type: self.assignee_type,
            sender: self.sender,
            extra: Default::default(),
        })
    }
}
