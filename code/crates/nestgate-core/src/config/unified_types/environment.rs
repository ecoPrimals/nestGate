//! Environment and system configuration types.

use serde::{Deserialize, Serialize};

/// Environment types for configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Development,
    Staging,
    Production,
    Testing,
}

/// System configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub service_name: String,
    pub version: String,
    pub log_level: String,
    pub debug_mode: bool,
    pub metrics_enabled: bool,
    pub tracing_enabled: bool,
    pub performance_monitoring: bool,
    pub health_checks: bool,
}

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            service_name: "nestgate".to_string(),
            version: "1.0.0".to_string(),
            log_level: "info".to_string(),
            debug_mode: false,
            metrics_enabled: true,
            tracing_enabled: true,
            performance_monitoring: true,
            health_checks: true,
        }
    }
} 