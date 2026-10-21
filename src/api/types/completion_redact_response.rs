pub use crate::prelude::*;

/// Result of scrubbing or anonymizing a chat completion's message-adjacent text. Rows and identifiers are kept.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompletionRedactResponse {
    /// Chat completion id (UUID).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<CompletionRedactResponseMode>,
    /// Message rows rewritten in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages_redacted: Option<i64>,
    /// Message rows still waiting. Zero means this request finished the completion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining: Option<i64>,
}

impl CompletionRedactResponse {
    pub fn builder() -> CompletionRedactResponseBuilder {
        <CompletionRedactResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionRedactResponseBuilder {
    id: Option<String>,
    mode: Option<CompletionRedactResponseMode>,
    messages_redacted: Option<i64>,
    remaining: Option<i64>,
}

impl CompletionRedactResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mode(mut self, value: CompletionRedactResponseMode) -> Self {
        self.mode = Some(value);
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

    /// Consumes the builder and constructs a [`CompletionRedactResponse`].
    pub fn build(self) -> Result<CompletionRedactResponse, BuildError> {
        Ok(CompletionRedactResponse {
            id: self.id,
            mode: self.mode,
            messages_redacted: self.messages_redacted,
            remaining: self.remaining,
        })
    }
}
