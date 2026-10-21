//! # Apologist Agent API SDK
//!
//! The official Rust SDK for the Apologist Agent API.
//!
//! ## Getting Started
//!
//! ```rust
//! use apologist::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         api_key: Some("<value>".to_string()),
//!         ..Default::default()
//!     };
//!     let client = ApologistAgentClient::new(config).expect("Failed to build client");
//!     client
//!         .chat
//!         .list_chat_completions(
//!             &ListChatCompletionsQueryRequest {
//!                 ..Default::default()
//!             },
//!             Some(RequestOptions::new().additional_header(
//!                 "traceparent",
//!                 "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
//!             )),
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use api::*;
pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
