// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Adapter bridging `universal_primal_discovery::ServiceRegistry` to [`CapabilityResolver`].

use crate::unified_capabilities::{CapabilityMapper, UnifiedCapability};
use nestgate_types::error::Result;

use super::types::{CapabilityResolver, ResolvedService};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

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
                id: Arc::clone(&service.id),
                host: service.address.to_string(),
                port: service.port,
                protocol: Arc::clone(&service.protocol),
                capabilities: vec![capability],
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
                    id: Arc::clone(&service.id),
                    host: service.address.to_string(),
                    port: service.port,
                    protocol: Arc::clone(&service.protocol),
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
