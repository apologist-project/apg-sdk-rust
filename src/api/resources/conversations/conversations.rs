use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ConversationsClient {
    pub http_client: HttpClient,
}

impl ConversationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of conversations for the requesting agent, newest first.
    ///
    /// # Arguments
    ///
    /// * `per_page` - Results per page (clamped to 100).
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
    ///         .conversations
    ///         .list_conversations(
    ///             &ListConversationsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_conversations(
        &self,
        request: &ListConversationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListConversationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "conversations",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("per_page", request.per_page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a single conversation by internal UUID or team-scoped external id.
    ///
    /// # Arguments
    ///
    /// * `id` - The conversation UUID or team-scoped external id
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
    ///         .conversations
    ///         .get_conversation(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_conversation(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetConversationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("conversations/{}", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Pauses the agent on a conversation identified by internal UUID or team-scoped external id. Requires an API key.
    ///
    /// # Arguments
    ///
    /// * `id` - The conversation UUID or team-scoped external id
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
    ///         .conversations
    ///         .pause_conversation(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn pause_conversation(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<PauseConversationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("conversations/{}/pause", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Resumes the agent on a conversation identified by internal UUID or team-scoped external id. Requires an API key.
    ///
    /// # Arguments
    ///
    /// * `id` - The conversation UUID or team-scoped external id
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
    ///         .conversations
    ///         .resume_conversation(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn resume_conversation(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ResumeConversationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("conversations/{}/resume", id),
                None,
                None,
                options,
            )
            .await
    }
}
