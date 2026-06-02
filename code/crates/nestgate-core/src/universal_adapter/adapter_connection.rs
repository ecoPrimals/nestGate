// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Adapter constructors, capability discovery, and cached capability lookup.

use super::adapter_config::AdapterDiscoveryConfig;
use super::adapter_types::{CapabilityInfo, UniversalAdapter, UniversalAdapterConfig};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;

impl UniversalAdapter {
    /// Create new universal adapter instance
    ///
    /// This constructor loads discovery configuration from environment variables.
    /// For testing or custom configurations, use `with_discovery_config()`.
    #[must_use]
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            capabilities: HashMap::new(),
            discovery_cache: HashMap::new(),
            config: UniversalAdapterConfig::default(),
            discovery_config: Arc::new(AdapterDiscoveryConfig::from_env()),
        }
    }

    /// Create a new adapter with a specific discovery configuration
    ///
    /// This is the recommended constructor for testing and when you need
    /// explicit control over discovery endpoints.
    #[must_use]
    pub fn with_discovery_config(
        discovery_config: super::SharedDiscoveryConfig,
        endpoint: String,
    ) -> Self {
        Self {
            endpoint,
            capabilities: HashMap::new(),
            discovery_cache: HashMap::new(),
            config: UniversalAdapterConfig::default(),
            discovery_config,
        }
    }

    /// Discover all available capabilities (infant discovery pattern)
    /// This replaces hardcoded primal knowledge
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[expect(
        clippy::unused_async,
        reason = "cfg(test) awaits discover_capabilities; discovery steps are synchronous placeholders"
    )]
    pub async fn discover_capabilities(&mut self) -> Result<Vec<CapabilityInfo>, String> {
        // Clear existing capabilities for fresh discovery
        self.capabilities.clear();

        // Discover capabilities from all primals without hardcoding their names
        self.discover_orchestration_capabilities()?;
        self.discover_compute_capabilities()?;
        self.discover_security_capabilities()?;
        self.discover_ai_capabilities()?;
        self.discover_storage_capabilities()?;
        self.discover_ecosystem_capabilities()?;

        Ok(self.capabilities.values().cloned().collect())
    }

    /// Get capability by category (O(1) lookup)
    /// Replaces hardcoded primal connections
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_capability(&self, category: &str) -> Result<CapabilityInfo, String> {
        // Check cache first
        if let Some(cached) = self.discovery_cache.get(category)
            && SystemTime::now() < cached.expires_at
        {
            return Ok(cached.info.clone());
        }

        // Get from discovered capabilities
        self.capabilities
            .get(category)
            .cloned()
            .ok_or_else(|| format!("Capability '{category}' not found"))
    }

    /// Discover orchestration capabilities through dynamic discovery
    fn discover_orchestration_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self
            .discovery_config
            .get_discovery_endpoint("orchestration")
        {
            let capability = CapabilityInfo {
                category: String::from("orchestration"),
                provider: String::from("dynamic-orchestration"),
                endpoint: endpoint.to_string(),
                performance_tier: String::from("standard"),
                availability: 99.5,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert(String::from("orchestration"), capability);
        }
        Ok(())
    }

    /// Discover compute capabilities through dynamic discovery
    fn discover_compute_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self.discovery_config.get_discovery_endpoint("compute") {
            let capability = CapabilityInfo {
                category: String::from("compute"),
                provider: String::from("dynamic-compute"),
                endpoint: endpoint.to_string(),
                performance_tier: String::from("high_performance"),
                availability: 98.5,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert(String::from("compute"), capability);
        }
        Ok(())
    }

    /// Discover security capabilities through dynamic discovery
    fn discover_security_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self.discovery_config.get_discovery_endpoint("security") {
            let capability = CapabilityInfo {
                category: String::from("security"),
                provider: String::from("dynamic-security"),
                endpoint: endpoint.to_string(),
                performance_tier: String::from("enterprise"),
                availability: 99.9,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert(String::from("security"), capability);
        }
        Ok(())
    }

    /// Discover AI capabilities through dynamic discovery
    fn discover_ai_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self
            .discovery_config
            .get_discovery_endpoint("artificial_intelligence")
        {
            let capability = CapabilityInfo {
                category: String::from("artificial_intelligence"),
                provider: String::from("dynamic-ai"),
                endpoint: endpoint.to_string(),
                performance_tier: String::from("standard"),
                availability: 97.5,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert(String::from("artificial_intelligence"), capability);
        }
        Ok(())
    }

    /// Discover storage capabilities (`NestGate`'s self-knowledge)
    fn discover_storage_capabilities(&mut self) -> Result<(), String> {
        // NestGate knows its own storage capabilities
        let capability = CapabilityInfo {
            category: String::from("storage"),
            provider: String::from("nestgate-native"),
            endpoint: String::from("internal://nestgate/storage"),
            performance_tier: String::from("enterprise"),
            availability: 99.9,
            metadata: HashMap::new(),
            discovered_at: SystemTime::now(),
        };
        self.capabilities
            .insert(String::from("storage"), capability);
        Ok(())
    }

    /// Discover ecosystem capabilities through dynamic discovery
    fn discover_ecosystem_capabilities(&mut self) -> Result<(), String> {
        if let Some(endpoint) = self.discovery_config.get_discovery_endpoint("ecosystem") {
            let capability = CapabilityInfo {
                category: String::from("ecosystem"),
                provider: String::from("dynamic-ecosystem"),
                endpoint: endpoint.to_string(),
                performance_tier: String::from("standard"),
                availability: 99.0,
                metadata: HashMap::new(),
                discovered_at: SystemTime::now(),
            };
            self.capabilities
                .insert(String::from("ecosystem"), capability);
        }
        Ok(())
    }
}
