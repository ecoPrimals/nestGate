// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Testing utilities for discovery mechanism
//!
//! Provides mock implementations for testing without actual infrastructure.

use super::{Capability, DiscoveryMechanism, Result, ServiceInfo};
use crate::self_knowledge::SelfKnowledge;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mock discovery mechanism for testing
///
/// This allows tests to simulate service discovery without requiring
/// actual mDNS, Consul, or Kubernetes infrastructure.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_discovery::discovery_mechanism::testing::MockDiscovery;
/// use nestgate_discovery::discovery_mechanism::{DiscoveryMechanism, ServiceInfo};
/// use nestgate_core::self_knowledge::SelfKnowledge;
///
/// # async fn example() -> anyhow::Result<()> {
/// let mut mock = MockDiscovery::new();
///
/// // Simulate a discovered service
/// let orchestrator = ServiceInfo {
///     id: "orch-1".to_string(),
///     name: "Orchestrator".to_string(),
///     capabilities: vec!["orchestration".to_string()],
///     endpoint: "localhost:8080".to_string(),
///     metadata: Default::default(),
///     health_endpoint: None,
/// };
/// mock.add_service(orchestrator);
///
/// // Now queries will find it
/// let found = mock.find_by_capability("orchestration".to_string()).await?;
/// assert_eq!(found.len(), 1);
/// # Ok(())
/// # }
/// ```
pub struct MockDiscovery {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    announced: Arc<RwLock<Option<ServiceInfo>>>,
}

impl MockDiscovery {
    /// Create a new mock discovery
    #[must_use]
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            announced: Arc::new(RwLock::new(None)),
        }
    }

    /// Add a service to the mock registry
    ///
    /// This simulates another primal being discovered.
    pub fn add_service(&mut self, service: ServiceInfo) {
        let services = Arc::clone(&self.services);
        tokio::spawn(async move {
            let mut registry = services.write().await;
            registry.insert(service.id.clone(), service);
        });
    }

    /// Remove a service from the mock registry
    pub fn remove_service(&mut self, service_id: &str) {
        let service_id = service_id.to_string();
        let services = Arc::clone(&self.services);
        tokio::spawn(async move {
            let mut registry = services.write().await;
            registry.remove(&service_id);
        });
    }

    /// Get the service that was announced
    pub async fn get_announced(&self) -> Option<ServiceInfo> {
        self.announced.read().await.clone()
    }

    /// Clear all services
    pub async fn clear(&self) {
        self.services.write().await.clear();
    }
}

impl Default for MockDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryMechanism for MockDiscovery {
    fn announce(
        &self,
        self_knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let announced = Arc::clone(&self.announced);
        let sk = self_knowledge.clone();
        Box::pin(async move {
            let service_info = ServiceInfo {
                id: sk.id.as_str().to_string(),
                name: sk.name.clone(),
                capabilities: sk.capabilities.clone(),
                endpoint: sk
                    .endpoints
                    .get("api")
                    .map_or_else(|| "unknown".to_string(), std::string::ToString::to_string),
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("version".to_string(), sk.version.clone());
                    meta
                },
                health_endpoint: sk
                    .endpoints
                    .get("health")
                    .map(std::string::ToString::to_string),
            };

            let mut announced = announced.write().await;
            *announced = Some(service_info);

            Ok(())
        })
    }

    fn find_by_capability(
        &self,
        capability: Capability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ServiceInfo>>> + Send + '_>> {
        let services = Arc::clone(&self.services);
        Box::pin(async move {
            let services = services.read().await;
            let matching: Vec<ServiceInfo> = services
                .values()
                .filter(|s| s.capabilities.contains(&capability))
                .cloned()
                .collect();
            Ok(matching)
        })
    }

    fn find_by_id(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ServiceInfo>>> + Send + '_>> {
        let services = Arc::clone(&self.services);
        let id = id.to_string();
        Box::pin(async move {
            let services = services.read().await;
            Ok(services.get(&id).cloned())
        })
    }

    fn health_check(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        let services = Arc::clone(&self.services);
        let service_id = service_id.to_string();
        Box::pin(async move {
            let services = services.read().await;
            Ok(services.contains_key(&service_id))
        })
    }

    fn deregister(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let services = Arc::clone(&self.services);
        let service_id = service_id.to_string();
        Box::pin(async move {
            let mut services = services.write().await;
            services.remove(&service_id);
            Ok(())
        })
    }

    fn mechanism_name(&self) -> &'static str {
        "mock"
    }
}

/// Builder for creating test services
pub struct TestServiceBuilder {
    id: String,
    name: String,
    capabilities: Vec<String>,
    endpoint: String,
    metadata: HashMap<String, String>,
}

impl TestServiceBuilder {
    /// Create a new test service builder
    pub fn new(id: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            name: id.clone(),
            id,
            capabilities: Vec::new(),
            endpoint: "localhost:8080".to_string(),
            metadata: HashMap::new(),
        }
    }

    /// Set the service name
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Add a capability
    #[must_use]
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }

    /// Add multiple capabilities
    #[must_use]
    pub fn capabilities(mut self, caps: Vec<String>) -> Self {
        self.capabilities.extend(caps);
        self
    }

    /// Set the endpoint
    #[must_use]
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = endpoint.into();
        self
    }

    /// Add metadata
    #[must_use]
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Build the service info
    #[must_use]
    pub fn build(self) -> ServiceInfo {
        ServiceInfo {
            id: self.id,
            name: self.name,
            capabilities: self.capabilities,
            endpoint: self.endpoint,
            metadata: self.metadata,
            health_endpoint: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_discovery_announce() {
        let mock = MockDiscovery::new();
        let self_knowledge = SelfKnowledge::builder()
            .with_id("test-service")
            .with_name("Test Service")
            .with_version("1.0.0")
            .with_capability("storage")
            .with_endpoint("api", "0.0.0.0:8080".parse().unwrap())
            .build()
            .unwrap();

        mock.announce(&self_knowledge).await.unwrap();

        let announced = mock.get_announced().await;
        assert!(announced.is_some());
        let announced = announced.unwrap();
        assert_eq!(announced.id, "test-service");
        assert_eq!(announced.name, "Test Service");
    }

    #[tokio::test]
    async fn test_mock_discovery_find_by_capability() {
        let mut mock = MockDiscovery::new();

        // Add test services
        let service1 = TestServiceBuilder::new("service-1")
            .name("Storage Service")
            .capability("storage")
            .capability("zfs")
            .build();

        let service2 = TestServiceBuilder::new("service-2")
            .name("Orchestrator")
            .capability("orchestration")
            .build();

        mock.add_service(service1);
        mock.add_service(service2);

        // Give time for async adds to complete
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Find by capability
        let storage_services = mock
            .find_by_capability("storage".to_string())
            .await
            .unwrap();
        assert_eq!(storage_services.len(), 1);
        assert_eq!(storage_services[0].name, "Storage Service");

        let orch_services = mock
            .find_by_capability("orchestration".to_string())
            .await
            .unwrap();
        assert_eq!(orch_services.len(), 1);
        assert_eq!(orch_services[0].name, "Orchestrator");
    }

    #[tokio::test]
    async fn test_mock_discovery_health_check() {
        let mut mock = MockDiscovery::new();

        let service = TestServiceBuilder::new("test-service")
            .capability("storage")
            .build();

        mock.add_service(service);
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Service should be healthy
        let healthy = mock.health_check("test-service").await.unwrap();
        assert!(healthy);

        // Non-existent service should not be healthy
        let healthy = mock.health_check("nonexistent").await.unwrap();
        assert!(!healthy);
    }

    #[tokio::test]
    async fn test_test_service_builder() {
        let service = TestServiceBuilder::new("my-service")
            .name("My Service")
            .capability("storage")
            .capability("compute")
            .endpoint("192.168.1.100:8080")
            .metadata("version", "2.0.0")
            .metadata("region", "us-west")
            .build();

        assert_eq!(service.id, "my-service");
        assert_eq!(service.name, "My Service");
        assert_eq!(service.capabilities.len(), 2);
        assert!(service.capabilities.contains(&"storage".to_string()));
        assert!(service.capabilities.contains(&"compute".to_string()));
        assert_eq!(service.endpoint, "192.168.1.100:8080");
        assert_eq!(service.metadata.get("version").unwrap(), "2.0.0");
        assert_eq!(service.metadata.get("region").unwrap(), "us-west");
    }
}
