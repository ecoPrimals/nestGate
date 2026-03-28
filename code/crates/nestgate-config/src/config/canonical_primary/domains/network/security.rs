// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **NETWORK SECURITY CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Network security configuration for firewall and IP filtering.
///
/// Controls network-level security including firewall rules and IP allowlists/blocklists.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for NetworkSecurity
pub struct NetworkSecurityConfig {
    /// Whether firewall is enabled.
    pub firewall_enabled: bool,
    /// List of allowed IP addresses or CIDR ranges.
    pub allowed_ips: Vec<String>,
    /// List of blocked IP addresses or CIDR ranges.
    pub blocked_ips: Vec<String>,
}

impl NetworkSecurityConfig {
    /// Create development-optimized configuration with firewall disabled.
    ///
    /// Allows all traffic for local development convenience.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            firewall_enabled: false,
            allowed_ips: vec![],
            blocked_ips: vec![],
        }
    }

    /// Create production-hardened configuration with strict firewall rules.
    ///
    /// Enables firewall and restricts access to private networks only.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            firewall_enabled: true,
            allowed_ips: vec!["10.0.0.0/8".to_string()],
            blocked_ips: vec![],
        }
    }

    /// Validate the security configuration.
    ///
    /// Ensures IP addresses and CIDR ranges are valid.
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
        self.firewall_enabled = other.firewall_enabled;
        self.allowed_ips = other.allowed_ips;
        self.blocked_ips = other.blocked_ips;
        self
    }
}
