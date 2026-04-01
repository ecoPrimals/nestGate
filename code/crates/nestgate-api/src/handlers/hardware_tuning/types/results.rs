// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// **LIVE HARDWARE METRICS**
///
/// Real-time hardware performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Livehardwaremetrics
pub struct LiveHardwareMetrics {
    /// Current CPU utilization percentage
    pub cpu_usage: f64,
    /// Current memory utilization percentage
    pub memory_usage: f64,
    /// Current disk I/O rate
    pub disk_io: f64,
    /// Current network I/O rate
    pub network_io: f64,
    /// Current power consumption in watts
    pub power_consumption: f64,
    /// Current system temperature in Celsius
    pub temperature: f64,
    /// Current GPU utilization percentage
    pub gpu_usage: f64,
    /// Current disk usage percentage
    pub disk_usage: f64,
    /// Current network utilization percentage
    pub network_usage: f64,
    /// Timestamp when metrics were collected
    pub timestamp: DateTime<Utc>,
}

/// **TUNING RESULT**
///
/// Results from hardware tuning operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Tuningresult
pub struct TuningResult {
    /// Name of the tuning profile applied
    pub profile_name: String,
    /// List of optimizations that were applied
    pub optimizations_applied: Vec<String>,
    /// Estimated power consumption increase
    pub estimated_power_increase: f64,
    /// Measured performance improvement percentage
    pub performance_improvement: f64,
    /// Hardware metrics before tuning
    pub before_metrics: LiveHardwareMetrics,
    /// Hardware metrics after tuning
    pub after_metrics: LiveHardwareMetrics,
}

/// **BENCHMARK RESULT**
///
/// Results from hardware benchmark tests.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Benchmarkresult
pub struct BenchmarkResult {
    /// Type of benchmark that was run
    pub benchmark_type: String,
    /// Benchmark score achieved
    pub score: f64,
    /// Duration of the benchmark in milliseconds
    pub duration_ms: u64,
    /// Hardware metrics during benchmark
    pub metrics: LiveHardwareMetrics,
}

/// **PERFORMANCE SNAPSHOT**
///
/// Point-in-time performance snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancesnapshot
pub struct PerformanceSnapshot {
    /// Timestamp when snapshot was taken
    pub timestamp: DateTime<Utc>,
    /// CPU utilization at snapshot time
    pub cpu_usage: f64,
    /// Memory utilization at snapshot time
    pub memory_usage: f64,
    /// Disk I/O rate at snapshot time
    pub disk_io: f64,
    /// Network I/O rate at snapshot time
    pub network_io: f64,
}

/// **SYSTEM PROFILE**
///
/// System configuration profile for different workload types.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemprofile
pub struct SystemProfile {
    /// CPU tuning profile name
    pub cpu_profile: String,
    /// Memory tuning profile name
    pub memory_profile: String,
    /// Storage tuning profile name
    pub storage_profile: String,
    /// Network tuning profile name
    pub network_profile: String,
}
