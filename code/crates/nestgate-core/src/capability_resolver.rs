//! Unified Capability Resolver - Bridge for All Registry Systems
//!
//! **ARCHITECTURAL UNIFICATION**: This module creates a common interface for all
//! registry and discovery systems in the codebase, enabling them to work together.
//!
//! **SYSTEMS UNIFIED**:
//! 1. InMemoryServiceRegistry (service_discovery)
//! 2. ServiceRegistry (universal_primal_discovery)
//! 3. CapabilityDiscoveryManager
//! 4. Application layer capabilities
//!
//! **PHILOSOPHY**: One interface to rule them all - capability-based discovery
//! regardless of underlying implementation.

use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};
use crate::{NestGateError, Result};
use std::future::Future;
use std::pin::Pin;

/// Unified service endpoint result from capability resolution
#[derive(Debug, Clone)]
pub struct ResolvedService {
    /// Service identifier
    pub id: String,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Protocol (http, https, grpc, etc.)
    pub protocol: String,
    /// Capabilities this service provides
    pub capabilities: Vec<UnifiedCapability>,
    /// Service health indicator
    pub is_healthy: bool,
}

impl ResolvedService {
    /// Get full URL for this service
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }

    /// Get endpoint without protocol
    pub fn endpoint(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Unified capability resolver trait - implemented by all registry types
///
/// This trait provides a common interface for capability-based service discovery
/// regardless of the underlying registry implementation.
///
/// **Object-Safe**: Uses boxed futures to enable dynamic dispatch
pub trait CapabilityResolver: Send + Sync {
    /// Find a service by unified capability
    ///
    /// # Arguments
    /// * `capability` - The unified capability to search for
    ///
    /// # Returns
    /// The first healthy service that provides this capability
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>>;

    /// Find all services that provide a capability
    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>>;

    /// Check if a capability is available
    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>>;
}

/// Adapter for universal_primal_discovery::ServiceRegistry
///
/// Bridges the universal primal discovery system to the unified interface
pub struct PrimalDiscoveryAdapter<'a> {
    registry: &'a crate::universal_primal_discovery::service_registry::ServiceRegistry,
}

impl<'a> PrimalDiscoveryAdapter<'a> {
    /// Create adapter from primal discovery registry
    pub fn new(
        registry: &'a crate::universal_primal_discovery::service_registry::ServiceRegistry,
    ) -> Self {
        Self { registry }
    }
}

impl<'a> CapabilityResolver for PrimalDiscoveryAdapter<'a> {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let primal_cap = CapabilityMapper::to_primal(&capability);
            let service = self.registry.find_by_capability(&primal_cap).await?;

            Ok(ResolvedService {
                id: service.id.clone(),
                host: service.address.to_string(),
                port: service.port,
                protocol: service.protocol.clone(),
                capabilities: vec![capability.clone()],
                is_healthy: matches!(
                    service.health,
                    crate::universal_primal_discovery::capability_based_discovery::HealthStatus::Healthy
                ),
            })
        })
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let primal_cap = CapabilityMapper::to_primal(&capability);
            let services = self.registry.find_all_by_capability(&primal_cap).await?;

            Ok(services
                .into_iter()
                .map(|service| ResolvedService {
                    id: service.id.clone(),
                    host: service.address.to_string(),
                    port: service.port,
                    protocol: service.protocol.clone(),
                    capabilities: vec![capability.clone()],
                    is_healthy: matches!(
                        service.health,
                        crate::universal_primal_discovery::capability_based_discovery::HealthStatus::Healthy
                    ),
                })
                .collect())
        })
    }

    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let primal_cap = CapabilityMapper::to_primal(&capability);
            self.registry.find_by_capability(&primal_cap).await.is_ok()
        })
    }
}

/// Adapter for service_discovery::InMemoryServiceRegistry
///
/// Bridges the in-memory service registry to the unified interface
pub struct InMemoryRegistryAdapter<'a> {
    registry: &'a crate::service_discovery::registry::InMemoryServiceRegistry,
}

impl<'a> InMemoryRegistryAdapter<'a> {
    /// Create adapter from in-memory registry
    pub fn new(registry: &'a crate::service_discovery::registry::InMemoryServiceRegistry) -> Self {
        Self { registry }
    }
}

impl<'a> CapabilityResolver for InMemoryRegistryAdapter<'a> {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            // Convert to ServiceCapability for InMemoryServiceRegistry
            let service_cap = self.unified_to_service_capability(&capability);

            use crate::service_discovery::registry::UniversalServiceRegistry;
            let services = self
                .registry
                .discover_by_capabilities(vec![service_cap])
                .await?;

            let service = services.into_iter().next().ok_or_else(|| {
                NestGateError::internal_error(
                    format!("No service found for capability: {}", capability),
                    "capability_resolver",
                )
            })?;

            // Extract endpoint info from service
            let endpoint = service.endpoints.first().ok_or_else(|| {
                NestGateError::internal_error("Service has no endpoints", "capability_resolver")
            })?;

            // Parse URL to extract host and port
            let url = endpoint.url.parse::<url::Url>().map_err(|_| {
                NestGateError::internal_error("Invalid endpoint URL", "capability_resolver")
            })?;

            Ok(ResolvedService {
                id: service.service_id.to_string(),
                host: url.host_str().unwrap_or("localhost").to_string(),
                port: url.port().unwrap_or(8080),
                protocol: match endpoint.protocol {
                    crate::service_discovery::types::CommunicationProtocol::Http => "http",
                    crate::service_discovery::types::CommunicationProtocol::Grpc => "grpc",
                    crate::service_discovery::types::CommunicationProtocol::WebSocket => "ws",
                    _ => "http",
                }
                .to_string(),
                capabilities: vec![capability.clone()],
                is_healthy: true, // InMemoryRegistry doesn't track health separately
            })
        })
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let service_cap = self.unified_to_service_capability(&capability);
            use crate::service_discovery::registry::UniversalServiceRegistry;
            let services = self
                .registry
                .discover_by_capabilities(vec![service_cap])
                .await?;

            Ok(services
                .into_iter()
                .filter_map(|service| {
                    service.endpoints.first().and_then(|endpoint| {
                        // Parse URL to extract host and port
                        endpoint.url.parse::<url::Url>().ok().map(|url| ResolvedService {
                            id: service.service_id.to_string(),
                            host: url.host_str().unwrap_or("localhost").to_string(),
                            port: url.port().unwrap_or(8080),
                            protocol: match endpoint.protocol {
                                crate::service_discovery::types::CommunicationProtocol::Http => "http",
                                crate::service_discovery::types::CommunicationProtocol::Grpc => "grpc",
                                crate::service_discovery::types::CommunicationProtocol::WebSocket => "ws",
                                _ => "http",
                            }.to_string(),
                            capabilities: vec![capability.clone()],
                            is_healthy: true,
                        })
                    })
                })
                .collect())
        })
    }

    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let service_cap = self.unified_to_service_capability(&capability);
            use crate::service_discovery::registry::UniversalServiceRegistry;
            self.registry
                .discover_by_capabilities(vec![service_cap])
                .await
                .map(|services| !services.is_empty())
                .unwrap_or(false)
        })
    }
}

impl<'a> InMemoryRegistryAdapter<'a> {
    /// Convert UnifiedCapability to ServiceCapability
    fn unified_to_service_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> crate::service_discovery::types::ServiceCapability {
        use crate::service_discovery::types::{
            AIModality, CommunicationProtocol, OrchestrationScope, SecurityFunction,
            ServiceCapability, StorageType,
        };

        match capability {
            UnifiedCapability::Storage
            | UnifiedCapability::ZfsManagement
            | UnifiedCapability::ObjectStorage => ServiceCapability::Storage(StorageType::Object),
            UnifiedCapability::BlockStorage => ServiceCapability::Storage(StorageType::Block),
            UnifiedCapability::FileStorage => ServiceCapability::Storage(StorageType::FileSystem),
            UnifiedCapability::Networking | UnifiedCapability::HttpApi => {
                ServiceCapability::Network(CommunicationProtocol::Http)
            }
            UnifiedCapability::Grpc => ServiceCapability::Network(CommunicationProtocol::Grpc),
            UnifiedCapability::Websocket => {
                ServiceCapability::Network(CommunicationProtocol::WebSocket)
            }
            UnifiedCapability::Mqtt => {
                ServiceCapability::Network(CommunicationProtocol::MessageQueue)
            }
            UnifiedCapability::Compute
            | UnifiedCapability::Orchestration
            | UnifiedCapability::TaskExecution
            | UnifiedCapability::Scheduling => {
                ServiceCapability::Orchestration(OrchestrationScope::Service)
            }
            UnifiedCapability::Security | UnifiedCapability::Authentication => {
                ServiceCapability::Security(SecurityFunction::Authentication)
            }
            UnifiedCapability::Authorization => {
                ServiceCapability::Security(SecurityFunction::Authorization)
            }
            UnifiedCapability::Encryption => {
                ServiceCapability::Security(SecurityFunction::Encryption)
            }
            UnifiedCapability::SecretManagement => {
                ServiceCapability::Security(SecurityFunction::CertificateManagement)
            }
            UnifiedCapability::ArtificialIntelligence
            | UnifiedCapability::ModelServing
            | UnifiedCapability::Training
            | UnifiedCapability::Inference => ServiceCapability::AI(AIModality::MachineLearning),
            _ => ServiceCapability::Custom {
                namespace: "nestgate".to_string(),
                capability: capability.to_string(),
                version: "1.0".to_string(),
            },
        }
    }
}

/// Environment-based capability resolver (fallback)
///
/// Resolves capabilities using environment variables only.
/// Used when no registry is available.
pub struct EnvironmentResolver;

impl EnvironmentResolver {
    /// Create new environment resolver
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityResolver for EnvironmentResolver {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let env_var = CapabilityMapper::env_var_name(&capability);
            let value = std::env::var(&env_var).map_err(|_| {
                NestGateError::internal_error(
                    format!(
                        "Capability '{}' not configured. Set {} environment variable.",
                        capability, env_var
                    ),
                    "environment_resolver",
                )
            })?;

            // Parse URL or host:port format
            if let Ok(url) = value.parse::<url::Url>() {
                let port = url
                    .port()
                    .or_else(|| {
                        // Use default port for protocol if not specified
                        match url.scheme() {
                            "https" => Some(443),
                            "http" => Some(80),
                            "grpc" => Some(9090),
                            _ => None,
                        }
                    })
                    .unwrap_or(8080);

                Ok(ResolvedService {
                    id: "env-configured".to_string(),
                    host: url.host_str().unwrap_or("localhost").to_string(),
                    port,
                    protocol: url.scheme().to_string(),
                    capabilities: vec![capability.clone()],
                    is_healthy: true,
                })
            } else if let Some((host, port_str)) = value.split_once(':') {
                let port = port_str.parse().map_err(|_| {
                    NestGateError::internal_error(
                        format!("Invalid port in {}: {}", env_var, port_str),
                        "environment_resolver",
                    )
                })?;
                Ok(ResolvedService {
                    id: "env-configured".to_string(),
                    host: host.to_string(),
                    port,
                    protocol: "http".to_string(),
                    capabilities: vec![capability.clone()],
                    is_healthy: true,
                })
            } else {
                Err(NestGateError::internal_error(
                    format!(
                        "Invalid endpoint format in {}: {}. Expected URL or host:port",
                        env_var, value
                    ),
                    "environment_resolver",
                ))
            }
        })
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            // Environment resolver returns single service
            self.resolve_capability(&capability)
                .await
                .map(|service| vec![service])
        })
    }

    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let env_var = CapabilityMapper::env_var_name(&capability);
            std::env::var(&env_var).is_ok()
        })
    }
}

/// Composite resolver - tries multiple resolvers in order
///
/// Enables fallback chain: Registry -> Environment -> Error
pub struct CompositeResolver<'a> {
    resolvers: Vec<Box<dyn CapabilityResolver + 'a>>,
}

impl<'a> CompositeResolver<'a> {
    /// Create new composite resolver
    pub fn new() -> Self {
        Self {
            resolvers: Vec::new(),
        }
    }

    /// Add a resolver to the chain
    pub fn with_resolver(mut self, resolver: Box<dyn CapabilityResolver + 'a>) -> Self {
        self.resolvers.push(resolver);
        self
    }

    /// Create default resolver chain (registry -> environment)
    pub fn default_chain(
        registry: Option<&'a crate::universal_primal_discovery::service_registry::ServiceRegistry>,
    ) -> Self {
        let mut composite = Self::new();

        if let Some(reg) = registry {
            composite = composite.with_resolver(Box::new(PrimalDiscoveryAdapter::new(reg)));
        }

        composite = composite.with_resolver(Box::new(EnvironmentResolver::new()));

        composite
    }
}

impl<'a> Default for CompositeResolver<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> CapabilityResolver for CompositeResolver<'a> {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            for resolver in &self.resolvers {
                if let Ok(service) = resolver.resolve_capability(&capability).await {
                    return Ok(service);
                }
            }

            Err(NestGateError::internal_error(
                format!(
                    "Capability '{}' could not be resolved by any resolver",
                    capability
                ),
                "composite_resolver",
            ))
        })
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            let mut all_services = Vec::new();

            for resolver in &self.resolvers {
                if let Ok(services) = resolver.resolve_capability_all(&capability).await {
                    all_services.extend(services);
                }
            }

            if all_services.is_empty() {
                Err(NestGateError::internal_error(
                    format!("No services found for capability: {}", capability),
                    "composite_resolver",
                ))
            } else {
                Ok(all_services)
            }
        })
    }

    fn has_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            for resolver in &self.resolvers {
                if resolver.has_capability(&capability).await {
                    return true;
                }
            }
            false
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_resolver() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_HTTP_API_ENDPOINT",
            "http://localhost:8080",
        );

        let resolver = EnvironmentResolver::new();
        let result = resolver
            .resolve_capability(&UnifiedCapability::HttpApi)
            .await;

        assert!(result.is_ok());
        let service = result.unwrap();
        assert_eq!(service.host, "localhost");
        assert_eq!(service.port, 8080);
        assert_eq!(service.protocol, "http");

        std::env::remove_var("NESTGATE_CAPABILITY_HTTP_API_ENDPOINT");
    }

    #[tokio::test]
    async fn test_environment_resolver_host_port() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_STORAGE_ENDPOINT",
            "http://storage-server:9000",
        );

        let resolver = EnvironmentResolver::new();
        let result = resolver
            .resolve_capability(&UnifiedCapability::Storage)
            .await;

        assert!(result.is_ok());
        let service = result.unwrap();
        assert_eq!(service.host, "storage-server");
        assert_eq!(service.port, 9000);

        std::env::remove_var("NESTGATE_CAPABILITY_STORAGE_ENDPOINT");
    }

    #[tokio::test]
    async fn test_environment_resolver_missing_capability() {
        std::env::remove_var("NESTGATE_CAPABILITY_COMPUTE_ENDPOINT");

        let resolver = EnvironmentResolver::new();
        let result = resolver
            .resolve_capability(&UnifiedCapability::Compute)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_has_capability() {
        std::env::set_var(
            "NESTGATE_CAPABILITY_METRICS_ENDPOINT",
            "http://localhost:9090",
        );

        let resolver = EnvironmentResolver::new();
        assert!(resolver.has_capability(&UnifiedCapability::Metrics).await);
        assert!(!resolver.has_capability(&UnifiedCapability::Compute).await);

        std::env::remove_var("NESTGATE_CAPABILITY_METRICS_ENDPOINT");
    }
}
