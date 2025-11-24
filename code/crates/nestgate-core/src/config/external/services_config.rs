use std::collections::HashMap;
use std::sync::Arc;

/// Thread-safe configuration for service endpoints
/// Captures environment variables at initialization to prevent race conditions
#[derive(Debug, Clone)]
pub struct ServicesConfig {
    // Core service URLs
    discovery_url: Option<String>,
    adapter_url: Option<String>,
    health_url: Option<String>,
    metrics_url: Option<String>,
    config_url: Option<String>,

    // Primal service URLs
    songbird_url: Option<String>,
    toadstool_url: Option<String>,
    beardog_url: Option<String>,
    squirrel_url: Option<String>,
    biomeos_url: Option<String>,

    // External service URLs (dynamic NESTGATE_EXTERNAL_*)
    external_services: HashMap<String, String>,
}

/// Shared immutable reference to ServicesConfig
pub type SharedServicesConfig = Arc<ServicesConfig>;

impl ServicesConfig {
    /// Create a new empty configuration (all values None)
    pub fn new() -> Self {
        Self {
            discovery_url: None,
            adapter_url: None,
            health_url: None,
            metrics_url: None,
            config_url: None,
            songbird_url: None,
            toadstool_url: None,
            beardog_url: None,
            squirrel_url: None,
            biomeos_url: None,
            external_services: HashMap::new(),
        }
    }

    /// Create configuration from current environment variables
    /// This captures env vars at initialization time, making it thread-safe
    pub fn from_env() -> Self {
        let mut config = Self::new();

        // Core services
        config.discovery_url = std::env::var("NESTGATE_DISCOVERY_URL").ok();
        config.adapter_url = std::env::var("NESTGATE_ADAPTER_URL").ok();
        config.health_url = std::env::var("NESTGATE_HEALTH_URL").ok();
        config.metrics_url = std::env::var("NESTGATE_METRICS_URL").ok();
        config.config_url = std::env::var("NESTGATE_CONFIG_URL").ok();

        // Primal services
        config.songbird_url = std::env::var("NESTGATE_SONGBIRD_URL").ok();
        config.toadstool_url = std::env::var("NESTGATE_TOADSTOOL_URL").ok();
        config.beardog_url = std::env::var("NESTGATE_BEARDOG_URL").ok();
        config.squirrel_url = std::env::var("NESTGATE_SQUIRREL_URL").ok();
        config.biomeos_url = std::env::var("NESTGATE_BIOMEOS_URL").ok();

        // Scan for dynamic NESTGATE_EXTERNAL_* entries
        for (key, value) in std::env::vars() {
            if let Some(name) = key.strip_prefix("NESTGATE_EXTERNAL_") {
                config.external_services.insert(name.to_lowercase(), value);
            }
        }

        config
    }

    // Core service accessors with defaults

    pub fn get_discovery_url(&self) -> String {
        self.discovery_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/discovery",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    pub fn get_adapter_url(&self) -> String {
        self.adapter_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/adapter",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    pub fn get_health_url(&self) -> String {
        self.health_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/health",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    pub fn get_metrics_url(&self) -> String {
        self.metrics_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!("{}/metrics", config.build_endpoint(9090))
        })
    }

    pub fn get_config_url(&self) -> String {
        self.config_url.clone().unwrap_or_else(|| {
            let config = crate::config::discovery_config::ServiceDiscoveryConfig::default();
            format!(
                "{}/config",
                config.build_endpoint(config.discovery_base_port)
            )
        })
    }

    // Production accessors (for required URLs)

    pub fn get_discovery_url_required(&self) -> Option<&str> {
        self.discovery_url.as_deref()
    }

    pub fn get_adapter_url_required(&self) -> Option<&str> {
        self.adapter_url.as_deref()
    }

    pub fn get_health_url_required(&self) -> Option<&str> {
        self.health_url.as_deref()
    }

    // Primal service accessors (all optional)

    pub fn get_songbird_url(&self) -> Option<&str> {
        self.songbird_url.as_deref()
    }

    pub fn get_toadstool_url(&self) -> Option<&str> {
        self.toadstool_url.as_deref()
    }

    pub fn get_beardog_url(&self) -> Option<&str> {
        self.beardog_url.as_deref()
    }

    pub fn get_squirrel_url(&self) -> Option<&str> {
        self.squirrel_url.as_deref()
    }

    pub fn get_biomeos_url(&self) -> Option<&str> {
        self.biomeos_url.as_deref()
    }

    // External services

    pub fn get_external_service(&self, name: &str) -> Option<&str> {
        self.external_services.get(name).map(|s| s.as_str())
    }

    pub fn get_all_external_services(&self) -> &HashMap<String, String> {
        &self.external_services
    }

    // Builder methods for tests

    pub fn with_discovery_url(mut self, url: String) -> Self {
        self.discovery_url = Some(url);
        self
    }

    pub fn with_adapter_url(mut self, url: String) -> Self {
        self.adapter_url = Some(url);
        self
    }

    pub fn with_health_url(mut self, url: String) -> Self {
        self.health_url = Some(url);
        self
    }

    pub fn with_metrics_url(mut self, url: String) -> Self {
        self.metrics_url = Some(url);
        self
    }

    pub fn with_config_url(mut self, url: String) -> Self {
        self.config_url = Some(url);
        self
    }

    pub fn with_songbird_url(mut self, url: String) -> Self {
        self.songbird_url = Some(url);
        self
    }

    pub fn with_toadstool_url(mut self, url: String) -> Self {
        self.toadstool_url = Some(url);
        self
    }

    pub fn with_beardog_url(mut self, url: String) -> Self {
        self.beardog_url = Some(url);
        self
    }

    pub fn with_squirrel_url(mut self, url: String) -> Self {
        self.squirrel_url = Some(url);
        self
    }

    pub fn with_biomeos_url(mut self, url: String) -> Self {
        self.biomeos_url = Some(url);
        self
    }

    pub fn with_external_service(mut self, name: String, url: String) -> Self {
        self.external_services.insert(name, url);
        self
    }
}

impl Default for ServicesConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_services_config_new() {
        let config = ServicesConfig::new();

        // Should use default URLs (ServiceDiscoveryConfig defaults to 127.0.0.1)
        assert_eq!(
            config.get_discovery_url(),
            "http://127.0.0.1:8080/discovery"
        );
        assert_eq!(config.get_adapter_url(), "http://127.0.0.1:8080/adapter");
        assert!(config.get_songbird_url().is_none());
    }

    #[test]
    fn test_services_config_builder() {
        let config = ServicesConfig::new()
            .with_discovery_url("http://discovery:8080".to_string())
            .with_songbird_url("http://songbird:9000".to_string())
            .with_external_service("custom".to_string(), "http://custom:8000".to_string());

        assert_eq!(config.get_discovery_url(), "http://discovery:8080");
        assert_eq!(config.get_songbird_url(), Some("http://songbird:9000"));
        assert_eq!(
            config.get_external_service("custom"),
            Some("http://custom:8000")
        );
    }

    #[test]
    fn test_services_config_production_required() {
        let config = ServicesConfig::new()
            .with_discovery_url("http://prod-discovery:8080".to_string())
            .with_adapter_url("http://prod-adapter:8080".to_string())
            .with_health_url("http://prod-health:8080".to_string());

        assert!(config.get_discovery_url_required().is_some());
        assert!(config.get_adapter_url_required().is_some());
        assert!(config.get_health_url_required().is_some());
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_concurrent_services_config_access() {
        // Create two different configurations
        let config1 = Arc::new(
            ServicesConfig::new().with_discovery_url("http://discovery1:8080".to_string()),
        );
        let config2 = Arc::new(
            ServicesConfig::new().with_discovery_url("http://discovery2:8080".to_string()),
        );

        // Spawn concurrent tasks accessing different configs
        let handle1 = {
            let config = Arc::clone(&config1);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_discovery_url(), "http://discovery1:8080");
                }
            })
        };

        let handle2 = {
            let config = Arc::clone(&config2);
            tokio::spawn(async move {
                for _ in 0..100 {
                    assert_eq!(config.get_discovery_url(), "http://discovery2:8080");
                }
            })
        };

        handle1.await.expect("Thread join failed in test");
        handle2.await.expect("Thread join failed in test");
    }

    #[test]
    fn test_services_config_external_services() {
        let config = ServicesConfig::new()
            .with_external_service(
                "huggingface".to_string(),
                "https://api.huggingface.co".to_string(),
            )
            .with_external_service(
                "ncbi".to_string(),
                "https://eutils.ncbi.nlm.nih.gov".to_string(),
            );

        assert_eq!(
            config.get_external_service("huggingface"),
            Some("https://api.huggingface.co")
        );
        assert_eq!(
            config.get_external_service("ncbi"),
            Some("https://eutils.ncbi.nlm.nih.gov")
        );
        assert_eq!(config.get_all_external_services().len(), 2);
    }

    #[test]
    fn test_services_config_defaults() {
        let config = ServicesConfig::new();

        // All core services should have 127.0.0.1 defaults (from ServiceDiscoveryConfig)
        assert!(config.get_discovery_url().contains("127.0.0.1"));
        assert!(config.get_adapter_url().contains("127.0.0.1"));
        assert!(config.get_health_url().contains("127.0.0.1"));
        assert!(config.get_metrics_url().contains("127.0.0.1"));
        assert!(config.get_config_url().contains("127.0.0.1"));
    }
}
