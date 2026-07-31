pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetEvaluationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl GetEvaluationResponse {
    pub fn builder() -> GetEvaluationResponseBuilder {
        <GetEvaluationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetEvaluationResponseBuilder {
    data: Option<HashMap<String, serde_json::Value>>,
}

impl GetEvaluationResponseBuilder {
    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetEvaluationResponse`].
    pub fn build(self) -> Result<GetEvaluationResponse, BuildError> {
        Ok(GetEvaluationResponse { data: self.data })
    }
}
