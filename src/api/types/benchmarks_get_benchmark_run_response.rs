pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetBenchmarkRunResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl GetBenchmarkRunResponse {
    pub fn builder() -> GetBenchmarkRunResponseBuilder {
        <GetBenchmarkRunResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBenchmarkRunResponseBuilder {
    data: Option<HashMap<String, serde_json::Value>>,
}

impl GetBenchmarkRunResponseBuilder {
    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBenchmarkRunResponse`].
    pub fn build(self) -> Result<GetBenchmarkRunResponse, BuildError> {
        Ok(GetBenchmarkRunResponse { data: self.data })
    }
}
