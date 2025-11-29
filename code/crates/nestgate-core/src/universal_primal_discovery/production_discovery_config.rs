use std::collections::HashMap;
use std::env;
use std::sync::Arc;

/// Configuration for production service discovery, capturing environment variables
/// for service endpoints, resource limits, and operation timeouts.
#[derive(Debug, Clone)]
/// Configuration for ProductionDiscovery
pub struct ProductionDiscoveryConfig {
    // Service endpoint environment variables
    service_hosts: HashMap<String, String>,
    service_ports: HashMap<String, u16>,
    service_binds: HashMap<String, String>,

    // Resource limits
    resource_limits: HashMap<String, usize>,

    // Operation timeouts
    operation_timeouts: HashMap<String, u64>, // in seconds
}

/// Type alias for Sharedproductiondiscoveryconfig
pub type SharedProductionDiscoveryConfig = Arc<ProductionDiscoveryConfig>;

impl ProductionDiscoveryConfig {
    /// Creates a new `ProductionDiscoveryConfig` with empty values.
    pub fn new() -> Self {
        Self {
            service_hosts: HashMap::new(),
            service_ports: HashMap::new(),
            service_binds: HashMap::new(),
            resource_limits: HashMap::new(),
            operation_timeouts: HashMap::new(),
        }
    }

    /// Creates a new `ProductionDiscoveryConfig` by reading environment variables.
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Discover service endpoints
        let service_names = [
            "API",
            "WEB",
            "METRICS",
            "HEALTH",
            "ADMIN",
            "WEBSOCKET",
            "NETWORK",
            "STORAGE",
            "ZFS",
            "MCP",
            "AUTOMATION",
        ];

        for service_name in &service_names {
            // Discover host
            if let Ok(host) = env::var(format!("{}_HOST", service_name)) {
                config.service_hosts.insert(service_name.to_string(), host);
            }

            // Discover port
            if let Ok(port_str) = env::var(format!("{}_PORT", service_name)) {
                if let Ok(port) = port_str.parse::<u16>() {
                    config.service_ports.insert(service_name.to_string(), port);
                }
            }

            // Discover bind address
            if let Ok(bind) = env::var(format!("{}_BIND", service_name)) {
                config.service_binds.insert(service_name.to_string(), bind);
            }
        }

        // Discover resource limits
        let limit_types = [
            ("MAX_CONNECTIONS", "max_connections"),
            ("MAX_MEMORY_MB", "max_memory"),
            ("MAX_CPU_PERCENT", "max_cpu"),
            ("MAX_DISK_GB", "max_disk"),
            ("MAX_REQUESTS_PER_SEC", "rate_limit"),
        ];

        for (env_suffix, limit_name) in &limit_types {
            if let Ok(value_str) = env::var(format!("NESTGATE_{}", env_suffix)) {
                if let Ok(value) = value_str.parse::<usize>() {
                    config.resource_limits.insert(limit_name.to_string(), value);
                }
            }
        }

        // Discover operation timeouts
        let timeout_types = [
            ("CONNECT", "connect"),
            ("REQUEST", "request"),
            ("OPERATION", "operation"),
            ("HEARTBEAT", "heartbeat"),
            ("DISCOVERY", "discovery"),
        ];

        for (env_suffix, timeout_name) in &timeout_types {
            if let Ok(value_str) = env::var(format!("NESTGATE_{}_TIMEOUT", env_suffix)) {
                if let Ok(value_secs) = value_str.parse::<u64>() {
                    config
                        .operation_timeouts
                        .insert(timeout_name.to_string(), value_secs);
                }
            }
        }

        config
    }

    // Getter methods

    /// Get service host from environment, returns None if not set
    pub fn get_service_host(&self, service_name: &str) -> Option<&str> {
        self.service_hosts.get(service_name).map(|s| s.as_str())
    }

    /// Get service port from environment, returns None if not set
    pub fn get_service_port(&self, service_name: &str) -> Option<u16> {
        self.service_ports.get(service_name).copied()
    }

    /// Get service bind address from environment, returns None if not set
    pub fn get_service_bind(&self, service_name: &str) -> Option<&str> {
        self.service_binds.get(service_name).map(|s| s.as_str())
    }

    /// Get resource limit from environment, returns None if not set
    pub fn get_resource_limit(&self, limit_name: &str) -> Option<usize> {
        self.resource_limits.get(limit_name).copied()
    }

    /// Get operation timeout from environment (in seconds), returns None if not set
    pub fn get_operation_timeout(&self, timeout_name: &str) -> Option<u64> {
        self.operation_timeouts.get(timeout_name).copied()
    }

    /// Get all resource limits
    pub fn get_all_resource_limits(&self) -> &HashMap<String, usize> {
        &self.resource_limits
    }

    /// Get all operation timeouts
    pub fn get_all_operation_timeouts(&self) -> &HashMap<String, u64> {
        &self.operation_timeouts
    }

    // Builder methods for testing

    /// Builder method to set Service Host
    pub fn with_service_host(mut self, service_name: String, host: String) -> Self {
        self.service_hosts.insert(service_name, host);
        self
    }

    /// Builder method to set Service Port
    pub fn with_service_port(mut self, service_name: String, port: u16) -> Self {
        self.service_ports.insert(service_name, port);
        self
    }

    /// Builder method to set Service Bind
    pub fn with_service_bind(mut self, service_name: String, bind: String) -> Self {
        self.service_binds.insert(service_name, bind);
        self
    }

    /// Builder method to set Resource Limit
    pub fn with_resource_limit(mut self, limit_name: String, value: usize) -> Self {
        self.resource_limits.insert(limit_name, value);
        self
    }

    /// Builder method to set Operation Timeout
    pub fn with_operation_timeout(mut self, timeout_name: String, seconds: u64) -> Self {
        self.operation_timeouts.insert(timeout_name, seconds);
        self
    }
}

impl Default for ProductionDiscoveryConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ProductionDiscoveryConfig::new();
        assert!(config.get_service_host("API").is_none());
        assert!(config.get_service_port("API").is_none());
        assert!(config.get_resource_limit("max_connections").is_none());
    }

    #[test]
    fn test_builder_pattern() {
        let config = ProductionDiscoveryConfig::new()
            .with_service_host("API".to_string(), "api.example.com".to_string())
            .with_service_port("API".to_string(), 8080)
            .with_service_bind("API".to_string(), "0.0.0.0".to_string())
            .with_resource_limit("max_connections".to_string(), 1000)
            .with_operation_timeout("connect".to_string(), 30);

        assert_eq!(config.get_service_host("API"), Some("api.example.com"));
        assert_eq!(config.get_service_port("API"), Some(8080));
        assert_eq!(config.get_service_bind("API"), Some("0.0.0.0"));
        assert_eq!(config.get_resource_limit("max_connections"), Some(1000));
        assert_eq!(config.get_operation_timeout("connect"), Some(30));
    }

    #[test]
    fn test_getters() {
        let mut config = ProductionDiscoveryConfig::new();
        config
            .service_hosts
            .insert("WEB".to_string(), "web.example.com".to_string());
        config.service_ports.insert("WEB".to_string(), 3000);

        assert_eq!(config.get_service_host("WEB"), Some("web.example.com"));
        assert_eq!(config.get_service_port("WEB"), Some(3000));
        assert!(config.get_service_host("NONEXISTENT").is_none());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            ProductionDiscoveryConfig::new()
                .with_service_host("API".to_string(), "api.example.com".to_string())
                .with_service_port("API".to_string(), 8080)
                .with_resource_limit("max_connections".to_string(), 1000),
        );

        let handles: Vec<_> = (0..100)
            .map(|_| {
                let cfg = config.clone();
                tokio::spawn(async move {
                    let _ = cfg.get_service_host("API");
                    let _ = cfg.get_service_port("API");
                    let _ = cfg.get_service_bind("API");
                    let _ = cfg.get_resource_limit("max_connections");
                    let _ = cfg.get_operation_timeout("connect");
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("Task should complete successfully");
        }
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_different_configs() {
        let config1 = Arc::new(
            ProductionDiscoveryConfig::new()
                .with_service_host("API".to_string(), "api1.example.com".to_string())
                .with_service_port("API".to_string(), 8080),
        );
        let config2 = Arc::new(
            ProductionDiscoveryConfig::new()
                .with_service_host("API".to_string(), "api2.example.com".to_string())
                .with_service_port("API".to_string(), 9090),
        );

        let handle1 = tokio::spawn({
            let cfg = config1.clone();
            async move {
                (
                    cfg.get_service_host("API").map(|s| s.to_string()),
                    cfg.get_service_port("API"),
                )
            }
        });
        let handle2 = tokio::spawn({
            let cfg = config2.clone();
            async move {
                (
                    cfg.get_service_host("API").map(|s| s.to_string()),
                    cfg.get_service_port("API"),
                )
            }
        });

        let (host1, port1) = handle1.await.unwrap();
        let (host2, port2) = handle2.await.unwrap();

        assert_eq!(host1, Some("api1.example.com".to_string()));
        assert_eq!(port1, Some(8080));
        assert_eq!(host2, Some("api2.example.com".to_string()));
        assert_eq!(port2, Some(9090));
    }
}
