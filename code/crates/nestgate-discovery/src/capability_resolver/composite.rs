// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Priority-ordered resolver chain (registry → environment, etc.).

use crate::unified_capabilities::UnifiedCapability;
use nestgate_types::error::{NestGateError, Result};

use super::environment::EnvironmentResolver;
use super::primal_discovery::PrimalDiscoveryAdapter;
use super::types::{CapabilityResolver, ResolvedService};
use std::future::Future;
use std::pin::Pin;

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
