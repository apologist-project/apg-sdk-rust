pub use crate::prelude::*;

/// Result of scrubbing or anonymizing a user's message-adjacent text. Rows and identifiers are kept.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserRedactResponse {
    /// Internal user id (UUID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<UserRedactResponseMode>,
    /// When the erase request was stamped. The hourly cron finishes leftover rows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redact_requested_at: Option<String>,
    /// Message rows rewritten in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_redacted: Option<i64>,
    /// Message rows still waiting. Zero means this request finished the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl UserRedactResponse {
    pub fn builder() -> UserRedactResponseBuilder {
        <UserRedactResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserRedactResponseBuilder {
    id: Option<String>,
    mode: Option<UserRedactResponseMode>,
    redact_requested_at: Option<String>,
    messages_redacted: Option<i64>,
    remaining: Option<i64>,
}

impl UserRedactResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: UserRedactResponseMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn redact_requested_at(mut self, value: impl Into<String>) -> Self {
        self.redact_requested_at = Some(value.into());
        self
    }

    pub fn messages_redacted(mut self, value: i64) -> Self {
        self.messages_redacted = Some(value);
        self
    }

    pub fn remaining(mut self, value: i64) -> Self {
        self.remaining = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserRedactResponse`].
    pub fn build(self) -> Result<UserRedactResponse, BuildError> {
        Ok(UserRedactResponse {
            id: self.id,
            mode: self.mode,
            redact_requested_at: self.redact_requested_at,
            messages_redacted: self.messages_redacted,
            remaining: self.remaining,
        })
    }
}
