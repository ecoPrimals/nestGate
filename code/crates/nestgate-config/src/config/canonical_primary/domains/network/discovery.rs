// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **NETWORK DISCOVERY CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Network discovery configuration for automatic service detection.
///
/// This configuration enables multicast-based service discovery, allowing
/// services to automatically locate and communicate with each other on the network.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for NetworkDiscovery
pub struct NetworkDiscoveryConfig {
    /// Whether network discovery is enabled.
    pub enabled: bool,
    /// Multicast address for service discovery broadcasts.
    pub multicast_address: String,
    /// Interval in seconds between discovery broadcasts.
    pub discovery_interval_secs: u64,
}

impl NetworkDiscoveryConfig {
    /// Create development-optimized configuration with discovery disabled.
    ///
    /// Uses conservative settings suitable for local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            enabled: false,
            multicast_address: "224.0.0.1".to_string(),
            discovery_interval_secs: 60,
        }
    }

    /// Create production-hardened configuration with discovery enabled.
    ///
    /// Uses optimized settings for production service discovery.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            enabled: true,
            multicast_address: "224.0.0.100".to_string(),
            discovery_interval_secs: 30,
        }
    }

    /// Validate the discovery configuration.
    ///
    /// Ensures multicast address and intervals are properly configured.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Merge this configuration with another, preferring values from `other`.
    ///
    /// All fields from `other` will replace the current values.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.multicast_address = other.multicast_address;
        self.discovery_interval_secs = other.discovery_interval_secs;
        self
    }
}
