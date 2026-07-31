pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetChatCompletionResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,
}

impl GetChatCompletionResponse {
    pub fn builder() -> GetChatCompletionResponseBuilder {
        <GetChatCompletionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetChatCompletionResponseBuilder {
    data: Option<HashMap<String, serde_json::Value>>,
}

impl GetChatCompletionResponseBuilder {
    pub fn data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetChatCompletionResponse`].
    pub fn build(self) -> Result<GetChatCompletionResponse, BuildError> {
        Ok(GetChatCompletionResponse { data: self.data })
    }
}
