use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SharesClient {
    pub http_client: HttpClient,
}

impl SharesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Public, unauthenticated read of the messages behind a share token. The token is the bearer capability and enforces tenant isolation against the host agent. An empty or invalid token yields an empty messages array.
    ///
    /// # Arguments
    ///
    /// * `token` - The share token
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
    ///         .shares
    ///         .get_shared_messages(&"token".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_shared_messages(
        &self,
        token: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetSharedMessagesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("shares/{}", token),
                None,
                None,
                options,
            )
            .await
    }
}
