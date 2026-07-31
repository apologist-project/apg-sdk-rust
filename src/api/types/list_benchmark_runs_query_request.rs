pub use crate::prelude::*;

/// Query parameters for listBenchmarkRuns
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListBenchmarkRunsQueryRequest {
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
    pub min_responses: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_responses: Option<String>,
}

impl ListBenchmarkRunsQueryRequest {
    pub fn builder() -> ListBenchmarkRunsQueryRequestBuilder {
        <ListBenchmarkRunsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBenchmarkRunsQueryRequestBuilder {
    page: Option<i64>,
    per_page: Option<i64>,
    min_timestamp: Option<String>,
    max_timestamp: Option<String>,
    min_duration: Option<String>,
    max_duration: Option<String>,
    min_score: Option<String>,
    max_score: Option<String>,
    passed: Option<String>,
    min_responses: Option<String>,
    max_responses: Option<String>,
}

impl ListBenchmarkRunsQueryRequestBuilder {
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

    pub fn min_responses(mut self, value: impl Into<String>) -> Self {
        self.min_responses = Some(value.into());
        self
    }

    pub fn max_responses(mut self, value: impl Into<String>) -> Self {
        self.max_responses = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListBenchmarkRunsQueryRequest`].
    pub fn build(self) -> Result<ListBenchmarkRunsQueryRequest, BuildError> {
        Ok(ListBenchmarkRunsQueryRequest {
            page: self.page,
            per_page: self.per_page,
            min_timestamp: self.min_timestamp,
            max_timestamp: self.max_timestamp,
            min_duration: self.min_duration,
            max_duration: self.max_duration,
            min_score: self.min_score,
            max_score: self.max_score,
            passed: self.passed,
            min_responses: self.min_responses,
            max_responses: self.max_responses,
        })
    }
}
