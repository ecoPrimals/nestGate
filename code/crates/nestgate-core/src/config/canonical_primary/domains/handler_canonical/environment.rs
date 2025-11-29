// **ENVIRONMENT HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for HandlerEnvironment
pub struct HandlerEnvironmentConfig {
    /// Environment
    pub environment: String,
    /// Debug
    pub debug: Option<HandlerDebugConfig>,
    /// Features
    pub features: HandlerFeatureConfig,
    /// Overrides
    pub overrides: HandlerOverrideConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for HandlerDebug
pub struct HandlerDebugConfig {
    /// Debug Logging
    pub debug_logging: bool,
    /// Log Execution Times
    pub log_execution_times: bool,
    /// Log Parameters
    pub log_parameters: bool,
    /// Log Results
    pub log_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for HandlerFeature
pub struct HandlerFeatureConfig {
    /// Feature Flags
    pub feature_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for HandlerOverride
pub struct HandlerOverrideConfig {
    /// Overrides
    pub overrides: HashMap<String, serde_json::Value>,
}

impl Default for HandlerEnvironmentConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            debug: Some(HandlerDebugConfig::default()),
            features: HandlerFeatureConfig::default(),
            overrides: HandlerOverrideConfig::default(),
        }
    }
}

impl Default for HandlerDebugConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            debug_logging: true,
            log_execution_times: false,
            log_parameters: false,
            log_results: false,
        }
    }
}

impl HandlerEnvironmentConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
