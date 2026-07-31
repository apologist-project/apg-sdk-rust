pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CorpusSearchRequest {
    #[serde(default)]
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<CorpusSearchRequestFilters>,
}

impl CorpusSearchRequest {
    pub fn builder() -> CorpusSearchRequestBuilder {
        <CorpusSearchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CorpusSearchRequestBuilder {
    query: Option<String>,
    prompt_id: Option<String>,
    limit: Option<i64>,
    filters: Option<CorpusSearchRequestFilters>,
}

impl CorpusSearchRequestBuilder {
    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn prompt_id(mut self, value: impl Into<String>) -> Self {
        self.prompt_id = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn filters(mut self, value: CorpusSearchRequestFilters) -> Self {
        self.filters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CorpusSearchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`query`](CorpusSearchRequestBuilder::query)
    pub fn build(self) -> Result<CorpusSearchRequest, BuildError> {
        Ok(CorpusSearchRequest {
            query: self
                .query
                .ok_or_else(|| BuildError::missing_field("query"))?,
            prompt_id: self.prompt_id,
            limit: self.limit,
            filters: self.filters,
        })
    }
}
