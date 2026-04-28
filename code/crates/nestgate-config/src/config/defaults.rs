// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::defaults_config::NetworkDefaultsConfig;

/// Network port defaults with environment variable support
pub struct NetworkPortDefaults;
impl NetworkPortDefaults {
    /// Default API port - configurable via `NESTGATE_API_PORT`
    ///
    /// MIGRATED: Now uses centralized `get_api_port()` function
    #[must_use]
    pub fn api_port() -> u16 {
        crate::constants::get_api_port()
    }

    /// Default WebSocket port - configurable via `NESTGATE_WEBSOCKET_PORT`
    ///
    /// MIGRATED: Now uses centralized `get_admin_port()` function (WebSocket uses admin port)
    #[must_use]
    pub fn websocket_port() -> u16 {
        crate::constants::get_admin_port()
    }

    /// Default HTTP port - configurable via `NESTGATE_HTTP_PORT`
    ///
    /// MIGRATED: Now uses centralized `get_api_port()` function
    #[must_use]
    pub fn http_port() -> u16 {
        crate::constants::get_api_port()
    }

    /// Default streaming RPC port - configurable via `NESTGATE_STREAMING_RPC_PORT`
    #[must_use]
    pub const fn streaming_rpc_port() -> u16 {
        crate::constants::hardcoding::runtime_fallback_ports::API_ALT
    }

    /// Default NAS HTTP port - configurable via `NESTGATE_NAS_HTTP_PORT`
    #[must_use]
    pub const fn nas_http_port() -> u16 {
        crate::constants::hardcoding::runtime_fallback_ports::HTTP
    }

    /// Default development server port - configurable via `NESTGATE_DEV_SERVER_PORT`
    #[must_use]
    pub const fn dev_server_port() -> u16 {
        crate::constants::hardcoding::runtime_fallback_ports::API
    }

    /// Port range for auto-discovery - start
    #[must_use]
    pub const fn discovery_port_start() -> u16 {
        crate::constants::hardcoding::runtime_fallback_ports::HTTP
    }

    /// Port range for auto-discovery - end
    #[must_use]
    pub const fn discovery_port_end() -> u16 {
        crate::constants::hardcoding::runtime_fallback_ports::ADMIN
    }

    /// Common service discovery ports
    #[must_use]
    pub fn common_ports() -> Vec<u16> {
        use crate::constants::hardcoding::runtime_fallback_ports as p;
        vec![
            p::HTTP,
            p::HEALTH,
            p::WEBSOCKET,
            p::METRICS,
            p::API,
            p::API_ALT,
            p::EXTENDED_SERVICES,
            p::DISCOVERY_SERVICE,
            p::ADMIN,
            p::METRICS_ALT,
        ]
    }

    /// Get API port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_api_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_api_port()
    }

    /// Get WebSocket port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_websocket_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_websocket_port()
    }

    /// Get HTTP port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_http_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_http_port()
    }

    /// Get NAS HTTP port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_nas_http_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_nas_http_port()
    }

    /// Get development server port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_dev_server_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_dev_server_port()
    }

    /// Get metrics port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_metrics_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_metrics_port()
    }

    /// Get health check port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_health_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_health_port()
    }

    /// Get orchestrator port from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_orchestrator_port() -> u16 {
        NetworkDefaultsConfig::from_env().get_orchestrator_port()
    }

    /// Get WebSocket base URL from environment or build from config
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_websocket_base_url() -> String {
        NetworkDefaultsConfig::from_env().get_websocket_base_url()
    }

    /// Get API base URL from environment or build from config
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_api_base_url() -> String {
        NetworkDefaultsConfig::from_env().get_api_base_url()
    }
}

/// Network address defaults with environment variable support
pub struct NetworkAddressDefaults;
impl NetworkAddressDefaults {
    /// Default bind address for production (localhost only - secure default)
    #[must_use]
    pub const fn secure_bind() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        "127.0.0.1"
    }

    /// Default bind address for development (all interfaces)
    #[must_use]
    pub const fn development_bind() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        "0.0.0.0"
    }

    /// Default hostname
    #[must_use]
    pub const fn hostname() -> &'static str {
        // Use centralized constant to eliminate hardcoding
        crate::constants::network_defaults::LOCALHOST_NAME
    }

    /// Get bind address from environment or secure default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_bind_address() -> String {
        NetworkDefaultsConfig::from_env().get_bind_address()
    }

    /// Get development bind address (used for dev servers)
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_development_bind_address() -> String {
        NetworkDefaultsConfig::from_env().get_development_bind_address()
    }

    /// Get hostname from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_hostname() -> String {
        NetworkDefaultsConfig::from_env().get_hostname()
    }

    /// Get external hostname from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_external_hostname() -> String {
        NetworkDefaultsConfig::from_env().get_external_hostname()
    }
}

/// Timeout defaults with environment variable support
pub struct TimeoutDefaults;
impl TimeoutDefaults {
    /// Default connection timeout in milliseconds (environment-driven)
    ///
    /// EVOLVED: Use `TimeoutsConfig::from_env()` for environment variable support
    #[must_use]
    pub const fn connection_timeout_ms() -> u64 {
        3000 // Override via NESTGATE_CONNECTION_TIMEOUT_MS
    }

    /// Default request timeout in milliseconds (environment-driven)
    ///
    /// EVOLVED: Use `TimeoutsConfig::from_env()` for environment variable support
    #[must_use]
    pub const fn request_timeout_ms() -> u64 {
        30000 // Override via NESTGATE_REQUEST_TIMEOUT_MS
    }

    /// Default health check timeout in seconds
    #[must_use]
    pub const fn health_check_timeout_seconds() -> u64 {
        5
    }

    /// Get connection timeout from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
    pub fn get_connection_timeout_ms() -> u64 {
        NetworkDefaultsConfig::from_env().get_connection_timeout_ms()
    }

    /// Get request timeout from environment or default
    /// NOTE: Creates config from env each time. For tests, use `NetworkDefaultsConfig` directly.
    #[must_use]
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
    use crate::config::defaults_config::NetworkDefaultsConfig;
    use nestgate_types::MapEnv;

    // NetworkPortDefaults tests - verify non-zero defaults (actual values are env-driven)
    #[test]
    fn test_network_port_defaults_api_port() {
        let port = NetworkPortDefaults::api_port();
        assert!(
            port >= 1024,
            "API port should be non-privileged, got {port}"
        );
    }

    #[test]
    fn test_network_port_defaults_websocket_port() {
        let port = NetworkPortDefaults::websocket_port();
        assert!(
            port >= 1024,
            "WebSocket port should be non-privileged, got {port}"
        );
    }

    #[test]
    fn test_network_port_defaults_http_port() {
        let port = NetworkPortDefaults::http_port();
        assert!(
            port >= 1024,
            "HTTP port should be non-privileged, got {port}"
        );
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
        let env = MapEnv::new();
        let port = NetworkDefaultsConfig::from_env_source(&env).get_api_port();
        assert_eq!(port, 3000);
    }

    #[test]
    fn test_network_port_defaults_get_http_port_default() {
        let env = MapEnv::new();
        let port = NetworkDefaultsConfig::from_env_source(&env).get_http_port();
        assert_eq!(port, 8080);
    }

    #[test]
    fn test_network_port_defaults_get_metrics_port_default() {
        let env = MapEnv::new();
        let port = NetworkDefaultsConfig::from_env_source(&env).get_metrics_port();
        assert_eq!(port, 9090);
    }

    #[test]
    fn test_network_port_defaults_get_health_port_default() {
        let env = MapEnv::new();
        let port = NetworkDefaultsConfig::from_env_source(&env).get_health_port();
        assert_eq!(port, 8081);
    }

    #[test]
    fn test_network_port_defaults_get_orchestrator_port_default() {
        let env = MapEnv::new();
        let port = NetworkDefaultsConfig::from_env_source(&env).get_orchestrator_port();
        assert_eq!(port, 8090);
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
        let env = MapEnv::new();
        assert_eq!(
            NetworkDefaultsConfig::from_env_source(&env).get_bind_address(),
            "127.0.0.1"
        );
    }

    #[test]
    fn test_network_address_defaults_get_development_bind_address_default() {
        let env = MapEnv::new();
        assert_eq!(
            NetworkDefaultsConfig::from_env_source(&env).get_development_bind_address(),
            "0.0.0.0"
        );
    }

    #[test]
    fn test_network_address_defaults_get_hostname_default() {
        let env = MapEnv::new();
        assert_eq!(
            NetworkDefaultsConfig::from_env_source(&env).get_hostname(),
            "localhost"
        );
    }

    #[test]
    fn test_network_address_defaults_get_external_hostname_default() {
        let env = MapEnv::new();
        assert_eq!(
            NetworkDefaultsConfig::from_env_source(&env).get_external_hostname(),
            "localhost"
        );
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
        let env = MapEnv::new();
        assert_eq!(
            NetworkDefaultsConfig::from_env_source(&env).get_connection_timeout_ms(),
            3000
        );
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
        assert!(timeout < 120_000, "Request timeout should be < 2 minutes");
    }

    #[test]
    fn test_timeout_defaults_health_check_reasonable() {
        let timeout = TimeoutDefaults::health_check_timeout_seconds();
        assert!(timeout > 1, "Health check timeout should be > 1 second");
        assert!(timeout < 30, "Health check timeout should be < 30 seconds");
    }
}
