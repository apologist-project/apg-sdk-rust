pub use crate::prelude::*;

/// Query parameters for listChatCompletions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChatCompletionsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Results per page (clamped to 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bible_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flagged: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favorited: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liked: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<String>,
}

impl ListChatCompletionsQueryRequest {
    pub fn builder() -> ListChatCompletionsQueryRequestBuilder {
        <ListChatCompletionsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChatCompletionsQueryRequestBuilder {
    page: Option<i64>,
    per_page: Option<i64>,
    agent_id: Option<String>,
    channel_id: Option<String>,
    bible_id: Option<String>,
    cached: Option<String>,
    client: Option<String>,
    config_id: Option<String>,
    conversation_id: Option<String>,
    device_id: Option<String>,
    flagged: Option<String>,
    favorited: Option<String>,
    language: Option<String>,
    liked: Option<String>,
    session_id: Option<String>,
    user_id: Option<String>,
    min_timestamp: Option<String>,
    max_timestamp: Option<String>,
}

impl ListChatCompletionsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
        self
    }

    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn bible_id(mut self, value: impl Into<String>) -> Self {
        self.bible_id = Some(value.into());
        self
    }

    pub fn cached(mut self, value: impl Into<String>) -> Self {
        self.cached = Some(value.into());
        self
    }

    pub fn client(mut self, value: impl Into<String>) -> Self {
        self.client = Some(value.into());
        self
    }

    pub fn config_id(mut self, value: impl Into<String>) -> Self {
        self.config_id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn flagged(mut self, value: impl Into<String>) -> Self {
        self.flagged = Some(value.into());
        self
    }

    pub fn favorited(mut self, value: impl Into<String>) -> Self {
        self.favorited = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn liked(mut self, value: impl Into<String>) -> Self {
        self.liked = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn min_timestamp(mut self, value: impl Into<String>) -> Self {
        self.min_timestamp = Some(value.into());
        self
    }

    pub fn max_timestamp(mut self, value: impl Into<String>) -> Self {
        self.max_timestamp = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListChatCompletionsQueryRequest`].
    pub fn build(self) -> Result<ListChatCompletionsQueryRequest, BuildError> {
        Ok(ListChatCompletionsQueryRequest {
            page: self.page,
            per_page: self.per_page,
            agent_id: self.agent_id,
            channel_id: self.channel_id,
            bible_id: self.bible_id,
            cached: self.cached,
            client: self.client,
            config_id: self.config_id,
            conversation_id: self.conversation_id,
            device_id: self.device_id,
            flagged: self.flagged,
            favorited: self.favorited,
            language: self.language,
            liked: self.liked,
            session_id: self.session_id,
            user_id: self.user_id,
            min_timestamp: self.min_timestamp,
            max_timestamp: self.max_timestamp,
        })
    }
}
