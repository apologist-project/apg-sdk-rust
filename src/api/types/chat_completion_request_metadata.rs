pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionRequestMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memories: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// Acquisition / campaign referral code stored on the user first-write-wins. Empty values are ignored; an existing user referral_code is never overwritten. The Agent UI maps ?ref=, then ?referral_code=, then ?utm_campaign= into this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_prompt: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<String>,
    /// String key/value pairs substituted into `{key}` placeholders in the assembled system prompt. Never persisted; omitted from response metadata. Reserved system keys (language, bible, translation, passages, date/geo tokens) cannot be overridden.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, Option<String>>>,
}

impl ChatCompletionRequestMetadata {
    pub fn builder() -> ChatCompletionRequestMetadataBuilder {
        <ChatCompletionRequestMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionRequestMetadataBuilder {
    anonymous: Option<bool>,
    conversation: Option<String>,
    language: Option<String>,
    max_memories: Option<i64>,
    parent_url: Option<String>,
    parent_host: Option<String>,
    session: Option<String>,
    device: Option<String>,
    referral_code: Option<String>,
    shared_prompt: Option<i64>,
    translation: Option<String>,
    variables: Option<HashMap<String, Option<String>>>,
}

impl ChatCompletionRequestMetadataBuilder {
    pub fn anonymous(mut self, value: bool) -> Self {
        self.anonymous = Some(value);
        self
    }

    pub fn conversation(mut self, value: impl Into<String>) -> Self {
        self.conversation = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn max_memories(mut self, value: i64) -> Self {
        self.max_memories = Some(value);
        self
    }

    pub fn parent_url(mut self, value: impl Into<String>) -> Self {
        self.parent_url = Some(value.into());
        self
    }

    pub fn parent_host(mut self, value: impl Into<String>) -> Self {
        self.parent_host = Some(value.into());
        self
    }

    pub fn session(mut self, value: impl Into<String>) -> Self {
        self.session = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn referral_code(mut self, value: impl Into<String>) -> Self {
        self.referral_code = Some(value.into());
        self
    }

    pub fn shared_prompt(mut self, value: i64) -> Self {
        self.shared_prompt = Some(value);
        self
    }

    pub fn translation(mut self, value: impl Into<String>) -> Self {
        self.translation = Some(value.into());
        self
    }

    pub fn variables(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionRequestMetadata`].
    pub fn build(self) -> Result<ChatCompletionRequestMetadata, BuildError> {
        Ok(ChatCompletionRequestMetadata {
            anonymous: self.anonymous,
            conversation: self.conversation,
            language: self.language,
            max_memories: self.max_memories,
            parent_url: self.parent_url,
            parent_host: self.parent_host,
            session: self.session,
            device: self.device,
            referral_code: self.referral_code,
            shared_prompt: self.shared_prompt,
            translation: self.translation,
            variables: self.variables,
        })
    }
}
