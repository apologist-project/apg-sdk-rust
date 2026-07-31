use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ChannelsClient {
    pub http_client: HttpClient,
}

impl ChannelsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Returns the status of the Discord channel. Used as a lightweight health/verification endpoint.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
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
    ///         .channels
    ///         .get_discord_channel_status(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_discord_channel_status(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<GetDiscordChannelStatusResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("channels/{}/discord", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Receives Discord interaction callbacks for the channel. Requests are verified via Ed25519 signature headers; unsigned or invalid requests are rejected. Payload shape is defined by Discord.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
    /// * `request` - Discord interaction payload.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .channels
    ///         .receive_discord_interaction(
    ///             &"id".to_string(),
    ///             &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("x-signature-ed25519", "x-signature-ed25519")
    ///                     .additional_header("x-signature-timestamp", "x-signature-timestamp"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn receive_discord_interaction(
        &self,
        id: &str,
        request: &HashMap<String, serde_json::Value>,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("channels/{}/discord", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Handles the Meta webhook verification handshake, echoing `hub.challenge` when `hub.verify_token` matches the channel's configured token.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
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
    ///         .channels
    ///         .verify_facebook_webhook(
    ///             &"id".to_string(),
    ///             &VerifyFacebookWebhookQueryRequest {
    ///                 hub_mode: VerifyFacebookWebhookRequestHubMode::Subscribe,
    ///                 hub_verify_token: "hub.verify_token".to_string(),
    ///                 hub_challenge: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn verify_facebook_webhook(
        &self,
        id: &str,
        request: &VerifyFacebookWebhookQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("channels/{}/facebook", id),
                None,
                QueryBuilder::new()
                    .serialize("hub.mode", Some(request.hub_mode.clone()))
                    .string("hub.verify_token", request.hub_verify_token.clone())
                    .string("hub.challenge", request.hub_challenge.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Receives Facebook/Messenger (and Instagram-style) message events for the channel. Payload shape is defined by Meta.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
    /// * `request` - Meta webhook payload.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .channels
    ///         .receive_facebook_message(
    ///             &"id".to_string(),
    ///             &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn receive_facebook_message(
        &self,
        id: &str,
        request: &HashMap<String, serde_json::Value>,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("channels/{}/facebook", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Returns a static HTML privacy policy page for the Instagram integration.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Text response
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
    ///         .channels
    ///         .get_instagram_privacy_policy(&"id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_instagram_privacy_policy(
        &self,
        id: &str,
        options: Option<RequestOptions>,
    ) -> Result<String, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("channels/{}/instagram/privacy", id),
                None,
                None,
                options,
            )
            .await
    }

    /// Receives Telegram bot update events for the channel. Non-message updates are acknowledged and ignored. Payload shape is defined by Telegram.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
    /// * `request` - Telegram update payload.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .channels
    ///         .receive_telegram_update(
    ///             &"id".to_string(),
    ///             &HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn receive_telegram_update(
        &self,
        id: &str,
        request: &HashMap<String, serde_json::Value>,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("channels/{}/telegram", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Receives inbound Twilio messages for the channel as form-encoded data. Payload fields are defined by Twilio.
    ///
    /// # Arguments
    ///
    /// * `id` - The channel id
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Empty response
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
    ///         .channels
    ///         .receive_twilio_message(
    ///             &"id".to_string(),
    ///             &ReceiveTwilioMessageRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn receive_twilio_message(
        &self,
        id: &str,
        request: &ReceiveTwilioMessageRequest,
        options: Option<RequestOptions>,
    ) -> Result<(), ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("channels/{}/twilio", id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
