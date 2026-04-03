// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **⚠️ DEPRECATED - USE CANONICAL INSTEAD**
//!
//! This module is deprecated. Use the canonical automation configuration types instead.
//!
//! **Migration**: Use `nestgate_core::config::canonical_primary::domains::automation::AutomationConfig` or
//! `crate::types::CanonicalAutomationConfig`

#![allow(deprecated)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

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
/// Configuration for Automation
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
    #[serde(rename = "_orchestration_endpoint")]
    pub orchestration_endpoint: Option<String>,
}
impl Default for AutomationConfig {
    /// Returns the default instance
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
            orchestration_endpoint: None,
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
            orchestration_endpoint: Some({
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
            orchestration_endpoint: Some({
                // ✅ MIGRATED: Now uses centralized runtime configuration
                use nestgate_core::config::runtime::get_config;
                get_config().network.api_base_url()
            }),
        }
    }
}

/// Analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Analysis
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
    /// Returns the default instance
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
/// Configuration for Prediction
pub struct PredictionConfig {
    /// Prediction window in days
    pub prediction_window_days: u32,
    /// Minimum confidence threshold
    pub min_confidence: f64,
    /// Model parameters
    pub model_params: HashMap<String, f64>,
}
impl Default for PredictionConfig {
    /// Returns the default instance
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
/// Configuration for Lifecycle
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
    /// Returns the default instance
    fn default() -> Self {
        Self {
            hot_retention_days: 30,
            warm_retention_days: 90,
            cold_retention_days: 365,
            auto_migration: true,
        }
    }
}

/// Canonical network configuration type.
///
/// Discovery and orchestration configuration is handled by
/// `nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig`.
/// The former `DiscoveryConfig` struct and `network-integration` feature have been removed
/// as orchestration concerns are delegated to the network capability provider.
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
        assert!(config.orchestration_endpoint.is_some());
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
    #[allow(deprecated)]
    fn test_automation_config_serialization() {
        let config = AutomationConfig::default();
        let serialized =
            serde_json::to_string(&config).expect("Test: config serialization should succeed");
        assert!(serialized.contains("optimization_interval_hours"));
    }
}
