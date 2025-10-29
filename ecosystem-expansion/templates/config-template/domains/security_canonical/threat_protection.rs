//! **THREAT PROTECTION CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatProtectionConfig {
    pub enabled: bool,
    pub intrusion_detection: IntrusionDetectionConfig,
    pub firewall: FirewallConfig,
    pub ddos_protection: DdosProtectionConfig,
    pub malware_protection: MalwareProtectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetectionConfig {
    pub enabled: bool,
    pub sensitivity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallConfig {
    pub enabled: bool,
    pub rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DdosProtectionConfig {
    pub enabled: bool,
    pub threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalwareProtectionConfig {
    pub enabled: bool,
    pub scan_uploads: bool,
}

impl Default for ThreatProtectionConfig {
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
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: "medium".to_string(),
        }
    }
}

impl Default for FirewallConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: vec!["allow-http".to_string(), "allow-https".to_string()],
        }
    }
}

impl Default for DdosProtectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 1000,
        }
    }
}

impl Default for MalwareProtectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            scan_uploads: true,
        }
    }
}

impl ThreatProtectionConfig {
    pub fn production_hardened() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn compliance_focused() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 