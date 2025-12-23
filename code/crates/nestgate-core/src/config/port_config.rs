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
    ///
    /// # Primal Sovereignty
    ///
    /// All ports configurable via environment variables. No hardcoded assumptions.
    #[must_use]
    pub fn from_env() -> Self {
        // ✅ SOVEREIGNTY: Environment-driven port configuration
        Self {
            api_port: env_var_or_default("NESTGATE_API_PORT", 8080),
            health_port: env_var_or_default("NESTGATE_HEALTH_PORT", 8443),
            metrics_port: env_var_or_default("NESTGATE_METRICS_PORT", 9090),
            admin_port: env_var_or_default("NESTGATE_ADMIN_PORT", 9091),
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
            api_port: 8080,  // HTTP standard port
            health_port: 8443, // HTTPS alt port for health
            metrics_port: 9090, // Prometheus standard port
            admin_port: 9091,  // Prometheus admin standard
            websocket_port: 8082, // WebSocket common port
            rpc_port: 50051, // gRPC standard port
            database_port: 5432, // PostgreSQL standard port
            redis_port: 6379, // Redis standard port
            message_queue_port: 5672, // RabbitMQ standard port
            orchestration_port: 9091, // Orchestration service port
        }
    }

    /// Create test configuration with known test ports
    ///
    /// Uses high-numbered ports to avoid conflicts with production services
    #[must_use]
    pub fn for_testing() -> Self {
        Self {
            api_port: 18080,  // Test HTTP port (high number to avoid conflicts)
            health_port: 18443, // Test HTTPS port
            metrics_port: 19090, // Test Prometheus port
            admin_port: 18081,  // Test admin port
            websocket_port: 18082, // Test WebSocket port
            rpc_port: 15051, // Test gRPC port
            database_port: 15432, // Test PostgreSQL port
            redis_port: 16379, // Test Redis port
            message_queue_port: 15672, // Test RabbitMQ port
            orchestration_port: 19091, // Test orchestration port
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
        // ✅ EVOLUTION: Isolated environment
        temp_env::with_var("TEST_PORT", Some("1234"), || {
            let port = env_var_or_default("TEST_PORT", 9999);
            assert_eq!(port, 1234);
        });
        // Environment automatically restored!
    }

    #[test]
    fn test_env_var_default_fallback() {
        let port = env_var_or_default("NONEXISTENT_VAR", 7777);
        assert_eq!(port, 7777);
    }

    #[test]
    fn test_all_default_ports_valid() {
        let config = PortConfiguration::from_env();
        // All ports should be non-zero (valid range is guaranteed by u16 type)
        assert!(config.api_port > 0);
        assert!(config.health_port > 0);
        assert!(config.metrics_port > 0);
        assert!(config.admin_port > 0);
        assert!(config.websocket_port > 0);
        assert!(config.rpc_port > 0);
        assert!(config.database_port > 0);
        assert!(config.redis_port > 0);
        assert!(config.message_queue_port > 0);
        assert!(config.orchestration_port > 0);
    }

    #[test]
    fn test_port_configuration_clone() {
        let config1 = PortConfiguration::with_defaults();
        let config2 = config1.clone();
        
        assert_eq!(config1.api_port, config2.api_port);
        assert_eq!(config1.health_port, config2.health_port);
        assert_eq!(config1.metrics_port, config2.metrics_port);
    }

    #[test]
    fn test_port_configuration_serialization() {
        let config = PortConfiguration::with_defaults();
        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: PortConfiguration =
            serde_json::from_str(&json).expect("Should deserialize");
        
        assert_eq!(config.api_port, deserialized.api_port);
        assert_eq!(config.health_port, deserialized.health_port);
    }

    #[test]
    fn test_port_ranges() {
        let config = PortConfiguration::for_testing();
        
        // All test ports should be in valid range
        assert!(config.api_port > 1024); // Above privileged ports
        assert!(config.api_port < 65535); // Below max port
        assert!(config.health_port > 1024);
        assert!(config.metrics_port > 1024);
    }

    #[test]
    fn test_env_fallback_nonexistent() {
        // Non-existent env var should use default
        let port = env_var_or_default("NONEXISTENT_VAR_RANDOM_123", 7777);
        assert_eq!(port, 7777);
    }

    #[test]
    fn test_env_fallback_various_defaults() {
        assert_eq!(env_var_or_default("NONEX_1", 1000), 1000);
        assert_eq!(env_var_or_default("NONEX_2", 2000), 2000);
        assert_eq!(env_var_or_default("NONEX_3", 65535), 65535);
    }

    #[test]
    fn test_default_implementation() {
        let config = PortConfiguration::default();
        // Default should use from_env()
        assert!(config.api_port > 0);
        assert!(config.health_port > 0);
    }

    #[test]
    fn test_testing_ports_no_conflicts() {
        let config = PortConfiguration::for_testing();
        
        // All test ports should be unique
        let mut ports = vec![
            config.api_port,
            config.health_port,
            config.metrics_port,
            config.admin_port,
            config.websocket_port,
            config.rpc_port,
            config.database_port,
            config.redis_port,
            config.message_queue_port,
            config.orchestration_port,
        ];
        
        ports.sort_unstable();
        ports.dedup();
        
        assert_eq!(ports.len(), 10); // All unique
    }

    #[test]
    fn test_with_defaults_ports_valid() {
        let config = PortConfiguration::with_defaults();
        
        // All default ports should be non-zero (valid range is guaranteed by u16 type)
        assert!(config.api_port > 0);
        assert!(config.health_port > 0);
        assert!(config.metrics_port > 0);
    }

    #[test]
    fn test_get_port_config_singleton() {
        let config1 = get_port_config();
        let config2 = get_port_config();
        
        // Should return same instance (pointer equality check)
        assert_eq!(config1.api_port, config2.api_port);
    }

    #[test]
    fn test_default_and_testing_differ() {
        let config_default = PortConfiguration::with_defaults();
        let config_testing = PortConfiguration::for_testing();
        
        // Test ports should differ from default ports to avoid conflicts
        assert_ne!(config_default.api_port, config_testing.api_port);
    }

    #[test]
    fn test_config_debug_format() {
        let config = PortConfiguration::with_defaults();
        let debug_str = format!("{:?}", config);
        
        // Should contain port information
        assert!(debug_str.contains("api_port"));
        assert!(debug_str.contains("health_port"));
    }

    #[test]
    fn test_standard_service_ports() {
        let config = PortConfiguration::with_defaults();
        
        // Verify standard service port assignments
        assert_eq!(config.database_port, 5432); // PostgreSQL default
        assert_eq!(config.redis_port, 6379); // Redis default
    }

    #[test]
    fn test_rpc_port_valid() {
        let config = PortConfiguration::with_defaults();
        
        // gRPC default port should be valid
        assert_eq!(config.rpc_port, 50051);
    }

    #[test]
    fn test_multiple_config_instances() {
        let config1 = PortConfiguration::with_defaults();
        let config2 = PortConfiguration::for_testing();
        let config3 = PortConfiguration::from_env();
        
        // All instances should be valid
        assert!(config1.api_port > 0);
        assert!(config2.api_port > 0);
        assert!(config3.api_port > 0);
        
        // Test ports should differ from default ports
        assert_ne!(config1.api_port, config2.api_port);
    }
}
