pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetSharedMessagesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl GetSharedMessagesResponse {
    pub fn builder() -> GetSharedMessagesResponseBuilder {
        <GetSharedMessagesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetSharedMessagesResponseBuilder {
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl GetSharedMessagesResponseBuilder {
    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetSharedMessagesResponse`].
    pub fn build(self) -> Result<GetSharedMessagesResponse, BuildError> {
        Ok(GetSharedMessagesResponse {
            messages: self.messages,
        })
    }
}
