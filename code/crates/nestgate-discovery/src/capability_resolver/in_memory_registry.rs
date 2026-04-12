// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! In-memory service registry adapter and unified-capability → service-capability mapping.

use crate::service_discovery::registry::UniversalServiceRegistry;
use crate::unified_capabilities::UnifiedCapability;
use nestgate_config::constants::system::DEFAULT_SERVICE_NAME;
use nestgate_types::error::{NestGateError, Result};
use nestgate_types::{EnvSource, ProcessEnv};

use super::types::{CapabilityResolver, ResolvedService};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

fn primal_namespace_from_env_source(env: &(impl EnvSource + ?Sized)) -> String {
    env.get("NESTGATE_PRIMAL_NAMESPACE")
        .or_else(|| env.get("NESTGATE_SERVICE_NAME"))
        .unwrap_or_else(|| DEFAULT_SERVICE_NAME.to_string())
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

            let endpoint = service.endpoints.first().ok_or_else(|| {
                NestGateError::internal_error("Service has no endpoints", "capability_resolver")
            })?;

            let url = endpoint.url.parse::<url::Url>().map_err(|_| {
                NestGateError::internal_error("Invalid endpoint URL", "capability_resolver")
            })?;

            let host = url
                .host_str()
                .ok_or_else(|| {
                    NestGateError::configuration_error(
                        "endpoint_host",
                        format!("Service endpoint URL missing host: {}", endpoint.url),
                    )
                })?
                .to_string();

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
                capabilities: vec![capability],
                is_healthy: true,
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
                        endpoint.url.parse::<url::Url>().ok().and_then(|url| {
                            let host = url.host_str()?.to_string();

                            let port = url.port().or(match endpoint.protocol {
                                crate::service_discovery::types::CommunicationProtocol::Http
                                | crate::service_discovery::types::CommunicationProtocol::WebSocket => {
                                    Some(80)
                                }
                                crate::service_discovery::types::CommunicationProtocol::Grpc => {
                                    Some(9090)
                                }
                                _ => None,
                            })?;

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
        self.unified_to_service_capability_from_env_source(capability, &ProcessEnv)
    }

    fn unified_to_service_capability_from_env_source(
        &self,
        capability: &UnifiedCapability,
        env: &(impl EnvSource + ?Sized),
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
                namespace: primal_namespace_from_env_source(env),
                capability: capability.to_string(),
                version: String::from("1.0"),
            },
        }
    }
}
