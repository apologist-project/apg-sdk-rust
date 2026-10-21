pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CtaMatchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// ISO language code, or `auto` to detect from the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    /// Chat messages in OpenAI format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    /// Flat string key/value pairs substituted into `{key}` placeholders in evaluator prompts run during CTA matching. Not persisted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, Option<String>>>,
}

impl CtaMatchRequest {
    pub fn builder() -> CtaMatchRequestBuilder {
        <CtaMatchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CtaMatchRequestBuilder {
    conversation: Option<String>,
    client: Option<String>,
    device: Option<String>,
    language: Option<String>,
    limit: Option<i64>,
    messages: Option<Vec<ChatMessage>>,
    session: Option<String>,
    user: Option<String>,
    variables: Option<HashMap<String, Option<String>>>,
}

impl CtaMatchRequestBuilder {
    pub fn conversation(mut self, value: impl Into<String>) -> Self {
        self.conversation = Some(value.into());
        self
    }

    pub fn client(mut self, value: impl Into<String>) -> Self {
        self.client = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ChatMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn session(mut self, value: impl Into<String>) -> Self {
        self.session = Some(value.into());
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn variables(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CtaMatchRequest`].
    pub fn build(self) -> Result<CtaMatchRequest, BuildError> {
        Ok(CtaMatchRequest {
            conversation: self.conversation,
            client: self.client,
            device: self.device,
            language: self.language,
            limit: self.limit,
            messages: self.messages,
            session: self.session,
            user: self.user,
            variables: self.variables,
        })
    }
}
