//
// Configuration structures and types used across zero-cost service examples.

use serde::{Deserialize, Serialize};
use crate::zero_cost::ZeroCostServiceHealth;

/// **Configuration for the example service**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleConfigManagerConfig {
    /// Configuration storage path
    /// Maximum number of configurations to cache
    pub max_cached_configs: usize,
    /// Auto-save interval in seconds
    pub auto_save_interval_s: u64,
    /// Enable configuration validation
    pub validate_configs: bool,
}
impl Default for ExampleConfigManagerConfig {
    fn default() -> Self {
        Self {
            max_cached_configs: 100,
            auto_save_interval_s: 300, // 5 minutes
            validate_configs: true,
        }
    }
}

/// **Health status for configuration manager**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleConfigManagerHealth {
    /// Number of configurations currently loaded
    pub loaded_configs: usize,
    /// Number of configurations cached in memory
    pub cached_configs: usize,
    /// Last successful save timestamp
    pub last_save: Option<std::time::SystemTime>,
    /// Configuration validation status
    pub validation_status: String,
    /// Service uptime in seconds
    pub uptime_seconds: u64,
}
impl From<ExampleConfigManagerHealth> for ZeroCostServiceHealth {
    fn from(health: ExampleConfigManagerHealth) -> Self {
        Self {
            status: if health.cached_configs > 0 { "healthy".to_string() } else { "degraded".to_string() },
            uptime: std::time::Duration::from_secs(health.uptime_seconds),
            last_check: std::time::SystemTime::now(),
            details: serde_json::json!({
                "loaded_configs": health.loaded_configs,
                "cached_configs": health.cached_configs,
                "last_save": health.last_save,
                "validation_status": health.validation_status
            }),
        }
    }
} 