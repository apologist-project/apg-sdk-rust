pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetChatwootChannelStatusResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

impl GetChatwootChannelStatusResponse {
    pub fn builder() -> GetChatwootChannelStatusResponseBuilder {
        <GetChatwootChannelStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetChatwootChannelStatusResponseBuilder {
    status: Option<String>,
    channel: Option<String>,
    active: Option<bool>,
}

impl GetChatwootChannelStatusResponseBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn channel(mut self, value: impl Into<String>) -> Self {
        self.channel = Some(value.into());
        self
    }

    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetChatwootChannelStatusResponse`].
    pub fn build(self) -> Result<GetChatwootChannelStatusResponse, BuildError> {
        Ok(GetChatwootChannelStatusResponse {
            status: self.status,
            channel: self.channel,
            active: self.active,
        })
    }
}
