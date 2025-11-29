//! Port Configuration - Environment-Driven
//!
//! Dynamic port configuration system supporting environment variables and config files.
//! Eliminates hardcoded ports throughout the codebase.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

/// Port configuration for all NestGate services
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortConfiguration {
    /// API server port
    pub api_port: u16,
    /// Health check endpoint port
    pub health_port: u16,
    /// Metrics collection port
    pub metrics_port: u16,
    /// Admin interface port
    pub admin_port: u16,
    /// WebSocket server port
    pub websocket_port: u16,
    /// RPC service port
    pub rpc_port: u16,
    /// Database port
    pub database_port: u16,
    /// Redis cache port
    pub redis_port: u16,
    /// Message queue port
    pub message_queue_port: u16,
    /// Orchestration service port
    pub orchestration_port: u16,
}

impl PortConfiguration {
    /// Create configuration from environment variables with sensible defaults
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            api_port: env_var_or_default("NESTGATE_API_PORT", 8080),
            health_port: env_var_or_default("NESTGATE_HEALTH_PORT", 8443),
            metrics_port: env_var_or_default("NESTGATE_METRICS_PORT", 9090),
            admin_port: env_var_or_default("NESTGATE_ADMIN_PORT", 8081),
            websocket_port: env_var_or_default("NESTGATE_WEBSOCKET_PORT", 8082),
            rpc_port: env_var_or_default("NESTGATE_RPC_PORT", 50051),
            database_port: env_var_or_default("NESTGATE_DATABASE_PORT", 5432),
            redis_port: env_var_or_default("NESTGATE_REDIS_PORT", 6379),
            message_queue_port: env_var_or_default("NESTGATE_MQ_PORT", 5672),
            orchestration_port: env_var_or_default("NESTGATE_ORCHESTRATION_PORT", 9091),
        }
    }

    /// Create configuration with all defaults (for testing)
    #[must_use]
    pub fn with_defaults() -> Self {
        Self {
            api_port: 8080,
            health_port: 8443,
            metrics_port: 9090,
            admin_port: 8081,
            websocket_port: 8082,
            rpc_port: 50051,
            database_port: 5432,
            redis_port: 6379,
            message_queue_port: 5672,
            orchestration_port: 9091,
        }
    }

    /// Create test configuration with known test ports
    #[must_use]
    pub fn for_testing() -> Self {
        Self {
            api_port: 18080,
            health_port: 18443,
            metrics_port: 19090,
            admin_port: 18081,
            websocket_port: 18082,
            rpc_port: 15051,
            database_port: 15432,
            redis_port: 16379,
            message_queue_port: 15672,
            orchestration_port: 19091,
        }
    }
}

impl Default for PortConfiguration {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Global port configuration singleton
static PORT_CONFIG: OnceLock<PortConfiguration> = OnceLock::new();

/// Get the global port configuration
///
/// This is initialized once on first access and reused throughout the application.
#[must_use]
pub fn get_port_config() -> &'static PortConfiguration {
    PORT_CONFIG.get_or_init(PortConfiguration::from_env)
}

/// Initialize port configuration with custom values (useful for testing)
///
/// # Errors
///
/// Returns error if configuration is already initialized
pub fn init_port_config(config: PortConfiguration) -> Result<(), &'static str> {
    PORT_CONFIG
        .set(config)
        .map_err(|_| "Port configuration already initialized")
}

/// Helper function to parse environment variable or use default
fn env_var_or_default(var_name: &str, default: u16) -> u16 {
    std::env::var(var_name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_ports() {
        let config = PortConfiguration::with_defaults();
        assert_eq!(config.api_port, 8080);
        assert_eq!(config.health_port, 8443);
        assert_eq!(config.metrics_port, 9090);
    }

    #[test]
    fn test_test_ports() {
        let config = PortConfiguration::for_testing();
        assert_eq!(config.api_port, 18080);
        assert_eq!(config.health_port, 18443);
        // Test ports are all offset by 10000 to avoid conflicts
        assert!(config.api_port > 10000);
    }

    #[test]
    fn test_from_env_with_defaults() {
        // Without setting env vars, should use defaults
        let config = PortConfiguration::from_env();
        assert_eq!(config.api_port, 8080); // Default
    }

    #[test]
    fn test_env_var_parsing() {
        std::env::set_var("TEST_PORT", "1234");
        let port = env_var_or_default("TEST_PORT", 9999);
        assert_eq!(port, 1234);
        std::env::remove_var("TEST_PORT");
    }

    #[test]
    fn test_env_var_default_fallback() {
        let port = env_var_or_default("NONEXISTENT_VAR", 7777);
        assert_eq!(port, 7777);
    }
}
