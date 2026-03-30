// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **HARDWARE TUNING CONFIG**
///
/// Configuration for hardware tuning operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `HardwareTuning`
pub struct HardwareTuningConfig {
    /// Number of CPU cores to allocate
    pub cpu_cores: u32,
    /// Amount of memory in gigabytes to allocate
    pub memory_gb: u32,
    /// Whether CPU tuning is enabled
    pub cpu_tuning_enabled: bool,
    /// Whether memory optimization is enabled
    pub memory_optimization_enabled: bool,
    /// Whether GPU tuning is enabled
    pub gpu_tuning_enabled: bool,
    /// Monitoring interval for performance metrics
    pub monitoring_interval: Duration,
}

impl Default for HardwareTuningConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_cores: 8,
            memory_gb: 16,
            cpu_tuning_enabled: true,
            memory_optimization_enabled: true,
            gpu_tuning_enabled: false, // Disabled by default, enabled if GPU detected
            monitoring_interval: Duration::from_secs(5),
        }
    }
}
