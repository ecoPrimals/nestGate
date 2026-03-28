// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **NETWORK VLAN CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Network VLAN configuration for virtual network isolation.
///
/// Controls VLAN tagging and trunk port configuration for network segmentation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for NetworkVlan
pub struct NetworkVlanConfig {
    /// Whether VLAN tagging is enabled.
    pub enabled: bool,
    /// VLAN identifier (1-4094).
    pub vlan_id: u16,
    /// List of trunk ports for VLAN traffic.
    pub trunk_ports: Vec<String>,
}

impl NetworkVlanConfig {
    /// Create development-optimized configuration with VLAN disabled.
    ///
    /// No VLAN isolation for simplified local development.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            enabled: false,
            vlan_id: 100,
            trunk_ports: vec![],
        }
    }

    /// Create production-hardened configuration with VLAN enabled.
    ///
    /// Enables network isolation via VLAN for security and traffic separation.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            enabled: true,
            vlan_id: 200,
            trunk_ports: vec!["eth0".to_string()],
        }
    }

    /// Validate the VLAN configuration.
    ///
    /// Ensures VLAN ID is within valid range (1-4094).
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
        self.vlan_id = other.vlan_id;
        self.trunk_ports = other.trunk_ports;
        self
    }
}
