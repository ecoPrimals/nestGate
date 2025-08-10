#![doc = "
# NestGate MCP (Model Context Protocol) Integration

High-performance MCP protocol implementation providing seamless integration with AI systems,
language models, and external MCP-compatible services.

## Key Features

- **MCP Protocol Compliance**: Full implementation of Model Context Protocol specification
- **Session Management**: Secure session handling with authentication and authorization
- **Security Framework**: Multi-provider authentication with role-based access control
- **Streaming Support**: High-throughput streaming for large model interactions
- **Error Handling**: Comprehensive error management with retry logic

## Protocol Support

- **MCP v1.0**: Full protocol compliance with all standard operations
- **Authentication**: Multiple auth methods (API keys, OAuth2, certificates)
- **Authorization**: Role-based permissions for fine-grained access control  
- **Session Lifecycle**: Complete session management with cleanup and monitoring

## Performance

- **Throughput**: Optimized for high-frequency model interactions
- **Latency**: Sub-millisecond session operations  
- **Scalability**: Concurrent session support with resource pooling
- **Reliability**: Automatic retry and failover mechanisms

This crate enables secure, high-performance integration with AI systems and external
MCP-compatible services within the NestGate ecosystem.
"]

// Core MCP modules
pub mod adapter;
pub mod client;
pub mod config;
pub mod error;
pub mod protocol;
pub mod provider;
pub mod security;
pub mod service;
pub mod session;
pub mod storage;
pub mod types;

// Re-export commonly used types and traits
pub use client::{HttpOrchestratorClient, OrchestratorClient};
pub use config::{EnhancedMcpConfig, McpConfig, RetryConfig};
// Use unified error system instead of deprecated local error types
pub use nestgate_core::error::{NestGateError, Result};
pub use service::EnhancedMcpService;

// Re-export key types from sub-modules
pub use nestgate_core::diagnostics::SystemMetrics;
pub use types::{AuthConfig, ProviderCapabilities, ProviderConfig, TlsConfig};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default MCP service configuration
pub fn default_config() -> McpConfig {
    McpConfig::default()
}

/// Default enhanced MCP service configuration
pub fn default_enhanced_config() -> EnhancedMcpConfig {
    EnhancedMcpConfig::default()
}

/// Create a new HTTP orchestrator client
pub fn create_http_client(base_url: String) -> HttpOrchestratorClient {
    HttpOrchestratorClient::new(base_url)
}

/// Create a new enhanced MCP service
pub fn create_service(
    config: EnhancedMcpConfig,
    orchestrator_client: std::sync::Arc<dyn OrchestratorClient>,
) -> EnhancedMcpService {
    EnhancedMcpService::new(config, orchestrator_client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = default_config();
        assert_eq!(config.node_id, "default-node");
        assert!(!config.federation_enabled);
    }

    #[test]
    fn test_enhanced_config_creation() {
        let config = default_enhanced_config();
        assert_eq!(config.node_id, "default-node");
        assert_eq!(config.cluster_name, "default-cluster");
        assert!(!config.federation_enabled);
        assert_eq!(config.metrics_collection_interval, 30);
        assert_eq!(config.health_check_interval, 10);
    }

    #[test]
    fn test_version_info() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.chars().any(|c| c.is_ascii_digit()));
    }
}
