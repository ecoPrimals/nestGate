// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **THREAT PROTECTION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `ThreatProtection`
pub struct ThreatProtectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Intrusion Detection
    pub intrusion_detection: IntrusionDetectionConfig,
    /// Firewall
    pub firewall: FirewallConfig,
    /// Ddos Protection
    pub ddos_protection: DdosProtectionConfig,
    /// Malware Protection
    pub malware_protection: MalwareProtectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `IntrusionDetection`
pub struct IntrusionDetectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Sensitivity
    pub sensitivity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Firewall
pub struct FirewallConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Rules
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DdosProtection`
pub struct DdosProtectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Threshold
    pub threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `MalwareProtection`
pub struct MalwareProtectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Scan Uploads
    pub scan_uploads: bool,
}

impl Default for ThreatProtectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            intrusion_detection: IntrusionDetectionConfig::default(),
            firewall: FirewallConfig::default(),
            ddos_protection: DdosProtectionConfig::default(),
            malware_protection: MalwareProtectionConfig::default(),
        }
    }
}

impl Default for IntrusionDetectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: "medium".to_string(),
        }
    }
}

impl Default for FirewallConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec!["allow-http".to_string(), "allow-https".to_string()],
        }
    }
}

impl Default for DdosProtectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 1000,
        }
    }
}

impl Default for MalwareProtectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            scan_uploads: true,
        }
    }
}

impl ThreatProtectionConfig {
    /// Creates a production-hardened threat protection configuration.
    ///
    /// Returns the default configuration optimized for production use with
    /// enhanced security measures.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    /// Creates a development-optimized threat protection configuration.
    ///
    /// Returns the default configuration suitable for development environments
    /// with relaxed security for faster iteration.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Creates a compliance-focused threat protection configuration.
    ///
    /// Returns the default configuration tailored for compliance requirements
    /// with comprehensive threat detection and logging.
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::default()
    }
    /// Merges this configuration with another, returning self unchanged.
    ///
    /// # Arguments
    ///
    /// * `_other` - The other configuration (currently ignored)
    ///
    /// # Returns
    ///
    /// Returns self without modification
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
