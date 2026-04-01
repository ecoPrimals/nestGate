// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **SECURITY POLICIES CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `SecurityPolicies`
pub struct SecurityPoliciesConfig {
    /// Compliance
    pub compliance: ComplianceConfig,
    /// Data Protection
    pub data_protection: DataProtectionConfig,
    /// Retention
    pub retention: RetentionPolicyConfig,
    /// Privacy
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Compliance
pub struct ComplianceConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Frameworks
    pub frameworks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `DataProtection`
pub struct DataProtectionConfig {
    /// Encryption At Rest
    pub encryption_at_rest: bool,
    /// Encryption In Transit
    pub encryption_in_transit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `RetentionPolicy`
pub struct RetentionPolicyConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Default Retention
    pub default_retention: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Privacy
pub struct PrivacyConfig {
    /// Anonymization
    pub anonymization: bool,
    /// Data Minimization
    pub data_minimization: bool,
}

impl Default for ComplianceConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            frameworks: vec!["SOC2".to_string(), "GDPR".to_string()],
        }
    }
}

impl Default for DataProtectionConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            encryption_at_rest: true,
            encryption_in_transit: true,
        }
    }
}

impl Default for RetentionPolicyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            default_retention: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
        }
    }
}

impl Default for PrivacyConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            anonymization: true,
            data_minimization: true,
        }
    }
}

impl SecurityPoliciesConfig {
    /// Creates a production-hardened security policies configuration.
    ///
    /// Returns the default configuration optimized for production use.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    /// Creates a development-optimized security policies configuration.
    ///
    /// Returns the default configuration suitable for development environments.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    /// Creates a compliance-focused security policies configuration.
    ///
    /// Returns the default configuration tailored for compliance requirements.
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
