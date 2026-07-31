use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct EvaluatorsClient {
    pub http_client: HttpClient,
}

impl EvaluatorsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of evaluations for the evaluator, scoped to the requesting agent.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID or key of the evaluator
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
    ///         .evaluators
    ///         .list_evaluations(
    ///             &"id".to_string(),
    ///             &ListEvaluationsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_evaluations(
        &self,
        id: &str,
        request: &ListEvaluationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListEvaluationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("evaluators/{}/evaluations", id),
                None,
                QueryBuilder::new()
                    .int("page", request.page.clone())
                    .int("per_page", request.per_page.clone())
                    .string("min_timestamp", request.min_timestamp.clone())
                    .string("max_timestamp", request.max_timestamp.clone())
                    .string("min_duration", request.min_duration.clone())
                    .string("max_duration", request.max_duration.clone())
                    .string("min_score", request.min_score.clone())
                    .string("max_score", request.max_score.clone())
                    .string("passed", request.passed.clone())
                    .string("benchmark", request.benchmark.clone())
                    .string("benchmark_run_id", request.benchmark_run_id.clone())
                    .string(
                        "benchmark_question_id",
                        request.benchmark_question_id.clone(),
                    )
                    .build(),
                options,
            )
            .await
    }

    /// Runs an evaluation on the provided content using the specified evaluator
    ///
    /// # Arguments
    ///
    /// * `id` - The ID or key of the evaluator
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
    ///         .evaluators
    ///         .evaluate_content(
    ///             &"id".to_string(),
    ///             &EvaluatorRequest {
    ///                 content: EvaluatorRequestContent::String("content".to_string()),
    ///                 frequency_penalty: None,
    ///                 confidence_threshold: None,
    ///                 model: None,
    ///                 presence_penalty: None,
    ///                 reasoning_effort: None,
    ///                 verbosity: None,
    ///                 temperature: None,
    ///                 top_p: None,
    ///                 variables: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn evaluate_content(
        &self,
        id: &str,
        request: &EvaluatorRequest,
        options: Option<RequestOptions>,
    ) -> Result<EvaluateContentResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("evaluators/{}/evaluations", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single evaluation for the evaluator, scoped to the requesting agent.
    ///
    /// # Arguments
    ///
    /// * `id` - The id or key of the evaluator
    /// * `evaluation_id` - The id or UUID of the evaluation
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
    ///         .evaluators
    ///         .get_evaluation(&"id".to_string(), &"evaluationId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_evaluation(
        &self,
        id: &str,
        evaluation_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetEvaluationResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("evaluators/{}/evaluations/{}", id, evaluation_id),
                None,
                None,
                options,
            )
            .await
    }
}
