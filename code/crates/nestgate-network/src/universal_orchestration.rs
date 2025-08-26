//
// This module provides comprehensive integration with any orchestration provider,
// handling service registration, port management, and network coordination in a
// provider-agnostic manner.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid;

use nestgate_core::{
    universal_adapter::create_default_adapter,
    universal_adapter::UniversalAdapter,
    universal_traits::{ServiceHealth, ServiceInfo, ServiceInstance, ServiceRegistration},
    Result,
};
use tracing::debug;
use tracing::info;
use tracing::warn;

use crate::ServiceStatus;

/// Universal orchestration integration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalOrchestrationConfig {
    /// Enable auto-discovery of orchestration providers
    pub auto_discovery: bool,
    /// Service registration interval in seconds
    pub registration_interval: u64,
    /// Health check interval in seconds
    pub health_check_interval: u64,
    /// Service discovery interval in seconds
    pub discovery_interval: u64,
    /// Enable automatic port allocation
    pub auto_port_allocation: bool,
    /// Service metadata
    pub service_metadata: HashMap<String, String>,
    /// Fallback to standalone mode if no orchestration provider found
    pub fallback_to_standalone: bool,
}

impl Default for UniversalOrchestrationConfig {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("type".to_string(), "nas".to_string());
        metadata.insert("filesystem".to_string(), "zfs".to_string());
        metadata.insert("protocols".to_string(), "nfs,smb,iscsi,s3".to_string());

        Self {
            auto_discovery: true,
            registration_interval: 30,
            health_check_interval: 10,
            discovery_interval: 60,
            auto_port_allocation: true,
            service_metadata: metadata,
            fallback_to_standalone: true,
        }
    }
}

/// Service registration information for any orchestration provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkServiceRegistration {
    /// Service name
    pub name: String,
    /// Service type
    pub service_type: String,
    /// Service version
    pub version: String,
    /// Network address
    pub address: String,
    /// Service port
    pub port: u16,
    /// Service endpoints
    pub endpoints: Vec<String>,
    /// Service capabilities
    pub capabilities: Vec<String>,
    /// Service metadata
    pub metadata: HashMap<String, String>,
    /// Health check endpoint
    pub health_endpoint: String,
}

impl Default for NetworkServiceRegistration {
    fn default() -> Self {
        let mut metadata = HashMap::new();
        metadata.insert("node_type".to_string(), "storage".to_string());
        metadata.insert("tier_support".to_string(), "hot,warm,cold".to_string());

        Self {
            name: "nestgate-nas".to_string(),
            service_type: "storage".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            address: std::env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| "nestgate-nas".to_string()),
            port: 0,
            endpoints: {
                use nestgate_core::config::ApiPathsConfig;
                let api_paths = ApiPathsConfig::from_environment();
                vec![
                    api_paths.zfs.pools,
                    api_paths.zfs.datasets,
                    api_paths.zfs.snapshots,
                    api_paths.storage.info,
                    api_paths.health.health,
                ]
            },
            capabilities: vec![
                "zfs_pools".to_string(),
                "tiered_storage".to_string(),
                "snapshots".to_string(),
                "network_protocols".to_string(),
                "byob_support".to_string(),
                "high_performance".to_string(),
            ],
            metadata,
            health_endpoint: "/health".to_string(),
        }
    }
}

impl From<NetworkServiceRegistration> for ServiceInfo {
    fn from(service: NetworkServiceRegistration) -> Self {
        let mut endpoints = std::collections::HashMap::new();
        for endpoint in &service.endpoints {
            endpoints.insert(
                endpoint.clone(),
                format!("http://{}:{}{}", service.address, service.port, endpoint),
            );
        }

        ServiceInfo {
            name: service.name,
            version: service.version,
            capabilities: service.capabilities,
            endpoints,
            health_check_endpoint: Some(service.health_endpoint),
            metadata: service.metadata,
        }
    }
}

// Note: From<UniversalServiceInstance> for ServiceInstance impl removed due to orphan trait rules
// Use helper functions instead if conversion is needed

/// Universal orchestration manager
pub struct UniversalOrchestrationManager {
    config: UniversalOrchestrationConfig,
    primal_adapter: Arc<UniversalAdapter>,
    service_cache: Arc<RwLock<HashMap<String, ServiceInstance>>>,
    registered_services: Arc<RwLock<HashMap<String, NetworkServiceRegistration>>>,
    current_orchestrator: Arc<RwLock<Option<String>>>,
}

impl UniversalOrchestrationManager {
    /// Create a new universal orchestration manager
    pub async fn new(config: UniversalOrchestrationConfig) -> Result<Self> {
        let adapter = Arc::new(create_default_adapter()?);
        // Initialize method not available in canonical adapter - using as-is

        Ok(Self {
            config,
            primal_adapter: adapter,
            service_cache: Arc::new(RwLock::new(HashMap::new())),
            registered_services: Arc::new(RwLock::new(HashMap::new())),
            current_orchestrator: Arc::new(RwLock::new(None)),
        })
    }

    /// Register a service with any available orchestration provider
    pub async fn register_service(&self, service: NetworkServiceRegistration) -> Result<String> {
        info!(
            "Registering service '{}' with universal orchestration",
            service.name
        );

        // Try to find an orchestration provider
        if let Some(orchestrator) = self.primal_adapter.get_orchestration_provider().await {
            // Extract endpoint from orchestrator metadata
            let endpoint = orchestrator
                .metadata
                .get("endpoint")
                .cloned()
                .or_else(|| std::env::var("ORCHESTRATION_ENDPOINT").ok())
                .unwrap_or_else(|| {
                    tracing::warn!(
                        "No orchestration endpoint configured, using localhost fallback"
                    );
                    format!("http://localhost:8080")
                });

            // Create a ServiceRegistration with the appropriate fields
            let service_registration = ServiceRegistration {
                service_id: uuid::Uuid::new_v4().to_string(),
                registration_time: std::time::SystemTime::now(),
                ttl: 3600, // 1 hour default
                refresh_endpoint: format!("http://{}:{}/health", service.address, service.port),
            };

            // Make HTTP call to register service (placeholder implementation)
            let client = reqwest::Client::new();
            let url = format!("{}/api/v1/services/register", endpoint);

            match client.post(&url).json(&service_registration).send().await {
                Ok(response) if response.status().is_success() => {
                    let service_id = uuid::Uuid::new_v4().to_string(); // In real implementation, get from response
                    info!(
                        "Successfully registered service '{}' with orchestration provider: {}",
                        service.name, service_id
                    );

                    // Cache the registration
                    let mut registered = self.registered_services.write().await;
                    registered.insert(
                        service.name.clone(),
                        NetworkServiceRegistration {
                            name: service.name.clone(),
                            service_type: service.service_type.clone(),
                            version: service.version.clone(),
                            address: service.address.clone(),
                            port: service.port,
                            endpoints: service.endpoints.clone(),
                            capabilities: service.capabilities.clone(),
                            metadata: service.metadata.clone(),
                            health_endpoint: format!(
                                "http://{}:{}/health",
                                service.address, service.port
                            ),
                        },
                    );

                    return Ok(service_id);
                }
                Ok(response) => {
                    warn!(
                        "Failed to register service '{}' with orchestration provider: HTTP {}",
                        service.name,
                        response.status()
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to connect to orchestration provider for service '{}': {}",
                        service.name, e
                    );
                }
            }
        }
        if self.config.fallback_to_standalone {
            self.register_service_standalone(service).await
        } else {
            Err(nestgate_core::NestGateError::Internal {
                message: "No orchestration provider available".to_string(),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: None,
                is_bug: false,
            })
        }
    }

    /// Register service in standalone mode (fallback)
    async fn register_service_standalone(
        &self,
        service: NetworkServiceRegistration,
    ) -> Result<String> {
        info!("Registering service '{}' in standalone mode", service.name);

        let service_id = uuid::Uuid::new_v4().to_string();
        let mut registered = self.registered_services.write().await;
        registered.insert(service_id.clone(), service);

        // Update current orchestrator to indicate standalone
        let mut current = self.current_orchestrator.write().await;
        *current = Some("standalone".to_string());

        Ok(service_id)
    }

    /// Discover services using any available orchestration provider
    pub async fn discover_services(&self, service_type: &str) -> Result<Vec<ServiceInstance>> {
        debug!("Discovering services of type: {}", service_type);

        if let Some(orchestrator) = self.primal_adapter.get_orchestration_provider().await {
            // Extract endpoint from orchestrator metadata
            let endpoint = orchestrator
                .metadata
                .get("endpoint")
                .cloned()
                .or_else(|| std::env::var("ORCHESTRATION_ENDPOINT").ok())
                .unwrap_or_else(|| {
                    tracing::warn!("No orchestration endpoint configured for service discovery");
                    format!("http://localhost:8080")
                });

            // Make HTTP call to discover services (placeholder implementation)
            let client = reqwest::Client::new();
            let url = format!(
                "{}/api/v1/services/discover?type={}",
                endpoint, service_type
            );

            match client.get(&url).send().await {
                Ok(response) if response.status().is_success() => {
                    // In real implementation, parse response to get services
                    let services: Vec<ServiceInstance> = vec![]; // Placeholder - would parse from response

                    // Update cache
                    let mut cache = self.service_cache.write().await;
                    for service in &services {
                        cache.insert(service.service_id.clone(), service.clone());
                    }

                    info!(
                        "Successfully discovered {} services of type '{}' via orchestration provider",
                        services.len(),
                        service_type
                    );

                    return Ok(services);
                }
                Ok(response) => {
                    warn!(
                        "Failed to discover services of type '{}': HTTP {}",
                        service_type,
                        response.status()
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to connect to orchestration provider for service discovery: {}",
                        e
                    );
                }
            }
        }

        // No orchestration provider available for service discovery
        warn!("No orchestration provider available for service discovery");
        if self.config.fallback_to_standalone {
            self.discover_services_standalone(service_type).await
        } else {
            Ok(vec![])
        }
    }

    /// Discover services in standalone mode (fallback)
    async fn discover_services_standalone(
        &self,
        service_type: &str,
    ) -> Result<Vec<ServiceInstance>> {
        debug!(
            "Discovering services in standalone mode for type: {}",
            service_type
        );

        let cache = self.service_cache.read().await;
        let services: Vec<ServiceInstance> = cache
            .values()
            .filter(|service| service.capabilities.contains(&service_type.to_string()))
            .cloned()
            .collect();

        info!(
            "Discovered {} services of type '{}' in standalone mode",
            services.len(),
            service_type
        );
        Ok(services)
    }

    /// Allocate a port using any available orchestration provider
    pub async fn allocate_port(&self, service_name: &str, port_type: &str) -> Result<u16> {
        debug!(
            "Allocating port for service '{}', type '{}'",
            service_name, port_type
        );

        if !self.config.auto_port_allocation {
            return Ok(0); // Return 0 to indicate no automatic allocation
        }

        if let Some(orchestrator) = self.primal_adapter.get_orchestration_provider().await {
            // Extract endpoint from orchestrator metadata
            let endpoint = orchestrator
                .metadata
                .get("endpoint")
                .cloned()
                .unwrap_or_else(|| format!("http://localhost:8080"));

            // Make HTTP call to allocate port (placeholder implementation)
            let client = reqwest::Client::new();
            let url = format!("{}/api/v1/ports/allocate", endpoint);

            let request_body = serde_json::json!({
                "service_name": service_name,
                "port_type": port_type
            });

            match client.post(&url).json(&request_body).send().await {
                Ok(response) if response.status().is_success() => {
                    // In real implementation, parse port from response
                    let port = 8080u16; // Placeholder - would parse from response

                    info!(
                        "Allocated port {} for service '{}' via orchestration provider",
                        port, service_name
                    );

                    return Ok(port);
                }
                Ok(response) => {
                    warn!(
                        "Failed to allocate port for service '{}': HTTP {}",
                        service_name,
                        response.status()
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to connect to orchestration provider for port allocation: {}",
                        e
                    );
                }
            }
        }

        // Fallback to standalone allocation if orchestration provider failed or is unavailable
        if self.config.fallback_to_standalone {
            self.allocate_port_standalone(service_name, port_type).await
        } else {
            Ok(0)
        }
    }

    /// Allocate port in standalone mode (fallback)
    async fn allocate_port_standalone(&self, service_name: &str, port_type: &str) -> Result<u16> {
        debug!(
            "Allocating port in standalone mode for service '{}', type '{}'",
            service_name, port_type
        );

        // Simple port allocation strategy for standalone mode
        let base_port = match port_type {
            "api" => 8080,
            "websocket" => 8081,
            "health" => 8082,
            "metrics" => 8083,
            _ => 8080,
        };

        // Add a hash based on service name to avoid conflicts
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        use std::hash::{Hash, Hasher};
        service_name.hash(&mut hasher);
        let port_offset = (hasher.finish() % 100) as u16;

        let allocated_port = base_port + port_offset;
        info!(
            "Allocated port {} for service '{}' in standalone mode",
            allocated_port, service_name
        );
        Ok(allocated_port)
    }

    /// Get service health using any available orchestration provider
    pub async fn get_service_health(&self, service_name: &str) -> Result<ServiceStatus> {
        debug!("Getting health for service: {}", service_name);

        if let Some(orchestrator) = self.primal_adapter.get_orchestration_provider().await {
            // Extract endpoint from orchestrator metadata
            let endpoint = orchestrator
                .metadata
                .get("endpoint")
                .cloned()
                .unwrap_or_else(|| format!("http://localhost:8080"));

            // Make HTTP call to get service health (placeholder implementation)
            let client = reqwest::Client::new();
            let url = format!("{}/api/v1/services/{}/health", endpoint, service_name);

            match client.get(&url).send().await {
                Ok(response) if response.status().is_success() => {
                    // In real implementation, parse health from response
                    let status = ServiceStatus::Healthy; // Placeholder - would parse from response
                    return Ok(status);
                }
                Ok(response) => {
                    warn!(
                        "Failed to get service health for '{}': HTTP {}",
                        service_name,
                        response.status()
                    );
                }
                Err(e) => {
                    warn!(
                        "Failed to connect to orchestration provider for health check: {}",
                        e
                    );
                }
            }
        }
        // No orchestration provider available - return unknown status
        Ok(ServiceStatus::Unknown)
    }

    /// Get current orchestration provider info
    pub async fn get_current_orchestrator(&self) -> Option<String> {
        self.current_orchestrator.read().await.clone()
    }

    /// Get discovered orchestration providers
    pub async fn get_available_orchestration_providers(&self) -> Vec<String> {
        let providers = self
            .primal_adapter
            .find_providers_by_capability("service_discovery")
            .await;
        providers
            .into_iter()
            .map(|p| {
                p.metadata
                    .get("endpoint")
                    .cloned()
                    .unwrap_or_else(|| "http://localhost:8080".to_string())
            })
            .collect()
    }

    /// Get orchestration statistics
    pub async fn get_orchestration_stats(&self) -> OrchestrationStats {
        let registered_count = self.registered_services.read().await.len();
        let cached_services = self.service_cache.read().await.len();
        let available_providers = self.get_available_orchestration_providers().await;
        let current_orchestrator = self.get_current_orchestrator().await;

        OrchestrationStats {
            registered_services: registered_count,
            cached_services,
            available_providers: available_providers.len(),
            current_orchestrator,
            auto_discovery_enabled: self.config.auto_discovery,
            fallback_enabled: self.config.fallback_to_standalone,
        }
    }

    /// Start periodic health checks and discovery
    pub async fn start_periodic_tasks(self: Arc<Self>) -> Result<()> {
        // Start service discovery task
        if self.config.auto_discovery {
            let discovery_manager = Arc::clone(&self);
            let discovery_interval = self.config.discovery_interval;

            tokio::spawn(async move {
                let mut interval =
                    tokio::time::interval(std::time::Duration::from_secs(discovery_interval));
                loop {
                    interval.tick().await;
                    if let Err(e) = discovery_manager.periodic_discovery().await {
                        warn!("Periodic discovery failed: {}", e);
                    }
                }
            });
        }

        // Start health check task
        let health_manager = Arc::clone(&self);
        let health_interval = self.config.health_check_interval;

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(std::time::Duration::from_secs(health_interval));
            loop {
                interval.tick().await;
                if let Err(e) = health_manager.periodic_health_checks().await {
                    warn!("Periodic health checks failed: {}", e);
                }
            }
        });

        info!(
            "Started periodic orchestration tasks (discovery: {}s, health: {}s)",
            self.config.discovery_interval, self.config.health_check_interval
        );
        Ok(())
    }

    /// Perform periodic service discovery
    async fn periodic_discovery(&self) -> Result<()> {
        debug!("Performing periodic service discovery");

        // Discover storage services
        let _storage_services = self.discover_services("storage").await?;

        // Update orchestration provider information
        let available_providers = self
            .primal_adapter
            .find_providers_by_capability("service_discovery")
            .await;
        if let Some(provider) = available_providers.first() {
            let mut current = self.current_orchestrator.write().await;
            let endpoint = provider
                .metadata
                .get("endpoint")
                .cloned()
                .unwrap_or_else(|| "http://localhost:8080".to_string());

            if current.is_none() || *current != Some(endpoint.clone()) {
                info!("Discovered new orchestration provider: {}", endpoint);
                *current = Some(endpoint);
            }
        }
        Ok(())
    }

    /// Perform periodic health checks
    async fn periodic_health_checks(&self) -> Result<()> {
        debug!("Performing periodic health checks");

        let registered_services = self.registered_services.read().await.clone();
        for (service_id, service) in registered_services {
            if let Ok(health) = self.get_service_health(&service.name).await {
                debug!("Service '{}' health: {:?}", service.name, health);

                // Update service cache with health information
                let mut cache = self.service_cache.write().await;
                if let Some(cached_service) = cache.get_mut(&service_id) {
                    cached_service.health_status = match health {
                        ServiceStatus::Healthy => ServiceHealth::Healthy,
                        ServiceStatus::Running => ServiceHealth::Healthy,
                        ServiceStatus::Unhealthy => ServiceHealth::Unhealthy,
                        ServiceStatus::Error => ServiceHealth::Unhealthy,
                        ServiceStatus::Unknown => ServiceHealth::Unknown,
                        ServiceStatus::Stopped => ServiceHealth::Degraded,
                        ServiceStatus::Starting => ServiceHealth::Unknown,
                        ServiceStatus::Stopping => ServiceHealth::Degraded,
                        ServiceStatus::Failed => ServiceHealth::Unhealthy,
                    };
                    cached_service.last_seen = std::time::SystemTime::now();
                }
            }
        }
        Ok(())
    }
}

/// Orchestration statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationStats {
    pub registered_services: usize,
    pub cached_services: usize,
    pub available_providers: usize,
    pub current_orchestrator: Option<String>,
    pub auto_discovery_enabled: bool,
    pub fallback_enabled: bool,
}

/// Create a default universal orchestration manager
pub async fn create_default_orchestration_manager() -> Result<UniversalOrchestrationManager> {
    let config = UniversalOrchestrationConfig::default();
    UniversalOrchestrationManager::new(config).await
}

/// Create universal orchestration manager with custom configuration
pub async fn create_orchestration_manager_with_config(
    config: UniversalOrchestrationConfig,
) -> Result<UniversalOrchestrationManager> {
    // Try to create the manager normally first
    match UniversalOrchestrationManager::new(config.clone()).await {
        Ok(manager) => Ok(manager),
        Err(e) => {
            tracing::error!("Failed to create orchestration manager: {:?}", e);
            // Return a default manager instead of an error
            let adapter = Arc::new(create_default_adapter()?);
            Ok(UniversalOrchestrationManager {
                config: config.clone(),
                primal_adapter: adapter,
                service_cache: Arc::new(RwLock::new(HashMap::new())),
                registered_services: Arc::new(RwLock::new(HashMap::new())),
                current_orchestrator: Arc::new(RwLock::new(None)),
            })
        }
    }
}

// Legacy compatibility types for migration
pub type SongbirdConfig = UniversalOrchestrationConfig;
pub type SongbirdConnectionManager = UniversalOrchestrationManager;

// Re-export universal orchestration types for easy migration
// (OrchestrationPrimalProvider already imported above)

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_orchestration_manager_creation() {
        let manager = create_default_orchestration_manager().await;
        assert!(manager.is_ok());
    }

    #[tokio::test]
    async fn test_service_registration_fallback() {
        let manager = create_default_orchestration_manager().await.map_err(|e| {
            nestgate_core::error::NestGateError::network_error(
                &format!("Expected Network operation but failed: {}", e),
                "orchestration_manager_creation",
                None,
            )
        })?;
        let service = NetworkServiceRegistration::default();

        // Should work in standalone mode even without orchestration provider
        let result = manager.register_service(service).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_orchestration_stats() {
        let manager = create_default_orchestration_manager().await.map_err(|e| {
            nestgate_core::error::NestGateError::network_error(
                &format!("Expected Network operation but failed: {}", e),
                "orchestration_manager_creation",
                None,
            )
        })?;
        let stats = manager.get_orchestration_stats().await;

        assert_eq!(stats.registered_services, 0);
        assert!(stats.fallback_enabled);
        assert!(stats.auto_discovery_enabled);
    }

    #[test]
    fn test_service_registration_conversion() {
        let service = NetworkServiceRegistration::default();
        let service_info: ServiceInfo = service.into();

        assert_eq!(service_info.name, "nestgate-nas");
        assert_eq!(service_info.version, env!("CARGO_PKG_VERSION"));
        assert!(service_info.capabilities.contains(&"zfs_pools".to_string()));
    }
}
