// **OPTIMIZATION PROFILES CONFIGURATION**

use super::{CpuPerformanceConfig, IoPerformanceConfig, MemoryPerformanceConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationProfiles {
    pub active_profile: String,
    pub profiles: HashMap<String, OptimizationProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationProfile {
    pub name: String,
    pub description: String,
    pub cpu_override: Option<CpuPerformanceConfig>,
    pub memory_override: Option<MemoryPerformanceConfig>,
    pub io_override: Option<IoPerformanceConfig>,
}
