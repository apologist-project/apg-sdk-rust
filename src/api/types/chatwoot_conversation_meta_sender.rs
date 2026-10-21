pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatwootConversationMetaSender {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl ChatwootConversationMetaSender {
    pub fn builder() -> ChatwootConversationMetaSenderBuilder {
        <ChatwootConversationMetaSenderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatwootConversationMetaSenderBuilder {
    id: Option<i64>,
    name: Option<String>,
}

impl ChatwootConversationMetaSenderBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatwootConversationMetaSender`].
    pub fn build(self) -> Result<ChatwootConversationMetaSender, BuildError> {
        Ok(ChatwootConversationMetaSender {
            id: self.id,
            name: self.name,
            extra: Default::default(),
        })
    }
}
