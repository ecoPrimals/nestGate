// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Unified Capability Resolver - Bridge for All Registry Systems
//!
//! **ARCHITECTURAL UNIFICATION**: This module creates a common interface for all
//! registry and discovery systems in the codebase, enabling them to work together.
//!
//! **SYSTEMS UNIFIED**:
//! 1. `InMemoryServiceRegistry` (`service_discovery`)
//! 2. `ServiceRegistry` (`universal_primal_discovery`)
//! 3. `CapabilityDiscoveryManager`
//! 4. Application layer capabilities
//!
//! **PHILOSOPHY**: One interface to rule them all - capability-based discovery
//! regardless of underlying implementation.

use crate::service_discovery::registry::UniversalServiceRegistry;
use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};
use nestgate_types::error::{NestGateError, Result};
use std::future::{Future, ready};
use std::pin::Pin;
use std::sync::Arc;

/// Unified service endpoint result from capability resolution
#[derive(Debug, Clone)]
pub struct ResolvedService {
    /// Service identifier
    pub id: Arc<str>,
    /// Host address
    pub host: String,
    /// Port number
    pub port: u16,
    /// Protocol (http, https, grpc, etc.)
    pub protocol: Arc<str>,
    /// Capabilities this service provides
    pub capabilities: Vec<UnifiedCapability>,
    /// Service health indicator
    pub is_healthy: bool,
}

impl ResolvedService {
    /// Get full URL for this service
    #[must_use]
    pub fn url(&self) -> String {
        format!("{}://{}:{}", &*self.protocol, self.host, self.port)
    }

    /// Get endpoint without protocol
    #[must_use]
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

/// Adapter for `universal_primal_discovery::ServiceRegistry`
///
/// Bridges the universal primal discovery system to the unified interface
pub struct PrimalDiscoveryAdapter<'a> {
    registry: &'a crate::universal_primal_discovery::service_registry::ServiceRegistry,
}

impl<'a> PrimalDiscoveryAdapter<'a> {
    /// Create adapter from primal discovery registry
    #[must_use]
    pub const fn new(
        registry: &'a crate::universal_primal_discovery::service_registry::ServiceRegistry,
    ) -> Self {
        Self { registry }
    }
}

impl CapabilityResolver for PrimalDiscoveryAdapter<'_> {
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

/// Adapter for `service_discovery::InMemoryServiceRegistry`
///
/// Bridges the in-memory service registry to the unified interface
pub struct InMemoryRegistryAdapter<'a> {
    registry: &'a crate::service_discovery::registry::InMemoryServiceRegistry,
}

impl<'a> InMemoryRegistryAdapter<'a> {
    /// Create adapter from in-memory registry
    #[must_use]
    pub const fn new(
        registry: &'a crate::service_discovery::registry::InMemoryServiceRegistry,
    ) -> Self {
        Self { registry }
    }
}

impl CapabilityResolver for InMemoryRegistryAdapter<'_> {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(async move {
            // Convert to ServiceCapability for InMemoryServiceRegistry
            let service_cap = self.unified_to_service_capability(&capability);

            let services = self
                .registry
                .discover_by_capabilities(vec![service_cap])
                .await?;

            let service = services.into_iter().next().ok_or_else(|| {
                NestGateError::internal_error(
                    format!("No service found for capability: {capability}"),
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

            // Extract host - no hardcoded fallback, error if missing
            let host = url
                .host_str()
                .ok_or_else(|| {
                    NestGateError::configuration_error(
                        "endpoint_host",
                        format!("Service endpoint URL missing host: {}", endpoint.url),
                    )
                })?
                .to_string();

            // Extract port with protocol-based defaults
            let port = url
                .port()
                .or(match endpoint.protocol {
                    crate::service_discovery::types::CommunicationProtocol::Http
                    | crate::service_discovery::types::CommunicationProtocol::WebSocket => Some(80),
                    crate::service_discovery::types::CommunicationProtocol::Grpc => Some(9090),
                    _ => None,
                })
                .ok_or_else(|| {
                    NestGateError::configuration_error(
                        "endpoint_port",
                        format!(
                            "Service endpoint URL missing port and no default for protocol: {}",
                            endpoint.url
                        ),
                    )
                })?;

            Ok(ResolvedService {
                id: Arc::from(service.service_id.to_string()),
                host,
                port,
                protocol: match endpoint.protocol {
                    crate::service_discovery::types::CommunicationProtocol::Grpc => {
                        Arc::from("grpc")
                    }
                    crate::service_discovery::types::CommunicationProtocol::WebSocket => {
                        Arc::from("ws")
                    }
                    _ => Arc::from("http"),
                },
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
            let services = self
                .registry
                .discover_by_capabilities(vec![service_cap])
                .await?;

            Ok(services
                .into_iter()
                .filter_map(|service| {
                    service.endpoints.first().and_then(|endpoint| {
                        // Parse URL to extract host and port
                        endpoint.url.parse::<url::Url>().ok().and_then(|url| {
                            // Extract host - skip service if host is missing
                            let host = url.host_str()?.to_string();

                            // Extract port with protocol-based defaults
                            let port = url.port().or(match endpoint.protocol {
                                crate::service_discovery::types::CommunicationProtocol::Http
                                | crate::service_discovery::types::CommunicationProtocol::WebSocket => {
                                    Some(80)
                                }
                                crate::service_discovery::types::CommunicationProtocol::Grpc => {
                                    Some(9090)
                                }
                                _ => None,
                            })?; // Skip service if no port and no default

                            Some(ResolvedService {
                                id: Arc::from(service.service_id.to_string()),
                                host,
                                port,
                                protocol: match endpoint.protocol {
                                    crate::service_discovery::types::CommunicationProtocol::Grpc => {
                                        Arc::from("grpc")
                                    }
                                    crate::service_discovery::types::CommunicationProtocol::WebSocket => {
                                        Arc::from("ws")
                                    }
                                    _ => Arc::from("http"),
                                },
                                capabilities: vec![capability.clone()],
                                is_healthy: true,
                            })
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
            self.registry
                .discover_by_capabilities(vec![service_cap])
                .await
                .map(|services| !services.is_empty())
                .unwrap_or(false)
        })
    }
}

impl InMemoryRegistryAdapter<'_> {
    /// Convert `UnifiedCapability` to `ServiceCapability`
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
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for EnvironmentResolver {
    fn default() -> Self {
        Self::new()
    }
}

/// Shared env parsing for [`EnvironmentResolver`] — avoids nested `resolve_capability().await`
/// inside `resolve_capability_all` (one fewer `dyn Future` layer on that path).
fn resolve_capability_from_env(capability: UnifiedCapability) -> Result<ResolvedService> {
    let env_var = CapabilityMapper::env_var_name(&capability);
    let value = std::env::var(&env_var).map_err(|_| {
        NestGateError::internal_error(
            format!(
                "Capability '{capability}' not configured. Set {env_var} environment variable."
            ),
            "environment_resolver",
        )
    })?;

    // Parse URL or host:port format
    if let Ok(url) = value.parse::<url::Url>() {
        // Extract host - error if missing
        let host = url
            .host_str()
            .ok_or_else(|| {
                NestGateError::configuration_error(
                    "capability_endpoint_host",
                    format!("Environment variable {env_var} has URL without host: {value}"),
                )
            })?
            .to_string();

        // Extract port with protocol-based defaults
        let port = url.port().or_else(|| {
            // Use default port for protocol if not specified
            match url.scheme() {
                "https" => Some(443),
                "http" | "ws" | "wss" => Some(80),
                "grpc" => Some(9090),
                _ => None,
            }
        }).ok_or_else(|| NestGateError::configuration_error(
            "capability_endpoint_port",
            format!("Environment variable {} has URL without port and no default for scheme: {}", env_var, url.scheme())
        ))?;

        Ok(ResolvedService {
            id: Arc::from("env-configured"),
            host,
            port,
            protocol: Arc::from(url.scheme().to_string()),
            capabilities: vec![capability],
            is_healthy: true,
        })
    } else if let Some((host, port_str)) = value.split_once(':') {
        let port = port_str.parse().map_err(|_| {
            NestGateError::internal_error(
                format!("Invalid port in {env_var}: {port_str}"),
                "environment_resolver",
            )
        })?;
        Ok(ResolvedService {
            id: Arc::from("env-configured"),
            host: host.to_string(),
            port,
            protocol: Arc::from("http"),
            capabilities: vec![capability],
            is_healthy: true,
        })
    } else {
        Err(NestGateError::internal_error(
            format!("Invalid endpoint format in {env_var}: {value}. Expected URL or host:port"),
            "environment_resolver",
        ))
    }
}

impl CapabilityResolver for EnvironmentResolver {
    fn resolve_capability(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<ResolvedService>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(ready(resolve_capability_from_env(capability)))
    }

    fn resolve_capability_all(
        &self,
        capability: &UnifiedCapability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ResolvedService>>> + Send + '_>> {
        let capability = capability.clone();
        Box::pin(ready(
            resolve_capability_from_env(capability).map(|service| vec![service]),
        ))
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            resolvers: Vec::new(),
        }
    }

    /// Add a resolver to the chain
    #[must_use]
    pub fn with_resolver(mut self, resolver: Box<dyn CapabilityResolver + 'a>) -> Self {
        self.resolvers.push(resolver);
        self
    }

    /// Create default resolver chain (registry -> environment)
    #[must_use]
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

impl Default for CompositeResolver<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl CapabilityResolver for CompositeResolver<'_> {
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
                format!("Capability '{capability}' could not be resolved by any resolver"),
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
                    format!("No services found for capability: {capability}"),
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
    use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};

    #[test]
    fn resolved_service_url_and_endpoint() {
        let s = ResolvedService {
            id: "1".into(),
            host: "127.0.0.1".into(),
            port: 8080,
            protocol: "http".into(),
            capabilities: vec![],
            is_healthy: true,
        };
        assert_eq!(s.url(), "http://127.0.0.1:8080");
        assert_eq!(s.endpoint(), "127.0.0.1:8080");
    }

    #[test]
    fn resolved_service_url_includes_non_http_scheme() {
        let s = ResolvedService {
            id: "g".into(),
            host: "10.0.0.2".into(),
            port: 9090,
            protocol: "grpc".into(),
            capabilities: vec![],
            is_healthy: true,
        };
        assert_eq!(s.url(), "grpc://10.0.0.2:9090");
    }

    #[test]
    fn capability_mapper_env_var_name_format() {
        let name = CapabilityMapper::env_var_name(&UnifiedCapability::Storage);
        assert!(name.contains("STORAGE"));
        assert!(name.starts_with("NESTGATE_CAPABILITY_"));
        assert!(name.ends_with("_ENDPOINT"));
    }

    #[tokio::test]
    async fn composite_resolver_empty_chain_fails() {
        let c = CompositeResolver::new();
        let err = c
            .resolve_capability(&UnifiedCapability::Storage)
            .await
            .expect_err("no resolvers");
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn unified_capability_display_covers_variants() {
        use UnifiedCapability::*;
        let samples = [
            (Storage, "storage"),
            (ZfsManagement, "zfs-management"),
            (Custom("x".into()), "custom:x"),
            (ArtificialIntelligence, "ai"),
        ];
        for (cap, needle) in samples {
            let s = cap.to_string();
            assert!(s.contains(needle), "{cap:?} -> {s}");
        }
    }

    #[tokio::test]
    async fn composite_resolve_capability_all_errors_when_empty() {
        let cap = UnifiedCapability::Storage;
        let c = CompositeResolver::new();
        let err = c.resolve_capability_all(&cap).await.expect_err("empty");
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn environment_resolver_new_and_default() {
        let _ = EnvironmentResolver::new();
        let _ = EnvironmentResolver::default();
    }

    #[tokio::test]
    async fn in_memory_registry_adapter_resolve_storage_service() {
        use crate::service_discovery::registry::{
            InMemoryServiceRegistry, UniversalServiceRegistry,
        };
        use crate::service_discovery::types::{
            CommunicationProtocol, IntegrationPreferences, ResourceSpec, ServiceCapability,
            ServiceEndpoint as SvcEp, ServiceMetadata, StorageType, UniversalServiceRegistration,
        };
        use uuid::Uuid;

        let reg = InMemoryServiceRegistry::new();
        let sid = Uuid::new_v4();
        let registration = UniversalServiceRegistration {
            service_id: sid,
            metadata: ServiceMetadata {
                name: "storage-a".into(),
                ..Default::default()
            },
            capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
            resources: ResourceSpec::default(),
            endpoints: vec![SvcEp {
                url: "http://127.0.0.1:8080/path".into(),
                protocol: CommunicationProtocol::Http,
                health_check: None,
            }],
            integration: IntegrationPreferences::default(),
            extensions: Default::default(),
        };
        reg.register_service(registration).await.expect("register");

        let adapter = InMemoryRegistryAdapter::new(&reg);
        let resolved = adapter
            .resolve_capability(&UnifiedCapability::Storage)
            .await
            .expect("resolve");
        assert_eq!(resolved.host, "127.0.0.1");
        assert_eq!(resolved.port, 8080);
        assert_eq!(resolved.protocol.as_ref(), "http");

        let all = adapter
            .resolve_capability_all(&UnifiedCapability::Storage)
            .await
            .expect("resolve all");
        assert_eq!(all.len(), 1);

        assert!(adapter.has_capability(&UnifiedCapability::Storage).await);
    }

    #[tokio::test]
    async fn environment_resolver_missing_env_returns_error() {
        let cap = UnifiedCapability::Custom("round3_missing_env_only".into());
        let err = EnvironmentResolver::new()
            .resolve_capability(&cap)
            .await
            .expect_err("unset env");
        assert!(!err.to_string().is_empty());
    }

    #[tokio::test]
    async fn composite_resolver_in_memory_plus_environment_fallback() {
        use crate::service_discovery::registry::{
            InMemoryServiceRegistry, UniversalServiceRegistry,
        };
        use crate::service_discovery::types::{
            CommunicationProtocol, IntegrationPreferences, ResourceSpec, ServiceCapability,
            ServiceEndpoint as SvcEp, ServiceMetadata, StorageType, UniversalServiceRegistration,
        };
        use uuid::Uuid;

        let reg = InMemoryServiceRegistry::new();
        let sid = Uuid::new_v4();
        reg.register_service(UniversalServiceRegistration {
            service_id: sid,
            metadata: ServiceMetadata {
                name: "s".into(),
                ..Default::default()
            },
            capabilities: vec![ServiceCapability::Storage(StorageType::Object)],
            resources: ResourceSpec::default(),
            endpoints: vec![SvcEp {
                url: "http://192.168.1.10:9000".into(),
                protocol: CommunicationProtocol::Http,
                health_check: None,
            }],
            integration: IntegrationPreferences::default(),
            extensions: Default::default(),
        })
        .await
        .expect("reg");

        let composite = CompositeResolver::new()
            .with_resolver(Box::new(InMemoryRegistryAdapter::new(&reg)))
            .with_resolver(Box::new(EnvironmentResolver::new()));
        let s = composite
            .resolve_capability(&UnifiedCapability::Storage)
            .await
            .expect("composite");
        assert_eq!(s.host, "192.168.1.10");

        let merged = composite
            .resolve_capability_all(&UnifiedCapability::Storage)
            .await
            .expect("all");
        assert!(!merged.is_empty());
        assert!(composite.has_capability(&UnifiedCapability::Storage).await);
    }
}
