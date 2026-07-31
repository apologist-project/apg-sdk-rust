pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiveTwilioMessageRequest {
    #[serde(rename = "From")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "Body")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

impl ReceiveTwilioMessageRequest {
    pub fn builder() -> ReceiveTwilioMessageRequestBuilder {
        <ReceiveTwilioMessageRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiveTwilioMessageRequestBuilder {
    from: Option<String>,
    body: Option<String>,
}

impl ReceiveTwilioMessageRequestBuilder {
    pub fn from(mut self, value: impl Into<String>) -> Self {
        self.from = Some(value.into());
        self
    }

    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiveTwilioMessageRequest`].
    pub fn build(self) -> Result<ReceiveTwilioMessageRequest, BuildError> {
        Ok(ReceiveTwilioMessageRequest {
            from: self.from,
            body: self.body,
        })
    }
}
