pub use crate::prelude::*;

/// Agent-wide pause or resume result, including fan-out counts.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentPauseState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paused: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paused_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumed_at: Option<String>,
    /// Conversations that received a transition message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emitted: Option<i64>,
    /// Conversations skipped during fan-out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
}

impl AgentPauseState {
    pub fn builder() -> AgentPauseStateBuilder {
        <AgentPauseStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentPauseStateBuilder {
    is_paused: Option<bool>,
    paused_at: Option<String>,
    resumed_at: Option<String>,
    emitted: Option<i64>,
    skipped: Option<i64>,
}

impl AgentPauseStateBuilder {
    pub fn is_paused(mut self, value: bool) -> Self {
        self.is_paused = Some(value);
        self
    }

    pub fn paused_at(mut self, value: impl Into<String>) -> Self {
        self.paused_at = Some(value.into());
        self
    }

    pub fn resumed_at(mut self, value: impl Into<String>) -> Self {
        self.resumed_at = Some(value.into());
        self
    }

    pub fn emitted(mut self, value: i64) -> Self {
        self.emitted = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentPauseState`].
    pub fn build(self) -> Result<AgentPauseState, BuildError> {
        Ok(AgentPauseState {
            is_paused: self.is_paused,
            paused_at: self.paused_at,
            resumed_at: self.resumed_at,
            emitted: self.emitted,
            skipped: self.skipped,
        })
    }
}
