/// **OPTIMIZATION MODULE**
/// Performance optimization configuration - extracted from monolithic config
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Optimization settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSettings {
    /// Enable optimization
    pub enabled: bool,
    /// CPU optimization
    pub cpu_optimization: bool,
    /// Memory optimization
    pub memory_optimization: bool,
    /// I/O optimization
    pub io_optimization: bool,
    /// Optimization interval
    pub optimization_interval: Duration,
}

impl SmartDefault for OptimizationSettings {
    fn smart_default() -> Self {
        Self {
            enabled: true,
            cpu_optimization: true,
            memory_optimization: true,
            io_optimization: true,
            optimization_interval: Duration::from_secs(600),
        }
    }
}

impl Default for OptimizationSettings {
    fn default() -> Self {
        Self::smart_default()
    }
}
