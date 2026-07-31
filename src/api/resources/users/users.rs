use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct UsersClient {
    pub http_client: HttpClient,
}

impl UsersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of users for the agent's team, with applied tags expanded as { id, name } and the persisted responder id.
    ///
    /// # Arguments
    ///
    /// * `per_page` - Results per page (clamped to 100).
    /// * `tags` - Comma-separated tag ids.
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
    ///         .users
    ///         .list_users(
    ///             &ListUsersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_users(
        &self,
        request: &ListUsersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListUsersResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "users",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("per_page", request.per_page.clone())
                    .string("external_id", request.external_id.clone())
                    .string("tags", request.tags.clone())
                    .string("responder_id", request.responder_id.clone())
                    .string("min_timestamp", request.min_timestamp.clone())
                    .string("max_timestamp", request.max_timestamp.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a paginated list of user flag definitions for the agent's team (all columns from user_flags), ordered by id ascending.
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
    ///         .users
    ///         .list_user_flags(
    ///             &ListUserFlagsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_user_flags(
        &self,
        request: &ListUserFlagsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListUserFlagsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                "users/flags",
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("per_page", request.per_page.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Returns a single user by external id or internal id, with expanded tags and the persisted responder for the agent.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's external id or internal id
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
    ///     client.users.get_user(&"user_id".to_string(), None).await;
    /// }
    /// ```
    pub async fn get_user(
        &self,
        user_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("users/{}", user_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a user's external_id and/or tags and upserts the persisted responder for the agent. Only provided fields are changed.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user's external id or internal id
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
    ///         .users
    ///         .update_user(
    ///             &"user_id".to_string(),
    ///             &UserUpdateRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_user(
        &self,
        user_id: &str,
        request: &UserUpdateRequest,
        options: Option<RequestOptions>,
    ) -> Result<UpdateUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("users/{}", user_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
