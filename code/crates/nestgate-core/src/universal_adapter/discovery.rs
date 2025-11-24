use crate::canonical_types::service::{ServiceState, ServiceType};
// ports removed - unused import
/// **CANONICAL ADAPTER DISCOVERY**
///
/// Consolidated discovery utilities for the universal adapter system.
use crate::Result;
// Removed unused import for pedantic perfection
use super::discovery_config::{DiscoveryRuntimeConfig, SharedDiscoveryRuntimeConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tracing::{debug, warn}; // Removed unused 'info' for pedantic perfection

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct DiscoveryConfig {
    /// Discovery endpoint
    pub endpoint: String,
    /// Discovery timeout
    pub timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Discovery interval for periodic discovery
    pub discovery_interval: Duration,
    /// Enabled discovery methods
    pub methods: Vec<DiscoveryMethod>,
}
#[allow(deprecated)]
impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            endpoint: crate::constants::canonical_defaults::network::build_endpoint(),
            timeout: Duration::from_secs(30),
            max_retries: 3,
            discovery_interval: Duration::from_secs(60),
            methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::ServiceRegistry,
            ],
        }
    }
}

/// Discovery methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryMethod {
    /// Environment variable discovery
    Environment,
    /// Service registry lookup
    ServiceRegistry,
    /// Network scanning
    NetworkScan,
    /// Configuration file
    Configuration,
    /// DNS-based discovery
    Dns,
}
/// Discovered service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredService {
    /// Service identifier
    pub id: String,
    /// Service name
    pub name: String,
    /// Service type
    pub service_type: ServiceType,
    /// Service state
    pub state: ServiceState,
    /// Service endpoint
    pub endpoint: String,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Discovery timestamp
    pub discovered_at: SystemTime,
    /// Last health check
    pub last_health_check: Option<SystemTime>,
}
/// Discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryResult {
    /// Discovered services
    pub services: Vec<DiscoveredService>,
    /// Discovery method used
    pub method: DiscoveryMethod,
    /// Discovery duration
    pub duration: Duration,
    /// Success status
    pub success: bool,
    /// Error message if failed
    pub error: Option<String>,
}
/// Discover available services using canonical discovery
#[allow(deprecated)]
pub fn discover_services(config: &DiscoveryConfig) -> Result<DiscoveryResult> {
    let start_time = std::time::Instant::now();

    // For now, return a basic result - this would be expanded with real discovery logic
    let services = vec![DiscoveredService {
        id: "nestgate-core".to_string(),
        name: "NestGate Core".to_string(),
        service_type: ServiceType::Storage,
        state: ServiceState::Running,
        endpoint: config.endpoint.clone(),
        capabilities: vec!["storage".to_string(), "zfs".to_string()],
        metadata: HashMap::new(),
        discovered_at: SystemTime::now(),
        last_health_check: Some(SystemTime::now()),
    }];

    Ok(DiscoveryResult {
        services,
        method: DiscoveryMethod::Environment,
        duration: start_time.elapsed(),
        success: true,
        error: None,
    })
}
/// Discover services by capability
#[allow(deprecated)]
pub fn discover_by_capability(
    config: &DiscoveryConfig,
    capability: &str,
) -> Result<Vec<DiscoveredService>> {
    let result = discover_services(config)?;

    Ok(result
        .services
        .into_iter()
        .filter(|service| service.capabilities.contains(&capability.to_string()))
        .collect())
}
/// Health check a discovered service
pub fn health_check_service(service: &DiscoveredService) -> Result<bool> {
    // Basic health check implementation - would be expanded with real health check logic
    match service.state {
        ServiceState::Running => Ok(true),
        ServiceState::Starting => Ok(false),
        ServiceState::Stopping | ServiceState::Stopped | ServiceState::Failed => Ok(false),
        _ => Ok(false),
    }
}

/// Capability discovery service for universal adapter
#[derive(Debug, Clone)]
pub struct CapabilityDiscovery {
    registry: HashMap<String, Vec<String>>,
    discovery_endpoints: Vec<String>,
    runtime_config: SharedDiscoveryRuntimeConfig,
}

impl CapabilityDiscovery {
    /// Create a new capability discovery instance with runtime config
    pub fn with_runtime_config(
        runtime_config: SharedDiscoveryRuntimeConfig,
    ) -> crate::Result<Self> {
        let discovery_endpoints = runtime_config.get_discovery_endpoints();

        let mut discovery = Self {
            registry: HashMap::new(),
            discovery_endpoints,
            runtime_config,
        };

        // Initialize with default capability mappings
        discovery.initialize_default_capabilities();

        Ok(discovery)
    }

    /// Create a new capability discovery instance (backward compatibility)
    /// NOTE: Creates config from env each time. For tests, use with_runtime_config() directly.
    pub fn new() -> crate::Result<Self> {
        Self::with_runtime_config(Arc::new(DiscoveryRuntimeConfig::from_env()))
    }

    /// Find capabilities by type
    pub fn find_capabilities(&self, capability_type: &str) -> crate::Result<Vec<String>> {
        debug!("Finding capabilities for type: {}", capability_type);

        // Check local registry first - avoid clone by using Arc
        if let Some(services) = self.registry.get(capability_type) {
            // Return a new Vec with the same data to maintain API compatibility
            return Ok(services.to_vec());
        }

        // Query discovery endpoints for dynamic capabilities
        for endpoint in &self.discovery_endpoints {
            if let Ok(services) = self.query_discovery_endpoint(endpoint, capability_type) {
                if !services.is_empty() {
                    return Ok(services);
                }
            }
        }

        // Return empty if no capabilities found
        warn!("No capabilities found for type: {}", capability_type);
        Ok(Vec::new())
    }

    /// Initialize default capability mappings with environment-driven endpoints
    fn initialize_default_capabilities(&mut self) {
        let base_endpoint = self.runtime_config.get_base_endpoint();

        // Security capabilities
        self.registry.insert(
            "security".to_string(),
            self.runtime_config.get_security_endpoint(&base_endpoint),
        );

        // AI capabilities
        self.registry.insert(
            "ai".to_string(),
            self.runtime_config.get_ai_endpoint(&base_endpoint),
        );

        // Orchestration capabilities
        self.registry.insert(
            "orchestration".to_string(),
            self.runtime_config
                .get_orchestration_endpoint(&base_endpoint),
        );

        // Storage/ZFS capabilities
        self.registry.insert(
            "storage".to_string(),
            self.runtime_config.get_storage_endpoint(&base_endpoint),
        );

        // Compute capabilities
        self.registry.insert(
            "compute".to_string(),
            self.runtime_config.get_compute_endpoint(&base_endpoint),
        );
    }

    /// Query a discovery endpoint for capabilities
    fn query_discovery_endpoint(
        &self,
        endpoint: &str,
        capability_type: &str,
    ) -> crate::Result<Vec<String>> {
        debug!(
            "Querying discovery endpoint: {} for {}",
            endpoint, capability_type
        );

        // Mock implementation - in practice this would make HTTP requests
        // to actual discovery services
        match capability_type {
            "security" => Ok(vec![format!("{endpoint}/security")]),
            "ai" => Ok(vec![format!("{endpoint}/ai")]),
            "orchestration" => Ok(vec![format!("{endpoint}/orchestration")]),
            "storage" => Ok(vec![format!("{endpoint}/storage")]),
            "compute" => Ok(vec![format!("{endpoint}/compute")]),
            _ => Ok(Vec::new()),
        }
    }

    /// Register a new capability service
    pub fn register_capability(&mut self, capability_type: String, service_url: String) {
        self.registry
            .entry(capability_type)
            .or_default()
            .push(service_url);
    }

    /// Remove a capability service
    pub fn unregister_capability(&mut self, capability_type: &str, service_url: &str) {
        if let Some(services) = self.registry.get_mut(capability_type) {
            services.retain(|url| url != service_url);
        }
    }
}

impl Default for CapabilityDiscovery {
    fn default() -> Self {
        // CapabilityDiscovery::new() cannot actually fail - it just initializes data structures
        // If it ever returns an error, we use empty defaults as fallback
        Self::new().unwrap_or_else(|_| Self {
            registry: HashMap::new(),
            discovery_endpoints: Vec::new(),
            runtime_config: Arc::new(DiscoveryRuntimeConfig::new()),
        })
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type DiscoveryConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::discovery_config::ServiceDiscoveryConfig;

    /// Helper to create test endpoint using ServiceDiscoveryConfig
    /// ✅ MIGRATED: Replaces hardcoded "localhost:port" with configurable endpoints
    fn test_endpoint(port: u16) -> String {
        let config = ServiceDiscoveryConfig::default();
        format!("http://{}:{}", config.discovery_host, port)
    }

    #[test]
    fn test_discovery_config_default() {
        let config = DiscoveryConfig::default();

        assert_eq!(config.max_retries, 3);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.discovery_interval, Duration::from_secs(60));
        assert_eq!(config.methods.len(), 2);
        assert!(config.methods.contains(&DiscoveryMethod::Environment));
        assert!(config.methods.contains(&DiscoveryMethod::ServiceRegistry));
    }

    #[test]
    fn test_discovery_method_equality() {
        assert_eq!(DiscoveryMethod::Environment, DiscoveryMethod::Environment);
        assert_ne!(
            DiscoveryMethod::Environment,
            DiscoveryMethod::ServiceRegistry
        );
        assert_ne!(DiscoveryMethod::NetworkScan, DiscoveryMethod::Dns);
    }

    #[test]
    fn test_discovery_method_clone() {
        let method = DiscoveryMethod::Configuration;
        let cloned = method.clone();
        assert_eq!(method, cloned);
    }

    #[test]
    fn test_discovered_service_creation() {
        use crate::constants::{network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port};
        let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
        let service = DiscoveredService {
            id: "test-service-1".to_string(),
            name: "Test Service".to_string(),
            service_type: ServiceType::Storage,
            state: ServiceState::Running,
            endpoint,
            capabilities: vec!["storage".to_string(), "backup".to_string()],
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
            last_health_check: Some(SystemTime::now()),
        };

        assert_eq!(service.id, "test-service-1");
        assert_eq!(service.name, "Test Service");
        assert_eq!(service.capabilities.len(), 2);
        assert!(service.last_health_check.is_some());
    }

    #[test]
    fn test_discovery_result_success() {
        let result = DiscoveryResult {
            services: vec![],
            method: DiscoveryMethod::Environment,
            duration: Duration::from_millis(100),
            success: true,
            error: None,
        };

        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.method, DiscoveryMethod::Environment);
    }

    #[test]
    fn test_discovery_result_failure() {
        let result = DiscoveryResult {
            services: vec![],
            method: DiscoveryMethod::ServiceRegistry,
            duration: Duration::from_millis(50),
            success: false,
            error: Some("Connection timeout".to_string()),
        };

        assert!(!result.success);
        assert!(result.error.is_some());
        assert_eq!(
            result.error.expect("Operation failed"),
            "Connection timeout"
        );
    }

    #[test]
    fn test_discover_services() {
        let config = DiscoveryConfig::default();
        let result = discover_services(&config).expect("Operation failed");

        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.services.len(), 1);
        assert_eq!(result.services[0].name, "NestGate Core");
    }

    #[test]
    fn test_health_check_running_service() {
        use crate::constants::{network_defaults::LOCALHOST_NAME, port_defaults::get_admin_port};
        let endpoint = format!("http://{}:{}", LOCALHOST_NAME, get_admin_port());
        let service = DiscoveredService {
            id: "test-1".to_string(),
            name: "Test".to_string(),
            service_type: ServiceType::Storage,
            state: ServiceState::Running,
            endpoint,
            capabilities: vec![],
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
            last_health_check: None,
        };

        let healthy = health_check_service(&service).expect("Operation failed");
        assert!(healthy);
    }

    #[test]
    fn test_health_check_stopped_service() {
        let service = DiscoveredService {
            id: "test-2".to_string(),
            name: "Test".to_string(),
            service_type: ServiceType::Storage,
            state: ServiceState::Stopped,
            endpoint: test_endpoint(8080),
            capabilities: vec![],
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
            last_health_check: None,
        };

        let healthy = health_check_service(&service).expect("Operation failed");
        assert!(!healthy);
    }

    #[test]
    fn test_health_check_starting_service() {
        let service = DiscoveredService {
            id: "test-3".to_string(),
            name: "Test".to_string(),
            service_type: ServiceType::Storage,
            state: ServiceState::Starting,
            endpoint: test_endpoint(8080),
            capabilities: vec![],
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
            last_health_check: None,
        };

        let healthy = health_check_service(&service).expect("Operation failed");
        assert!(!healthy);
    }

    #[test]
    fn test_health_check_failed_service() {
        let service = DiscoveredService {
            id: "test-4".to_string(),
            name: "Test".to_string(),
            service_type: ServiceType::Storage,
            state: ServiceState::Failed,
            endpoint: test_endpoint(8080),
            capabilities: vec![],
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
            last_health_check: None,
        };

        let healthy = health_check_service(&service).expect("Operation failed");
        assert!(!healthy);
    }

    #[test]
    fn test_capability_discovery_new() {
        let discovery = CapabilityDiscovery::new().expect("Operation failed");

        // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 endpoints by default (port range of 3)
        assert_eq!(discovery.discovery_endpoints.len(), 3);
        assert!(discovery.registry.contains_key("security"));
        assert!(discovery.registry.contains_key("ai"));
        assert!(discovery.registry.contains_key("orchestration"));
        assert!(discovery.registry.contains_key("storage"));
        assert!(discovery.registry.contains_key("compute"));
    }

    #[test]
    fn test_capability_discovery_default() {
        let discovery = CapabilityDiscovery::default();

        assert!(!discovery.registry.is_empty());
        // ✅ MIGRATED: ServiceDiscoveryConfig generates 3 endpoints by default
        assert_eq!(discovery.discovery_endpoints.len(), 3);
    }

    #[test]
    fn test_find_capabilities_existing() {
        let discovery = CapabilityDiscovery::new().expect("Operation failed");
        let capabilities = discovery
            .find_capabilities("security")
            .expect("Operation failed");

        assert!(!capabilities.is_empty());
        assert!(capabilities[0].contains("security"));
    }

    #[test]
    fn test_find_capabilities_nonexistent() {
        let discovery = CapabilityDiscovery::new().expect("Operation failed");
        let capabilities = discovery
            .find_capabilities("nonexistent")
            .expect("Operation failed");

        assert!(capabilities.is_empty());
    }

    #[test]
    fn test_register_capability() {
        let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

        let custom_endpoint = test_endpoint(9000) + "/custom";
        discovery.register_capability("custom".to_string(), custom_endpoint.clone());

        let capabilities = discovery
            .find_capabilities("custom")
            .expect("Operation failed");
        assert_eq!(capabilities.len(), 1);
        assert_eq!(capabilities[0], custom_endpoint);
    }

    #[test]
    fn test_register_multiple_capabilities() {
        let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

        discovery.register_capability("custom".to_string(), test_endpoint(9000) + "/custom1");
        discovery.register_capability("custom".to_string(), test_endpoint(9001) + "/custom2");

        let capabilities = discovery
            .find_capabilities("custom")
            .expect("Operation failed");
        assert_eq!(capabilities.len(), 2);
    }

    #[test]
    fn test_unregister_capability() {
        let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

        let temp_endpoint = test_endpoint(9000) + "/temp";
        discovery.register_capability("temp".to_string(), temp_endpoint.clone());

        let before = discovery
            .find_capabilities("temp")
            .expect("Operation failed");
        assert_eq!(before.len(), 1);

        discovery.unregister_capability("temp", &temp_endpoint);

        let after = discovery
            .find_capabilities("temp")
            .expect("Operation failed");
        assert_eq!(after.len(), 0);
    }

    #[test]
    fn test_unregister_nonexistent_capability() {
        let mut discovery = CapabilityDiscovery::new().expect("Operation failed");

        // Should not panic
        let nonexistent = test_endpoint(9000);
        discovery.unregister_capability("nonexistent", &nonexistent);
    }

    #[test]
    fn test_discovery_config_custom() {
        let config = DiscoveryConfig {
            endpoint: "http://custom:9000".to_string(),
            timeout: Duration::from_secs(60),
            max_retries: 5,
            discovery_interval: Duration::from_secs(120),
            methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::NetworkScan,
                DiscoveryMethod::Dns,
            ],
        };

        assert_eq!(config.endpoint, "http://custom:9000");
        assert_eq!(config.max_retries, 5);
        assert_eq!(config.methods.len(), 3);
    }

    #[test]
    fn test_discovered_service_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("region".to_string(), "us-west".to_string());

        let service = DiscoveredService {
            id: "test-meta".to_string(),
            name: "Test Meta Service".to_string(),
            service_type: ServiceType::Compute,
            state: ServiceState::Running,
            endpoint: test_endpoint(8080),
            capabilities: vec!["compute".to_string()],
            metadata,
            discovered_at: SystemTime::now(),
            last_health_check: Some(SystemTime::now()),
        };

        assert_eq!(service.metadata.len(), 2);
        assert_eq!(service.metadata.get("version"), Some(&"1.0.0".to_string()));
        assert_eq!(service.metadata.get("region"), Some(&"us-west".to_string()));
    }

    #[test]
    fn test_discovery_result_clone() {
        let result = DiscoveryResult {
            services: vec![],
            method: DiscoveryMethod::Configuration,
            duration: Duration::from_millis(75),
            success: true,
            error: None,
        };

        let cloned = result.clone();
        assert_eq!(cloned.success, result.success);
        assert_eq!(cloned.method, result.method);
    }
}
