// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// This module contains all the data structures, enums, and types used by the
// performance optimization engine.

use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

use crate::types::StorageTier;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Performance engine configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct PerformanceEngineConfig {
    pub monitoring_interval: std::time::Duration,
    pub optimization_interval: std::time::Duration,
    pub bottleneck_detection_interval: std::time::Duration,
    pub max_concurrent_optimizations: usize,
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

/// Optimization state tracking
#[derive(Debug, Clone, PartialEq, Default)]
pub enum OptimizationState {
    #[default]
    Idle,
    Collecting,
    Analyzing,
    Optimizing,
    Validating,
    Applied,
}
/// ZFS performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPerformanceMetrics {
    pub timestamp: SystemTime,
    pub pool_metrics: HashMap<String, ZfsPoolMetrics>,
    pub dataset_metrics: HashMap<String, ZfsDatasetMetrics>,
    pub system_memory: SystemMemoryUsage,
    pub arc_stats: ArcStatistics,
}
/// ZFS pool metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsPoolMetrics {
    pub pool_name: String,
    pub read_ops: f64,
    pub write_ops: f64,
    pub read_bandwidth: f64,
    pub write_bandwidth: f64,
    pub latency: f64,
    pub cache_hit_ratio: f64,
    pub fragmentation: f64,
}
/// ZFS dataset metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsDatasetMetrics {
    pub dataset_name: String,
    pub access_pattern: AccessPattern,
    pub dedup_ratio: f64,
    pub record_size: u64,
}
/// Access pattern classification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessPattern {
    Sequential,
    Random,
    Mixed,
}
/// System memory usage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemoryUsage {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}
/// ARC statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArcStatistics {
    pub size: u64,
    pub target_size: u64,
    pub hit_ratio: f64,
    pub miss_ratio: f64,
}
/// ZFS bottleneck detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsBottleneck {
    pub bottleneck_type: ZfsBottleneckType,
    pub severity: BottleneckSeverity,
    pub pool_name: String,
    pub dataset_name: Option<String>,
