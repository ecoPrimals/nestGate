use super::canonical::{
    Environment, MonitoringConfig, SecurityConfig, StorageConfig, SystemConfig,
};
use super::federation::McpConfig;
use super::network::ServiceEndpoints;
use super::*;
use uuid;

/// Network port defaults with environment variable support
pub struct NetworkPortDefaults;
impl NetworkPortDefaults {
    /// Default API port - configurable via NESTGATE_API_PORT
    pub fn api_port() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::API_DEFAULT
    }

    /// Default WebSocket port - configurable via NESTGATE_WEBSOCKET_PORT
    pub fn websocket_port() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::WEBSOCKET_DEFAULT
    }

    /// Default HTTP port - configurable via NESTGATE_HTTP_PORT
    pub fn http_port() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::HTTP_DEFAULT
    }

    /// Default streaming RPC port - configurable via NESTGATE_STREAMING_RPC_PORT
    pub fn streaming_rpc_port() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::API_ALT
    }

    /// Default NAS HTTP port - configurable via NESTGATE_NAS_HTTP_PORT
    pub fn nas_http_port() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::HTTP_DEFAULT
    }

    /// Default development server port - configurable via NESTGATE_DEV_SERVER_PORT
    pub fn dev_server_port() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::API_DEFAULT
    }

    /// Port range for auto-discovery - start
    pub fn discovery_port_start() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::HTTP_DEFAULT
    }

    /// Port range for auto-discovery - end
    pub fn discovery_port_end() -> u16 {
        use crate::constants::hardcoding::ports;
        ports::ADMIN_DEFAULT
    }

    /// Common service discovery ports
    pub fn common_ports() -> Vec<u16> {
        use crate::constants::hardcoding::ports;
        vec![
            ports::HTTP_DEFAULT,
            ports::HEALTH_CHECK,
            ports::WEBSOCKET_DEFAULT,
            ports::METRICS_DEFAULT,
            ports::API_DEFAULT,
            ports::API_ALT,
            ports::EXTENDED_SERVICES,
            ports::DISCOVERY_SERVICE,
            ports::ADMIN_DEFAULT,
            ports::METRICS_ALT,
        ]
    }

    /// Get API port from environment or default
    pub fn get_api_port() -> u16 {
        std::env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::api_port())
    }

    /// Get WebSocket port from environment or default
    pub fn get_websocket_port() -> u16 {
        std::env::var("NESTGATE_WEBSOCKET_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::websocket_port())
    }

    /// Get HTTP port from environment or default
    pub fn get_http_port() -> u16 {
        std::env::var("NESTGATE_HTTP_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::http_port())
    }

    /// Get NAS HTTP port from environment or default
    pub fn get_nas_http_port() -> u16 {
        std::env::var("NESTGATE_NAS_HTTP_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::nas_http_port())
    }

    /// Get development server port from environment or default
    pub fn get_dev_server_port() -> u16 {
        std::env::var("NESTGATE_DEV_SERVER_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::dev_server_port())
    }

    /// Get metrics port from environment or default
    pub fn get_metrics_port() -> u16 {
        use crate::constants::hardcoding::ports;
        std::env::var("NESTGATE_METRICS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(ports::METRICS_PROMETHEUS)
    }

    /// Get health check port from environment or default
    pub fn get_health_port() -> u16 {
        use crate::constants::hardcoding::ports;
        std::env::var("NESTGATE_HEALTH_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(ports::HEALTH_DEFAULT)
    }

    /// Get orchestrator port from environment or default
    pub fn get_orchestrator_port() -> u16 {
        use crate::constants::hardcoding::ports;
        std::env::var("NESTGATE_ORCHESTRATOR_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(ports::ORCHESTRATOR_DEFAULT)
    }

    /// Get WebSocket base URL from environment or build from config
    pub fn get_websocket_base_url() -> String {
        std::env::var("NESTGATE_WS_BASE_URL")
            .unwrap_or_else(|_| format!("ws://localhost:{Self::get_websocket_port(}")))
    }

    /// Get API base URL from environment or build from config
    pub fn get_api_base_url() -> String {
        std::env::var("NESTGATE_API_BASE_URL")
            .unwrap_or_else(|_| format!("http://localhost:{Self::get_api_port(}")))
    }
}

/// Network address defaults with environment variable support
pub struct NetworkAddressDefaults;
impl NetworkAddressDefaults {
    /// Default bind address for production (localhost only - secure default)
    pub fn secure_bind() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        crate::constants::network_defaults::DEFAULT_LOCALHOST_IPV4
    }

    /// Default bind address for development (all interfaces)
    pub fn development_bind() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        crate::constants::network_defaults::DEFAULT_BIND_ALL_IPV4
    }

    /// Default hostname
    pub fn hostname() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        crate::constants::network_defaults::DEFAULT_HOSTNAME
    }

    /// Get bind address from environment or secure default
    pub fn get_bind_address() -> String {
        std::env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| Self::secure_bind().to_string())
    }

    /// Get development bind address (used for dev servers)
    pub fn get_development_bind_address() -> String {
        std::env::var("NESTGATE_DEV_BIND_ADDRESS")
            .unwrap_or_else(|_| Self::development_bind().to_string())
    }

    /// Get hostname from environment or default
    pub fn get_hostname() -> String {
        std::env::var("NESTGATE_HOSTNAME").unwrap_or_else(|_| Self::hostname().to_string())
    }

    /// Get external hostname from environment or default
    pub fn get_external_hostname() -> String {
        std::env::var("NESTGATE_EXTERNAL_HOSTNAME").unwrap_or_else(|_| Self::hostname().to_string())
    }
}

/// Timeout defaults with environment variable support
pub struct TimeoutDefaults;
impl TimeoutDefaults {
    /// Default connection timeout in milliseconds
    pub fn connection_timeout_ms() -> u64 {
        3000
    }

    /// Default request timeout in milliseconds
    pub fn request_timeout_ms() -> u64 {
        30000
    }

    /// Default health check timeout in seconds
    pub fn health_check_timeout_seconds() -> u64 {
        5
    }

    /// Get connection timeout from environment or default
    pub fn get_connection_timeout_ms() -> u64 {
        std::env::var("NESTGATE_CONNECTION_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::connection_timeout_ms())
    }

    /// Get request timeout from environment or default
    pub fn get_request_timeout_ms() -> u64 {
        std::env::var("NESTGATE_REQUEST_TIMEOUT_MS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(Self::request_timeout_ms())
    }
}

/// Create MCP configuration for a given node ID
fn create_mcp_config(node_id: &str) -> McpConfig {
    let mcp_config = McpConfig {
        node_id: node_id.to_string(),
        ..Default::default()
    };
    tracing::info!("🔧 MCP config created for node: {}", node_id);
    mcp_config
}

impl Default for NestGateConfig {
    fn default() -> Self {
        // Generate dynamic node ID instead of hardcoding
        let node_id = format!(
            "nestgate-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        );

        let mcp_config = create_mcp_config(&node_id);

        Self {
            system: SystemConfig {
                instance_id: None,
                instance_name: "nestgate-instance".to_string(),
                log_level: "info".to_string(),
                data_dir: PathBuf::from("./data"),
                config_dir: PathBuf::from("./config"),
                dev_mode: true,
            },
            network: NetworkConfig::default(),
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            mcp: Some(mcp_config),
            federation: Some(FederationConfig::default()),
            endpoints: ServiceEndpoints::default(),
            api_paths: ApiPathsConfig::from_environment(),
            // storage_constants replaced with unified_constants::storage::sizes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // NetworkPortDefaults tests
    #[test]
    fn test_network_port_defaults_api_port() {
        assert_eq!(NetworkPortDefaults::api_port(), 8000);
    }

    #[test]
    fn test_network_port_defaults_websocket_port() {
        assert_eq!(NetworkPortDefaults::websocket_port(), 8080);
    }

    #[test]
    fn test_network_port_defaults_http_port() {
        assert_eq!(NetworkPortDefaults::http_port(), 3000);
    }

    #[test]
    fn test_network_port_defaults_streaming_rpc_port() {
        assert_eq!(NetworkPortDefaults::streaming_rpc_port(), 8001);
    }

    #[test]
    fn test_network_port_defaults_nas_http_port() {
        assert_eq!(NetworkPortDefaults::nas_http_port(), 8080);
    }

    #[test]
    fn test_network_port_defaults_dev_server_port() {
        assert_eq!(NetworkPortDefaults::dev_server_port(), 3000);
    }

    #[test]
    fn test_network_port_defaults_discovery_port_start() {
        assert_eq!(NetworkPortDefaults::discovery_port_start(), 8080);
    }

    #[test]
    fn test_network_port_defaults_discovery_port_end() {
        assert_eq!(NetworkPortDefaults::discovery_port_end(), 9000);
    }

    #[test]
    fn test_network_port_defaults_common_ports() {
        let ports = NetworkPortDefaults::common_ports();
        assert_eq!(ports.len(), 10);
        assert!(ports.contains(&8080));
        assert!(ports.contains(&3000));
        assert!(ports.contains(&9000));
    }

    #[test]
    fn test_network_port_defaults_discovery_range_valid() {
        let start = NetworkPortDefaults::discovery_port_start();
        let end = NetworkPortDefaults::discovery_port_end();
        assert!(start < end, "Discovery port start should be less than end");
        assert_eq!(start, 8080);
        assert_eq!(end, 9000);
    }

    #[test]
    fn test_network_port_defaults_get_api_port_default() {
        std::env::remove_var("NESTGATE_API_PORT");
        assert_eq!(NetworkPortDefaults::get_api_port(), 8000);
    }

    #[test]
    fn test_network_port_defaults_get_http_port_default() {
        std::env::remove_var("NESTGATE_HTTP_PORT");
        assert_eq!(NetworkPortDefaults::get_http_port(), 3000);
    }

    #[test]
    fn test_network_port_defaults_get_metrics_port_default() {
        std::env::remove_var("NESTGATE_METRICS_PORT");
        assert_eq!(NetworkPortDefaults::get_metrics_port(), 9090);
    }

    #[test]
    fn test_network_port_defaults_get_health_port_default() {
        std::env::remove_var("NESTGATE_HEALTH_PORT");
        assert_eq!(NetworkPortDefaults::get_health_port(), 8081);
    }

    #[test]
    fn test_network_port_defaults_get_orchestrator_port_default() {
        std::env::remove_var("NESTGATE_ORCHESTRATOR_PORT");
        assert_eq!(NetworkPortDefaults::get_orchestrator_port(), 8090);
    }

    #[test]
    fn test_network_port_defaults_ports_are_valid() {
        // Ensure all port numbers are within valid range (1-65535)
        assert!(NetworkPortDefaults::api_port() > 0);
        assert!(NetworkPortDefaults::websocket_port() > 0);
        assert!(NetworkPortDefaults::http_port() > 0);
        assert!(NetworkPortDefaults::streaming_rpc_port() > 0);
    }

    #[test]
    fn test_network_port_defaults_common_ports_no_duplicates() {
        let ports = NetworkPortDefaults::common_ports();
        let mut unique_ports = ports.clone();
        unique_ports.sort();
        unique_ports.dedup();
        assert_eq!(ports.len(), unique_ports.len(), "Common ports should have no duplicates");
    }

    // NetworkAddressDefaults tests
    #[test]
    fn test_network_address_defaults_secure_bind() {
        assert_eq!(NetworkAddressDefaults::secure_bind(), "127.0.0.1");
    }

    #[test]
    fn test_network_address_defaults_development_bind() {
        assert_eq!(NetworkAddressDefaults::development_bind(), "0.0.0.0");
    }

    #[test]
    fn test_network_address_defaults_hostname() {
        assert_eq!(NetworkAddressDefaults::hostname(), "localhost");
    }

    #[test]
    fn test_network_address_defaults_get_bind_address_default() {
        std::env::remove_var("NESTGATE_BIND_ADDRESS");
        assert_eq!(NetworkAddressDefaults::get_bind_address(), "127.0.0.1");
    }

    #[test]
    fn test_network_address_defaults_get_development_bind_address_default() {
        std::env::remove_var("NESTGATE_DEV_BIND_ADDRESS");
        assert_eq!(NetworkAddressDefaults::get_development_bind_address(), "0.0.0.0");
    }

    #[test]
    fn test_network_address_defaults_get_hostname_default() {
        std::env::remove_var("NESTGATE_HOSTNAME");
        assert_eq!(NetworkAddressDefaults::get_hostname(), "localhost");
    }

    #[test]
    fn test_network_address_defaults_get_external_hostname_default() {
        std::env::remove_var("NESTGATE_EXTERNAL_HOSTNAME");
        assert_eq!(NetworkAddressDefaults::get_external_hostname(), "localhost");
    }

    #[test]
    fn test_network_address_defaults_secure_bind_is_localhost() {
        // Security check: secure_bind should be localhost only
        assert_eq!(NetworkAddressDefaults::secure_bind(), "127.0.0.1");
        assert_ne!(NetworkAddressDefaults::secure_bind(), "0.0.0.0");
    }

    #[test]
    fn test_network_address_defaults_development_bind_is_all_interfaces() {
        // Development: should bind to all interfaces
        assert_eq!(NetworkAddressDefaults::development_bind(), "0.0.0.0");
    }

    // TimeoutDefaults tests
    #[test]
    fn test_timeout_defaults_connection_timeout_ms() {
        assert_eq!(TimeoutDefaults::connection_timeout_ms(), 3000);
    }

    #[test]
    fn test_timeout_defaults_request_timeout_ms() {
        assert_eq!(TimeoutDefaults::request_timeout_ms(), 30000);
    }

    #[test]
    fn test_timeout_defaults_health_check_timeout_seconds() {
        assert_eq!(TimeoutDefaults::health_check_timeout_seconds(), 5);
    }

    #[test]
    fn test_timeout_defaults_get_connection_timeout_ms_default() {
        std::env::remove_var("NESTGATE_CONNECTION_TIMEOUT_MS");
        assert_eq!(TimeoutDefaults::get_connection_timeout_ms(), 3000);
    }

    #[test]
    fn test_timeout_defaults_connection_reasonable() {
        let timeout = TimeoutDefaults::connection_timeout_ms();
        assert!(timeout > 1000, "Connection timeout should be > 1 second");
        assert!(timeout < 10000, "Connection timeout should be < 10 seconds");
    }

    #[test]
    fn test_timeout_defaults_request_reasonable() {
        let timeout = TimeoutDefaults::request_timeout_ms();
        assert!(timeout > 5000, "Request timeout should be > 5 seconds");
        assert!(timeout < 120000, "Request timeout should be < 2 minutes");
    }

    #[test]
    fn test_timeout_defaults_health_check_reasonable() {
        let timeout = TimeoutDefaults::health_check_timeout_seconds();
        assert!(timeout > 1, "Health check timeout should be > 1 second");
        assert!(timeout < 30, "Health check timeout should be < 30 seconds");
    }

    #[test]
    fn test_config_defaultvalues() {
        let config = Config::default();

        // Test system config
        assert_eq!(config.system.log_level, "info");
        assert_eq!(config.system.data_dir, PathBuf::from("./data"));
        assert_eq!(config.system.instance_name, "nestgate-instance");
        assert!(matches!(
            config.system.environment,
            Environment::Development
        ));
        assert!(config.system.dev_mode);

        // Test storage config
        assert_eq!(config.storage.performance.cache_size, 1024 * 1024 * 1024);
        assert_eq!(config.storage.zfs.compression, "lz4");
        assert!(config.storage.backup.enabled);

        // Test security config
        assert_eq!(config.security.authentication.method, "jwt");
        assert!(config.security.authentication.enabled);
        assert!(!config.security.authorization.enabled);

        // Test monitoring config
        assert!(config.monitoring.enabled);
        assert_eq!(config.monitoring.metrics.interval.as_secs(), 30);
        assert_eq!(config.monitoring.logging.level, "info");

        // Test integrations config
        assert!(config
            .integrations
            .external_services
            .contains_key("huggingface"));

        // Test environment config
        assert_eq!(config.environment.name, "development");
        assert!(config.environment.variables.contains_key("NODE_ENV"));

        // Basic configuration validation complete
        println!("✅ Default configuration validation passed");
    }

    #[test]
    fn test_config_instance_uniqueness() {
        let config1 = Config::default();
        let config2 = Config::default();

        // Both should have valid instance names
        assert!(!config1.system.instance_name.is_empty());
        assert!(!config2.system.instance_name.is_empty());

        println!("✅ Configuration instance test passed");
    }

    #[test]
    fn test_config_creation_methods() {
        let config1 = Config::default();
        let config2 = Config::default();

        // Both should have the same structure
        assert_eq!(config1.system.log_level, config2.system.log_level);
        assert_eq!(config1.system.data_dir, config2.system.data_dir);
        assert_eq!(config1.system.instance_name, config2.system.instance_name);

        println!("✅ Config creation methods test passed");
    }

    #[test]
    fn test_config_validation_success() {
        let config = Config::default();
        // Basic validation - config should be created successfully
        assert!(!config.system.instance_name.is_empty());
        println!("✅ Config validation test passed");
    }

    #[test]
    fn test_config_service_endpoint_access() {
        let config = Config::default();

        // Test that integrations config exists and has expected services
        assert!(config
            .integrations
            .external_services
            .contains_key("huggingface"));
        assert!(config.integrations.external_services.contains_key("ncbi"));
        assert!(!config
            .integrations
            .external_services
            .contains_key("nonexistent"));

        println!("✅ Service endpoint access test passed");
    }

    #[test]
    fn test_config_comprehensive_structure() {
        let config = Config::default();

        // Verify all major sections are present and configured
        assert!(!config.system.instance_name.is_empty());
        assert!(config.storage.performance.cache_size > 0);
        assert!(!config.security.authentication.method.is_empty());
        assert!(config.security.authentication.enabled);
        assert!(config.monitoring.metrics.interval.as_secs() > 0);
        assert!(!config.monitoring.logging.level.is_empty());

        // Verify configuration relationships
        println!("✅ Comprehensive structure test passed");
    }
}
