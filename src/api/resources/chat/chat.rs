use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ChatClient {
    pub http_client: HttpClient,
}

impl ChatClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of chat completions (prompts) for the agent, with applied tags expanded as { id, name } and share metadata.
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
    ///         .chat
    ///         .list_chat_completions(
    ///             &ListChatCompletionsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_chat_completions(
        &self,
        request: &ListChatCompletionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListChatCompletionsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "chat/completions",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("per_page", request.per_page.clone())
                    .string("agent_id", request.agent_id.clone())
                    .string("channel_id", request.channel_id.clone())
                    .string("bible_id", request.bible_id.clone())
                    .string("cached", request.cached.clone())
                    .string("client", request.client.clone())
                    .string("config_id", request.config_id.clone())
                    .string("conversation_id", request.conversation_id.clone())
                    .string("device_id", request.device_id.clone())
                    .string("flagged", request.flagged.clone())
                    .string("favorited", request.favorited.clone())
                    .string("language", request.language.clone())
                    .string("liked", request.liked.clone())
                    .string("session_id", request.session_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("min_timestamp", request.min_timestamp.clone())
                    .string("max_timestamp", request.max_timestamp.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Creates a chat completion using the agent's configured model. Supports both streaming and non-streaming responses.
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
    ///         .chat
    ///         .create_chat_completion(
    ///             &ChatCompletionRequest::Unknown(serde_json::json!({"key":"value"})),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_chat_completion(
        &self,
        request: &ChatCompletionRequest,
        options: Option<RequestOptions>,
    ) -> Result<ChatCompletionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "chat/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Updates the like status of a specific chat completion
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the chat completion
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
    ///         .chat
    ///         .like_completion(&"id".to_string(), &LikeRequest { liked: true }, None)
    ///         .await;
    /// }
    /// ```
    pub async fn like_completion(
        &self,
        id: &str,
        request: &LikeRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("chat/completions/{}/like", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Updates the flagged status of a specific chat completion
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the chat completion
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
    ///         .chat
    ///         .flag_completion(&"id".to_string(), &FlagRequest { flagged: true }, None)
    ///         .await;
    /// }
    /// ```
    pub async fn flag_completion(
        &self,
        id: &str,
        request: &FlagRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("chat/completions/{}/flag", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Adds user feedback to a specific chat completion
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the chat completion
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
    ///         .chat
    ///         .feedback_completion(
    ///             &"id".to_string(),
    ///             &FeedbackRequest {
    ///                 feedback: "feedback".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn feedback_completion(
        &self,
        id: &str,
        request: &FeedbackRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("chat/completions/{}/feedback", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Creates a share record for a specific chat completion
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the chat completion
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
    ///         .chat
    ///         .share_completion(
    ///             &"id".to_string(),
    ///             &ShareRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn share_completion(
        &self,
        id: &str,
        request: &ShareRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("chat/completions/{}/share", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single chat completion (prompt) by numeric id or UUID, including applied tags, guardrail/cta metadata, share metadata, and automation results.
    ///
    /// # Arguments
    ///
    /// * `id` - The numeric id or UUID of the chat completion
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
    ///         .chat
    ///         .get_chat_completion(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_chat_completion(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetChatCompletionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("chat/completions/{}", id),
                None,
                None,
                options,
            )
            .await
    }
}
