pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BenchmarkRunRequest {
    /// Content to evaluate. Required when `source_id` is supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<BenchmarkRunRequestContent>,
    /// Completion UUID whose stored response should be evaluated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_responses: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_question_variants: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<BenchmarkRunRequestReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<BenchmarkRunRequestVerbosity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_threshold: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
}

impl BenchmarkRunRequest {
    pub fn builder() -> BenchmarkRunRequestBuilder {
        <BenchmarkRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BenchmarkRunRequestBuilder {
    content: Option<BenchmarkRunRequestContent>,
    completion_id: Option<String>,
    source_id: Option<i64>,
    model: Option<String>,
    num_responses: Option<i64>,
    use_question_variants: Option<bool>,
    reasoning_effort: Option<BenchmarkRunRequestReasoningEffort>,
    verbosity: Option<BenchmarkRunRequestVerbosity>,
    score_threshold: Option<f64>,
    value_threshold: Option<f64>,
    temperature: Option<f64>,
    top_p: Option<f64>,
    frequency_penalty: Option<f64>,
    presence_penalty: Option<f64>,
}

impl BenchmarkRunRequestBuilder {
    pub fn content(mut self, value: BenchmarkRunRequestContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn completion_id(mut self, value: impl Into<String>) -> Self {
        self.completion_id = Some(value.into());
        self
    }

    pub fn source_id(mut self, value: i64) -> Self {
        self.source_id = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn num_responses(mut self, value: i64) -> Self {
        self.num_responses = Some(value);
        self
    }

    pub fn use_question_variants(mut self, value: bool) -> Self {
        self.use_question_variants = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: BenchmarkRunRequestReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn verbosity(mut self, value: BenchmarkRunRequestVerbosity) -> Self {
        self.verbosity = Some(value);
        self
    }

    pub fn score_threshold(mut self, value: f64) -> Self {
        self.score_threshold = Some(value);
        self
    }

    pub fn value_threshold(mut self, value: f64) -> Self {
        self.value_threshold = Some(value);
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

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BenchmarkRunRequest`].
    pub fn build(self) -> Result<BenchmarkRunRequest, BuildError> {
        Ok(BenchmarkRunRequest {
            content: self.content,
            completion_id: self.completion_id,
            source_id: self.source_id,
            model: self.model,
            num_responses: self.num_responses,
            use_question_variants: self.use_question_variants,
            reasoning_effort: self.reasoning_effort,
            verbosity: self.verbosity,
            score_threshold: self.score_threshold,
            value_threshold: self.value_threshold,
            temperature: self.temperature,
            top_p: self.top_p,
            frequency_penalty: self.frequency_penalty,
            presence_penalty: self.presence_penalty,
        })
    }
}
