pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MatchCtasResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctas: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl MatchCtasResponse {
    pub fn builder() -> MatchCtasResponseBuilder {
        <MatchCtasResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MatchCtasResponseBuilder {
    ctas: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl MatchCtasResponseBuilder {
    pub fn ctas(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.ctas = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MatchCtasResponse`].
    pub fn build(self) -> Result<MatchCtasResponse, BuildError> {
        Ok(MatchCtasResponse { ctas: self.ctas })
    }
}
