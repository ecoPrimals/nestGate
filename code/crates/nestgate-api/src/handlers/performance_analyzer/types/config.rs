// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Deprecated analysis configuration and canonical alias.

use serde::{Deserialize, Serialize};

/// Configuration for performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::PerformanceAnalysisConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::PerformanceAnalysisConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for `PerformanceAnalysis`
pub struct PerformanceAnalysisConfig {
    /// Enable CPU monitoring
    pub enable_cpu_monitoring: bool,
    /// Enable memory monitoring
    pub enable_memory_monitoring: bool,
    /// Enable disk monitoring
    pub enable_disk_monitoring: bool,
    /// Enable network monitoring
    pub enable_network_monitoring: bool,
    /// Enable ZFS monitoring
    pub enable_zfs_monitoring: bool,
    /// Analysis interval in seconds
    pub analysis_interval_seconds: u64,
    /// Maximum history entries to keep
    pub max_history_entries: usize,
}

impl Default for PerformanceAnalysisConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enable_cpu_monitoring: true,
            enable_memory_monitoring: true,
            enable_disk_monitoring: true,
            enable_network_monitoring: true,
            enable_zfs_monitoring: true,
            analysis_interval_seconds: 30,
            max_history_entries: 1000,
        }
    }
}

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Performanceanalysisconfigcanonical
pub type PerformanceAnalysisConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
