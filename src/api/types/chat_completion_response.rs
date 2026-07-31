pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ChatCompletionResponseChoicesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ChatCompletionResponseUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached: Option<bool>,
}

impl ChatCompletionResponse {
    pub fn builder() -> ChatCompletionResponseBuilder {
        <ChatCompletionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionResponseBuilder {
    id: Option<String>,
    object: Option<String>,
    created: Option<i64>,
    model: Option<String>,
    choices: Option<Vec<ChatCompletionResponseChoicesItem>>,
    usage: Option<ChatCompletionResponseUsage>,
    cached: Option<bool>,
}

impl ChatCompletionResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn choices(mut self, value: Vec<ChatCompletionResponseChoicesItem>) -> Self {
        self.choices = Some(value);
        self
    }

    pub fn usage(mut self, value: ChatCompletionResponseUsage) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn cached(mut self, value: bool) -> Self {
        self.cached = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionResponse`].
    pub fn build(self) -> Result<ChatCompletionResponse, BuildError> {
        Ok(ChatCompletionResponse {
            id: self.id,
            object: self.object,
            created: self.created,
            model: self.model,
            choices: self.choices,
            usage: self.usage,
            cached: self.cached,
        })
    }
}
