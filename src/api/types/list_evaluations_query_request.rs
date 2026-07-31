pub use crate::prelude::*;

/// Query parameters for listEvaluations
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEvaluationsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// Results per page (clamped to 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_score: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_run_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark_question_id: Option<String>,
}

impl ListEvaluationsQueryRequest {
    pub fn builder() -> ListEvaluationsQueryRequestBuilder {
        <ListEvaluationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEvaluationsQueryRequestBuilder {
    page: Option<i64>,
    per_page: Option<i64>,
    min_timestamp: Option<String>,
    max_timestamp: Option<String>,
    min_duration: Option<String>,
    max_duration: Option<String>,
    min_score: Option<String>,
    max_score: Option<String>,
    passed: Option<String>,
    benchmark: Option<String>,
    benchmark_run_id: Option<String>,
    benchmark_question_id: Option<String>,
}

impl ListEvaluationsQueryRequestBuilder {
    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn per_page(mut self, value: i64) -> Self {
        self.per_page = Some(value);
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

    pub fn min_duration(mut self, value: impl Into<String>) -> Self {
        self.min_duration = Some(value.into());
        self
    }

    pub fn max_duration(mut self, value: impl Into<String>) -> Self {
        self.max_duration = Some(value.into());
        self
    }

    pub fn min_score(mut self, value: impl Into<String>) -> Self {
        self.min_score = Some(value.into());
        self
    }

    pub fn max_score(mut self, value: impl Into<String>) -> Self {
        self.max_score = Some(value.into());
        self
    }

    pub fn passed(mut self, value: impl Into<String>) -> Self {
        self.passed = Some(value.into());
        self
    }

    pub fn benchmark(mut self, value: impl Into<String>) -> Self {
        self.benchmark = Some(value.into());
        self
    }

    pub fn benchmark_run_id(mut self, value: impl Into<String>) -> Self {
        self.benchmark_run_id = Some(value.into());
        self
    }

    pub fn benchmark_question_id(mut self, value: impl Into<String>) -> Self {
        self.benchmark_question_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEvaluationsQueryRequest`].
    pub fn build(self) -> Result<ListEvaluationsQueryRequest, BuildError> {
        Ok(ListEvaluationsQueryRequest {
            page: self.page,
            per_page: self.per_page,
            min_timestamp: self.min_timestamp,
            max_timestamp: self.max_timestamp,
            min_duration: self.min_duration,
            max_duration: self.max_duration,
            min_score: self.min_score,
            max_score: self.max_score,
            passed: self.passed,
            benchmark: self.benchmark,
            benchmark_run_id: self.benchmark_run_id,
            benchmark_question_id: self.benchmark_question_id,
        })
    }
}
