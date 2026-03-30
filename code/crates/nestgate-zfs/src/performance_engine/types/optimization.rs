// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::time::SystemTime;

use super::bottlenecks::ZfsBottleneck;

/// Optimization state tracking
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum OptimizationState {
    /// No optimization in progress
    #[default]
    Idle,
    /// Collecting metrics
    Collecting,
    /// Analyzing performance data
    Analyzing,
    /// Applying optimizations
    Optimizing,
    /// Validating optimization results
    Validating,
    /// Optimization successfully applied
    Applied,
}

/// Performance optimization result
#[derive(Debug, Clone, Default)]
pub struct PerformanceOptimizationResult {
    /// List of optimizations that were applied
    pub applied_optimizations: Vec<AppliedOptimization>,
    /// Overall performance improvement percentage
    pub performance_improvement: f64,
    /// List of bottlenecks that were resolved
    pub bottlenecks_resolved: Vec<ZfsBottleneck>,
    /// Additional recommendations for further optimization
    pub recommendations: Vec<String>,
}
impl PerformanceOptimizationResult {
    /// Merge another optimization result into this one
    ///
    /// Combines optimizations, improvements, bottlenecks, and recommendations
    /// from multiple optimization runs.
    pub fn merge_with(&mut self, other: Self) {
        self.applied_optimizations
            .extend(other.applied_optimizations);
        self.performance_improvement += other.performance_improvement;
        self.bottlenecks_resolved.extend(other.bottlenecks_resolved);
        self.recommendations.extend(other.recommendations);
    }
}

/// Applied optimization tracking
#[derive(Debug, Clone)]
pub struct AppliedOptimization {
    /// Type of optimization applied
    pub optimization_type: OptimizationType,
    /// Human-readable description of the optimization
    pub description: String,
    /// Measured performance impact (percentage improvement)
    pub performance_impact: f64,
    /// Timestamp when optimization was applied
    pub applied_at: SystemTime,
}
/// Types of optimizations
#[derive(Debug, Clone)]
pub enum OptimizationType {
    /// Cache-related optimization
    CacheOptimization,
    /// Latency reduction optimization
    LatencyOptimization,
    /// Throughput improvement optimization
    ThroughputOptimization,
    /// Defragmentation operation
    FragmentationDefrag,
    /// ARC (cache) tuning
    ArcTuning,
    /// Record size optimization
    RecordSizeOptimization,
    /// Compression algorithm optimization
    CompressionOptimization,
}

/// ZFS tuning operation result
#[derive(Debug, Clone, Default)]
pub struct ZfsTuningResult {
    /// Whether tuning parameters were successfully applied
    pub tuning_applied: bool,
    /// Map of parameter names to their new values
    pub parameter_changes: std::collections::HashMap<String, String>,
    /// Expected performance improvement as percentage
    pub expected_improvement: f64,
    /// Whether validation testing is required after tuning
    pub validation_required: bool,
}
