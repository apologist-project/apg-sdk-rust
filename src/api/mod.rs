//! API client and types for the Apologist Agent API
//!
//! This module contains all the API definitions including request/response types
//! and client implementations for interacting with the API.
//!
//! ## Modules
//!
//! - [`resources`] - Service clients and endpoints
//! - [`types`] - Request, response, and model types

pub mod resources;
pub mod types;

pub use resources::{
    AgentClient, ApologistAgentClient, BenchmarksClient, ChannelsClient, ChatClient,
    ConversationsClient, CorpusClient, CtAsClient, EvaluatorsClient, SharesClient, UsersClient,
    WebhooksClient,
};
pub use types::*;
