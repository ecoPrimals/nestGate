// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Discovery mechanism builder
//!
//! Provides a builder pattern for creating discovery mechanisms with
//! configuration options like timeouts, caching, and auto-detection.

use super::{DiscoveryMechanism, mdns};
use nestgate_types::error::Result;
use std::time::Duration;

#[cfg(feature = "consul")]
use super::consul;

#[cfg(feature = "kubernetes")]
use super::k8s;

/// Discovery mechanism builder
pub struct DiscoveryBuilder {
    /// Timeout for discovery operations
    pub(super) timeout: Duration,
    /// Cache duration for discovered services
    pub(super) cache_duration: Duration,
    /// Preferred mechanism (if multiple available)
    pub(super) preferred_mechanism: Option<String>,
}

impl Default for DiscoveryBuilder {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(5),
            cache_duration: Duration::from_secs(60),
            preferred_mechanism: None,
        }
    }
}

impl DiscoveryBuilder {
    /// Create new builder
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set operation timeout
    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set cache duration
    #[must_use]
    pub const fn with_cache_duration(mut self, duration: Duration) -> Self {
        self.cache_duration = duration;
        self
    }

    /// Prefer a specific mechanism
    #[must_use]
    pub fn prefer_mechanism(mut self, mechanism: impl Into<String>) -> Self {
        self.preferred_mechanism = Some(mechanism.into());
        self
    }

    /// Auto-detect best available discovery mechanism
    ///
    /// Detection order (by preference):
    /// 1. Kubernetes (if `KUBERNETES_SERVICE_HOST` set)
    /// 2. Consul (if `CONSUL_HTTP_ADDR` set)
    /// 3. mDNS (default fallback)
    pub fn detect(self) -> Result<Box<dyn DiscoveryMechanism>> {
        // Check for kubernetes
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            #[cfg(feature = "kubernetes")]
            return Ok(Box::new(k8s::KubernetesDiscovery::new(&self)?));

            #[cfg(not(feature = "kubernetes"))]
            tracing::info!("Kubernetes detected but feature not enabled, falling back");
        }

        // Check for consul
        if std::env::var("CONSUL_HTTP_ADDR").is_ok() {
            #[cfg(feature = "consul")]
            return Ok(Box::new(consul::ConsulDiscovery::new(&self)?));

            #[cfg(not(feature = "consul"))]
            tracing::info!("Consul detected but feature not enabled, falling back");
        }

        // Default to mDNS
        Ok(Box::new(mdns::MdnsDiscovery::new(&self)?))
    }

    /// Build mDNS discovery (default)
    pub fn build_mdns(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(mdns::MdnsDiscovery::new(&self)?))
    }

    /// Build Consul discovery
    #[cfg(feature = "consul")]
    pub fn build_consul(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(consul::ConsulDiscovery::new(&self)?))
    }

    /// Build Kubernetes discovery
    #[cfg(feature = "kubernetes")]
    pub fn build_kubernetes(self) -> Result<Box<dyn DiscoveryMechanism>> {
        Ok(Box::new(k8s::KubernetesDiscovery::new(&self)?))
    }
}
