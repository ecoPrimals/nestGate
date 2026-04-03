// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! mDNS-style discovery (optional, standalone / development)
//!
//! **Production discovery belongs to the ecosystem platform and orchestration provider**; this in-memory registry exists only for
//! local tooling and tests when the `mdns` crate feature is enabled.
//!
//! This module provides a simplified in-memory implementation. A full mDNS stack would use the
//! multicast DNS protocol (for example via avahi or dns-sd).

use super::{Capability, DiscoveryBuilder, DiscoveryMechanism, ServiceInfo};
use crate::self_knowledge::SelfKnowledge;
use dashmap::DashMap;
use nestgate_types::error::Result;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// In-memory service registry for mDNS
///
/// This is a simple implementation that stores services in memory.
/// In production, this would use actual mDNS protocol (avahi-daemon, dns-sd, etc.)
/// Lock-free with `DashMap` for better concurrent discovery performance
type ServiceRegistry = Arc<DashMap<String, ServiceInfo>>;

/// mDNS discovery mechanism
///
/// **Note**: This is a simplified in-memory implementation for testing.
/// Production mDNS would use actual mDNS protocol (avahi-daemon, dns-sd).
///
/// **FUTURE**: Implement active timeout enforcement for queries and cache expiration.
/// Fields are reserved for future use when full mDNS protocol is needed.
pub struct MdnsDiscovery {
    /// Query timeout (reserved for future mDNS protocol implementation)
    _timeout: Duration,
    /// Cache duration (reserved for future mDNS protocol implementation)
    _cache_duration: Duration,
    /// In-memory service registry (lock-free)
    registry: ServiceRegistry,
    /// Our announced service ID
    announced_service_id: Arc<RwLock<Option<String>>>,
}

impl MdnsDiscovery {
    /// Create new mDNS discovery (lock-free registry)
    pub fn new(builder: &DiscoveryBuilder) -> Result<Self> {
        Ok(Self {
            _timeout: builder.timeout,
            _cache_duration: builder.cache_duration,
            registry: Arc::new(DashMap::new()),
            announced_service_id: Arc::new(RwLock::new(None)),
        })
    }

    /// Create service info from self-knowledge
    fn create_service_info(self_knowledge: &SelfKnowledge) -> ServiceInfo {
        let primary_endpoint = self_knowledge
            .endpoints
            .get("api")
            .map_or_else(|| "unknown".to_string(), std::string::ToString::to_string);

        let health_endpoint = self_knowledge
            .endpoints
            .get("health")
            .map(std::string::ToString::to_string);

        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), self_knowledge.version.clone());
        metadata.insert(
            "endpoints".to_string(),
            format!("{:?}", self_knowledge.endpoints),
        );

        ServiceInfo {
            id: self_knowledge.id.as_str().to_string(),
            name: self_knowledge.name.clone(),
            capabilities: self_knowledge.capabilities.clone(),
            endpoint: primary_endpoint,
            metadata,
            health_endpoint,
        }
    }
}

impl DiscoveryMechanism for MdnsDiscovery {
    fn announce(
        &self,
        self_knowledge: &SelfKnowledge,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let registry = Arc::clone(&self.registry);
        let announced_service_id = Arc::clone(&self.announced_service_id);
        let service_info = Self::create_service_info(self_knowledge);
        let service_id = service_info.id.clone();
        let announce_name = self_knowledge.name.clone();
        let announce_caps = self_knowledge.capabilities.clone();

        Box::pin(async move {
            tracing::info!(
                "mDNS announce: {} with capabilities: {:?}",
                announce_name,
                announce_caps
            );

            // Store in registry (lock-free)
            registry.insert(service_id.clone(), service_info);

            // Remember our service ID
            let mut announced = announced_service_id.write().await;
            *announced = Some(service_id);

            tracing::info!("Successfully announced to mDNS registry");
            Ok(())
        })
    }

    fn find_by_capability(
        &self,
        capability: Capability,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ServiceInfo>>> + Send + '_>> {
        let registry = Arc::clone(&self.registry);
        Box::pin(async move {
            tracing::debug!("mDNS query for capability: {:?}", capability);

            // Lock-free iteration
            let matching_services: Vec<ServiceInfo> = registry
                .iter()
                .map(|entry| entry.value().clone())
                .filter(|service| service.capabilities.contains(&capability))
                .collect();

            tracing::debug!(
                "Found {} services with capability '{}'",
                matching_services.len(),
                capability
            );

            Ok(matching_services)
        })
    }

    fn find_by_id(
        &self,
        id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<Option<ServiceInfo>>> + Send + '_>> {
        let registry = Arc::clone(&self.registry);
        let id = id.to_string();
        Box::pin(async move {
            tracing::debug!("mDNS lookup service: {}", id);

            // Lock-free lookup
            let service = registry.get(&id).map(|entry| entry.value().clone());

            if service.is_some() {
                tracing::debug!("Found service: {}", id);
            } else {
                tracing::debug!("Service not found: {}", id);
            }

            Ok(service)
        })
    }

    fn health_check(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>> {
        let registry = Arc::clone(&self.registry);
        let service_id = service_id.to_string();
        Box::pin(async move {
            tracing::debug!("mDNS health check: {}", service_id);

            // Check if service exists in registry (lock-free)
            let healthy = registry.contains_key(&service_id);

            tracing::debug!("Service {} health: {}", service_id, healthy);
            Ok(healthy)
        })
    }

    fn deregister(
        &self,
        service_id: &str,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        let registry = Arc::clone(&self.registry);
        let announced_service_id = Arc::clone(&self.announced_service_id);
        let service_id = service_id.to_string();
        Box::pin(async move {
            tracing::info!("mDNS deregister: {}", service_id);

            // Lock-free remove
            registry.remove(&service_id);

            // Clear announced service if it was us
            let mut announced = announced_service_id.write().await;
            if announced.as_ref().map(std::string::String::as_str) == Some(service_id.as_str()) {
                *announced = None;
            }

            tracing::info!("Successfully deregistered from mDNS registry");
            Ok(())
        })
    }

    fn mechanism_name(&self) -> &'static str {
        "mdns"
    }
}
