// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::StorageTier;

use super::bottlenecks::ZfsBottleneck;
use super::metrics::{AccessPattern, ZfsPerformanceMetrics};

/// Workload pattern analysis for storage optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkloadPattern {
    /// Detected I/O access pattern (sequential, random, or mixed)
    pub access_pattern: AccessPattern,
    /// Distribution of I/O request sizes as histogram
    pub io_size_distribution: HashMap<String, f64>,
    /// Ratio of read operations to write operations
    pub read_write_ratio: f64,
    /// Temporal locality score (0.0 to 1.0, higher = better cache performance)
    pub temporal_locality: f64,
}
/// ZFS configuration context for optimization decisions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfigurationContext {
    /// Name of the ZFS pool being configured
    pub pool_name: String,
    /// Optional dataset name for dataset-specific configuration
    pub dataset_name: Option<String>,
    /// Current ZFS configuration parameters (key-value pairs)
    pub current_configuration: HashMap<String, String>,
    /// Detected workload pattern for this storage
    pub workload_pattern: WorkloadPattern,
    /// System hardware and resource capabilities
    pub system_capabilities: SystemCapabilities,
}
/// ZFS expertise context for intelligent optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsExpertiseContext {
    /// Storage tier classification (hot, warm, cold, archive)
    pub storage_tier: StorageTier,
    /// Historical access patterns observed
    pub access_patterns: Vec<AccessPattern>,
    /// Current performance metrics snapshot
    pub current_performance: ZfsPerformanceMetrics,
    /// List of identified performance bottlenecks
    pub identified_bottlenecks: Vec<ZfsBottleneck>,
}
/// System hardware capabilities and resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemCapabilities {
    /// Number of CPU cores available
    pub cpu_cores: u32,
    /// Total system memory in gigabytes
    pub memory_gb: u32,
    /// Type of storage hardware (e.g., "`NVMe`", "SSD", "HDD")
    pub storage_type: String,
    /// Network bandwidth capacity in gigabits per second
    pub network_bandwidth_gbps: f64,
}
