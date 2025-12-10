use std::collections::HashMap;
use std::env;
use std::sync::Arc;

/// Configuration for external network endpoints, capturing environment variables
/// for service host/port combinations.
#[derive(Debug, Clone)]
/// Configuration for NetworkEnv
pub struct NetworkEnvConfig {
    // Map of prefix → (host, port)
    endpoints: HashMap<String, (Option<String>, Option<u16>)>,
}

/// Type alias for Sharednetworkenvconfig
pub type SharedNetworkEnvConfig = Arc<NetworkEnvConfig>;

impl NetworkEnvConfig {
    /// Creates a new `NetworkEnvConfig` with empty values.
    pub fn new() -> Self {
        Self {
            endpoints: HashMap::new(),
        }
    }

    /// Creates a new `NetworkEnvConfig` by reading environment variables
    /// for common service prefixes.
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Common service prefixes
        let prefixes = [
            "NESTGATE_API",
            "NESTGATE_DB",
            "NESTGATE_REDIS",
            "NESTGATE_METRICS",
            "NESTGATE_DISCOVERY",
            "NESTGATE_WEBSOCKET",
            "NESTGATE_ADMIN",
            "NESTGATE_HEALTH",
        ];

        for prefix in &prefixes {
            let host = env::var(format!("{}_HOST", prefix)).ok();
            let port = env::var(format!("{}_PORT", prefix))
                .ok()
                .and_then(|p| p.parse::<u16>().ok());

            if host.is_some() || port.is_some() {
                config.endpoints.insert(prefix.to_string(), (host, port));
            }
        }

        config
    }

    // Getter methods

    /// Get host for a given prefix, returns None if not set
    pub fn get_host(&self, prefix: &str) -> Option<&str> {
        self.endpoints
            .get(prefix)
            .and_then(|(host, _)| host.as_deref())
    }

    /// Get port for a given prefix, returns None if not set
    pub fn get_port(&self, prefix: &str) -> Option<u16> {
        self.endpoints.get(prefix).and_then(|(_, port)| *port)
    }

    /// Check if host is set for a given prefix
    pub fn has_host(&self, prefix: &str) -> bool {
        self.endpoints
            .get(prefix)
            .and_then(|(host, _)| host.as_ref())
            .is_some()
    }

    /// Check if port is set for a given prefix
    pub fn has_port(&self, prefix: &str) -> bool {
        self.endpoints
            .get(prefix)
            .and_then(|(_, port)| *port)
            .is_some()
    }

    // Builder methods for testing

    /// Builder method to set Endpoint
    pub fn with_endpoint(
        mut self,
        prefix: String,
        host: Option<String>,
        port: Option<u16>,
    ) -> Self {
        self.endpoints.insert(prefix, (host, port));
        self
    }

    /// Builder method to set Host
    pub fn with_host(mut self, prefix: String, host: String) -> Self {
        let entry = self.endpoints.entry(prefix).or_insert((None, None));
        entry.0 = Some(host);
        self
    }

    /// Builder method to set Port
    pub fn with_port(mut self, prefix: String, port: u16) -> Self {
        let entry = self.endpoints.entry(prefix).or_insert((None, None));
        entry.1 = Some(port);
        self
    }
}

impl Default for NetworkEnvConfig {
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
        let config = NetworkEnvConfig::new();
        assert!(config.get_host("NESTGATE_API").is_none());
        assert!(config.get_port("NESTGATE_API").is_none());
    }

    #[test]
    fn test_builder_pattern() {
        let config = NetworkEnvConfig::new()
            .with_host("NESTGATE_API".to_string(), "api.example.com".to_string())
            .with_port("NESTGATE_API".to_string(), 8080)
            .with_endpoint(
                "NESTGATE_DB".to_string(),
                Some("db.example.com".to_string()),
                Some(5432),
            );

        assert_eq!(config.get_host("NESTGATE_API"), Some("api.example.com"));
        assert_eq!(config.get_port("NESTGATE_API"), Some(8080));
        assert_eq!(config.get_host("NESTGATE_DB"), Some("db.example.com"));
        assert_eq!(config.get_port("NESTGATE_DB"), Some(5432));
    }

    #[test]
    fn test_has_methods() {
        let config = NetworkEnvConfig::new()
            .with_host("NESTGATE_API".to_string(), "api.example.com".to_string());

        assert!(config.has_host("NESTGATE_API"));
        assert!(!config.has_port("NESTGATE_API"));
        assert!(!config.has_host("NESTGATE_DB"));
    }

    #[test]
    fn test_partial_config() {
        let config = NetworkEnvConfig::new().with_port("NESTGATE_REDIS".to_string(), 6379);

        assert!(config.has_port("NESTGATE_REDIS"));
        assert!(!config.has_host("NESTGATE_REDIS"));
        assert_eq!(config.get_port("NESTGATE_REDIS"), Some(6379));
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_access() {
        let config = Arc::new(
            NetworkEnvConfig::new()
                .with_host("NESTGATE_API".to_string(), "api.example.com".to_string())
                .with_port("NESTGATE_API".to_string(), 8080)
                .with_host("NESTGATE_DB".to_string(), "db.example.com".to_string())
                .with_port("NESTGATE_DB".to_string(), 5432),
        );

        let handles: Vec<_> = (0..100)
            .map(|_| {
                let cfg = config.clone();
                tokio::spawn(async move {
                    let _ = cfg.get_host("NESTGATE_API");
                    let _ = cfg.get_port("NESTGATE_API");
                    let _ = cfg.get_host("NESTGATE_DB");
                    let _ = cfg.get_port("NESTGATE_DB");
                    let _ = cfg.has_host("NESTGATE_REDIS");
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
            NetworkEnvConfig::new()
                .with_host("NESTGATE_API".to_string(), "api1.example.com".to_string())
                .with_port("NESTGATE_API".to_string(), 8080),
        );
        let config2 = Arc::new(
            NetworkEnvConfig::new()
                .with_host("NESTGATE_API".to_string(), "api2.example.com".to_string())
                .with_port("NESTGATE_API".to_string(), 9090),
        );

        let handle1 = tokio::spawn({
            let cfg = config1.clone();
            async move {
                let host = cfg.get_host("NESTGATE_API").map(|s| s.to_string());
                let port = cfg.get_port("NESTGATE_API");
                (host, port)
            }
        });
        let handle2 = tokio::spawn({
            let cfg = config2.clone();
            async move {
                let host = cfg.get_host("NESTGATE_API").map(|s| s.to_string());
                let port = cfg.get_port("NESTGATE_API");
                (host, port)
            }
        });

        let (host1, port1) = handle1
            .await
            .expect("First task should complete successfully");
        let (host2, port2) = handle2
            .await
            .expect("Second task should complete successfully");

        assert_eq!(host1, Some("api1.example.com".to_string()));
        assert_eq!(port1, Some(8080));
        assert_eq!(host2, Some("api2.example.com".to_string()));
        assert_eq!(port2, Some(9090));
    }
}
