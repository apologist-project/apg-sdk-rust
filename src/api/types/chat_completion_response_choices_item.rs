pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionResponseChoicesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
}

impl ChatCompletionResponseChoicesItem {
    pub fn builder() -> ChatCompletionResponseChoicesItemBuilder {
        <ChatCompletionResponseChoicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionResponseChoicesItemBuilder {
    index: Option<i64>,
    message: Option<ChatMessage>,
    logprobs: Option<HashMap<String, serde_json::Value>>,
    finish_reason: Option<String>,
}

impl ChatCompletionResponseChoicesItemBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn message(mut self, value: ChatMessage) -> Self {
        self.message = Some(value);
        self
    }

    pub fn logprobs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn finish_reason(mut self, value: impl Into<String>) -> Self {
        self.finish_reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionResponseChoicesItem`].
    pub fn build(self) -> Result<ChatCompletionResponseChoicesItem, BuildError> {
        Ok(ChatCompletionResponseChoicesItem {
            index: self.index,
            message: self.message,
            logprobs: self.logprobs,
            finish_reason: self.finish_reason,
        })
    }
}
