use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct CtAsClient {
    pub http_client: HttpClient,
}

impl CtAsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Finds matching CTAs based on conversation context, user, session, device, or messages
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apologist::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApologistAgentClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ct_as
    ///         .match_ctas(
    ///             &CtaMatchRequest::Unknown(serde_json::json!({"key":"value"})),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn match_ctas(
        &self,
        request: &CtaMatchRequest,
        options: Option<RequestOptions>,
    ) -> Result<MatchCtasResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "ctas/match",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records that a user clicked on a specific CTA
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the CTA
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apologist::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApologistAgentClient::new(config).expect("Failed to build client");
    ///     client
    ///         .ct_as
    ///         .log_cta_click(
    ///             &"id".to_string(),
    ///             &CtaClickRequest {
    ///                 prompt_id: "prompt_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn log_cta_click(
        &self,
        id: &str,
        request: &CtaClickRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("ctas/{}/click", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
