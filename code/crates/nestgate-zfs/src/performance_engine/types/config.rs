// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

/// Performance engine configuration
///
/// DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceEngineConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceEngineConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[derive(Debug, Clone)]
pub struct PerformanceEngineConfig {
    /// Interval between monitoring cycles
    pub monitoring_interval: std::time::Duration,
    /// Interval between optimization runs
    pub optimization_interval: std::time::Duration,
    /// Interval for bottleneck detection
    pub bottleneck_detection_interval: std::time::Duration,
    /// Maximum number of concurrent optimizations
    pub max_concurrent_optimizations: usize,
    /// Enable AI-powered guidance
    pub enable_ai_guidance: bool,
}
impl Default for PerformanceEngineConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: std::time::Duration::from_secs(5),
            optimization_interval: std::time::Duration::from_secs(30),
            bottleneck_detection_interval: std::time::Duration::from_secs(10),
            max_concurrent_optimizations: 3,
            enable_ai_guidance: true,
        }
    }
}

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
pub type PerformanceEngineConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
