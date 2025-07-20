use super::federation::McpConfig;
use super::*;
use uuid;

/// Network port defaults with environment variable support
pub struct NetworkPortDefaults;

impl NetworkPortDefaults {
    /// Default API port - configurable via NESTGATE_API_PORT
    pub const fn api_port() -> u16 {
        8000
    }

    /// Default WebSocket port - configurable via NESTGATE_WEBSOCKET_PORT
    pub const fn websocket_port() -> u16 {
        8080
    }

    /// Default HTTP port - configurable via NESTGATE_HTTP_PORT
    pub const fn http_port() -> u16 {
        3000
    }

    /// Default streaming RPC port - configurable via NESTGATE_STREAMING_RPC_PORT
    pub const fn streaming_rpc_port() -> u16 {
        8001
    }

    /// Default NAS HTTP port - configurable via NESTGATE_NAS_HTTP_PORT
    pub const fn nas_http_port() -> u16 {
        8080
    }

    /// Default development server port - configurable via NESTGATE_DEV_SERVER_PORT
    pub const fn dev_server_port() -> u16 {
        3000
    }

    /// Port range for auto-discovery - start
    pub const fn discovery_port_start() -> u16 {
        8080
    }

    /// Port range for auto-discovery - end
    pub const fn discovery_port_end() -> u16 {
        9000
    }

    /// Common service discovery ports
    pub fn common_ports() -> Vec<u16> {
        vec![8080, 8081, 8082, 8090, 3000, 3001, 3002, 3010, 9000, 9001]
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
}

/// Network address defaults with environment variable support
pub struct NetworkAddressDefaults;

impl NetworkAddressDefaults {
    /// Default bind address for production (localhost only - secure default)
    pub const fn secure_bind() -> &'static str {
        "127.0.0.1"
    }

    /// Default bind address for development (all interfaces)
    pub const fn development_bind() -> &'static str {
        "0.0.0.0"
    }

    /// Default hostname
    pub const fn hostname() -> &'static str {
        "localhost"
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
    pub const fn connection_timeout_ms() -> u64 {
        3000
    }

    /// Default request timeout in milliseconds
    pub const fn request_timeout_ms() -> u64 {
        30000
    }

    /// Default health check timeout in seconds
    pub const fn health_check_timeout_seconds() -> u64 {
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

impl Default for Config {
    fn default() -> Self {
        // Generate dynamic node ID instead of hardcoding
        let node_id = format!(
            "nestgate-{}",
            &uuid::Uuid::new_v4().simple().to_string()[..8]
        );

        let mcp_config = create_mcp_config(&node_id);

        Self {
            system: SystemConfig {
                log_level: "info".to_string(),
                // Use relative paths - Songbird manages absolute paths
                data_dir: "./data".to_string(),
                temp_dir: "./tmp".to_string(),
                max_concurrent_ops: 1000,
                node_id: node_id.clone(),
                environment: "development".to_string(),
            },
            storage: StorageConfig::default(),
            security: SecurityConfig::default(),
            monitoring: MonitoringConfig::default(),
            mcp: Some(mcp_config),
            federation: Some(FederationConfig::default()),
            endpoints: ServiceEndpoints::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default_values() {
        let config = Config::default();

        // Test system config
        assert_eq!(config.system.log_level, "info");
        assert_eq!(config.system.data_dir, "./data");
        assert_eq!(config.system.temp_dir, "./tmp");
        assert_eq!(config.system.max_concurrent_ops, 1000);
        assert_eq!(config.system.environment, "development");
        assert!(!config.system.node_id.is_empty());
        assert!(config.system.node_id.starts_with("nestgate-"));

        // Test storage config
        assert_eq!(config.storage.cache_size, 1024 * 1024 * 1024);
        assert_eq!(config.storage.max_file_size, 1024 * 1024 * 1024 * 100);

        // Test security config
        assert_eq!(config.security.auth_method, "jwt");
        assert_eq!(config.security.encryption_algorithm, "aes-256-gcm");
        assert_eq!(config.security.key_rotation_days, 30);
        assert_eq!(config.security.max_failed_attempts, 5);

        // Test monitoring config
        assert_eq!(config.monitoring.metrics_interval, 30);
        assert_eq!(config.monitoring.log_level, "info");

        // Test MCP config
        assert!(config.mcp.is_some());
        let mcp = config.mcp.as_ref().unwrap();
        assert!(!mcp.enabled);
        assert!(!mcp.federation_enabled);

        // Test federation config
        assert!(config.federation.is_some());
        let federation = config.federation.as_ref().unwrap();
        assert!(!federation.enabled);
        assert_eq!(federation.mode, "standalone");
        assert!(federation.peers.is_empty());

        // Test endpoints config - Universal architecture has external services only by default
        // Primal services (beardog, songbird, squirrel, toadstool) are discovered dynamically
        assert!(config.endpoints.has_service("huggingface"));
        assert!(config.endpoints.has_service("ncbi"));
        assert!(!config.endpoints.api_base_url.is_empty());
        assert!(!config.endpoints.websocket_base_url.is_empty());
        assert!(!config.endpoints.static_base_url.is_empty());
    }

    #[test]
    fn test_config_node_id_uniqueness() {
        let config1 = Config::default();
        let config2 = Config::default();

        // Node IDs should be different
        assert_ne!(config1.system.node_id, config2.system.node_id);

        // Both should start with nestgate-
        assert!(config1.system.node_id.starts_with("nestgate-"));
        assert!(config2.system.node_id.starts_with("nestgate-"));

        // MCP node IDs should match system node IDs
        assert_eq!(
            config1.system.node_id,
            config1.mcp.as_ref().unwrap().node_id
        );
        assert_eq!(
            config2.system.node_id,
            config2.mcp.as_ref().unwrap().node_id
        );
    }

    #[test]
    fn test_config_creation_methods() {
        let config1 = Config::new();
        let config2 = Config::default();

        // Both should have the same structure (but different node IDs)
        assert_eq!(config1.system.log_level, config2.system.log_level);
        assert_eq!(config1.system.data_dir, config2.system.data_dir);
        assert_eq!(config1.system.temp_dir, config2.system.temp_dir);
        assert_eq!(
            config1.system.max_concurrent_ops,
            config2.system.max_concurrent_ops
        );
        assert_eq!(config1.system.environment, config2.system.environment);

        // Node IDs should be different
        assert_ne!(config1.system.node_id, config2.system.node_id);
    }

    #[test]
    fn test_config_validation_success() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_service_endpoint_access() {
        let config = Config::default();

        // Test external service endpoints (always available)
        assert!(config.get_endpoint("huggingface").is_some());
        assert!(config.get_endpoint("ncbi").is_some());
        assert!(config.get_endpoint("nonexistent").is_none());

        // Test external endpoint values
        assert_eq!(
            config.get_endpoint("huggingface"),
            Some("https://api.huggingface.co")
        );
        assert_eq!(
            config.get_endpoint("ncbi"),
            Some("https://api.ncbi.nlm.nih.gov")
        );

        // Universal architecture: Primal endpoints are discovered dynamically
        // They're only hardcoded when NESTGATE_ENABLE_LEGACY_ENDPOINTS=true
        // Test that primal endpoints are NOT hardcoded by default (universal behavior)
        assert!(config.get_endpoint("beardog").is_none());
        assert!(config.get_endpoint("songbird").is_none());
        assert!(config.get_endpoint("squirrel").is_none());
        assert!(config.get_endpoint("toadstool").is_none());
    }

    #[test]
    fn test_config_comprehensive_structure() {
        let config = Config::default();

        // Verify all major sections are present and configured
        assert!(!config.system.node_id.is_empty());
        assert!(config.storage.cache_size > 0);
        assert!(config.storage.max_file_size > 0);
        assert!(!config.security.auth_method.is_empty());
        assert!(!config.security.encryption_algorithm.is_empty());
        assert!(config.security.key_rotation_days > 0);
        assert!(config.security.max_failed_attempts > 0);
        assert!(config.monitoring.metrics_interval > 0);
        assert!(!config.monitoring.log_level.is_empty());
        assert!(config.monitoring.log_retention_days > 0);
        assert!(config.mcp.is_some());
        assert!(config.federation.is_some());
        assert!(!config.endpoints.services.is_empty());

        // Verify configuration relationships
        if let Some(mcp) = &config.mcp {
            assert_eq!(mcp.node_id, config.system.node_id);
        }

        // Verify security defaults
        assert!(config.security.rbac.enabled);
        assert!(!config.security.rbac.default_role.is_empty());
        assert!(!config.security.rbac.roles.is_empty());
        assert!(config.security.rbac.roles.contains_key("admin"));
        assert!(config.security.rbac.roles.contains_key("user"));
        assert!(config.security.rbac.roles.contains_key("readonly"));

        // Verify monitoring defaults
        assert!(config.monitoring.prometheus.is_some());
        assert!(!config.monitoring.alerts.enabled); // Disabled by default

        // Verify federation defaults
        if let Some(federation) = &config.federation {
            assert!(!federation.enabled);
            assert_eq!(federation.mode, "standalone");
            assert!(federation.peers.is_empty());
        }
    }
}
