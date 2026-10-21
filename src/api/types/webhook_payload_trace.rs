pub use crate::prelude::*;

/// OpenTelemetry context for the API request that produced this event. Present when the Agent created or continued a W3C trace.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WebhookPayloadTrace {
    /// 32-character hex trace id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// 16-character hex span id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

impl WebhookPayloadTrace {
    pub fn builder() -> WebhookPayloadTraceBuilder {
        <WebhookPayloadTraceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookPayloadTraceBuilder {
    trace_id: Option<String>,
    span_id: Option<String>,
}

impl WebhookPayloadTraceBuilder {
    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookPayloadTrace`].
    pub fn build(self) -> Result<WebhookPayloadTrace, BuildError> {
        Ok(WebhookPayloadTrace {
            trace_id: self.trace_id,
            span_id: self.span_id,
        })
    }
}
