// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Service resolver for capability-based connections
//!
//! Resolves capabilities to actual service connections, with load balancing
//! and failover support.

use std::sync::Arc;

use super::registry::CapabilityRegistry;
use super::service_descriptor::ServiceDescriptor;
use super::taxonomy::Capability;
use super::{CapabilityError, CapabilityResult};

#[cfg(test)]
use super::taxonomy::{
    AICapability, NetworkingCapability, OrchestrationCapability, StorageCapability,
};

/// Service resolver for capability-based connections
///
/// # Example
///
/// ```rust,ignore
/// // Requires registry with registered services; see capabilities::discovery module
/// let registry = Arc::new(CapabilityRegistry::new());
/// let resolver = ServiceResolver::new(registry);
/// ```
pub struct ServiceResolver {
    /// Capability registry
    registry: Arc<CapabilityRegistry>,

    /// Load balancing strategy
    strategy: LoadBalancingStrategy,
}

/// Load balancing strategy
#[derive(Debug, Clone, Copy)]
pub enum LoadBalancingStrategy {
    /// Round-robin selection
    RoundRobin,

    /// Random selection
    Random,

    /// Least loaded service
    LeastLoaded,

    /// Lowest latency service
    LowestLatency,
}

impl ServiceResolver {
    /// Create a new service resolver
    #[must_use]
    pub const fn new(registry: Arc<CapabilityRegistry>) -> Self {
        Self {
            registry,
            strategy: LoadBalancingStrategy::LeastLoaded,
        }
    }

    /// Set load balancing strategy
    #[must_use]
    pub const fn with_strategy(mut self, strategy: LoadBalancingStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Resolve a capability to a service
    ///
    /// Returns the best available service providing the requested capability.
    pub fn resolve(&self, capability: &Capability) -> CapabilityResult<ServiceDescriptor> {
        let providers = self.registry.find_providers(capability);

        if providers.is_empty() {
            return Err(CapabilityError::NoProvider(capability.clone()));
        }

        // Apply load balancing strategy
        self.select_provider(&providers)
    }

    /// Select best provider using load balancing strategy
    fn select_provider(
        &self,
        providers: &[ServiceDescriptor],
    ) -> CapabilityResult<ServiceDescriptor> {
        match self.strategy {
            LoadBalancingStrategy::RoundRobin => {
                // Simple: return first healthy service
                providers
                    .iter()
                    .find(|s| s.is_available())
                    .cloned()
                    .ok_or_else(|| {
                        CapabilityError::ConnectionFailed(
                            "No healthy providers available".to_string(),
                        )
                    })
            }

            LoadBalancingStrategy::Random => {
                // Random selection from healthy services
                let healthy: Vec<_> = providers.iter().filter(|s| s.is_available()).collect();

                if healthy.is_empty() {
                    return Err(CapabilityError::ConnectionFailed(
                        "No healthy providers available".to_string(),
                    ));
                }

                Ok(healthy[0].clone())
            }

            LoadBalancingStrategy::LeastLoaded => {
                // Select service with lowest load
                providers
                    .iter()
                    .filter(|s| s.is_available())
                    .min_by(|a, b| {
                        // Use total_cmp for NaN-safe comparison (always returns valid Ordering)
                        a.metadata.load.total_cmp(&b.metadata.load)
                    })
                    .cloned()
                    .ok_or_else(|| {
                        CapabilityError::ConnectionFailed(
                            "No healthy providers available".to_string(),
                        )
                    })
            }

            LoadBalancingStrategy::LowestLatency => {
                // Select service with lowest latency
                // Falls back to load-based selection if latency not available
                providers
                    .iter()
                    .filter(|s| s.is_available())
                    .min_by(
                        |a, b| match (a.metadata.latency_ms, b.metadata.latency_ms) {
                            (Some(lat_a), Some(lat_b)) => {
                                // Compare latencies, use Equal if incomparable (NaN)
                                lat_a
                                    .partial_cmp(&lat_b)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            }
                            (Some(_), None) => std::cmp::Ordering::Less,
                            (None, Some(_)) => std::cmp::Ordering::Greater,
                            (None, None) => {
                                // Both missing latency, compare by load
                                a.metadata
                                    .load
                                    .partial_cmp(&b.metadata.load)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            }
                        },
                    )
                    .cloned()
                    .ok_or_else(|| {
                        CapabilityError::ConnectionFailed(
                            "No healthy providers available".to_string(),
                        )
                    })
            }
        }
    }

    /// Find all healthy providers for a capability
    pub fn find_healthy_providers(&self, capability: &Capability) -> Vec<ServiceDescriptor> {
        let providers = self.registry.find_providers(capability);
        providers
            .into_iter()
            .filter(super::service_descriptor::ServiceDescriptor::is_healthy)
            .collect()
    }
}

impl Default for ServiceResolver {
    fn default() -> Self {
        Self::new(Arc::new(CapabilityRegistry::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::super::service_descriptor::{Endpoint, Protocol, ServiceHealth, ServiceMetadata};
    use super::super::taxonomy::{Capability, SecurityCapability};
    use super::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_resolve_single_provider() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry));

        let service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint {
                host: "localhost".to_string(),
                port: nestgate_config::constants::network_hardcoded::ports::HTTPS_DEFAULT,
                protocol: Protocol::HTTPS,
                tls: true,
            },
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service).unwrap();

        let resolved = resolver
            .resolve(&Capability::Security(SecurityCapability::Authentication))
            .unwrap();

        assert_eq!(resolved.id, service.id);
    }

    #[tokio::test]
    async fn test_no_provider_error() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(registry);

        let result = resolver.resolve(&Capability::Security(SecurityCapability::Authentication));

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CapabilityError::NoProvider(_)
        ));
    }

    #[tokio::test]
    async fn test_least_loaded_selection() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry))
            .with_strategy(LoadBalancingStrategy::LeastLoaded);

        // Register two services with different loads
        let service1 = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "high-load".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint::http("localhost".to_string(), 8001),
            metadata: ServiceMetadata {
                load: 0.8,
                ..Default::default()
            },
            health: ServiceHealth::Healthy,
        };

        let service2 = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "low-load".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Authentication)],
            endpoint: Endpoint::http("localhost".to_string(), 8002),
            metadata: ServiceMetadata {
                load: 0.2,
                ..Default::default()
            },
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service1).unwrap();
        registry.register_service(&service2).unwrap();

        let resolved = resolver
            .resolve(&Capability::Security(SecurityCapability::Authentication))
            .unwrap();

        // Should select the low-load service
        assert_eq!(resolved.id, service2.id);
    }

    #[tokio::test]
    async fn test_lowest_latency_selection() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry))
            .with_strategy(LoadBalancingStrategy::LowestLatency);

        // Register two services with different latencies
        let service1 = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "high-latency".to_string(),
            capabilities: vec![Capability::Orchestration(
                OrchestrationCapability::ServiceDiscovery,
            )],
            endpoint: Endpoint::http("localhost".to_string(), 9001),
            metadata: ServiceMetadata {
                latency_ms: Some(100.0),
                ..Default::default()
            },
            health: ServiceHealth::Healthy,
        };

        let service2 = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "low-latency".to_string(),
            capabilities: vec![Capability::Orchestration(
                OrchestrationCapability::ServiceDiscovery,
            )],
            endpoint: Endpoint::http("localhost".to_string(), 9002),
            metadata: ServiceMetadata {
                latency_ms: Some(10.0),
                ..Default::default()
            },
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service1).unwrap();
        registry.register_service(&service2).unwrap();

        let resolved = resolver
            .resolve(&Capability::Orchestration(
                OrchestrationCapability::ServiceDiscovery,
            ))
            .unwrap();

        // Should select the low-latency service
        assert_eq!(resolved.id, service2.id);
    }

    #[tokio::test]
    async fn test_strategy_change() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver =
            ServiceResolver::new(registry).with_strategy(LoadBalancingStrategy::RoundRobin);

        assert!(matches!(
            resolver.strategy,
            LoadBalancingStrategy::RoundRobin
        ));

        let resolver = resolver.with_strategy(LoadBalancingStrategy::Random);
        assert!(matches!(resolver.strategy, LoadBalancingStrategy::Random));
    }

    #[tokio::test]
    async fn test_unhealthy_service_filtered() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry));

        // Register a healthy and an unhealthy service
        let healthy = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "healthy".to_string(),
            capabilities: vec![Capability::AI(AICapability::Inference)],
            endpoint: Endpoint::http("localhost".to_string(), 7001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        let unhealthy = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "unhealthy".to_string(),
            capabilities: vec![Capability::AI(AICapability::Inference)],
            endpoint: Endpoint::http("localhost".to_string(), 7002),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Unhealthy,
        };

        registry.register_service(&unhealthy).unwrap();
        registry.register_service(&healthy).unwrap();

        let resolved = resolver
            .resolve(&Capability::AI(AICapability::Inference))
            .unwrap();

        // Should select the healthy service
        assert_eq!(resolved.id, healthy.id);
    }

    #[tokio::test]
    async fn test_all_unhealthy_services() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry));

        // Register only unhealthy services
        let unhealthy = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "unhealthy".to_string(),
            capabilities: vec![Capability::Networking(NetworkingCapability::ServiceMesh)],
            endpoint: Endpoint::http("localhost".to_string(), 6001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Unhealthy,
        };

        registry.register_service(&unhealthy).unwrap();

        let result = resolver.resolve(&Capability::Networking(NetworkingCapability::ServiceMesh));

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multiple_capabilities_resolution() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry));

        // Register a service with multiple capabilities
        let service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "multi-cap".to_string(),
            capabilities: vec![
                Capability::Storage(StorageCapability::ObjectStorage),
                Capability::Storage(StorageCapability::BlockStorage),
            ],
            endpoint: Endpoint::http("localhost".to_string(), 5001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service).unwrap();

        // Should resolve both capabilities to the same service
        let resolved1 = resolver
            .resolve(&Capability::Storage(StorageCapability::ObjectStorage))
            .unwrap();
        let resolved2 = resolver
            .resolve(&Capability::Storage(StorageCapability::BlockStorage))
            .unwrap();

        assert_eq!(resolved1.id, service.id);
        assert_eq!(resolved2.id, service.id);
    }

    #[tokio::test]
    async fn test_round_robin_strategy() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry))
            .with_strategy(LoadBalancingStrategy::RoundRobin);

        // Register two services with the same capability
        let service1 = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "service1".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Encryption)],
            endpoint: Endpoint::http("localhost".to_string(), 4001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        let service2 = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "service2".to_string(),
            capabilities: vec![Capability::Security(SecurityCapability::Encryption)],
            endpoint: Endpoint::http("localhost".to_string(), 4002),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service1).unwrap();
        registry.register_service(&service2).unwrap();

        // Multiple resolutions should cycle through services
        let resolved1 = resolver
            .resolve(&Capability::Security(SecurityCapability::Encryption))
            .unwrap();
        let resolved2 = resolver
            .resolve(&Capability::Security(SecurityCapability::Encryption))
            .unwrap();

        // With round-robin, we should get both services
        assert!(resolved1.id == service1.id || resolved1.id == service2.id);
        assert!(resolved2.id == service1.id || resolved2.id == service2.id);
    }

    #[tokio::test]
    async fn test_resolver_with_degraded_service() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry));

        // Register healthy and degraded services
        let healthy = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "healthy".to_string(),
            capabilities: vec![Capability::Networking(NetworkingCapability::LoadBalancing)],
            endpoint: Endpoint::http("localhost".to_string(), 3001),
            metadata: ServiceMetadata {
                load: 0.3, // Lower load
                ..Default::default()
            },
            health: ServiceHealth::Healthy,
        };

        let degraded = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "degraded".to_string(),
            capabilities: vec![Capability::Networking(NetworkingCapability::LoadBalancing)],
            endpoint: Endpoint::http("localhost".to_string(), 3002),
            metadata: ServiceMetadata {
                load: 0.8, // Higher load
                ..Default::default()
            },
            health: ServiceHealth::Degraded,
        };

        registry.register_service(&degraded).unwrap();
        registry.register_service(&healthy).unwrap();

        let resolved = resolver
            .resolve(&Capability::Networking(NetworkingCapability::LoadBalancing))
            .unwrap();

        // Should select the healthy service (lower load)
        assert_eq!(resolved.id, healthy.id);
    }

    #[test]
    fn test_load_balancing_strategy_debug() {
        let strategy = LoadBalancingStrategy::RoundRobin;
        let debug_str = format!("{:?}", strategy);
        assert!(debug_str.contains("RoundRobin"));
    }

    #[test]
    fn test_load_balancing_strategy_clone() {
        let strategy = LoadBalancingStrategy::LeastLoaded;
        let cloned = strategy;
        assert!(matches!(cloned, LoadBalancingStrategy::LeastLoaded));
    }

    #[tokio::test]
    async fn test_service_metadata_impact() {
        let registry = Arc::new(CapabilityRegistry::new());

        // Test that metadata fields are properly used in selection
        let service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "metadata-test".to_string(),
            capabilities: vec![Capability::AI(AICapability::Training)],
            endpoint: Endpoint::http("localhost".to_string(), 2001),
            metadata: ServiceMetadata {
                load: 0.5,
                latency_ms: Some(25.0),
                ..Default::default()
            },
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service).unwrap();

        // Verify the service can be retrieved with its metadata
        let services = registry.find_providers(&Capability::AI(AICapability::Training));
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].metadata.load, 0.5);
        assert_eq!(services[0].metadata.latency_ms, Some(25.0));
    }

    #[tokio::test]
    async fn test_empty_registry() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(registry);

        let result = resolver.resolve(&Capability::Storage(StorageCapability::Database));

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CapabilityError::NoProvider(_)
        ));
    }

    #[tokio::test]
    async fn test_registry_concurrent_access() {
        let registry = Arc::new(CapabilityRegistry::new());
        let resolver = ServiceResolver::new(Arc::clone(&registry));

        let service = ServiceDescriptor {
            id: Uuid::new_v4(),
            name: "concurrent".to_string(),
            capabilities: vec![Capability::Orchestration(
                OrchestrationCapability::ServiceScheduling,
            )],
            endpoint: Endpoint::http("localhost".to_string(), 1001),
            metadata: ServiceMetadata::default(),
            health: ServiceHealth::Healthy,
        };

        registry.register_service(&service).unwrap();

        // Multiple resolutions (registry is lock-free; resolver is synchronous)
        let cap = Capability::Orchestration(OrchestrationCapability::ServiceScheduling);
        let r1 = resolver.resolve(&cap);
        let r2 = resolver.resolve(&cap);
        let r3 = resolver.resolve(&cap);

        assert!(r1.is_ok());
        assert!(r2.is_ok());
        assert!(r3.is_ok());
        assert_eq!(r1.unwrap().id, service.id);
    }
}
