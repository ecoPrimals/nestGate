// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **OPTIMIZATION PROFILES CONFIGURATION**

use super::{CpuPerformanceConfig, IoPerformanceConfig, MemoryPerformanceConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Optimization profiles for predefined performance configurations.
///
/// Allows switching between different performance profiles (e.g., "high-throughput", "low-latency").
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Optimizationprofiles
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
/// Optimizationprofile
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimization_profiles_serde_roundtrip() {
        let mut profiles = HashMap::new();
        profiles.insert(
            "p1".to_string(),
            OptimizationProfile {
                name: "p1".to_string(),
                description: "d".to_string(),
                cpu_override: None,
                memory_override: None,
                io_override: None,
            },
        );
        let o = OptimizationProfiles {
            active_profile: "p1".to_string(),
            profiles,
        };
        let s = serde_json::to_string(&o).expect("to_string");
        let _: OptimizationProfiles = serde_json::from_str(&s).expect("from_str");
    }
}
