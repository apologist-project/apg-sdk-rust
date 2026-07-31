pub use crate::prelude::*;

/// Query parameters for verifyFacebookWebhook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VerifyFacebookWebhookQueryRequest {
    #[serde(rename = "hub.mode")]
    pub hub_mode: VerifyFacebookWebhookRequestHubMode,
    #[serde(rename = "hub.verify_token")]
    #[serde(default)]
    pub hub_verify_token: String,
    #[serde(rename = "hub.challenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_challenge: Option<String>,
}

impl VerifyFacebookWebhookQueryRequest {
    pub fn builder() -> VerifyFacebookWebhookQueryRequestBuilder {
        <VerifyFacebookWebhookQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerifyFacebookWebhookQueryRequestBuilder {
    hub_mode: Option<VerifyFacebookWebhookRequestHubMode>,
    hub_verify_token: Option<String>,
    hub_challenge: Option<String>,
}

impl VerifyFacebookWebhookQueryRequestBuilder {
    pub fn hub_mode(mut self, value: VerifyFacebookWebhookRequestHubMode) -> Self {
        self.hub_mode = Some(value);
        self
    }

    pub fn hub_verify_token(mut self, value: impl Into<String>) -> Self {
        self.hub_verify_token = Some(value.into());
        self
    }

    pub fn hub_challenge(mut self, value: impl Into<String>) -> Self {
        self.hub_challenge = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerifyFacebookWebhookQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hub_mode`](VerifyFacebookWebhookQueryRequestBuilder::hub_mode)
    /// - [`hub_verify_token`](VerifyFacebookWebhookQueryRequestBuilder::hub_verify_token)
    pub fn build(self) -> Result<VerifyFacebookWebhookQueryRequest, BuildError> {
        Ok(VerifyFacebookWebhookQueryRequest {
            hub_mode: self
                .hub_mode
                .ok_or_else(|| BuildError::missing_field("hub_mode"))?,
            hub_verify_token: self
                .hub_verify_token
                .ok_or_else(|| BuildError::missing_field("hub_verify_token"))?,
            hub_challenge: self.hub_challenge,
        })
    }
}
