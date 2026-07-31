pub use crate::prelude::*;

/// Result of an evaluation run for CTA/guardrail events.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WebhookEvaluation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl WebhookEvaluation {
    pub fn builder() -> WebhookEvaluationBuilder {
        <WebhookEvaluationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WebhookEvaluationBuilder {
    score: Option<f64>,
    passed: Option<bool>,
    content: Option<String>,
}

impl WebhookEvaluationBuilder {
    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn passed(mut self, value: bool) -> Self {
        self.passed = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WebhookEvaluation`].
    pub fn build(self) -> Result<WebhookEvaluation, BuildError> {
        Ok(WebhookEvaluation {
            score: self.score,
            passed: self.passed,
            content: self.content,
            extra: Default::default(),
        })
    }
}
