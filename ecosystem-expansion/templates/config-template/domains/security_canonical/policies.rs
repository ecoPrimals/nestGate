//! **SECURITY POLICIES CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityPoliciesConfig {
    pub compliance: ComplianceConfig,
    pub data_protection: DataProtectionConfig,
    pub retention: RetentionPolicyConfig,
    pub privacy: PrivacyConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub enabled: bool,
    pub frameworks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProtectionConfig {
    pub encryption_at_rest: bool,
    pub encryption_in_transit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicyConfig {
    pub enabled: bool,
    pub default_retention: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    pub anonymization: bool,
    pub data_minimization: bool,
}


impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            frameworks: vec!["SOC2".to_string(), "GDPR".to_string()],
        }
    }
}

impl Default for DataProtectionConfig {
    fn default() -> Self {
        Self {
            encryption_at_rest: true,
            encryption_in_transit: true,
        }
    }
}

impl Default for RetentionPolicyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_retention: Duration::from_secs(365 * 24 * 60 * 60), // 1 year
        }
    }
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            anonymization: true,
            data_minimization: true,
        }
    }
}

impl SecurityPoliciesConfig {
    pub fn production_hardened() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn compliance_focused() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 