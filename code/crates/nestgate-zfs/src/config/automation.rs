// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use serde::{Deserialize, Serialize};

/// Dataset automation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DatasetAutomationConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DatasetAutomationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `DatasetAutomation`
pub struct DatasetAutomationConfig {
    /// Enable dataset automation
    pub enabled: bool,
    /// Automation scan interval (seconds)
    pub scan_interval_seconds: u64,
    /// Learning period for new datasets (days)
    pub learning_period_days: u32,
    /// Default automation policy
    pub default_policy: String,
    /// AI integration settings
    pub ai_settings: AiAutomationSettings,
}
/// AI-powered automation settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Aiautomationsettings
pub struct AiAutomationSettings {
    /// Enable AI-powered optimizations
    pub ai_enabled: bool,
    /// Tier prediction model configuration
    pub model_config: String,
    /// Performance monitoring interval
    pub monitoring_interval_seconds: u64,
    /// AI confidence threshold for recommendations
    pub confidence_threshold: f64,
}
impl Default for DatasetAutomationConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            scan_interval_seconds: 300, // 5 minutes
            learning_period_days: 7,
            default_policy: "balanced_performance".to_string(),
            ai_settings: AiAutomationSettings {
                ai_enabled: false,
                model_config: "default".to_string(),
                monitoring_interval_seconds: 300, // 5 minutes
                confidence_threshold: 0.8,
            },
        }
    }
}

impl Default for AiAutomationSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            ai_enabled: false,
            model_config: "default".to_string(),
            monitoring_interval_seconds: 300, // 5 minutes
            confidence_threshold: 0.8,
        }
    }
}

impl AiAutomationSettings {
    /// Create production-optimized AI automation settings
    #[must_use]
    pub fn production() -> Self {
        Self {
            ai_enabled: true,
            model_config: "production-optimized".to_string(),
            monitoring_interval_seconds: 120, // 2 minutes
            confidence_threshold: 0.9,
        }
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Datasetautomationconfigcanonical
pub type DatasetAutomationConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DatasetAutomationConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    #![expect(deprecated)]

    use super::*;

    #[test]
    fn dataset_automation_config_default_and_serde() {
        let c = DatasetAutomationConfig::default();
        assert!(c.enabled);
        let j = serde_json::to_string(&c).expect("serialize");
        let d: DatasetAutomationConfig = serde_json::from_str(&j).expect("deserialize");
        assert_eq!(d.scan_interval_seconds, c.scan_interval_seconds);
    }

    #[test]
    fn ai_automation_settings_default_production_and_serde() {
        let a = AiAutomationSettings::default();
        let p = AiAutomationSettings::production();
        assert!(p.ai_enabled);
        assert!(p.confidence_threshold > a.confidence_threshold);
        let j = serde_json::to_string(&a).expect("serialize");
        let _: AiAutomationSettings = serde_json::from_str(&j).expect("deserialize");
    }
}
