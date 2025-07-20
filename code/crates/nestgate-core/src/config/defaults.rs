use super::federation::McpConfig;
use super::*;
use uuid;

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
        assert!(config.storage.tiers.is_empty());

        // Test security config
        assert_eq!(config.security.auth_method, "jwt");
        assert_eq!(config.security.encryption_algorithm, "aes-256-gcm");
        assert_eq!(config.security.key_rotation_days, 30);
        assert_eq!(config.security.max_failed_attempts, 5);
        assert!(config.security.rbac.enabled);
        assert_eq!(config.security.rbac.default_role, "user");

        // Test monitoring config
        assert_eq!(config.monitoring.metrics_interval, 30);
        assert_eq!(config.monitoring.log_level, "info");
        assert_eq!(config.monitoring.log_retention_days, 30);
        assert!(config.monitoring.prometheus.is_some());
        assert!(!config.monitoring.alerts.enabled); // Disabled by default

        // Test MCP config
        assert!(config.mcp.is_some());
        let mcp = config.mcp.as_ref().unwrap();
        assert!(!mcp.enabled);
        assert!(!mcp.federation_enabled);
        assert!(!mcp.node_id.is_empty());

        // Test federation config
        assert!(config.federation.is_some());
        let federation = config.federation.as_ref().unwrap();
        assert!(!federation.enabled);
        assert_eq!(federation.mode, "standalone");
        assert!(federation.peers.is_empty());

        // Test endpoints config
        assert!(config.endpoints.has_service("beardog"));
        assert!(config.endpoints.has_service("songbird"));
        assert!(config.endpoints.has_service("squirrel"));
        assert!(config.endpoints.has_service("toadstool"));
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

        // Test getting service endpoints
        assert!(config.get_endpoint("beardog").is_some());
        assert!(config.get_endpoint("songbird").is_some());
        assert!(config.get_endpoint("squirrel").is_some());
        assert!(config.get_endpoint("toadstool").is_some());
        assert!(config.get_endpoint("huggingface").is_some());
        assert!(config.get_endpoint("ncbi").is_some());
        assert!(config.get_endpoint("nonexistent").is_none());

        // Test endpoint values
        assert_eq!(
            config.get_endpoint("beardog"),
            Some("http://localhost:8001")
        );
        assert_eq!(
            config.get_endpoint("songbird"),
            Some("http://localhost:8002")
        );
        assert_eq!(
            config.get_endpoint("squirrel"),
            Some("http://localhost:8003")
        );
        assert_eq!(
            config.get_endpoint("toadstool"),
            Some("http://localhost:8004")
        );
        assert_eq!(
            config.get_endpoint("huggingface"),
            Some("https://api.huggingface.co")
        );
        assert_eq!(
            config.get_endpoint("ncbi"),
            Some("https://api.ncbi.nlm.nih.gov")
        );
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
