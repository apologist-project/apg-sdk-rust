use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct BenchmarksClient {
    pub http_client: HttpClient,
}

impl BenchmarksClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns a paginated list of runs for a benchmark, scoped to the requesting agent. Each run carries nested evaluators, questions, and a flat evaluations array.
    ///
    /// # Arguments
    ///
    /// * `id` - The id or key of the benchmark
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
    ///         .benchmarks
    ///         .list_benchmark_runs(
    ///             &"id".to_string(),
    ///             &ListBenchmarkRunsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_benchmark_runs(
        &self,
        id: &str,
        request: &ListBenchmarkRunsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListBenchmarkRunsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("benchmarks/{}/runs", id),
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
                    .string("min_responses", request.min_responses.clone())
                    .string("max_responses", request.max_responses.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Executes a benchmark run and returns the aggregated result with nested evaluators, questions, and a flat evaluations array.
    ///
    /// # Arguments
    ///
    /// * `id` - The id or key of the benchmark
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
    ///         .benchmarks
    ///         .run_benchmark(
    ///             &"id".to_string(),
    ///             &BenchmarkRunRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn run_benchmark(
        &self,
        id: &str,
        request: &BenchmarkRunRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("benchmarks/{}/runs", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a single benchmark run by id or UUID, scoped to the requesting agent, including nested evaluators, questions, and evaluations.
    ///
    /// # Arguments
    ///
    /// * `id` - The id or key of the benchmark
    /// * `run_id` - The id or UUID of the run
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
    ///         .benchmarks
    ///         .get_benchmark_run(&"id".to_string(), &"runId".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_benchmark_run(
        &self,
        id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetBenchmarkRunResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("benchmarks/{}/runs/{}", id, run_id),
                None,
                None,
                options,
            )
            .await
    }
}
