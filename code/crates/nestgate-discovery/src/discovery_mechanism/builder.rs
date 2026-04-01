// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Discovery mechanism builder
//!
//! Provides a builder pattern for creating discovery mechanisms with
//! configuration options like timeouts, caching, and auto-detection.

use super::DiscoveryMechanism;
#[cfg(feature = "mdns")]
use super::mdns;
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
    /// 1. Kubernetes (if `KUBERNETES_SERVICE_HOST` set and the `kubernetes` feature is enabled)
    /// 2. Consul (if `NESTGATE_CONSUL_URL`, `CONSUL_HTTP_ADDR`, or `NESTGATE_CONSUL_HTTP_ADDR` set and the `consul` feature is enabled)
    /// 3. mDNS (fallback when the `mdns` feature is enabled)
    ///
    /// If no backend matches the environment **and** no optional backend feature is enabled that
    /// could satisfy detection, returns a configuration error from `nestgate_types::error`.
    pub fn detect(self) -> Result<Box<dyn DiscoveryMechanism>> {
        // Check for kubernetes
        if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
            #[cfg(feature = "kubernetes")]
            return Ok(Box::new(k8s::KubernetesDiscovery::new(&self)?));

            #[cfg(not(feature = "kubernetes"))]
            tracing::info!("Kubernetes detected but feature not enabled, falling back");
        }

        // Check for consul (capability-style URL or HashiCorp-compatible env)
        if std::env::var("NESTGATE_CONSUL_URL").is_ok()
            || std::env::var("CONSUL_HTTP_ADDR").is_ok()
            || std::env::var("NESTGATE_CONSUL_HTTP_ADDR").is_ok()
        {
            #[cfg(feature = "consul")]
            return Ok(Box::new(consul::ConsulDiscovery::new(&self)?));

            #[cfg(not(feature = "consul"))]
            tracing::info!("Consul detected but feature not enabled, falling back");
        }

        // Default to mDNS when the feature is enabled (standalone / dev)
        #[cfg(feature = "mdns")]
        {
            Ok(Box::new(mdns::MdnsDiscovery::new(&self)?))
        }
        #[cfg(not(feature = "mdns"))]
        {
            Err(nestgate_types::error::NestGateError::configuration_error(
                "discovery_backend",
                "No discovery backend available: enable the `mdns`, `consul`, or `kubernetes` crate feature, or configure environment for an enabled backend",
            ))
        }
    }

    /// Build mDNS discovery (requires the `mdns` feature)
    #[cfg(feature = "mdns")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::time::Duration;

    #[test]
    fn default_and_new_match_expected_timeouts() -> Result<()> {
        let d = DiscoveryBuilder::default();
        assert_eq!(d.timeout, Duration::from_secs(5));
        assert_eq!(d.cache_duration, Duration::from_secs(60));
        assert!(d.preferred_mechanism.is_none());
        let n = DiscoveryBuilder::new();
        assert_eq!(n.timeout, d.timeout);
        assert_eq!(n.cache_duration, d.cache_duration);
        Ok(())
    }

    #[test]
    fn builder_chain_sets_timeout_cache_and_preference() -> Result<()> {
        let b = DiscoveryBuilder::new()
            .with_timeout(Duration::from_secs(12))
            .with_cache_duration(Duration::from_secs(180))
            .prefer_mechanism("mdns");
        assert_eq!(b.timeout, Duration::from_secs(12));
        assert_eq!(b.cache_duration, Duration::from_secs(180));
        assert_eq!(b.preferred_mechanism.as_deref(), Some("mdns"));
        Ok(())
    }

    #[cfg(feature = "mdns")]
    #[test]
    fn build_mdns_produces_mechanism() -> Result<()> {
        let mech = DiscoveryBuilder::new().build_mdns()?;
        drop(mech);
        Ok(())
    }

    #[test]
    fn prefer_mechanism_accepts_into_string() -> Result<()> {
        let b = DiscoveryBuilder::new().prefer_mechanism(String::from("consul"));
        assert_eq!(b.preferred_mechanism.as_deref(), Some("consul"));
        Ok(())
    }
}
