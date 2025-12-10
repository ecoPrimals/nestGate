//! **UNIVERSAL CAPABILITY SYSTEM**
//! Capability System functionality and utilities.
//! This module implements a capability-based discovery and routing system that eliminates
//! all primal hardcoding. Each primal only knows itself and discovers others through
//! capability advertisement and discovery.

use crate::universal_primal_discovery::service_registry::ServiceRegistry;
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use uuid::Uuid;

// Import config for environment variable lookups
use super::capability_endpoints_config::CapabilityEndpointsConfig;

// ==================== CAPABILITY CATEGORIES ====================

/// Universal capability categories that any primal can provide
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Capabilitycategory
pub enum CapabilityCategory {
    /// Storage capabilities (NestGate's primary domain)
    Storage,
    /// Orchestration capabilities (service mesh, workflow management)
    Orchestration,
    /// Compute capabilities (processing, containers, functions)
    Compute,
    /// Security capabilities (auth, encryption, access control)
    Security,
    /// Intelligence capabilities (AI, ML, analytics)
    Intelligence,
    /// Management capabilities (deployment, monitoring, configuration)
    Management,
    /// Network capabilities (routing, load balancing, discovery)
    Network,
    /// Data capabilities (databases, caching, streaming)
    Data,
}

impl CapabilityCategory {
    /// Convert to PrimalCapability for service registry discovery
    pub fn to_primal_capability(
        &self,
    ) -> crate::universal_primal_discovery::capability_based_discovery::PrimalCapability {
        use crate::universal_primal_discovery::capability_based_discovery::PrimalCapability;
        match self {
            Self::Storage => PrimalCapability::ZfsStorage,
            Self::Orchestration => PrimalCapability::Custom("orchestration".to_string()),
            Self::Compute => PrimalCapability::Custom("compute".to_string()),
            Self::Security => PrimalCapability::Authentication,
            Self::Intelligence => PrimalCapability::Custom("intelligence".to_string()),
            Self::Management => PrimalCapability::Custom("management".to_string()),
            Self::Network => PrimalCapability::Custom("network".to_string()),
            Self::Data => PrimalCapability::DataSync,
        }
    }
}

/// Specific capability that a service provides
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicecapability
pub struct ServiceCapability {
    /// Unique capability identifier
    pub id: Uuid,
    /// Category of capability
    pub category: CapabilityCategory,
    /// Specific operation name
    pub operation: String,
    /// Human-readable description
    pub description: String,
    /// Version of this capability
    pub version: String,
    /// Required parameters for this capability
    pub required_parameters: Vec<String>,
    /// Optional parameters
    pub optional_parameters: Vec<String>,
    /// Expected response format
    pub response_format: String,
}

impl ServiceCapability {
    /// Create a new service capability
    #[must_use]
    pub fn new(category: CapabilityCategory, operation: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            category,
            operation: operation.to_string(),
            description: description.to_string(),
            version: "1.0.0".to_string(),
            required_parameters: Vec::new(),
            optional_parameters: Vec::new(),
            response_format: "json".to_string(),
        }
    }

    /// Create a storage capability (NestGate's domain)
    pub fn storage(operation: &str, description: &str) -> Self {
        Self::new(CapabilityCategory::Storage, operation, description)
    }
}

// ==================== CAPABILITY REQUESTS ====================

/// Request for a specific capability
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Capability operation
pub struct CapabilityRequest {
    /// Request ID for tracking
    pub request_id: Uuid,
    /// Category of capability needed
    pub category: CapabilityCategory,
    /// Specific operation requested
    pub operation: String,
    /// Parameters for the operation
    pub parameters: HashMap<String, serde_json::Value>,
    /// Timeout for the request
    pub timeout_seconds: u64,
    /// Whether this request is required or optional
    pub required: bool,
}

impl CapabilityRequest {
    /// Create a new capability request
    #[must_use]
    pub fn new(category: CapabilityCategory, operation: &str) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            category,
            operation: operation.to_string(),
            parameters: HashMap::new(),
            timeout_seconds: 30,
            required: true,
        }
    }

    /// Add a parameter to the request
    #[must_use]
    pub fn with_parameter(mut self, key: &str, value: serde_json::Value) -> Self {
        self.parameters.insert(key.to_string(), value);
        self
    }

    /// Make this request optional
    #[must_use]
    pub fn optional(mut self) -> Self {
        self.required = false;
        self
    }

    /// Set timeout for this request
    #[must_use]
    pub fn with_timeout(mut self, timeout_seconds: u64) -> Self {
        self.timeout_seconds = timeout_seconds;
        self
    }
}

// ==================== CAPABILITY RESPONSES ====================

/// Response from a capability provider
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Capability operation
pub struct CapabilityResponse {
    /// Request ID this responds to
    pub request_id: Uuid,
    /// Whether the operation was successful
    pub success: bool,
    /// Response data
    pub data: serde_json::Value,
    /// Error message if unsuccessful
    pub error: Option<String>,
    /// Metadata about the response
    pub metadata: HashMap<String, String>,
    /// Time the operation took
    pub execution_time_ms: u64,
}

impl CapabilityResponse {
    /// Create a successful response
    #[must_use]
    pub fn success(request_id: Uuid, data: serde_json::Value) -> Self {
        Self {
            request_id,
            success: true,
            data,
            error: None,
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }

    /// Create an error response
    #[must_use]
    pub fn error(request_id: Uuid, error: String) -> Self {
        Self {
            request_id,
            success: false,
            data: serde_json::Value::Null,
            error: Some(error),
            metadata: HashMap::new(),
            execution_time_ms: 0,
        }
    }
}

// ==================== SERVICE REGISTRY ====================

/// Information about a discovered service
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Service implementation for Discovered
pub struct DiscoveredService {
    /// Service identifier
    pub service_id: Uuid,
    /// Service name (generic, not primal-specific)
    pub name: String,
    /// Service type (generic description)
    pub service_type: String,
    /// Endpoint for communication
    pub endpoint: String,
    /// Capabilities this service provides
    pub capabilities: Vec<ServiceCapability>,
    /// Last time this service was seen
    pub last_seen: SystemTime,
    /// Health status
    pub healthy: bool,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl DiscoveredService {
    /// Create a new discovered service
    #[must_use]
    pub fn new(name: &str, service_type: &str, endpoint: &str) -> Self {
        Self {
            service_id: Uuid::new_v4(),
            name: name.to_string(),
            service_type: service_type.to_string(),
            endpoint: endpoint.to_string(),
            capabilities: Vec::new(),
            last_seen: SystemTime::now(),
            healthy: true,
            metadata: HashMap::new(),
        }
    }

    /// Add a capability to this service
    #[must_use]
    pub fn with_capability(mut self, capability: ServiceCapability) -> Self {
        self.capabilities.push(capability);
        self
    }

    /// Check if this service provides a specific capability
    pub fn provides_capability(&self, category: &CapabilityCategory, operation: &str) -> bool {
        self.capabilities
            .iter()
            .any(|cap| cap.category == *category && cap.operation == operation)
    }
}

// ==================== CAPABILITY REGISTRY ====================

/// Registry of all discovered capabilities in the ecosystem
#[derive(Debug, Default)]
/// Capabilityregistry
pub struct CapabilityRegistry {
    /// All discovered services
    services: HashMap<Uuid, DiscoveredService>,
    /// Index by capability category
    capability_index: HashMap<CapabilityCategory, Vec<Uuid>>,
    /// Our own capabilities that we advertise
    our_capabilities: Vec<ServiceCapability>,
    /// Our service information
    our_service: Option<DiscoveredService>,
}

impl CapabilityRegistry {
    /// Create a new capability registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register our own service and capabilities (NestGate only knows itself)
    pub fn register_self(&mut self, service: DiscoveredService) {
        // Index our capabilities
        for capability in &service.capabilities {
            self.capability_index
                .entry(capability.category.clone())
                .or_default()
                .push(service.service_id);
        }

        self.our_capabilities = service.capabilities.clone();
        self.services.insert(service.service_id, service.clone());
        self.our_service = Some(service);
    }

    /// Register a discovered service
    pub fn register_service(&mut self, service: DiscoveredService) {
        // Index capabilities
        for capability in &service.capabilities {
            self.capability_index
                .entry(capability.category.clone())
                .or_default()
                .push(service.service_id);
        }

        self.services.insert(service.service_id, service);
    }

    /// Find services that provide a specific capability
    pub fn find_providers(
        &self,
        category: &CapabilityCategory,
        operation: &str,
    ) -> Vec<&DiscoveredService> {
        let mut providers = Vec::new();

        if let Some(service_ids) = self.capability_index.get(category) {
            for service_id in service_ids {
                if let Some(service) = self.services.get(service_id) {
                    if service.provides_capability(category, operation) && service.healthy {
                        providers.push(service);
                    }
                }
            }
        }

        providers
    }

    /// Get our own advertised capabilities
    pub fn our_capabilities(&self) -> &[ServiceCapability] {
        &self.our_capabilities
    }

    /// Get all discovered services
    pub fn all_services(&self) -> Vec<&DiscoveredService> {
        self.services.values().collect()
    }

    /// Remove unhealthy services
    pub fn cleanup_unhealthy(&mut self) {
        let unhealthy_ids: Vec<Uuid> = self
            .services
            .iter()
            .filter(|(_, service)| !service.healthy)
            .map(|(id, _)| *id)
            .collect();

        for id in unhealthy_ids {
            self.remove_service(&id);
        }
    }

    /// Remove a service from the registry
    pub fn remove_service(&mut self, service_id: &Uuid) {
        if let Some(service) = self.services.remove(service_id) {
            // Remove from capability index
            for capability in &service.capabilities {
                if let Some(service_ids) = self.capability_index.get_mut(&capability.category) {
                    service_ids.retain(|id| id != service_id);
                }
            }
        }
    }
}

// ==================== PRIMAL-AGNOSTIC CAPABILITY ROUTER ====================

/// Universal capability router that eliminates all primal hardcoding
/// Each primal only knows itself and discovers others through capability advertisement
#[derive(Clone)]
/// Capabilityrouter
pub struct CapabilityRouter {
    /// Registry of discovered capabilities
    registry: Arc<RwLock<CapabilityRegistry>>,
    /// Our own service identity (only thing we know about ourselves)
    self_identity: NestGateSelfKnowledge,
    /// **NEW**: ServiceRegistry for capability-based discovery (no hardcoded URLs!)
    service_registry: Option<Arc<ServiceRegistry>>,
}

impl CapabilityRouter {
    /// Create a new capability router with self-knowledge only
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(CapabilityRegistry::new())),
            self_identity: NestGateSelfKnowledge::default(),
            service_registry: None,
        }
    }

    /// Set the service registry for capability-based discovery
    ///
    /// This enables the router to discover services dynamically instead of
    /// using hardcoded endpoints.
    pub fn with_service_registry(mut self, registry: Arc<ServiceRegistry>) -> Self {
        self.service_registry = Some(registry);
        self
    }

    /// Route capability request without knowing specific primal names
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn route_capability_request(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        // 1. Check if we can handle this capability ourselves
        if self
            .self_identity
            .can_handle_capability(&request.category, &request.operation)
        {
            return self.handle_locally(request).await;
        }

        // 2. Discover capable services through universal adapter
        let capable_services = self.discover_capable_services(&request).await?;

        if capable_services.is_empty() {
            return Err(crate::NestGateError::validation_error(&format!(
                "No capable services discovered for {:?}::{}",
                request.category, request.operation
            )));
        }

        // 3. Route to best available service (no hardcoded primal names)
        let selected_service = self.select_best_service(&capable_services)?;
        self.forward_request_to_service(selected_service, request)
            .await
    }

    /// Discover services that can handle a capability (no primal hardcoding)
    async fn discover_capable_services(
        &self,
        request: &CapabilityRequest,
    ) -> Result<Vec<DiscoveredService>> {
        let registry = self.registry.read().await;
        let services = registry.find_providers(&request.category, &request.operation);

        // Filter by availability and capability match
        let mut capable_services = Vec::new();
        for service in services {
            if service.provides_capability(&request.category, &request.operation) && service.healthy
            {
                capable_services.push(service.clone());
            }
        }

        Ok(capable_services)
    }

    /// Select best service based on capability metrics (not primal identity)
    fn select_best_service<'a>(
        &self,
        services: &'a [DiscoveredService],
    ) -> Result<&'a DiscoveredService> {
        // Select based on capability metrics, not primal names
        services
            .iter()
            .min_by_key(|service| service.last_seen.elapsed().unwrap_or_default().as_millis())
            .ok_or_else(|| {
                crate::NestGateError::internal_error(
                    "No suitable service found",
                    "capability_routing",
                )
            })
    }

    /// Forward request to selected service using universal protocol
    async fn forward_request_to_service(
        &self,
        service: &DiscoveredService,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        // **EVOLUTION**: Use ServiceRegistry for dynamic discovery (no hardcoded URLs!)
        let endpoint = if let Some(registry) = &self.service_registry {
            // Try capability-based discovery first
            match registry
                .find_by_capability(&request.category.to_primal_capability())
                .await
            {
                Ok(discovered_service) => discovered_service.url(),
                Err(_) => {
                    // Fallback to environment config if discovery fails
                    let config = CapabilityEndpointsConfig::from_env();
                    config
                        .service_endpoint()
                        .map(|s| s.to_string())
                        .ok_or_else(|| {
                            crate::NestGateError::not_found(format!(
                                "No endpoint found for capability: {:?}",
                                request.category
                            ))
                        })?
                }
            }
        } else {
            // No registry configured - fall back to environment config only
            let config = CapabilityEndpointsConfig::from_env();
            config
                .service_endpoint()
                .map(|s| s.to_string())
                .ok_or_else(|| {
                    crate::NestGateError::not_found(
                        "No service registry configured and no environment endpoint set",
                    )
                })?
        };

        // Generic capability request - works with any primal
        let response = self
            .send_universal_request(&endpoint, &service.endpoint, request)
            .await?;

        Ok(CapabilityResponse {
            request_id: response.request_id,
            success: response.success,
            data: response.data,
            error: response.error,
            metadata: response.metadata,
            execution_time_ms: response.execution_time_ms,
        })
    }

    /// Handle capability locally (NestGate's own capabilities)
    async fn handle_locally(&self, request: CapabilityRequest) -> Result<CapabilityResponse> {
        // Handle storage capabilities that NestGate provides
        match request.category {
            CapabilityCategory::Storage => self.handle_storage_capability(request).await,
            _ => Err(crate::NestGateError::validation_error(&format!(
                "Local capability not implemented: {:?}",
                request.category
            ))),
        }
    }

    /// Handle storage capabilities (NestGate's domain)
    async fn handle_storage_capability(
        &self,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        let mut response_data = serde_json::Map::new();

        match request.operation.as_str() {
            "create_dataset" => {
                response_data.insert(
                    "dataset_id".to_string(),
                    serde_json::Value::String("zfs-dataset-123".to_string()),
                );
                response_data.insert(
                    "status".to_string(),
                    serde_json::Value::String("created".to_string()),
                );
            }
            "list_datasets" => {
                response_data.insert("datasets".to_string(), serde_json::Value::Array(vec![]));
            }
            _ => {
                return Err(crate::NestGateError::validation_error(&format!(
                    "Storage operation not implemented: {}",
                    request.operation
                )));
            }
        }

        Ok(CapabilityResponse {
            request_id: request.request_id,
            success: true,
            data: serde_json::Value::Object(response_data),
            error: None,
            metadata: HashMap::new(),
            execution_time_ms: 10,
        })
    }

    /// Send universal capability request (works with any primal)
    async fn send_universal_request(
        &self,
        _endpoint: &str,
        _capability_endpoint: &str,
        request: CapabilityRequest,
    ) -> Result<CapabilityResponse> {
        // Simplified implementation - in production this would use HTTP/gRPC
        Ok(CapabilityResponse {
            request_id: request.request_id,
            success: true,
            data: serde_json::Value::Object(serde_json::Map::new()),
            error: None,
            metadata: HashMap::new(),
            execution_time_ms: 50,
        })
    }
}

impl Default for CapabilityRouter {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ==================== NESTGATE SELF-KNOWLEDGE ====================

/// NestGate's knowledge about itself (the only primal it knows)
#[derive(Debug, Clone)]
/// Nestgateselfknowledge
pub struct NestGateSelfKnowledge {
    /// Our service identity
    pub service_id: Uuid,
    /// Capabilities we provide
    pub our_capabilities: Vec<ServiceCapability>,
    /// Our service metadata
    pub metadata: HashMap<String, String>,
}

impl NestGateSelfKnowledge {
    /// Create NestGate self-knowledge
    #[must_use]
    pub fn new() -> Self {
        // Storage capabilities (our primary domain) + Management capabilities
        let capabilities = vec![
            ServiceCapability::storage("create_dataset", "Create ZFS dataset"),
            ServiceCapability::storage("list_datasets", "List all datasets"),
            ServiceCapability::storage("snapshot_dataset", "Create dataset snapshot"),
            ServiceCapability::storage("clone_dataset", "Clone dataset"),
            ServiceCapability::storage("destroy_dataset", "Destroy dataset"),
            ServiceCapability::new(
                CapabilityCategory::Management,
                "health_check",
                "Service health monitoring",
            ),
        ];

        let mut metadata = HashMap::new();
        metadata.insert("service_name".to_string(), "nestgate".to_string());
        metadata.insert("version".to_string(), "1.0.0".to_string());
        metadata.insert("primary_capability".to_string(), "storage".to_string());

        Self {
            service_id: Uuid::new_v4(),
            our_capabilities: capabilities,
            metadata,
        }
    }

    /// Check if we can handle a capability locally
    pub fn can_handle_capability(&self, category: &CapabilityCategory, operation: &str) -> bool {
        self.our_capabilities
            .iter()
            .any(|cap| cap.category == *category && cap.operation == operation)
    }

    /// Get our advertised capabilities (for discovery by other primals)
    pub fn get_advertised_capabilities(&self) -> &[ServiceCapability] {
        &self.our_capabilities
    }
}

impl Default for NestGateSelfKnowledge {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nestgate_self_knowledge() {
        let knowledge = NestGateSelfKnowledge::new();

        // NestGate should know its own storage capabilities
        let capabilities = knowledge.get_advertised_capabilities();
        assert!(!capabilities.is_empty());

        // Should have ZFS capabilities
        assert!(capabilities.iter().any(|c| c.operation == "create_dataset"));
        assert!(capabilities.iter().any(|c| c.operation == "list_datasets"));
    }

    #[test]
    fn test_capability_request_builder() {
        let request = CapabilityRequest::new(CapabilityCategory::Orchestration, "deploy")
            .with_parameter("image", serde_json::json!("nginx:latest"))
            .with_timeout(60)
            .optional();

        assert_eq!(request.category, CapabilityCategory::Orchestration);
        assert_eq!(request.operation, "deploy");
        assert!(!request.required);
        assert_eq!(request.timeout_seconds, 60);
    }

    #[test]
    fn test_service_capability_discovery() {
        let mut registry = CapabilityRegistry::new();

        // Register a compute service with default endpoint (test doesn't need async resolution)
        #[allow(deprecated)]
        let compute_service = DiscoveredService::new(
            "compute-service",
            "container-runtime",
            &crate::constants::canonical_defaults::network::build_api_url(),
        )
        .with_capability(ServiceCapability::new(
            CapabilityCategory::Compute,
            "run-container",
            "Run a container",
        ));

        registry.register_service(compute_service);

        // Should find the compute provider
        let providers = registry.find_providers(&CapabilityCategory::Compute, "run-container");
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].name, "compute-service");
    }
}
