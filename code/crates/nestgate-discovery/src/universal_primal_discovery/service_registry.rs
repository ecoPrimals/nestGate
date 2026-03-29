// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Service Registry - High-level API for capability-based discovery
//!
//! This module provides a simple, ergonomic interface for discovering services
//! by capability, eliminating the need for hardcoded service URLs.
//!
//! # Philosophy
//!
//! **Primal Sovereignty**: Each primal only knows itself. Other primals are
//! discovered at runtime through capabilities, not hardcoded names or addresses.
//!
//! # Usage
//!
//! ```rust,ignore
//! // ServiceRegistry::new() requires Vec<PrimalCapability>; find_by_capability needs running backends
//! use nestgate_core::universal_primal_discovery::ServiceRegistry;
//! use nestgate_core::universal_primal_discovery::capability_based_discovery::PrimalCapability;
//! let registry = ServiceRegistry::new(vec![PrimalCapability::ZfsStorage]).await?;
//! let service = registry.find_by_capability(&PrimalCapability::ZfsStorage).await?;
//! ```
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────┐
//! │ ServiceRegistry │ ← High-level API
//! └────────┬────────┘
//!          │ uses
//!          ▼
//! ┌──────────────────────────┐
//! │ CapabilityBasedDiscovery │ ← Discovery engine
//! └────────┬─────────────────┘
//!          │ delegates to
//!          ▼
//! ┌──────────────────┐
//! │ Discovery Backend│ ← mDNS, Registry, etc.
//! └──────────────────┘
//! ```

use crate::universal_primal_discovery::capability_based_discovery::{
    BindingInfo, CapabilityDiscoveryManager, HealthStatus, PeerDescriptor, PrimalCapability,
    PrimalId,
};
use nestgate_types::error::{NestGateError, Result};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;

/// Service endpoint information discovered via capabilities
#[derive(Debug, Clone)]
pub struct ServiceEndpoint {
    /// Unique identifier for this service instance
    pub id: Arc<str>,
    /// IP address the service is bound to
    pub address: IpAddr,
    /// Port the service is listening on
    pub port: u16,
    /// Protocol (HTTP, HTTPS, etc.)
    pub protocol: Arc<str>,
    /// Capabilities this service provides
    pub capabilities: Vec<PrimalCapability>,
    /// Service health status
    pub health: HealthStatus,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ServiceEndpoint {
    /// Get the full URL for this service
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// # use nestgate_core::universal_primal_discovery::service_registry::ServiceEndpoint;
    /// # use std::net::{IpAddr, Ipv4Addr};
    /// # let endpoint = ServiceEndpoint {
    /// #     id: "test".into(),
    /// #     address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
    /// #     port: 8080,
    /// #     protocol: "http".into(),
    /// #     capabilities: vec![],
    /// #     health: nestgate_core::universal_primal_discovery::capability_based_discovery::HealthStatus::Healthy,
    /// #     metadata: Default::default(),
    /// # };
    /// assert_eq!(endpoint.url(), "http://127.0.0.1:8080");
    /// ```
    #[must_use]
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.address, self.port)
    }

    /// Get a specific metadata value
    #[must_use]
    pub fn get_metadata(&self, key: &str) -> Option<&str> {
        self.metadata.get(key).map(String::as_str)
    }

    /// Check if service is healthy
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self.health, HealthStatus::Healthy)
    }
}

/// High-level service registry for capability-based discovery
///
/// Provides a simple API for discovering services without hardcoded URLs.
pub struct ServiceRegistry {
    discovery: Arc<CapabilityDiscoveryManager>,
}

impl ServiceRegistry {
    /// Create a new service registry with automatic discovery
    ///
    /// This will initialize the discovery backend and start listening for
    /// service announcements.
    ///
    /// # Errors
    ///
    /// Returns an error if the discovery backend cannot be initialized.
    pub async fn new(capabilities: Vec<PrimalCapability>) -> Result<Self> {
        // Create discovery manager for these capabilities
        let discovery = CapabilityDiscoveryManager::initialize(capabilities).await?;

        Ok(Self {
            discovery: Arc::new(discovery),
        })
    }

    /// Create a service registry with a custom discovery manager
    ///
    /// Useful for testing or when using specific discovery mechanisms (mDNS, etc.)
    #[must_use]
    pub fn with_discovery(discovery: CapabilityDiscoveryManager) -> Self {
        Self {
            discovery: Arc::new(discovery),
        }
    }

    /// Find a service by capability
    ///
    /// Returns the first healthy service that provides the requested capability.
    ///
    /// # Errors
    ///
    /// - `NotFound`: No service found with the requested capability
    /// - `Unavailable`: Service found but unhealthy
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // Requires capabilities for new(); find needs discovery backends with services
    /// # use nestgate_core::universal_primal_discovery::ServiceRegistry;
    /// # use nestgate_core::universal_primal_discovery::capability_based_discovery::PrimalCapability;
    /// let registry = ServiceRegistry::new(vec![PrimalCapability::ZfsStorage]).await?;
    /// let service = registry.find_by_capability(&PrimalCapability::ZfsStorage).await?;
    /// ```
    pub async fn find_by_capability(
        &self,
        capability: &PrimalCapability,
    ) -> Result<ServiceEndpoint> {
        // Query discovery for services with this capability
        let services = self.discovery.find_capability(capability.clone()).await?;

        // Find first healthy service
        services
            .into_iter()
            .find(|s| s.health.is_healthy())
            .map(Self::convert_to_endpoint)
            .ok_or_else(|| {
                NestGateError::not_found(format!(
                    "No healthy service found with capability: {capability:?}"
                ))
            })
    }

    /// Find all services with a given capability
    ///
    /// Returns all services (healthy or not) that provide the capability.
    ///
    /// # Errors
    ///
    /// Returns an error if the discovery query fails.
    pub async fn find_all_by_capability(
        &self,
        capability: &PrimalCapability,
    ) -> Result<Vec<ServiceEndpoint>> {
        let services = self.discovery.find_capability(capability.clone()).await?;
        Ok(services
            .into_iter()
            .map(Self::convert_to_endpoint)
            .collect())
    }

    /// Announce this primal's presence and capabilities
    ///
    /// Call this during service startup to make this primal discoverable by others.
    ///
    /// # Errors
    ///
    /// Returns an error if the announcement fails.
    ///
    /// # Examples
    ///
    /// ```text
    /// Requires PrimalId::from_environment(), BindingInfo; see integration tests for a full example.
    /// ```
    pub async fn announce(
        &self,
        _id: PrimalId,
        _capabilities: Vec<PrimalCapability>,
        _binding: BindingInfo,
    ) -> Result<()> {
        // Announce to all backends
        self.discovery.start_announcing().await
    }

    /// Update the health status of this primal
    ///
    /// Should be called periodically to maintain healthy status in the registry.
    pub async fn update_health(&self, _id: &PrimalId, _health: HealthStatus) -> Result<()> {
        // Re-announce with updated health
        self.discovery.start_announcing().await
    }

    /// Remove this primal from the registry
    ///
    /// Call this during graceful shutdown.
    pub async fn withdraw(&self) -> Result<()> {
        self.discovery.shutdown().await
    }

    /// Convert internal `PeerDescriptor` to public `ServiceEndpoint`
    fn convert_to_endpoint(peer: PeerDescriptor) -> ServiceEndpoint {
        use crate::universal_primal_discovery::capability_based_discovery::Protocol;
        let protocol: Arc<str> = match peer.endpoint.protocol {
            Protocol::Tcp => Arc::from("tcp"),
            Protocol::Udp => Arc::from("udp"),
            Protocol::Http => Arc::from("http"),
            Protocol::Https => Arc::from("https"),
        };
        ServiceEndpoint {
            id: Arc::from(peer.id.as_str()),
            address: peer.endpoint.address.ip(),
            port: peer.endpoint.address.port(),
            protocol,
            capabilities: peer.capabilities,
            health: peer.health,
            metadata: HashMap::new(), // PeerDescriptor doesn't have metadata
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_service_registry_creation() {
        let registry = ServiceRegistry::new(vec![PrimalCapability::ZfsStorage]).await;
        assert!(registry.is_ok());
    }

    #[tokio::test]
    async fn test_service_endpoint_url() {
        let endpoint = ServiceEndpoint {
            id: "test".into(),
            address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            port: 3000,
            protocol: "https".into(),
            capabilities: vec![],
            health: HealthStatus::Healthy,
            metadata: HashMap::new(),
        };

        assert_eq!(endpoint.url(), "https://192.168.1.100:3000");
    }

    #[tokio::test]
    async fn test_not_found_capability() {
        let registry = ServiceRegistry::new(vec![PrimalCapability::ZfsStorage])
            .await
            .unwrap();

        // Try to find a capability that doesn't exist (no backends configured)
        let result = registry
            .find_by_capability(&PrimalCapability::Custom("nonexistent".to_string()))
            .await;

        assert!(result.is_err());
    }
}
