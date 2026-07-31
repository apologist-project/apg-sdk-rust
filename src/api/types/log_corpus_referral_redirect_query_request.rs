pub use crate::prelude::*;

/// Query parameters for logCorpusReferralRedirect
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LogCorpusReferralRedirectQueryRequest {
    #[serde(default)]
    pub prompt_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// URL-encoded destination to redirect to after logging the referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl LogCorpusReferralRedirectQueryRequest {
    pub fn builder() -> LogCorpusReferralRedirectQueryRequestBuilder {
        <LogCorpusReferralRedirectQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogCorpusReferralRedirectQueryRequestBuilder {
    prompt_id: Option<String>,
    user_id: Option<String>,
    url: Option<String>,
}

impl LogCorpusReferralRedirectQueryRequestBuilder {
    pub fn prompt_id(mut self, value: impl Into<String>) -> Self {
        self.prompt_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LogCorpusReferralRedirectQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`prompt_id`](LogCorpusReferralRedirectQueryRequestBuilder::prompt_id)
    pub fn build(self) -> Result<LogCorpusReferralRedirectQueryRequest, BuildError> {
        Ok(LogCorpusReferralRedirectQueryRequest {
            prompt_id: self
                .prompt_id
                .ok_or_else(|| BuildError::missing_field("prompt_id"))?,
            user_id: self.user_id,
            url: self.url,
        })
    }
}
