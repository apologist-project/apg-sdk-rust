use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AgentClient {
    pub http_client: HttpClient,
}

impl AgentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Pauses the agent globally and fans out pause transition messages to open conversations. Requires an API key.
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
    ///     client.agent.pause_agent(None).await;
    /// }
    /// ```
    pub async fn pause_agent(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<PauseAgentResponse, ApiError> {
        self.http_client
            .execute_request(Method::POST, "pause", None, None, options)
            .await
    }

    /// Resumes the agent globally and fans out resume transition messages to open conversations. Requires an API key.
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
    ///     client.agent.resume_agent(None).await;
    /// }
    /// ```
    pub async fn resume_agent(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<ResumeAgentResponse, ApiError> {
        self.http_client
            .execute_request(Method::POST, "resume", None, None, options)
            .await
    }
}
