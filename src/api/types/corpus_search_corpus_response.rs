pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchCorpusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SearchCorpusResponse {
    pub fn builder() -> SearchCorpusResponseBuilder {
        <SearchCorpusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchCorpusResponseBuilder {
    results: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl SearchCorpusResponseBuilder {
    pub fn results(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchCorpusResponse`].
    pub fn build(self) -> Result<SearchCorpusResponse, BuildError> {
        Ok(SearchCorpusResponse {
            results: self.results,
        })
    }
}
