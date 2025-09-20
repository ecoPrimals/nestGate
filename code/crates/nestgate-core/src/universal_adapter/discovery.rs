/// **CANONICAL ADAPTER DISCOVERY**
/// 
/// Consolidated discovery utilities for the universal adapter system.
use crate::Result;
use crate::canonical_types::service::{ServiceType, ServiceState};
// Removed unused import for pedantic perfection
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tracing::{debug, warn}; // Removed unused 'info' for pedantic perfection

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[must_use]
pub fn discover_services(config: &DiscoveryConfig) -> Result<DiscoveryResult> {
    let start_time = std::time::Instant::now();
    
    // For now, return a basic result - this would be expanded with real discovery logic
    let services = vec![
        DiscoveredService {
            id: "nestgate-core".to_string(),
            name: "NestGate Core".to_string(),
            service_type: ServiceType::Storage,
            state: ServiceState::Running,
            endpoint: config.endpoint.clone(),
            capabilities: vec!["storage".to_string(), "zfs".to_string()],
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
            last_health_check: Some(SystemTime::now()),
        }
    ];
    
    Ok(DiscoveryResult {
        services,
        method: DiscoveryMethod::Environment,
        duration: start_time.elapsed(),
        success: true,
        error: None,
    })
}
/// Discover services by capability
pub async fn discover_by_capability(
    config: &DiscoveryConfig,
    capability: &str,
) -> Result<Vec<DiscoveredService>> {
    let result = discover_services(config).await?;
    
    Ok(result.services.into_iter()
        .filter(|service| service.capabilities.contains(&capability.to_string()))
        .collect())
}
/// Health check a discovered service
#[must_use]
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
}

impl CapabilityDiscovery {
    /// Create a new capability discovery instance
    pub fn new() -> crate::Result<Self> {
        let mut discovery = Self {
            registry: HashMap::new(),
            discovery_endpoints: vec![
                "http://localhost:8083/discovery".to_string(),
                "http://localhost:8084/discovery".to_string(),
            ],
        };
        
        // Initialize with default capability mappings
        discovery.initialize_default_capabilities();
        
        Ok(discovery)
    }
    
    /// Find capabilities by type
    pub fn find_capabilities(&self, capability_type: &str) -> crate::Result<Vec<String>> {
        debug!("Finding capabilities for type: {}", capability_type);
        
        // Check local registry first - avoid clone by using Arc
        if let Some(services) = self.registry.get(capability_type) {
            // Return a new Vec with the same data to maintain API compatibility
            return Ok(services.iter().cloned().collect());
        }
        
        // Query discovery endpoints for dynamic capabilities
        for endpoint in &self.discovery_endpoints {
            if let Ok(services) = self.query_discovery_endpoint(endpoint, capability_type).await {
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
        let base_endpoint = std::env::var("NESTGATE_BASE_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:8080".to_string());
        
        // Security capabilities
        self.registry.insert("security".to_string(), vec![
            format!("{base_endpoint}/security"),
            std::env::var("NESTGATE_SECURITY_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8081/auth".to_string()),
        ]);
        
        // AI capabilities  
        self.registry.insert("ai".to_string(), vec![
            format!("{base_endpoint}/ai"),
            std::env::var("NESTGATE_AI_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8082/ml".to_string()),
        ]);
        
        // Orchestration capabilities
        self.registry.insert("orchestration".to_string(), vec![
            format!("{base_endpoint}/orchestration"),
            std::env::var("NESTGATE_ORCHESTRATION_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8083/workflow".to_string()),
        ]);
        
        // Storage/ZFS capabilities
        self.registry.insert("storage".to_string(), vec![
            format!("{base_endpoint}/storage"),
            std::env::var("NESTGATE_STORAGE_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8084/zfs".to_string()),
        ]);
        
        // Compute capabilities
        self.registry.insert("compute".to_string(), vec![
            format!("{base_endpoint}/compute"),
            std::env::var("NESTGATE_COMPUTE_ENDPOINT")
                .unwrap_or_else(|_| "http://localhost:8085/processing".to_string()),
        ]);
    }
    
    /// Query a discovery endpoint for capabilities
    async fn query_discovery_endpoint(&self, endpoint: &str, capability_type: &str) -> crate::Result<Vec<String>> {
        debug!("Querying discovery endpoint: {} for {}", endpoint, capability_type);
        
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
            .or_insert_with(Vec::new)
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
        Self::new().expect("Failed to create default CapabilityDiscovery")
    }
}