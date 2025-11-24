//! **⚠️ DEPRECATED - USE CANONICAL INSTEAD**
//!
//! This module is deprecated. Use the canonical automation configuration types instead.
//!
//! **Migration**: Use `nestgate_core::config::canonical_primary::domains::automation::AutomationConfig` or
//! `crate::types::CanonicalAutomationConfig`

#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

use nestgate_core::error::utilities::safe_env_var_or_default;

/// **AUTOMATION CONFIG TYPES**
/// Configuration type definitions for the automation system
/// Main automation configuration (DEPRECATED)
///
/// **Migration Path**: Use `crate::types::CanonicalAutomationConfig` instead.
#[deprecated(
    since = "0.2.0",
    note = "Use nestgate_core::config::canonical_primary::domains::automation::AutomationConfig"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    /// Analysis configuration
    pub analysis: AnalysisConfig,
    /// Prediction configuration  
    pub prediction: PredictionConfig,
    /// Lifecycle management configuration
    pub lifecycle: LifecycleConfig,
    /// Optimization interval in hours
    pub optimization_interval_hours: u32,
    /// Prediction cache TTL in hours
    pub prediction_cache_ttl_hours: u32,
    /// Enable intelligent tier assignment
    pub enable_intelligent_tier_assignment: bool,
    /// Enable automatic optimization
    pub enable_automatic_optimization: bool,
    /// Minimum confidence threshold for predictions
    pub min_confidence_threshold: f64,
    /// Orchestration endpoint
    pub _orchestration_endpoint: Option<String>,
}
impl Default for AutomationConfig {
    fn default() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            prediction: PredictionConfig::default(),
            lifecycle: LifecycleConfig::default(),
            optimization_interval_hours: 24,
            prediction_cache_ttl_hours: 12,
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: true,
            min_confidence_threshold: 0.7,
            _orchestration_endpoint: None,
        }
    }
}

impl AutomationConfig {
    /// Create production configuration
    #[must_use]
    pub fn production() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            prediction: PredictionConfig::default(),
            lifecycle: LifecycleConfig::default(),
            optimization_interval_hours: 6, // More frequent optimization for production
            prediction_cache_ttl_hours: 24, // Longer cache for production
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: true,
            min_confidence_threshold: 0.8, // Higher confidence for production
            _orchestration_endpoint: Some({
                // ✅ MIGRATED: Now uses centralized runtime configuration
                use nestgate_core::config::runtime::get_config;
                get_config().network.api_base_url()
            }),
        }
    }

    /// Create development configuration
    #[must_use]
    pub fn development() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            prediction: PredictionConfig::default(),
            lifecycle: LifecycleConfig::default(),
            optimization_interval_hours: 1, // More frequent optimization for development
            prediction_cache_ttl_hours: 2,  // Shorter cache for development
            enable_intelligent_tier_assignment: true,
            enable_automatic_optimization: false, // Disable auto-optimization in dev
            min_confidence_threshold: 0.5,        // Lower confidence for development
            _orchestration_endpoint: Some({
                // ✅ MIGRATED: Now uses centralized runtime configuration
                use nestgate_core::config::runtime::get_config;
                get_config().network.api_base_url()
            }),
        }
    }
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    /// Scan interval
    pub scan_interval: Duration,
    /// Maximum file size to analyze
    pub max_file_size: u64,
    /// File extensions to include
    pub include_extensions: Vec<String>,
    /// File extensions to exclude
    pub exclude_extensions: Vec<String>,
}
impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            scan_interval: Duration::from_secs(3600), // 1 hour
            max_file_size: 1024 * 1024 * 1024,        // 1GB
            include_extensions: vec!["*".to_string()],
            exclude_extensions: vec![".tmp".to_string(), ".log".to_string()],
        }
    }
}

/// Prediction configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionConfig {
    /// Prediction window in days
    pub prediction_window_days: u32,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Model parameters
    pub model_params: HashMap<String, f64>,
}
impl Default for PredictionConfig {
    fn default() -> Self {
        Self {
            prediction_window_days: 30,
            min_confidence: 0.7,
            model_params: HashMap::new(),
        }
    }
}

/// Lifecycle configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleConfig {
    /// Hot tier retention in days
    pub hot_retention_days: u32,
    /// Warm tier retention in days
    pub warm_retention_days: u32,
    /// Cold tier retention in days
    pub cold_retention_days: u32,
    /// Enable automatic migration
    pub auto_migration: bool,
}
impl Default for LifecycleConfig {
    fn default() -> Self {
        Self {
            hot_retention_days: 30,
            warm_retention_days: 90,
            cold_retention_days: 365,
            auto_migration: true,
        }
    }
}

/// Discovery configuration for ecosystem services
#[cfg(feature = "network-integration")]
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::DiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::DiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct DiscoveryConfig {
    pub known_orchestration_endpoints: Vec<String>,
    pub discovery_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub multicast_enabled: bool,
    pub mdns_enabled: bool,
}
#[cfg(feature = "network-integration")]
impl DiscoveryConfig {
    #[must_use]
    pub fn from_automation_config(config: &AutomationConfig) -> Self {
        Self {
            known_orchestration_endpoints: vec![
                // ✅ MIGRATED: Now uses centralized runtime configuration
                config._orchestration_endpoint.clone().unwrap_or_else(|| {
                    use nestgate_core::config::runtime::{get_config, service_url};
                    service_url("songbird")
                        .or_else(|| service_url("orchestration"))
                        .unwrap_or_else(|| get_config().network.api_base_url())
                }),
                std::env::var("NESTGATE_ORCHESTRATION_BACKUP_ENDPOINT_1").unwrap_or_else(|_| {
                    use nestgate_core::config::runtime::get_config;
                    let cfg = get_config();
                    format!(
                        "http://{}:{}",
                        cfg.network.api_host,
                        cfg.network.api_port + 1
                    )
                }),
                std::env::var("NESTGATE_ORCHESTRATION_BACKUP_ENDPOINT_2").unwrap_or_else(|_| {
                    use nestgate_core::config::runtime::get_config;
                    let cfg = get_config();
                    format!(
                        "http://{}:{}",
                        cfg.network.api_host,
                        cfg.network.api_port + 2
                    )
                }),
            ],
            discovery_timeout_ms: 5000,
            health_check_interval_ms: std::env::var("NESTGATE_HEALTH_CHECK_INTERVAL_MS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30000),
            multicast_enabled: true,
            mdns_enabled: true,
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
#[allow(deprecated)]
pub type DiscoveryConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using DiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_automation_config_default() {
        let config = AutomationConfig::default();
        assert_eq!(config.optimization_interval_hours, 24);
        assert_eq!(config.prediction_cache_ttl_hours, 12);
        assert!(config.enable_intelligent_tier_assignment);
        assert!(config.enable_automatic_optimization);
        assert_eq!(config.min_confidence_threshold, 0.7);
    }

    #[test]
    #[allow(deprecated)]
    fn test_automation_config_production() {
        let config = AutomationConfig::production();
        assert_eq!(config.optimization_interval_hours, 6);
        assert_eq!(config.prediction_cache_ttl_hours, 24);
        assert_eq!(config.min_confidence_threshold, 0.8);
        assert!(config._orchestration_endpoint.is_some());
    }

    #[test]
    #[allow(deprecated)]
    fn test_automation_config_development() {
        let config = AutomationConfig::development();
        assert_eq!(config.optimization_interval_hours, 1);
        assert_eq!(config.prediction_cache_ttl_hours, 2);
        assert!(!config.enable_automatic_optimization);
        assert_eq!(config.min_confidence_threshold, 0.5);
    }

    #[test]
    fn test_analysis_config_default() {
        let config = AnalysisConfig::default();
        assert_eq!(config.max_file_size, 1024 * 1024 * 1024);
        assert_eq!(config.include_extensions, vec!["*"]);
        assert!(config.exclude_extensions.contains(&".tmp".to_string()));
    }

    #[test]
    fn test_prediction_config_default() {
        let config = PredictionConfig::default();
        assert_eq!(config.prediction_window_days, 30);
        assert_eq!(config.min_confidence, 0.7);
        assert!(config.model_params.is_empty());
    }

    #[test]
    fn test_lifecycle_config_default() {
        let config = LifecycleConfig::default();
        assert_eq!(config.hot_retention_days, 30);
        assert_eq!(config.warm_retention_days, 90);
        assert_eq!(config.cold_retention_days, 365);
        assert!(config.auto_migration);
    }

    #[test]
    #[allow(deprecated)]
    fn test_automation_config_clone() {
        let config1 = AutomationConfig::default();
        let config2 = config1.clone();
        assert_eq!(
            config1.optimization_interval_hours,
            config2.optimization_interval_hours
        );
    }

    #[test]
    fn test_analysis_config_custom() {
        let config = AnalysisConfig {
            scan_interval: Duration::from_secs(1800),
            max_file_size: 500 * 1024 * 1024,
            include_extensions: vec!["*.txt".to_string(), "*.md".to_string()],
            exclude_extensions: vec![],
        };
        assert_eq!(config.max_file_size, 524288000);
        assert_eq!(config.include_extensions.len(), 2);
    }

    #[test]
    fn test_prediction_config_with_params() {
        let mut model_params = HashMap::new();
        model_params.insert("learning_rate".to_string(), 0.01);
        model_params.insert("epochs".to_string(), 100.0);

        let config = PredictionConfig {
            prediction_window_days: 60,
            min_confidence: 0.9,
            model_params,
        };
        assert_eq!(config.prediction_window_days, 60);
        assert_eq!(config.model_params.len(), 2);
    }

    #[test]
    fn test_lifecycle_config_custom() {
        let config = LifecycleConfig {
            hot_retention_days: 7,
            warm_retention_days: 30,
            cold_retention_days: 180,
            auto_migration: false,
        };
        assert_eq!(config.hot_retention_days, 7);
        assert!(!config.auto_migration);
    }

    #[test]
    #[cfg(feature = "network-integration")]
    #[allow(deprecated)]
    fn test_discovery_config_from_automation() {
        let automation_config = AutomationConfig::default();
        let discovery = DiscoveryConfig::from_automation_config(&automation_config);
        assert_eq!(discovery.discovery_timeout_ms, 5000);
        assert!(discovery.multicast_enabled);
        assert!(discovery.mdns_enabled);
    }

    #[test]
    #[allow(deprecated)]
    fn test_automation_config_serialization() {
        let config = AutomationConfig::default();
        let serialized =
            serde_json::to_string(&config).expect("Test: config serialization should succeed");
        assert!(serialized.contains("optimization_interval_hours"));
    }
}
