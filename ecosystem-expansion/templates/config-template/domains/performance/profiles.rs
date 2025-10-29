//! **OPTIMIZATION PROFILES CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{CpuPerformanceConfig, MemoryPerformanceConfig, IoPerformanceConfig};

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