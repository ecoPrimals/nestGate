// **ENVIRONMENT HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerEnvironmentConfig {
    pub environment: String,
    pub debug: Option<HandlerDebugConfig>,
    pub features: HandlerFeatureConfig,
    pub overrides: HandlerOverrideConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandlerDebugConfig {
    pub debug_logging: bool,
    pub log_execution_times: bool,
    pub log_parameters: bool,
    pub log_results: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HandlerFeatureConfig {
    pub feature_flags: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HandlerOverrideConfig {
    pub overrides: HashMap<String, serde_json::Value>,
}

impl Default for HandlerEnvironmentConfig {
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
    pub const fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
