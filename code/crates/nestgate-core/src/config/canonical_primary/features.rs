// Features configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Features
pub struct FeaturesConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Enable Auto Scaling
    pub enable_auto_scaling: bool,
    /// Enable Load Balancing
    pub enable_load_balancing: bool,
    /// Enable Monitoring
    pub enable_monitoring: bool,
    /// Enable Metrics
    pub enable_metrics: bool,
    /// Enable Tracing
    pub enable_tracing: bool,
}

impl Default for FeaturesConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            enable_auto_scaling: false,
            enable_load_balancing: false,
            enable_monitoring: true,
            enable_metrics: true,
            enable_tracing: true,
        }
    }
}
