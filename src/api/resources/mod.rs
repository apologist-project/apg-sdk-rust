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
//! - **Agent**
//! - **Conversations**
//! - **Channels**
//! - **Shares**
//! - **Webhooks**

use crate::{ApiError, ClientConfig};

pub mod agent;
pub mod benchmarks;
pub mod channels;
pub mod chat;
pub mod conversations;
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
    pub agent: AgentClient,
    pub conversations: ConversationsClient,
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
            agent: AgentClient::new(config.clone())?,
            conversations: ConversationsClient::new(config.clone())?,
            channels: ChannelsClient::new(config.clone())?,
            shares: SharesClient::new(config.clone())?,
        })
    }
}

pub use agent::AgentClient;
pub use benchmarks::BenchmarksClient;
pub use channels::ChannelsClient;
pub use chat::ChatClient;
pub use conversations::ConversationsClient;
pub use corpus::CorpusClient;
pub use ct_as::CtAsClient;
pub use evaluators::EvaluatorsClient;
pub use shares::SharesClient;
pub use users::UsersClient;
pub use webhooks::WebhooksClient;
