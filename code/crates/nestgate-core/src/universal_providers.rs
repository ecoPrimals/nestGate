//! Universal Provider Wrappers
//!
//! This module provides wrapper implementations that adapt existing hardcoded
//! primal integrations to the universal provider interface, enabling gradual
//! migration to the universal architecture.


use std::sync::Arc;
use async_trait::async_trait;
use std::collections::HashMap;
use uuid::Uuid;


use crate::universal_traits::*;
use anyhow::Result;
use crate::{Result, NestGateError};

/// Universal Security Provider Wrapper
/// Provides a universal interface for any security provider (BearDog, custom, enterprise)
#[derive(Debug)]
#[allow(dead_code)]
pub struct UniversalSecurityWrapper {
    provider_name: String,
    endpoint: String,
    capabilities: Vec<String>,
    client: Option<Arc<dyn SecurityPrimalProvider>>,
}

/// Trait for any security client (BearDog, Vault, etc.)
#[async_trait]
pub trait SecurityClient: Send + Sync {
    async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken>;
    async fn encrypt(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>>;
    async fn decrypt(&self, encrypted: &[u8], algorithm: &str) -> Result<Vec<u8>>;
    async fn sign_data(&self, data: &[u8]) -> Result<Signature>;
    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool>;
    async fn health_check(&self) -> Result<bool>;
}

impl UniversalSecurityWrapper {
    /// Create a new universal security wrapper
    pub fn new(provider_name: String, endpoint: String, capabilities: Vec<String>) -> Self {
        Self {
            provider_name,
            endpoint,
            capabilities,
            client: None,
        }
    }

    /// Set the underlying security client (BearDog, etc.)
    pub fn with_client(mut self, client: Arc<dyn SecurityClient>) -> Self {
        self.client = Some(client);
        self
    }

    /// Auto-detect security provider type from endpoint
    pub fn auto_detect_provider_type(endpoint: &str) -> String {
        if endpoint.contains("beardog") || endpoint.contains("8443") {
            "beardog".to_string()
        } else if endpoint.contains("vault") {
            "vault".to_string()
        } else if endpoint.contains("keycloak") {
            "keycloak".to_string()
        } else {
            "generic".to_string()
        }
    }
}

#[async_trait]
impl SecurityPrimalProvider for UniversalSecurityWrapper {
    async fn authenticate(&self, domain: &str, credentials: &str) -> Result<AuthResult> {
        if let Some(client) = &self.client {
            client.authenticate(domain, credentials).await
        } else {
            // Mock authentication for testing
            Ok(AuthResult {
                authenticated: true,
                user_id: "mock_user".to_string(),
                token: "mock_token".to_string(),
                expires_at: std::time::SystemTime::now() + std::time::Duration::from_secs(3600),
                permissions: vec!["read".to_string(), "write".to_string()],
            })
        }
    }

    async fn validate_token(&self, token: &str) -> Result<bool> {
        if let Some(client) = &self.client {
            client.validate_token(token).await
        } else {
            // Mock validation
            Ok(!token.is_empty())
        }
    }

    async fn sign_data(&self, data: &[u8]) -> Result<Signature> {
        if let Some(client) = &self.client {
            client.sign_data(data).await
        } else {
            // Mock signing
            Ok(Signature {
                algorithm: "mock".to_string(),
                signature: data.to_vec(),
                key_id: "mock_key".to_string(),
            })
        }
    }

    async fn verify_signature(&self, data: &[u8], signature: &Signature) -> Result<bool> {
        if let Some(client) = &self.client {
            client.verify_signature(data, signature).await
        } else {
            // Mock verification
            Ok(signature.signature == data)
        }
    }

    async fn encrypt_data(&self, data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        if let Some(client) = &self.client {
            client.encrypt_data(data, algorithm).await
        } else {
            // Mock encryption (XOR with key for demo)
            let key = b"mock_key_1234567";
            let encrypted: Vec<u8> = data.iter().zip(key.iter().cycle())
                .map(|(data_byte, key_byte)| data_byte ^ key_byte)
                .collect();
            Ok(encrypted)
        }
    }

    async fn decrypt_data(&self, encrypted_data: &[u8], algorithm: &str) -> Result<Vec<u8>> {
        // For XOR encryption, decryption is the same operation
        self.encrypt_data(encrypted_data, algorithm).await
    }

    async fn get_credentials(&self, domain: &str) -> Result<Credentials> {
        if let Some(client) = &self.client {
            client.get_credentials(domain).await
        } else {
            // Return mock credentials
            Ok(Credentials {
                domain: domain.to_string(),
                token: "mock_token_for_domain".to_string(),
            })
        }
    }

    async fn get_key_id(&self) -> Result<String> {
        // Default implementation - could be customized per provider type
        Ok(format!("{}-key", self.provider_name))
    }

    async fn generate_validation_token(&self, data: &[u8]) -> Result<String> {
        // Generate a simple validation token - could delegate to underlying client
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(self.provider_name.as_bytes());
        Ok(format!("{:x}", hasher.finalize()))
    }

    async fn evaluate_boundary_access(
        &self,
        source: &str,
        destination: &str,
        operation: &str,
    ) -> Result<SecurityDecision> {
        // Default policy - could be customized per provider
        if source == destination {
            Ok(SecurityDecision::Allow)
        } else if operation == "read" {
            Ok(SecurityDecision::Allow)
        } else {
            Ok(SecurityDecision::RequireAuth)
        }
    }
}

/// Universal Orchestration Provider Wrapper  
/// Provides a universal interface for any orchestration provider (Songbird, Kubernetes, etc.)
#[derive(Debug)]
#[allow(dead_code)]
pub struct UniversalOrchestrationWrapper {
    provider_name: String,
    endpoint: String,
    capabilities: Vec<String>,
    client: Option<Arc<dyn OrchestrationPrimalProvider>>,
}

/// Trait for any orchestration client (Songbird, Kubernetes, etc.)
#[async_trait]
pub trait OrchestrationClient: Send + Sync {
    async fn register_service(&self, service: &ServiceRegistration) -> Result<String>;
    async fn discover_services(&self, service_type: &str) -> Result<Vec<ServiceInstance>>;
    async fn allocate_port(&self, service: &str, port_type: &str) -> Result<u16>;
    async fn release_port(&self, service: &str, port: u16) -> Result<()>;
    async fn health_check(&self) -> Result<bool>;
}

impl UniversalOrchestrationWrapper {
    pub fn new(provider_name: String, endpoint: String, capabilities: Vec<String>) -> Self {
        Self {
            provider_name,
            endpoint,
            capabilities,
            client: None,
        }
    }

    pub fn with_client(mut self, client: Arc<dyn OrchestrationClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn auto_detect_provider_type(endpoint: &str) -> String {
        if endpoint.contains("songbird") || endpoint.contains("8000") {
            "songbird".to_string()
        } else if endpoint.contains("kubernetes") || endpoint.contains("6443") {
            "kubernetes".to_string()
        } else if endpoint.contains("consul") || endpoint.contains("8500") {
            "consul".to_string()
        } else {
            "generic".to_string()
        }
    }
}

#[async_trait]
impl OrchestrationPrimalProvider for UniversalOrchestrationWrapper {
    async fn register_service(&self, service: &ServiceRegistration) -> Result<String> {
        if let Some(client) = &self.client {
            client.register_service(service).await
        } else {
            // Return a mock service ID
            Ok(format!("{}-{}", self.provider_name, uuid::Uuid::new_v4()))
        }
    }

    async fn discover_services(&self, service_type: &str) -> Result<Vec<ServiceInstance>> {
        if let Some(client) = &self.client {
            client.discover_services(service_type).await
        } else {
            // Return empty list as fallback
            Ok(vec![])
        }
    }

    async fn allocate_port(&self, service: &str, port_type: &str) -> Result<u16> {
        if let Some(client) = &self.client {
            client.allocate_port(service, port_type).await
        } else {
            // Return a random high port as fallback
            Ok(8080 + (fastrand::u16(..) % 1000))
        }
    }

    async fn release_port(&self, service: &str, port: u16) -> Result<()> {
        if let Some(client) = &self.client {
            client.release_port(service, port).await
        } else {
            // No-op fallback
            Ok(())
        }
    }

    async fn route_request(&self, request: &InterPrimalRequest) -> Result<InterPrimalResponse> {
        // Default routing implementation
        Ok(InterPrimalResponse {
            request_id: request.id,
            success: true,
            payload: serde_json::Value::Object(serde_json::Map::new()),
            error: None,
        })
    }

    async fn get_service_health(&self, service: &str) -> Result<ServiceHealth> {
        // Default health check
        tracing::debug!("Getting health for service: {}", service);
        Ok(ServiceHealth::Healthy)
    }

    async fn load_balance(
        &self,
        service: &str,
        request: &ServiceRequest,
    ) -> Result<ServiceResponse> {
        // Default load balancing (pass-through)
        Ok(ServiceResponse {
            request_id: request.id,
            success: true,
            payload: serde_json::Value::Object(serde_json::Map::new()),
        })
    }
}

/// Universal Compute Provider Wrapper
/// Provides a universal interface for any compute provider (ToadStool, Docker, etc.)
#[derive(Debug)]
#[allow(dead_code)]
pub struct UniversalComputeWrapper {
    provider_name: String,
    endpoint: String,
    capabilities: Vec<String>,
    client: Option<Arc<dyn ComputePrimalProvider>>,
}

/// Trait for any compute client (ToadStool, Docker, etc.)
#[async_trait]
pub trait ComputeClient: Send + Sync {
    async fn allocate_resources(&self, spec: &ResourceSpec) -> Result<ResourceAllocation>;
    async fn execute_workload(&self, workload: &WorkloadSpec) -> Result<WorkloadResult>;
    async fn monitor_performance(&self, allocation: &ResourceAllocation) -> Result<PerformanceMetrics>;
    async fn health_check(&self) -> Result<bool>;
}

impl UniversalComputeWrapper {
    pub fn new(provider_name: String, endpoint: String, capabilities: Vec<String>) -> Self {
        Self {
            provider_name,
            endpoint,
            capabilities,
            client: None,
        }
    }

    pub fn with_client(mut self, client: Arc<dyn ComputeClient>) -> Self {
        self.client = Some(client);
        self
    }

    pub fn auto_detect_provider_type(endpoint: &str) -> String {
        if endpoint.contains("toadstool") || endpoint.contains("9000") {
            "toadstool".to_string()
        } else if endpoint.contains("docker") {
            "docker".to_string()
        } else if endpoint.contains("kubernetes") {
            "kubernetes".to_string()
        } else {
            "generic".to_string()
        }
    }
}

#[async_trait]
impl ComputePrimalProvider for UniversalComputeWrapper {
    async fn allocate_resources(&self, spec: &ResourceSpec) -> Result<ResourceAllocation> {
        if let Some(client) = &self.client {
            client.allocate_resources(spec).await
        } else {
            // Return mock allocation
            Ok(ResourceAllocation {
                id: uuid::Uuid::new_v4().to_string(),
                allocated_resources: spec.clone(),
                status: "allocated".to_string(),
                created_at: std::time::SystemTime::now(),
            })
        }
    }

    async fn execute_workload(&self, workload: &WorkloadSpec) -> Result<WorkloadResult> {
        if let Some(client) = &self.client {
            client.execute_workload(workload).await
        } else {
            // Return mock result
            Ok(WorkloadResult {
                id: uuid::Uuid::new_v4().to_string(),
                exit_code: 0,
                stdout: "Mock execution successful".to_string(),
                stderr: "".to_string(),
                execution_time: 1000,
            })
        }
    }

    async fn monitor_performance(
        &self,
        allocation: &ResourceAllocation,
    ) -> Result<PerformanceMetrics> {
        if let Some(client) = &self.client {
            client.monitor_performance(allocation).await
        } else {
            // Return mock metrics
            Ok(PerformanceMetrics {
                cpu_usage: 0.5,
                memory_usage: 0.3,
                disk_io: 0.1,
                network_io: 0.2,
                timestamp: std::time::SystemTime::now(),
            })
        }
    }

    async fn scale_resources(
        &self,
        allocation: &ResourceAllocation,
        target: &ScalingTarget,
    ) -> Result<()> {
        // Default scaling implementation (no-op)
        tracing::info!(
            "Scaling resources for allocation {} to target: {:?}",
            allocation.id,
            target
        );
        Ok(())
    }

    async fn get_resource_utilization(&self) -> Result<ResourceUtilization> {
        // Return mock resource utilization
        Ok(ResourceUtilization {
            cpu_percent: 45.0,
            memory_percent: 60.0,
            disk_percent: 25.0,
            network_utilization: 15.0,
        })
    }

    async fn detect_platform(&self) -> Result<PlatformCapabilities> {
        // Return mock platform capabilities
        Ok(PlatformCapabilities {
            architecture: "x86_64".to_string(),
            os_type: "linux".to_string(),
            container_runtime: "docker".to_string(),
            gpu_support: false,
            features: vec!["containers".to_string(), "networking".to_string()],
        })
    }

    async fn optimize_allocation(
        &self,
        current: &ResourceAllocation,
        _metrics: &PerformanceMetrics,
    ) -> Result<OptimizationRecommendation> {
        // Return mock optimization recommendation
        Ok(OptimizationRecommendation {
            recommendations: vec![
                format!("Consider scaling down allocation {}", current.id),
                "Enable CPU throttling for better power efficiency".to_string(),
            ],
            expected_improvement: 0.15,
            confidence: 0.8,
        })
    }
}

/// Factory for creating universal provider wrappers
pub struct UniversalProviderFactory;

impl UniversalProviderFactory {
    /// Create a security provider from discovered primal info
    pub fn create_security_provider(
        provider_info: &crate::universal_adapter::DiscoveredPrimal,
    ) -> Arc<dyn SecurityPrimalProvider> {
        Arc::new(UniversalSecurityWrapper::new(
            provider_info.primal_type.clone(),
            provider_info.endpoint.clone(),
            provider_info.capabilities.clone(),
        ))
    }

    /// Create an orchestration provider from discovered primal info
    pub fn create_orchestration_provider(
        provider_info: &crate::universal_adapter::DiscoveredPrimal,
    ) -> Arc<dyn OrchestrationPrimalProvider> {
        Arc::new(UniversalOrchestrationWrapper::new(
            provider_info.primal_type.clone(),
            provider_info.endpoint.clone(),
            provider_info.capabilities.clone(),
        ))
    }

    /// Create a compute provider from discovered primal info
    pub fn create_compute_provider(
        provider_info: &crate::universal_adapter::DiscoveredPrimal,
    ) -> Arc<dyn ComputePrimalProvider> {
        Arc::new(UniversalComputeWrapper::new(
            provider_info.primal_type.clone(),
            provider_info.endpoint.clone(),
            provider_info.capabilities.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_type_detection() {
        assert_eq!(
            UniversalSecurityWrapper::auto_detect_provider_type("https://beardog.local:8443"),
            "beardog"
        );
        assert_eq!(
            UniversalOrchestrationWrapper::auto_detect_provider_type("http://songbird.local:8000"),
            "songbird"
        );
        assert_eq!(
            UniversalComputeWrapper::auto_detect_provider_type("http://toadstool.local:9000"),
            "toadstool"
        );
    }

    #[tokio::test]
    async fn test_security_wrapper_fallback() {
        let wrapper = UniversalSecurityWrapper::new(
            "test".to_string(),
            "http://test:8443".to_string(),
            vec!["encryption".to_string()],
        );

        // Should return an error when no client is configured
        let credentials = Credentials {
            username: "test".to_string(),
            password: "test".to_string(),
            additional_data: HashMap::new(),
        };

        let result = wrapper.authenticate("test", "test").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_orchestration_wrapper_fallback() {
        let wrapper = UniversalOrchestrationWrapper::new(
            "test".to_string(),
            "http://test:8000".to_string(),
            vec!["service_discovery".to_string()],
        );

        // Should return empty list when no client is configured
        let services = wrapper.discover_services("storage").await.unwrap();
        assert!(services.is_empty());
    }

    #[tokio::test]
    async fn test_compute_wrapper_fallback() {
        let wrapper = UniversalComputeWrapper::new(
            "test".to_string(),
            "http://test:9000".to_string(),
            vec!["resource_allocation".to_string()],
        );

        let spec = ResourceSpec {
            cpu_cores: Some(2.0),
            memory_mb: Some(4096),
            disk_mb: Some(100_000), // 100GB in MB
            gpu_count: Some(0),
            network_bandwidth: Some(1000),
        };

        // Should return mock allocation when no client is configured
        let allocation = wrapper.allocate_resources(&spec).await.unwrap();
        assert_eq!(allocation.status, "allocated");
    }
} 