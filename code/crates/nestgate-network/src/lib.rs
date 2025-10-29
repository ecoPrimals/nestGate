//! **NESTGATE NETWORK CRATE**
//!
//! This crate provides network functionality for the `NestGate` ecosystem,
//! including connection management, protocol handling, and service discovery.

use std::time::Duration;

/// CANONICAL MODERNIZATION: Use canonical Result type from nestgate-core
/// Type alias for Results used throughout the crate - migrated to canonical
pub use nestgate_core::Result;
// ==================== SECTION ====================

/// API module for network services
pub mod api;
/// Protocol handlers and management
pub mod handlers;
/// Orchestration adapter
pub mod orchestration_adapter;
/// Protocol definitions
pub mod protocol;
/// Main network service implementation
pub mod service;
/// Service discovery client (sovereignty compliant)
pub mod service_discovery_client;
/// Network types and configuration
pub mod types;
/// Unified network configuration
pub mod unified_network_config;
/// Unified network extensions
pub mod unified_network_extensions;
// Removed: Zero-cost orchestration types (delegated to orchestration primal)
/// Orchestration functionality
pub use orchestration_adapter::{OrchestrationAdapter, OrchestrationConfig};
/// Configuration migration utilities
/// These utilities help migrate from legacy configurations
/// to the new modular network system.
/// Main network service
pub use service::RealNetworkService as NetworkService;
/// Service discovery client (sovereignty compliant)
pub use service_discovery_client::{LocalServiceRegistry, ServiceDiscoveryClient};
/// Network configuration
pub use types::{NetworkConfig, NetworkConfigBuilder};
// Removed: Universal orchestration modules (delegated to orchestration primal via capability discovery)
// ==================== SECTION ====================

// **DEPRECATED CODE REMOVED**
//
// The following deprecated compatibility layers have been eliminated:
// - `real_network_service.rs` (893 lines) - Deprecated compatibility layer
//
// **Migration Path**:
// All functionality has been migrated to the modular system:
// - Use `service::NetworkService` for main network operations
// - Use `types::NetworkConfig` for configuration
// - Use `handlers::*` for protocol-specific operations
//
// **Performance Impact**:
// - Removed 893 lines of deprecated code
// - Eliminated compatibility overhead
// - Improved compile times and memory usage

// ==================== CONFIGURATION FUNCTIONS ====================

/// Default network configuration
#[must_use]
pub fn default_network_config() -> NetworkConfig {
    NetworkConfig::default()
}
/// Create production network configuration
#[must_use]
pub fn production_network_config() -> NetworkConfig {
    let mut config = NetworkConfig::default();
    config.network.api.max_connections = 2000;
    config.network.api.connection_timeout = Duration::from_secs(10);
    config
}
/// Create development network configuration  
#[must_use]
pub fn development_network_config() -> NetworkConfig {
    let mut config = NetworkConfig::default();
    config.network.api.max_connections = 100;
    config.network.api.connection_timeout = Duration::from_secs(30);
    config
}
// ==================== SECTION ====================

/// Network-specific result type
// Use canonical NetworkResult from nestgate_core::error
pub use nestgate_core::error::NetworkResult;
/// Network error types
#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    #[error("Timeout occurred: {b_operation:?}")]
    Timeout { b_operation: Option<String> },
    #[error("Configuration error: {field} - {message}")]
    Configuration { field: String, message: String },
    #[error("Protocol error: {protocol} - {message}")]
    Protocol { protocol: String, message: String },
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
}
// ==================== SECTION ====================

/// Network constants - use canonical constants system
pub mod constants {
    /// Default API port - use canonical constant
    pub use nestgate_core::constants::canonical_defaults::network::DEFAULT_API_PORT;
    /// Default internal port
    pub const DEFAULT_INTERNAL_PORT: u16 = 8081;

    /// Default connection timeout
    pub const DEFAULT_CONNECTION_TIMEOUT_SECONDS: u64 = 30;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = default_network_config();
        // Test that config is created successfully
        assert!(
            !config.extensions.port_range_start == 0 || config.extensions.port_range_start >= 1024
        );
    }
    #[test]
    fn test_production_config() {
        let config = production_network_config();
        assert_eq!(config.extensions.load_balancing.max_failures, 3);
        // Test production settings - config creation successful
        assert!(config.extensions.keep_alive_timeout_seconds > 0);
    }

    #[test]
    fn test_development_config() {
        let config = development_network_config();
        assert_eq!(config.extensions.load_balancing.max_failures, 3);
        // Test development settings - config creation successful
        assert!(config.extensions.keep_alive_timeout_seconds > 0);
    }
}
