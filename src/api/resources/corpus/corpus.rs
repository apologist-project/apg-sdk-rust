use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct CorpusClient {
    pub http_client: HttpClient,
}

impl CorpusClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Performs a semantic search across the agent's corpus of knowledge
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
    /// use apologist_ai_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .corpus
    ///         .search_corpus(
    ///             &CorpusSearchRequest {
    ///                 query: "query".to_string(),
    ///                 prompt_id: None,
    ///                 limit: None,
    ///                 filters: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn search_corpus(
        &self,
        request: &CorpusSearchRequest,
        options: Option<RequestOptions>,
    ) -> Result<SearchCorpusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                "corpus/search",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records that a user viewed a specific corpus item
    ///
    /// # Arguments
    ///
    /// * `model` - The model type (e.g., 'source')
    /// * `id` - The ID of the corpus item
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apologist_ai_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .corpus
    ///         .log_corpus_view(
    ///             &"model".to_string(),
    ///             &"id".to_string(),
    ///             &ViewRequest {
    ///                 prompt_id: "prompt_id".to_string(),
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn log_corpus_view(
        &self,
        model: &str,
        id: &str,
        request: &ViewRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("corpus/{}/{}/view", model, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records that a corpus item was shown to a user
    ///
    /// # Arguments
    ///
    /// * `model` - The model type (e.g., 'source')
    /// * `id` - The ID of the corpus item
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apologist_ai_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .corpus
    ///         .log_corpus_impression(
    ///             &"model".to_string(),
    ///             &"id".to_string(),
    ///             &ImpressionRequest {
    ///                 prompt_id: "prompt_id".to_string(),
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn log_corpus_impression(
        &self,
        model: &str,
        id: &str,
        request: &ImpressionRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("corpus/{}/{}/impression", model, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Records a referral for a corpus item and, when a `url` is supplied, issues a 302 redirect to it. Without a `url`, responds with a success message. Requires either the search API entitlement or a same-origin request.
    ///
    /// # Arguments
    ///
    /// * `model` - The model type (e.g., 'source')
    /// * `id` - The numeric ID of the corpus item
    /// * `url` - URL-encoded destination to redirect to after logging the referral.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apologist_ai_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .corpus
    ///         .log_corpus_referral_redirect(
    ///             &"model".to_string(),
    ///             &"id".to_string(),
    ///             &LogCorpusReferralRedirectQueryRequest {
    ///                 prompt_id: "prompt_id".to_string(),
    ///                 user_id: None,
    ///                 url: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn log_corpus_referral_redirect(
        &self,
        model: &str,
        id: &str,
        request: &LogCorpusReferralRedirectQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("corpus/{}/{}/referral", model, id),
                None,
                QueryBuilder::new()
                    .string("prompt_id", request.prompt_id.clone())
                    .string("user_id", request.user_id.clone())
                    .string("url", request.url.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Records that a user was referred to a corpus item
    ///
    /// # Arguments
    ///
    /// * `model` - The model type (e.g., 'source')
    /// * `id` - The ID of the corpus item
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use apologist_ai_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         api_key: Some("<value>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .corpus
    ///         .log_corpus_referral(
    ///             &"model".to_string(),
    ///             &"id".to_string(),
    ///             &ReferralRequest {
    ///                 prompt_id: "prompt_id".to_string(),
    ///                 user_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn log_corpus_referral(
        &self,
        model: &str,
        id: &str,
        request: &ReferralRequest,
        options: Option<RequestOptions>,
    ) -> Result<SuccessResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("corpus/{}/{}/referral", model, id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
