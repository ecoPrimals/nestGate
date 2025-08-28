//! **NESTGATE NETWORK CRATE**
//!
//! This crate provides network functionality for the NestGate ecosystem,
//! including connection management, protocol handling, and service discovery.

// ==================== SECTION ====================

/// Main network service implementation
pub mod service;

/// Network types and configuration
pub mod types;

/// Protocol handlers and management
pub mod handlers;

/// Zero-cost orchestration client
pub mod zero_cost_orchestration_client;

/// Unified network extensions
pub mod unified_network_extensions;

/// Universal orchestration
pub mod universal_orchestration;

/// Unified network configuration
pub mod unified_network_config;

/// Protocol definitions
pub mod protocol;

/// Orchestration adapter
pub mod orchestration_adapter;

/// Zero-cost orchestration types
pub mod zero_cost_orchestration_types;

// ==================== SECTION ====================

/// Main network service
pub use service::NetworkService;

/// Network configuration
pub use types::{NetworkConfig, NetworkConfigBuilder};

/// Orchestration functionality
pub use orchestration_adapter::{OrchestrationAdapter, OrchestrationConfig};

/// Zero-cost orchestration client
pub use zero_cost_orchestration_client::ZeroCostOrchestrationClient;

/// Universal orchestration
pub use universal_orchestration::{UniversalOrchestration, UniversalOrchestrationConfig};

// ==================== SECTION ====================

/// **DEPRECATED CODE REMOVED**
///
/// The following deprecated compatibility layers have been eliminated:
/// - `real_network_service.rs` (893 lines) - Deprecated compatibility layer
///
/// **Migration Path**:
/// All functionality has been migrated to the modular system:
/// - Use `service::NetworkService` for main network operations
/// - Use `types::NetworkConfig` for configuration
/// - Use `handlers::*` for protocol-specific operations
///
/// **Performance Impact**: 
/// - Removed 893 lines of deprecated code
/// - Eliminated compatibility overhead
/// - Improved compile times and memory usage

// ==================== SECTION ====================

/// Default network configuration
pub fn default_network_config() -> NetworkConfig {
    NetworkConfig::default()
}

/// Create production network configuration
pub fn production_network_config() -> NetworkConfig {
    let mut config = NetworkConfig::default();
    config.connection_pool.max_connections = 2000;
    config.timeouts.connection_timeout = std::time::Duration::from_secs(10);
    config.security.tls_enabled = true;
    config
}

/// Create development network configuration  
pub fn development_network_config() -> NetworkConfig {
    let mut config = NetworkConfig::default();
    config.connection_pool.max_connections = 100;
    config.timeouts.connection_timeout = std::time::Duration::from_secs(30);
    config.security.tls_enabled = false;
    config.debug.verbose_logging = true;
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
    
    #[error("Timeout occurred: {operation}")]
    Timeout { operation: String },
    
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
    pub use nestgate_core::constants::canonical::network::DEFAULT_API_PORT;
    
    /// Default internal port
    pub const DEFAULT_INTERNAL_PORT: u16 = 8081;
    
    /// Default connection timeout
// ==================== SECTION ====================
// Timeout constants have been moved to nestgate_core::canonical_modernization::canonical_constants::timeouts
// Use: nestgate_core::canonical_modernization::canonical_constants::timeouts::CONNECTION_TIMEOUT
// Use: nestgate_core::canonical_modernization::canonical_constants::timeouts::REQUEST_TIMEOUT
    
    /// Use canonical constants from nestgate-core
    pub use nestgate_core::constants::canonical::performance::{MAX_CONNECTIONS, NETWORK_BUFFER_SIZE as BUFFER_SIZE};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_creation() {
        let config = default_network_config();
        assert_eq!(config.api_port, constants::DEFAULT_API_PORT);
    }

    #[test]
    fn test_production_config() {
        let config = production_network_config();
        assert_eq!(config.connection_pool.max_connections, 2000);
        assert!(config.security.tls_enabled);
    }

    #[test]
    fn test_development_config() {
        let config = development_network_config();
        assert_eq!(config.connection_pool.max_connections, 100);
        assert!(!config.security.tls_enabled);
        assert!(config.debug.verbose_logging);
    }
}
