//! Service clients and API endpoints
//!
//! This module contains client implementations for:
//!
//! - **Chat**
//! - **Corpus**
//! - **Evaluators**
//! - **CTAs**
//! - **Users**
//! - **Benchmarks**
//! - **Channels**
//! - **Shares**
//! - **Webhooks**

use crate::{ApiError, ClientConfig};

pub mod benchmarks;
pub mod channels;
pub mod chat;
pub mod corpus;
pub mod ct_as;
pub mod evaluators;
pub mod shares;
pub mod users;
pub mod webhooks;
pub struct ApologistAgentClient {
    pub config: ClientConfig,
    pub chat: ChatClient,
    pub corpus: CorpusClient,
    pub evaluators: EvaluatorsClient,
    pub ct_as: CtAsClient,
    pub users: UsersClient,
    pub benchmarks: BenchmarksClient,
    pub channels: ChannelsClient,
    pub shares: SharesClient,
}

impl ApologistAgentClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            config: config.clone(),
            chat: ChatClient::new(config.clone())?,
            corpus: CorpusClient::new(config.clone())?,
            evaluators: EvaluatorsClient::new(config.clone())?,
            ct_as: CtAsClient::new(config.clone())?,
            users: UsersClient::new(config.clone())?,
            benchmarks: BenchmarksClient::new(config.clone())?,
            channels: ChannelsClient::new(config.clone())?,
            shares: SharesClient::new(config.clone())?,
        })
    }
}

pub use benchmarks::BenchmarksClient;
pub use channels::ChannelsClient;
pub use chat::ChatClient;
pub use corpus::CorpusClient;
pub use ct_as::CtAsClient;
pub use evaluators::EvaluatorsClient;
pub use shares::SharesClient;
pub use users::UsersClient;
pub use webhooks::WebhooksClient;
