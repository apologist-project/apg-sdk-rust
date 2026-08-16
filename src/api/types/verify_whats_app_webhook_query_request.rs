pub use crate::prelude::*;

/// Query parameters for verifyWhatsAppWebhook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VerifyWhatsAppWebhookQueryRequest {
    #[serde(rename = "hub.mode")]
    pub hub_mode: VerifyWhatsAppWebhookRequestHubMode,
    #[serde(rename = "hub.verify_token")]
    #[serde(default)]
    pub hub_verify_token: String,
    #[serde(rename = "hub.challenge")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hub_challenge: Option<String>,
}

impl VerifyWhatsAppWebhookQueryRequest {
    pub fn builder() -> VerifyWhatsAppWebhookQueryRequestBuilder {
        <VerifyWhatsAppWebhookQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerifyWhatsAppWebhookQueryRequestBuilder {
    hub_mode: Option<VerifyWhatsAppWebhookRequestHubMode>,
    hub_verify_token: Option<String>,
    hub_challenge: Option<String>,
}

impl VerifyWhatsAppWebhookQueryRequestBuilder {
    pub fn hub_mode(mut self, value: VerifyWhatsAppWebhookRequestHubMode) -> Self {
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

    /// Consumes the builder and constructs a [`VerifyWhatsAppWebhookQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hub_mode`](VerifyWhatsAppWebhookQueryRequestBuilder::hub_mode)
    /// - [`hub_verify_token`](VerifyWhatsAppWebhookQueryRequestBuilder::hub_verify_token)
    pub fn build(self) -> Result<VerifyWhatsAppWebhookQueryRequest, BuildError> {
        Ok(VerifyWhatsAppWebhookQueryRequest {
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
