pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvaluatorRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence_threshold: Option<f64>,
    pub content: EvaluatorRequestContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<EvaluatorRequestReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<EvaluatorRequestVerbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Flat string key/value pairs substituted into `{key}` placeholders in the evaluator prompt. Reserved keys (`options`, `option_descriptions`, `criteria`) cannot be overridden. Not persisted; omitted from the response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, Option<String>>>,
}

impl EvaluatorRequest {
    pub fn builder() -> EvaluatorRequestBuilder {
        <EvaluatorRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluatorRequestBuilder {
    frequency_penalty: Option<f64>,
    confidence_threshold: Option<f64>,
    content: Option<EvaluatorRequestContent>,
    model: Option<String>,
    presence_penalty: Option<f64>,
    reasoning_effort: Option<EvaluatorRequestReasoningEffort>,
    verbosity: Option<EvaluatorRequestVerbosity>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    variables: Option<HashMap<String, Option<String>>>,
}

impl EvaluatorRequestBuilder {
    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn confidence_threshold(mut self, value: f64) -> Self {
        self.confidence_threshold = Some(value);
        self
    }

    pub fn content(mut self, value: EvaluatorRequestContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: EvaluatorRequestReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn verbosity(mut self, value: EvaluatorRequestVerbosity) -> Self {
        self.verbosity = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn variables(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.variables = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluatorRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](EvaluatorRequestBuilder::content)
    pub fn build(self) -> Result<EvaluatorRequest, BuildError> {
        Ok(EvaluatorRequest {
            frequency_penalty: self.frequency_penalty,
            confidence_threshold: self.confidence_threshold,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            model: self.model,
            presence_penalty: self.presence_penalty,
            reasoning_effort: self.reasoning_effort,
            verbosity: self.verbosity,
            temperature: self.temperature,
            top_p: self.top_p,
            variables: self.variables,
        })
    }
}
