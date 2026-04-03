// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Capability registry and endpoint-backed discovery for the universal adapter.

use super::super::discovery_config::{DiscoveryRuntimeConfig, SharedDiscoveryRuntimeConfig};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, warn};

/// Capability discovery service for universal adapter
#[derive(Debug, Clone)]
/// Capabilitydiscovery
pub struct CapabilityDiscovery {
    pub(crate) registry: HashMap<String, Vec<String>>,
    pub(crate) discovery_endpoints: Vec<String>,
    runtime_config: SharedDiscoveryRuntimeConfig,
}

impl CapabilityDiscovery {
    /// Create a new capability discovery instance with runtime config
    pub fn with_runtime_config(
        runtime_config: SharedDiscoveryRuntimeConfig,
    ) -> crate::Result<Self> {
        let discovery_endpoints = runtime_config.get_discovery_endpoints();

        let mut discovery = Self {
            registry: HashMap::new(),
            discovery_endpoints,
            runtime_config,
        };

        // Initialize with default capability mappings
        discovery.initialize_default_capabilities();

        Ok(discovery)
    }

    /// Create a new capability discovery instance (backward compatibility)
    /// NOTE: Creates config from env each time. For tests, use with_runtime_config() directly.
    pub fn new() -> crate::Result<Self> {
        Self::with_runtime_config(Arc::new(DiscoveryRuntimeConfig::from_env()))
    }

    /// Find capabilities by type
    pub fn find_capabilities(&self, capability_type: &str) -> crate::Result<Vec<String>> {
        debug!("Finding capabilities for type: {}", capability_type);

        // Check local registry first - avoid clone by using Arc
        if let Some(services) = self.registry.get(capability_type) {
            // Return a new Vec with the same data to maintain API compatibility
            return Ok(services.clone());
        }

        // Query discovery endpoints for dynamic capabilities
        for endpoint in &self.discovery_endpoints {
            let services = self.query_discovery_endpoint(endpoint, capability_type);
            if !services.is_empty() {
                return Ok(services);
            }
        }

        // Return empty if no capabilities found
        warn!("No capabilities found for type: {}", capability_type);
        Ok(Vec::new())
    }

    /// Initialize default capability mappings with environment-driven endpoints
    fn initialize_default_capabilities(&mut self) {
        let base_endpoint = self.runtime_config.get_base_endpoint();

        // Security capabilities
        self.registry.insert(
            "security".to_string(),
            self.runtime_config.get_security_endpoint(&base_endpoint),
        );

        // AI capabilities
        self.registry.insert(
            "ai".to_string(),
            self.runtime_config.get_ai_endpoint(&base_endpoint),
        );

        // Orchestration capabilities
        self.registry.insert(
            "orchestration".to_string(),
            self.runtime_config
                .get_orchestration_endpoint(&base_endpoint),
        );

        // Storage/ZFS capabilities
        self.registry.insert(
            "storage".to_string(),
            self.runtime_config.get_storage_endpoint(&base_endpoint),
        );

        // Compute capabilities
        self.registry.insert(
            "compute".to_string(),
            self.runtime_config.get_compute_endpoint(&base_endpoint),
        );
    }

    /// Query a discovery endpoint for capabilities
    fn query_discovery_endpoint(&self, endpoint: &str, capability_type: &str) -> Vec<String> {
        debug!(
            "Querying discovery endpoint: {} for {}",
            endpoint, capability_type
        );

        // URL-based convention: in production this would issue JSON-RPC
        // discovery requests; for now, derive sub-paths from the base endpoint.
        match capability_type {
            "security" => vec![format!("{endpoint}/security")],
            "ai" => vec![format!("{endpoint}/ai")],
            "orchestration" => vec![format!("{endpoint}/orchestration")],
            "storage" => vec![format!("{endpoint}/storage")],
            "compute" => vec![format!("{endpoint}/compute")],
            _ => Vec::new(),
        }
    }

    /// Register a new capability service
    pub fn register_capability(&mut self, capability_type: String, service_url: String) {
        self.registry
            .entry(capability_type)
            .or_default()
            .push(service_url);
    }

    /// Remove a capability service
    pub fn unregister_capability(&mut self, capability_type: &str, service_url: &str) {
        if let Some(services) = self.registry.get_mut(capability_type) {
            services.retain(|url| url != service_url);
        }
    }
}

impl Default for CapabilityDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        // CapabilityDiscovery::new() cannot actually fail - it just initializes data structures
        // If it ever returns an error, we use empty defaults as fallback
        Self::new().unwrap_or_else(|_| Self {
            registry: HashMap::new(),
            discovery_endpoints: Vec::new(),
            runtime_config: Arc::new(DiscoveryRuntimeConfig::new()),
        })
    }
}
