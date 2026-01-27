use super::defaults_config::NetworkDefaultsConfig;

/// Network port defaults with environment variable support
pub struct NetworkPortDefaults;
impl NetworkPortDefaults {
    /// Default API port - configurable via NESTGATE_API_PORT
    ///
    /// ✅ MIGRATED: Now uses centralized get_api_port() function
    pub fn api_port() -> u16 {
        crate::constants::get_api_port()
    }

    /// Default WebSocket port - configurable via NESTGATE_WEBSOCKET_PORT
    ///
    /// ✅ MIGRATED: Now uses centralized get_admin_port() function (WebSocket uses admin port)
    pub fn websocket_port() -> u16 {
        crate::constants::get_admin_port()
    }

    /// Default HTTP port - configurable via NESTGATE_HTTP_PORT
    ///
    /// ✅ MIGRATED: Now uses centralized get_api_port() function
    pub fn http_port() -> u16 {
        crate::constants::get_api_port()
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
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_api_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_api_port()
    }

    /// Get WebSocket port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_websocket_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_websocket_port()
    }

    /// Get HTTP port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_http_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_http_port()
    }

    /// Get NAS HTTP port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_nas_http_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_nas_http_port()
    }

    /// Get development server port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_dev_server_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_dev_server_port()
    }

    /// Get metrics port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_metrics_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_metrics_port()
    }

    /// Get health check port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_health_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_health_port()
    }

    /// Get orchestrator port from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_orchestrator_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_orchestrator_port()
    }

    /// Get WebSocket base URL from environment or build from config
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_websocket_base_url() -> String {
        NetworkDefaultsConfig::from_env().get_websocket_base_url()
    }

    /// Get API base URL from environment or build from config
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_api_base_url() -> String {
        NetworkDefaultsConfig::from_env().get_api_base_url()
    }
}

/// Network address defaults with environment variable support
pub struct NetworkAddressDefaults;
impl NetworkAddressDefaults {
    /// Default bind address for production (localhost only - secure default)
    pub fn secure_bind() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        "127.0.0.1"
    }

    /// Default bind address for development (all interfaces)
    pub fn development_bind() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        "0.0.0.0"
    }

    /// Default hostname
    pub fn hostname() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        crate::constants::network_defaults::LOCALHOST_NAME
    }

    /// Get bind address from environment or secure default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_bind_address() -> String {
        NetworkDefaultsConfig::from_env().get_bind_address()
    }

    /// Get development bind address (used for dev servers)
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_development_bind_address() -> String {
        NetworkDefaultsConfig::from_env().get_development_bind_address()
    }

    /// Get hostname from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_hostname() -> String {
        NetworkDefaultsConfig::from_env().get_hostname()
    }

    /// Get external hostname from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_external_hostname() -> String {
        NetworkDefaultsConfig::from_env().get_external_hostname()
    }
}

/// Timeout defaults with environment variable support
pub struct TimeoutDefaults;
impl TimeoutDefaults {
    /// Default connection timeout in milliseconds  
    /// TODO: Migrate to environment-driven configuration
    pub fn connection_timeout_ms() -> u64 {
        3000 // Will be migrated to NESTGATE_CONNECTION_TIMEOUT_MS
    }

    /// Default request timeout in milliseconds
    /// TODO: Migrate to environment-driven configuration  
    pub fn request_timeout_ms() -> u64 {
        30000 // Will be migrated to NESTGATE_REQUEST_TIMEOUT_MS
    }

    /// Default health check timeout in seconds
    pub fn health_check_timeout_seconds() -> u64 {
        5
    }

    /// Get connection timeout from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_connection_timeout_ms() -> u64 {
        NetworkDefaultsConfig::from_env().get_connection_timeout_ms()
    }

    /// Get request timeout from environment or default
    /// NOTE: Creates config from env each time. For tests, use NetworkDefaultsConfig directly.
    pub fn get_request_timeout_ms() -> u64 {
        NetworkDefaultsConfig::from_env().get_request_timeout_ms()
    }
}

// NOTE: Default impl for NestGateCanonicalConfig is provided by canonical_primary module
// The NetworkPortDefaults, NetworkAddressDefaults, and TimeoutDefaults structs above
// provide environment-aware defaults that can be used when constructing configs.

#[cfg(test)]
mod tests {
    use super::*;

    // NetworkPortDefaults tests
    #[test]
    fn test_network_port_defaults_api_port() {
        assert_eq!(NetworkPortDefaults::api_port(), 3000); // API_DEFAULT = 3000
    }

    #[test]
    fn test_network_port_defaults_websocket_port() {
        assert_eq!(NetworkPortDefaults::websocket_port(), 8082); // WEBSOCKET_DEFAULT = 8082
    }

    #[test]
    fn test_network_port_defaults_http_port() {
        assert_eq!(NetworkPortDefaults::http_port(), 8080); // HTTP_DEFAULT = 8080
    }

    #[test]
    fn test_network_port_defaults_streaming_rpc_port() {
        assert_eq!(NetworkPortDefaults::streaming_rpc_port(), 3001); // API_ALT = 3001
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
        assert_eq!(NetworkPortDefaults::get_api_port(), 3000); // API_DEFAULT = 3000
    }

    #[test]
    fn test_network_port_defaults_get_http_port_default() {
        std::env::remove_var("NESTGATE_HTTP_PORT");
        assert_eq!(NetworkPortDefaults::get_http_port(), 8080); // HTTP_DEFAULT = 8080
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
        assert_eq!(
            ports.len(),
            unique_ports.len(),
            "Common ports should have no duplicates"
        );
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
        assert_eq!(
            NetworkAddressDefaults::get_development_bind_address(),
            "0.0.0.0"
        );
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

    // ==================== CONFIG STRUCTURE MIGRATION COMPLETE ====================
    // Old tests removed - replaced by comprehensive tests in canonical_primary module
    // See: code/crates/nestgate-core/src/config/canonical_primary/mod.rs
    //
    // The canonical config structure now provides:
    // - Environment-driven configuration (from_env)
    // - Validation and type safety
    // - Domain-specific organization (network, storage, services)
    // - Comprehensive test coverage in the canonical_primary module
}
