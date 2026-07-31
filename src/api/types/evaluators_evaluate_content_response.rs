pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EvaluateContentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<HashMap<String, serde_json::Value>>,
}

impl EvaluateContentResponse {
    pub fn builder() -> EvaluateContentResponseBuilder {
        <EvaluateContentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluateContentResponseBuilder {
    result: Option<HashMap<String, serde_json::Value>>,
}

impl EvaluateContentResponseBuilder {
    pub fn result(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.result = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluateContentResponse`].
    pub fn build(self) -> Result<EvaluateContentResponse, BuildError> {
        Ok(EvaluateContentResponse {
            result: self.result,
        })
    }
}
