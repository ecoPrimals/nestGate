// **PERFORMANCE ENVIRONMENT CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance environment configuration for environment-specific settings.
///
/// Provides environment-based overrides and feature flags for performance tuning.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PerformanceEnvironment
pub struct PerformanceEnvironmentConfig {
    /// Name of the deployment environment (e.g., "development", "production").
    pub environment: String,
    /// Environment-specific configuration overrides.
    pub overrides: HashMap<String, serde_json::Value>,
    /// Feature flags for enabling/disabling performance features.
    pub feature_flags: HashMap<String, bool>,
    /// Debug configuration for performance troubleshooting.
    pub debug: Option<PerformanceDebugConfig>,
}

/// Performance debug configuration for troubleshooting performance issues.
///
/// Controls debug logging and diagnostics for performance analysis.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for PerformanceDebug
pub struct PerformanceDebugConfig {
    /// Whether performance debugging is enabled.
    pub enabled: bool,
    /// Log level for performance debug messages.
    pub log_level: String,
}
