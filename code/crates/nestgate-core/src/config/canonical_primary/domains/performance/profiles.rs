// **OPTIMIZATION PROFILES CONFIGURATION**

use super::{CpuPerformanceConfig, IoPerformanceConfig, MemoryPerformanceConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Optimization profiles for predefined performance configurations.
///
/// Allows switching between different performance profiles (e.g., "high-throughput", "low-latency").
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationProfiles {
    /// Name of the currently active profile.
    pub active_profile: String,
    /// Map of profile names to their configurations.
    pub profiles: HashMap<String, OptimizationProfile>,
}

/// Individual optimization profile with performance overrides.
///
/// Defines a named set of performance settings that can be activated together.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OptimizationProfile {
    /// Name of the optimization profile.
    pub name: String,
    /// Human-readable description of the profile.
    pub description: String,
    /// CPU performance overrides for this profile.
    pub cpu_override: Option<CpuPerformanceConfig>,
    /// Memory performance overrides for this profile.
    pub memory_override: Option<MemoryPerformanceConfig>,
    /// I/O performance overrides for this profile.
    pub io_override: Option<IoPerformanceConfig>,
}
