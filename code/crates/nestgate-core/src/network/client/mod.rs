// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Modern HTTP Client Module
//!
//! A high-performance, type-safe HTTP client with connection pooling,
//! retry mechanisms, and comprehensive error handling using modern Rust patterns.
//!
//! # Architecture
//!
//! This module is organized into focused sub-modules:
//!
//! - **types**: HTTP primitives (Port, Method, StatusCode, Endpoint)
//! - **request**: Request and Response handling
//! - **config**: Client configuration
//! - **pool**: Connection pooling and lifecycle management
//! - **http**: High-level HTTP client implementation
//! - **error**: Domain-specific error types
//! - **utils**: Convenience functions
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```ignore
//! use nestgate_core::network::client::{HttpClient, Endpoint};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Create client with default configuration
//!     let client = HttpClient::default();
//!     
//!     // Create endpoint
//!     let endpoint = Endpoint::from_url("https://api.example.com")?;
//!     
//!     // Send GET request
//!     let response = client.get(&endpoint, "/users").await?;
//!     
//!     // Parse JSON response
//!     let users: Vec<User> = response.json()?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Custom Configuration
//!
//! ```ignore
//! use nestgate_core::network::client::{HttpClient, ClientConfig};
//! use std::time::Duration;
//!
//! let config = ClientConfig::default()
//!     .with_timeout(Duration::from_secs(60))
//!     .with_max_retries(5)
//!     .with_max_connections(20);
//!
//! let client = HttpClient::new(config);
//! ```
//!
//! ## Zero-Copy Requests
//!
//! ```ignore
//! use nestgate_core::network::client::Request;
//!
//! // Request uses lifetimes for zero-copy
//! let json_body = r#"{"name": "test"}"#;
//! let request = Request::post_json("/api/create", json_body);
//! ```
//!
//! # Features
//!
//! - **Type Safety**: Port, StatusCode, and Method are type-safe wrappers
//! - **Zero-Copy**: Request uses lifetimes to avoid allocations where possible
//! - **Connection Pooling**: Automatic connection reuse and lifecycle management
//! - **Retry Logic**: Exponential backoff for transient failures
//! - **Timeout Handling**: Per-request timeouts with graceful error handling
//! - **Statistics**: Connection and request statistics for monitoring
//!
//! # Performance
//!
//! - Zero-cost abstractions using Rust's type system
//! - Connection pooling reduces overhead
//! - Const generics for compile-time optimization
//! - Minimal allocations through lifetime management

// Sub-modules
pub mod config;
pub mod error;
pub mod http;
pub mod pool;
pub mod request;
pub mod types;
pub mod utils;

// Re-export main types for convenience
pub use config::ClientConfig;
pub use error::HttpClientError;
pub use http::{ClientStats, HttpClient};
pub use pool::{Connection, ConnectionPool, ConnectionStats, PoolStats};
pub use request::{HeaderMap, Request, RequestBody, Response};
pub use types::{Endpoint, Method, Port, Scheme, StatusCode, TimeoutMs};
pub use utils::{create_client, http_endpoint, https_endpoint, parse_endpoint};

// ==================== BACKWARD COMPATIBILITY ====================

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
#[allow(deprecated)]
pub type ClientConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// ==================== MODULE TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all main types are exported
        let _client: HttpClient = HttpClient::default();
        let _config: ClientConfig = ClientConfig::default();
        let _endpoint = Endpoint::from_url("http://localhost:8080");
    }

    #[test]
    fn test_type_safety() {
        // Verify type-safe wrappers work
        let port = Port::new(8080).unwrap();
        assert_eq!(port.get(), 8080);

        let status = StatusCode::OK;
        assert!(status.is_success());
    }

    #[tokio::test]
    async fn test_utils_functions() {
        // Verify utility functions work
        let client = create_client();
        drop(client);

        let endpoint = http_endpoint("localhost", 8080).await.unwrap();
        assert_eq!(endpoint.host, "localhost");
    }
}
