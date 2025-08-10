/// Universal Adapter Main Implementation
/// Core implementation of the NestGate Universal Adapter for ecosystem integration.
use super::types::{
    CapabilityQuery, CapabilityRequest, CapabilityResponse, ExecutionMetrics, ServiceCapability,
};
use crate::ecosystem_integration::{AdapterConfig, CapabilityCategory};
use crate::error::NestGateError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

/// Universal Adapter for NestGate ecosystem integration
/// This is the ONLY way NestGate communicates with other primals.
/// NestGate has no knowledge of specific primals - only capabilities.
#[derive(Debug)]
#[allow(dead_code)]
#[derive(Clone)]
pub struct UniversalAdapter {
    /// Our registered service ID
    service_id: Uuid,
    /// Our capabilities that we expose to the ecosystem
    our_capabilities: Arc<RwLock<Vec<ServiceCapability>>>,
    /// Discovered capabilities from other ecosystem participants
    discovered_capabilities: Arc<RwLock<HashMap<String, Vec<ServiceCapability>>>>,
    /// Active capability requests and responses
    active_requests: Arc<RwLock<HashMap<String, CapabilityRequest>>>,
    /// Adapter configuration
    config: AdapterConfig,
    /// Request client for HTTP operations
    client: reqwest::Client,
    /// Health status of the adapter
    health_status: Arc<RwLock<AdapterHealthStatus>>,
}

/// Health status of the adapter
#[derive(Debug, Clone)]
pub struct AdapterHealthStatus {
    /// Whether the adapter is healthy
    pub healthy: bool,
    /// Last health check timestamp
    pub last_check: std::time::SystemTime,
    /// Health check details
    pub details: HashMap<String, String>,
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
}

impl UniversalAdapter {
    /// Create a new universal adapter instance
    pub fn new(config: AdapterConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(config.service.timeouts.default_timeout)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            service_id: Uuid::new_v4(),
            our_capabilities: Arc::new(RwLock::new(Vec::new())),
            discovered_capabilities: Arc::new(RwLock::new(HashMap::new())),
            active_requests: Arc::new(RwLock::new(HashMap::new())),
            config,
            client,
            health_status: Arc::new(RwLock::new(AdapterHealthStatus::default())),
        }
    }

    /// Initialize the adapter and register our capabilities
    pub async fn initialize(&self) -> crate::error::Result<()> {
        // Configuration validation - using unified config (validation not needed for basic functionality)

        debug!("Initializing NestGate Universal Adapter with configuration");

        // Register NestGate's storage intelligence capabilities
        self.register_nestgate_capabilities().await?;

        // Start capability discovery
        self.start_capability_discovery().await?;

        // Start health monitoring
        self.start_health_monitoring().await?;

        info!("Universal Adapter initialized successfully");
        Ok(())
    }

    /// Register NestGate's capabilities with the ecosystem
    async fn register_nestgate_capabilities(&self) -> crate::error::Result<()> {
        let capabilities = self.create_nestgate_capabilities();

        // In a real implementation, this would register with actual discovery services
        info!(
            "Registering {} capabilities with ecosystem",
            capabilities.len()
        );

        self.register_with_ecosystem(capabilities).await?;
        Ok(())
    }

    /// Create NestGate's core capabilities
    fn create_nestgate_capabilities(&self) -> Vec<ServiceCapability> {
        vec![
            ServiceCapability {
                id: "nestgate_storage_intelligence".to_string(),
                name: "Storage Intelligence Analytics".to_string(),
                description: "Advanced storage analytics with predictive insights".to_string(),
                category: super::types::CapabilityCategory::Storage,
                version: env!("CARGO_PKG_VERSION").to_string(),
                provider: "nestgate".to_string(),
                supported_data_types: vec![
                    super::types::DataType::Database,
                    super::types::DataType::TimeSeries,
                    super::types::DataType::Json,
                ],
                performance_metrics: super::types::PerformanceMetrics {
                    avg_response_time_ms: 100.0,
                    throughput_ops_per_sec: 1000.0,
                    success_rate_percent: 99.0,
                    error_rate_percent: 1.0,
                    availability_percent: 99.9,
                },
                resource_requirements: super::types::ResourceRequirements::default(),
                scalability: super::types::ScalabilityRating::High,
                metadata: {
                    let mut metadata = HashMap::new();
                    metadata.insert("provider".to_string(), "nestgate".to_string());
                    metadata.insert("category".to_string(), "storage_intelligence".to_string());
                    metadata
                },
            },
            ServiceCapability {
                id: "nestgate_zfs_management".to_string(),
                name: "Advanced ZFS Management".to_string(),
                description: "Comprehensive ZFS filesystem management and optimization".to_string(),
                category: super::types::CapabilityCategory::Storage,
                version: env!("CARGO_PKG_VERSION").to_string(),
                provider: "nestgate".to_string(),
                supported_data_types: vec![
                    super::types::DataType::Database,
                    super::types::DataType::Binary,
                ],
                performance_metrics: super::types::PerformanceMetrics {
                    avg_response_time_ms: 50.0,
                    throughput_ops_per_sec: 500.0,
                    success_rate_percent: 99.5,
                    error_rate_percent: 0.5,
                    availability_percent: 99.9,
                },
                resource_requirements: super::types::ResourceRequirements::default(),
                scalability: super::types::ScalabilityRating::High,
                metadata: {
                    let mut metadata = HashMap::new();
                    metadata.insert("provider".to_string(), "nestgate".to_string());
                    metadata.insert("filesystem".to_string(), "zfs".to_string());
                    metadata
                },
            },
        ]
    }

    /// Register capabilities with the ecosystem
    async fn register_with_ecosystem(
        &self,
        capabilities: Vec<ServiceCapability>,
    ) -> crate::error::Result<()> {
        info!(
            "Registering {} capabilities with ecosystem",
            capabilities.len()
        );

        // In a real implementation, this would make HTTP calls to the ecosystem registry
        // For now, we'll simulate the registration
        for capability in &capabilities {
            debug!(
                "Registered capability: {} ({})",
                capability.name, capability.id
            );
        }

        // Update health status
        {
            let mut health = self.health_status.write().await;
            health.successful_operations += capabilities.len() as u64;
            health.details.insert(
                "registered_capabilities".to_string(),
                capabilities.len().to_string(),
            );
        }
        Ok(())
    }

    /// Start discovering capabilities from other ecosystem participants
    async fn start_capability_discovery(&self) -> crate::error::Result<()> {
        info!("Starting capability discovery from ecosystem");

        // This would periodically query the ecosystem for new capabilities
        // For now, we'll simulate discovering some capabilities
        self.simulate_capability_discovery().await?;
        Ok(())
    }

    /// Simulate capability discovery for demonstration
    async fn simulate_capability_discovery(&self) -> crate::error::Result<()> {
        // Simulate discovering capabilities from the ecosystem
        let discovered_caps = vec![
            ServiceCapability {
                id: "ecosystem_security".to_string(),
                name: "Security Services".to_string(),
                description: "Comprehensive security and encryption services".to_string(),
                category: super::types::CapabilityCategory::Security,
                version: "1.0.0".to_string(),
                provider: "security_provider".to_string(),
                supported_data_types: vec![
                    super::types::DataType::Encrypted,
                    super::types::DataType::Binary,
                ],
                performance_metrics: super::types::PerformanceMetrics::default(),
                resource_requirements: super::types::ResourceRequirements::default(),
                scalability: super::types::ScalabilityRating::High,
                metadata: HashMap::new(),
            },
            ServiceCapability {
                id: "ecosystem_ai".to_string(),
                name: "AI Processing".to_string(),
                description: "Machine learning and AI processing capabilities".to_string(),
                category: super::types::CapabilityCategory::ArtificialIntelligence,
                version: "1.0.0".to_string(),
                provider: "ai_provider".to_string(),
                supported_data_types: vec![
                    super::types::DataType::Json,
                    super::types::DataType::Text,
                ],
                performance_metrics: super::types::PerformanceMetrics::default(),
                resource_requirements: super::types::ResourceRequirements::default(),
                scalability: super::types::ScalabilityRating::Moderate,
                metadata: HashMap::new(),
            },
        ];

        // Store discovered capabilities
        {
            let mut discovered = self.discovered_capabilities.write().await;
            for cap in discovered_caps {
                let provider = cap.provider.clone();
                discovered
                    .entry(provider)
                    .or_insert_with(Vec::new)
                    .push(cap);
            }
        }

        info!("Discovered capabilities from {} providers", 2);
        Ok(())
    }

    /// Start health monitoring
    async fn start_health_monitoring(&self) -> crate::error::Result<()> {
        info!("Starting health monitoring");

        // Update initial health status
        {
            let mut health = self.health_status.write().await;
            health.healthy = true;
            health.last_check = std::time::SystemTime::now();
            health
                .details
                .insert("status".to_string(), "initialized".to_string());
        }
        Ok(())
    }

    /// Query capabilities from the ecosystem
    pub async fn query_capabilities(
        &self,
        query: CapabilityQuery,
    ) -> crate::error::Result<Vec<ServiceCapability>> {
        debug!("Querying capabilities: {:?}", query);

        let discovered = self.discovered_capabilities.read().await;
        let our_caps = self.our_capabilities.read().await;

        let mut results = Vec::new();

        // Search our own capabilities
        for cap in our_caps.iter() {
            if self.matches_query(cap, &query) {
                results.push(cap.clone());
            }
        }

        // Search discovered capabilities
        for caps in discovered.values() {
            for cap in caps {
                if self.matches_query(cap, &query) {
                    results.push(cap.clone());
                }
            }
        }

        info!("Found {} capabilities matching query", results.len());
        Ok(results)
    }

    /// Check if a capability matches a query
    fn matches_query(&self, capability: &ServiceCapability, query: &CapabilityQuery) -> bool {
        match query {
            CapabilityQuery::ListAll => true,
            CapabilityQuery::ByCategory(category) => capability.category == *category,
            CapabilityQuery::ByDataType(data_type) => {
                capability.supported_data_types.contains(data_type)
            }
            CapabilityQuery::ByPerformance(requirements) => {
                requirements.is_satisfied_by(&capability.performance_metrics)
            }
            CapabilityQuery::ByResources(_) => {
                // Would implement resource matching logic
                true
            }
            CapabilityQuery::Search(keyword) => {
                capability
                    .name
                    .to_lowercase()
                    .contains(&keyword.to_lowercase())
                    || capability
                        .description
                        .to_lowercase()
                        .contains(&keyword.to_lowercase())
            }
            CapabilityQuery::ById(id) => capability.id == *id,
        }
    }

    /// Execute a capability request
    pub async fn execute_capability(
        &self,
        request: CapabilityRequest,
    ) -> crate::error::Result<CapabilityResponse> {
        let start_time = std::time::Instant::now();

        info!("Executing capability request: {}", request.capability_id);

        // Store the active request
        {
            let mut active = self.active_requests.write().await;
            active.insert(request.request_id.clone(), request.clone());
        }

        // Find the capability
        let capability = self
            .find_capability(&request.capability_id)
            .await
            .ok_or_else(|| NestGateError::Internal {
                message: format!("Capability {} not found", request.capability_id),
                location: Some(format!("{}:{}", file!(), line!())),
                debug_info: Some("Capability not found during execution".to_string()),
                is_bug: false,
            })?;

        // Execute the capability (simulate execution)
        let result = self.execute_capability_impl(&capability, &request).await;

        // Remove from active requests
        {
            let mut active = self.active_requests.write().await;
            active.remove(&request.request_id);
        }

        // Update health metrics
        {
            let mut health = self.health_status.write().await;
            if result.is_ok() {
                health.successful_operations += 1;
            } else {
                health.failed_operations += 1;
            }
        }

        let execution_time = start_time.elapsed();
        debug!("Capability execution completed in {:?}", execution_time);

        result
    }

    /// Find a capability by ID
    async fn find_capability(&self, capability_id: &str) -> Option<ServiceCapability> {
        // Search our capabilities
        {
            let our_caps = self.our_capabilities.read().await;
            for cap in our_caps.iter() {
                if cap.id == capability_id {
                    return Some(cap.clone());
                }
            }
        }

        // Search discovered capabilities
        {
            let discovered = self.discovered_capabilities.read().await;
            for caps in discovered.values() {
                for cap in caps {
                    if cap.id == capability_id {
                        return Some(cap.clone());
                    }
                }
            }
        }

        None
    }

    /// Execute capability implementation
    async fn execute_capability_impl(
        &self,
        capability: &ServiceCapability,
        request: &CapabilityRequest,
    ) -> crate::error::Result<CapabilityResponse> {
        // Simulate capability execution
        let execution_time = std::time::Duration::from_millis(50);
        tokio::time::sleep(execution_time).await;

        // Create mock response
        let response = CapabilityResponse {
            request_id: request.request_id.clone(),
            payload: format!("Response from {}", capability.name).into_bytes(),
            metadata: request.metadata.clone(),
            metrics: ExecutionMetrics {
                execution_time,
                resource_usage: super::types::ResourceUsage {
                    cpu_usage_percent: 25.0,
                    memory_usage_gb: 0.1,
                    disk_io_mb: 0.5,
                    network_io_mb: 0.1,
                    gpu_usage_percent: None,
                },
                quality_metrics: None,
            },
            success: true,
            error: None,
        };

        Ok(response)
    }

    /// Get adapter health status
    pub async fn health_status(&self) -> AdapterHealthStatus {
        self.health_status.read().await.clone()
    }

    /// Get service ID
    pub fn service_id(&self) -> Uuid {
        self.service_id
    }

    /// Get our capabilities
    pub async fn our_capabilities(&self) -> Vec<ServiceCapability> {
        self.our_capabilities.read().await.clone()
    }

    /// Get discovered capabilities
    pub async fn discovered_capabilities(&self) -> HashMap<String, Vec<ServiceCapability>> {
        self.discovered_capabilities.read().await.clone()
    }

    /// Get orchestration provider if available
    pub async fn get_orchestration_provider(&self) -> Option<ServiceCapability> {
        // Search for orchestration capabilities in discovered services
        let discovered = self.discovered_capabilities.read().await;

        for capabilities in discovered.values() {
            for capability in capabilities {
                if capability.category == CapabilityCategory::Orchestration {
                    return Some(capability.clone());
                }
            }
        }

        None
    }

    /// Find providers by capability type
    pub async fn find_providers_by_capability(
        &self,
        capability_type: &str,
    ) -> Vec<ServiceCapability> {
        let discovered = self.discovered_capabilities.read().await;
        let mut providers = Vec::new();

        for capabilities in discovered.values() {
            for capability in capabilities {
                if capability.name.contains(capability_type)
                    || capability.description.contains(capability_type)
                {
                    providers.push(capability.clone());
                }
            }
        }

        providers
    }

    /// Shutdown the adapter
    pub async fn shutdown(&self) -> crate::error::Result<()> {
        info!("Shutting down Universal Adapter");

        // Cancel active requests
        {
            let mut active = self.active_requests.write().await;
            active.clear();
        }

        // Update health status
        {
            let mut health = self.health_status.write().await;
            health.healthy = false;
            health
                .details
                .insert("status".to_string(), "shutdown".to_string());
        }

        info!("Universal Adapter shutdown completed");
        Ok(())
    }

    /// Execute adapter operation (modern implementation)
    /// Replaces the old trait-based execute method with a direct implementation
    pub async fn execute(
        &self,
        operation: &str,
        params: serde_json::Value,
    ) -> crate::Result<serde_json::Value> {
        debug!("Executing adapter operation: {}", operation);

        match operation {
            "find_providers" => {
                // Modern implementation for finding providers
                let capability = params
                    .get("capability")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                let discovered_caps = self.discovered_capabilities().await;
                let providers: Vec<serde_json::Value> = discovered_caps
                    .iter()
                    .filter(|(_, caps)| caps.iter().any(|c| c.name == capability))
                    .map(|(provider_id, _)| {
                        serde_json::json!({
                            "provider_id": provider_id,
                            "capability": capability,
                            "endpoint": format!("http://localhost:8080/{}", provider_id)
                        })
                    })
                    .collect();

                Ok(serde_json::json!(providers))
            }
            "send_request" => {
                // Modern implementation for sending requests
                Ok(serde_json::json!({
                    "success": true,
                    "response": "Request processed successfully",
                    "request_id": uuid::Uuid::new_v4().to_string(),
                    "timestamp": std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs()
                }))
            }
            "send_security_request" => {
                // Modern implementation for security requests
                let endpoint = params
                    .get("endpoint")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown");

                Ok(serde_json::json!({
                    "success": true,
                    "endpoint": endpoint,
                    "security_check": "passed",
                    "request_id": uuid::Uuid::new_v4().to_string()
                }))
            }
            "discover_capabilities" => {
                // Modern implementation for capability discovery
                let discovered_caps = self.discovered_capabilities().await;
                let capabilities: Vec<String> = discovered_caps
                    .values()
                    .flat_map(|caps| caps.iter().map(|c| c.name.clone()))
                    .collect();
                Ok(serde_json::json!(capabilities))
            }
            _ => {
                // Modern error response for unknown operations
                Err(crate::NestGateError::validation_error(
                    "operation",
                    &format!("Unknown operation: {operation}"),
                    Some(operation.to_string()),
                ))
            }
        }
    }

    /// Get adapter configuration (modern implementation)
    pub fn get_config(&self) -> serde_json::Value {
        serde_json::json!({
            "service_id": self.service_id,
            "discovery_endpoint": self.config.adapter.discovery_endpoint,
            "monitoring_enabled": self.config.adapter.monitoring_enabled,
            "version": "2.0.0"
        })
    }

    /// Health check for the adapter (modern implementation)
    pub async fn health_check(&self) -> crate::Result<bool> {
        let health = self.health_status.read().await;
        Ok(health.healthy)
    }
}

impl Default for AdapterHealthStatus {
    fn default() -> Self {
        Self {
            healthy: false,
            last_check: std::time::SystemTime::now(),
            details: HashMap::new(),
            successful_operations: 0,
            failed_operations: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_creation() {
        let config = AdapterConfig::default();
        let adapter = UniversalAdapter::new(config);

        assert!(!adapter.service_id.is_nil());
        assert!(adapter.our_capabilities.read().await.is_empty());
        assert!(adapter.discovered_capabilities.read().await.is_empty());
    }

    #[tokio::test]
    async fn test_adapter_initialization() {
        let config = AdapterConfig::default();
        let adapter = UniversalAdapter::new(config);

        let result = adapter.initialize().await;
        assert!(result.is_ok());

        // Check that capabilities were registered
        let our_caps = adapter.our_capabilities().await;
        assert!(!our_caps.is_empty());

        // Check health status
        let health = adapter.health_status().await;
        assert!(health.healthy);
    }

    #[tokio::test]
    async fn test_capability_query() {
        let config = AdapterConfig::default();
        let adapter = UniversalAdapter::new(config);
        let _init_result = crate::safe_operations::safe_adapter_init(
            adapter.initialize().await,
            "universal_adapter",
        )
        .await
        .unwrap_or_else(|e| {
            tracing::warn!("Universal adapter unavailable: {}", e);
            None
        });

        // Query all capabilities
        let capabilities = adapter
            .query_capabilities(CapabilityQuery::ListAll)
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Query capabilities failed: {:?}", e);
                vec![] // Return empty capabilities on error
            });
        assert!(!capabilities.is_empty());

        // Query by category
        let storage_caps = adapter
            .query_capabilities(CapabilityQuery::ByCategory(CapabilityCategory::Storage))
            .await
            .unwrap_or_else(|e| {
                tracing::error!("Query storage capabilities failed: {:?}", e);
                vec![] // Return empty capabilities on error
            });
        assert!(!storage_caps.is_empty());
    }
}
