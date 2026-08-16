pub use crate::prelude::*;

/// A conversation scoped to the requesting agent.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Conversation {
    /// Internal conversation id (UUID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Team-scoped external conversation id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_paused_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_resumed_at: Option<String>,
}

impl Conversation {
    pub fn builder() -> ConversationBuilder {
        <ConversationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConversationBuilder {
    id: Option<String>,
    external_id: Option<String>,
    agent_id: Option<i64>,
    team_id: Option<i64>,
    tags: Option<HashMap<String, serde_json::Value>>,
    started_at: Option<String>,
    ended_at: Option<String>,
    agent_paused: Option<bool>,
    agent_paused_at: Option<String>,
    agent_resumed_at: Option<String>,
}

impl ConversationBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn agent_id(mut self, value: i64) -> Self {
        self.agent_id = Some(value);
        self
    }

    pub fn team_id(mut self, value: i64) -> Self {
        self.team_id = Some(value);
        self
    }

    pub fn tags(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn started_at(mut self, value: impl Into<String>) -> Self {
        self.started_at = Some(value.into());
        self
    }

    pub fn ended_at(mut self, value: impl Into<String>) -> Self {
        self.ended_at = Some(value.into());
        self
    }

    pub fn agent_paused(mut self, value: bool) -> Self {
        self.agent_paused = Some(value);
        self
    }

    pub fn agent_paused_at(mut self, value: impl Into<String>) -> Self {
        self.agent_paused_at = Some(value.into());
        self
    }

    pub fn agent_resumed_at(mut self, value: impl Into<String>) -> Self {
        self.agent_resumed_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Conversation`].
    pub fn build(self) -> Result<Conversation, BuildError> {
        Ok(Conversation {
            id: self.id,
            external_id: self.external_id,
            agent_id: self.agent_id,
            team_id: self.team_id,
            tags: self.tags,
            started_at: self.started_at,
            ended_at: self.ended_at,
            agent_paused: self.agent_paused,
            agent_paused_at: self.agent_paused_at,
            agent_resumed_at: self.agent_resumed_at,
        })
    }
}
