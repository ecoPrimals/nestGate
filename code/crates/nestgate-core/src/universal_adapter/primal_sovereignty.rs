//! # Primal Sovereignty Universal Adapter
//! Primal Sovereignty functionality and utilities.
//! Implements the core principle: "Each primal only knows itself and discovers 
//! others through the universal adapter"

use crate::error::NestGateError;
use std::collections::HashMap;
use std::time::Duration;
// Removed unused import for pedantic perfection

/// Capability types that can be discovered through the universal adapter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CapabilityType {
    Storage,
    Orchestration,
    Security,
    ArtificialIntelligence,
    Compute,
    Management,
}

/// Discovered capability information
#[derive(Debug, Clone)]
pub struct DiscoveredCapability {
    pub id: String,
    pub capability_type: CapabilityType,
    pub endpoint: String,
    pub provider_type: String,  // Generic, not primal-specific
    pub operations: Vec<String>,
    pub health_status: HealthStatus,
}

/// Health status of a discovered capability
#[derive(Debug, Clone)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Universal adapter for primal sovereignty
pub struct UniversalAdapter {
    discovery_methods: Vec<DiscoveryMethod>,
    capability_cache: HashMap<CapabilityType, DiscoveredCapability>,
    #[allow(dead_code)] // Framework field - intentionally unused
    discovery_timeout: Duration,
}

/// Methods for discovering capabilities
#[derive(Debug, Clone)]
pub enum DiscoveryMethod {
    Environment,
    NetworkScan,
    ServiceRegistry,
    CapabilityBroadcast,
}

impl UniversalAdapter {
    /// Create new universal adapter with default configuration
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn new() -> Result<Self, NestGateError>  {
        Ok(Self {
            discovery_methods: vec![
                DiscoveryMethod::Environment,
                DiscoveryMethod::ServiceRegistry,
                DiscoveryMethod::NetworkScan,
            ],
            capability_cache: HashMap::new(),
            discovery_timeout: Duration::from_secs(5),
        })
    }
    
    /// Discover a capability by type (primal-agnostic)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn discover_capability(&mut self, capability_type: CapabilityType) -> Result<DiscoveredCapability, NestGateError>  {
        // Check cache first
        if let Some(cached) = self.capability_cache.get(&capability_type) {
            if self.is_capability_healthy(cached).await? {
                return Ok(cached.clone());
            }
        }
        
        // Try discovery methods in order
        for method in &self.discovery_methods {
            if let Ok(capability) = self.try_discovery_method(method, &capability_type).await {
                self.capability_cache.insert(capability_type.clone(), capability.clone());
                return Ok(capability);
            }
        }
        
        Err(NestGateError::not_found(&format!("No {capability_type_name(&capability_type} capability found through any discovery method"))))
    }
    
    /// Request a capability operation (provider-agnostic)
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn request_capability(&self, capability_id: &str, request: CapabilityRequest) -> Result<CapabilityResponse, NestGateError>  {
        // Find the capability
        let capability = self.capability_cache.values()
            .find(|c| c.id == capability_id)
            .ok_or_else(|| NestGateError::not_found(&format!("Capability not found: {capability_id}")))?;
        
        // Make the request through the universal adapter
        self.execute_capability_request(capability, request).await
    }
    
    /// Chain multiple capabilities for network effects
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn chain_capabilities(&self, workflow: Vec<CapabilityRequest>) -> Result<Vec<CapabilityResponse>, NestGateError>  {
        let mut responses = Vec::new();
        
        for request in workflow {
            // Discover the required capability
            let mut adapter = UniversalAdapter::new()?;
            let capability = adapter.discover_capability(request.capability_type.clone()).await?;
            
            // Execute the request
            let response = self.execute_capability_request(&capability, request).await?;
            responses.push(response);
        }
        
        Ok(responses)
    }
    
    // Private implementation methods
    async fn try_discovery_method(&self, method: &DiscoveryMethod, capability_type: &CapabilityType) -> Result<DiscoveredCapability, NestGateError> {
        let discovery_result = match method {
            DiscoveryMethod::Environment => self.discover_from_environment(capability_type).await,
            DiscoveryMethod::NetworkScan => self.discover_from_network(capability_type).await,
            DiscoveryMethod::ServiceRegistry => self.discover_from_registry(capability_type).await,
            DiscoveryMethod::CapabilityBroadcast => self.discover_from_broadcast(capability_type).await,
        };
        
        discovery_result
    }
    
    async fn discover_from_environment(&self, capability_type: &CapabilityType) -> Result<DiscoveredCapability, NestGateError> {
        let env_var = match capability_type {
            CapabilityType::Orchestration => "ORCHESTRATION_DISCOVERY_ENDPOINT",
            CapabilityType::Security => "SECURITY_DISCOVERY_ENDPOINT",
            CapabilityType::ArtificialIntelligence => "AI_DISCOVERY_ENDPOINT",
            CapabilityType::Compute => "COMPUTE_DISCOVERY_ENDPOINT",
            CapabilityType::Management => "MANAGEMENT_DISCOVERY_ENDPOINT",
            CapabilityType::Storage => "STORAGE_DISCOVERY_ENDPOINT",
        };
        
        let endpoint = std::env::var(env_var)
            .map_err(|_| NestGateError::not_found(&format!("Environment variable {env_var} not set")))?;
        
        Ok(DiscoveredCapability {
            id: format!("{capability_type_name(capability_type}-env-discovered")),
            capability_type: capability_type.clone(),
            endpoint,
            provider_type: "environment-configured".to_string(),
            operations: vec!["*".to_string()], // All operations supported
            health_status: HealthStatus::Unknown,
        })
    }
    
    async fn discover_from_network(&self, _capability_type: &CapabilityType) -> Result<DiscoveredCapability, NestGateError> {
        // Network scanning implementation
        Err(NestGateError::not_found("Network discovery not yet implemented"))
    }
    
    async fn discover_from_registry(&self, _capability_type: &CapabilityType) -> Result<DiscoveredCapability, NestGateError> {
        // Service registry discovery implementation
        Err(NestGateError::not_found("Registry discovery not yet implemented"))
    }
    
    async fn discover_from_broadcast(&self, _capability_type: &CapabilityType) -> Result<DiscoveredCapability, NestGateError> {
        // Capability broadcast discovery implementation
        Err(NestGateError::not_found("Broadcast discovery not yet implemented"))
    }
    
    async fn is_capability_healthy(&self, _capability: &DiscoveredCapability) -> Result<bool, NestGateError> {
        // Health check implementation
        Ok(true) // Simplified for now
    }
    
    async fn execute_capability_request(&self, _capability: &DiscoveredCapability, _request: CapabilityRequest) -> Result<CapabilityResponse, NestGateError> {
        // Request execution implementation
        Ok(CapabilityResponse {
            status: "success".to_string(),
            data: serde_json::Value::Null,
        })
    }
}

/// Request to a capability
#[derive(Debug, Clone)]
pub struct CapabilityRequest {
    pub capability_type: CapabilityType,
    pub operation: String,
    pub payload: serde_json::Value,
}

/// Response from a capability
#[derive(Debug, Clone)]
pub struct CapabilityResponse {
    pub status: String,
    pub data: serde_json::Value,
}

fn capability_type_name(capability_type: &CapabilityType) -> &'static str {
    match capability_type {
        CapabilityType::Storage => "storage",
        CapabilityType::Orchestration => "orchestration",
        CapabilityType::Security => "security",
        CapabilityType::ArtificialIntelligence => "artificial_intelligence",
        CapabilityType::Compute => "compute",
        CapabilityType::Management => "management",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_universal_adapter_creation() {
        let adapter = UniversalAdapter::new()
            .expect("Failed to create UniversalAdapter for test");
        assert_eq!(adapter.discovery_methods.len(), 3);
    }
    
    #[tokio::test]
    async fn test_capability_discovery_from_environment() {
        std::env::set_var("ORCHESTRATION_DISCOVERY_ENDPOINT", "http://test:8081/capabilities");
        
        let mut adapter = UniversalAdapter::new()
            .expect("Failed to create UniversalAdapter for test");
        let result = adapter.discover_capability(CapabilityType::Orchestration).await;
        
        assert!(result.is_ok());
        let capability = result
            .expect("Failed to discover orchestration capability in test");
        assert_eq!(capability.capability_type, CapabilityType::Orchestration);
        assert!(capability.endpoint.contains("test:8081"));
    }
}