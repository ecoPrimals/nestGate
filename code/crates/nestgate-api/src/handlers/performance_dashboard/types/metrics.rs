// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **PERFORMANCE METRICS TYPES**
//!
//! Types for system performance metrics and real-time data collection.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **REAL TIME METRICS**
///
/// Real-time performance metrics collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Realtimemetrics
pub struct RealTimeMetrics {
    /// Timestamp when metrics were collected
    pub timestamp: SystemTime,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage percentage
    pub memory_usage: f64,
    /// Disk I/O rate
    pub disk_io: f64,
    /// Network throughput
    pub network_throughput: f64,
    /// Active connections count
    pub active_connections: u32,
    /// Response time in milliseconds
    pub response_time_ms: f64,
}

/// **POOL PERFORMANCE TRENDS**
///
/// Performance trends for storage pools.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Poolperformancetrends
pub struct PoolPerformanceTrends {
    /// Pool identifier
    pub pool_id: String,
    /// Pool name
    pub pool_name: String,
    /// Performance trend data
    pub trend_data: Vec<PoolTrendPoint>,
    /// Overall trend direction
    pub overall_trend: String,
    /// Performance score
    pub performance_score: f64,
}

/// **POOL TREND POINT**
///
/// Individual data point in pool performance trend.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Pooltrendpoint
pub struct PoolTrendPoint {
    /// Timestamp
    pub timestamp: SystemTime,
    /// Read IOPS
    pub read_iops: f64,
    /// Write IOPS
    pub write_iops: f64,
    /// Read throughput
    pub read_throughput: f64,
    /// Write throughput
    pub write_throughput: f64,
    /// Latency
    pub latency: f64,
}

/// **SYSTEM METRICS**
///
/// System-level performance metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemmetrics
pub struct SystemMetrics {
    /// CPU utilization percentage
    pub cpu_utilization: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Total memory in bytes
    pub total_memory_bytes: u64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    /// Network I/O in bytes per second
    pub network_io_bps: u64,
    /// Load average
    pub load_average: [f64; 3],
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

/// **SYSTEM PERFORMANCE SNAPSHOT**
///
/// Snapshot of system performance at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemperformancesnapshot
pub struct SystemPerformanceSnapshot {
    /// Timestamp when snapshot was taken
    pub timestamp: SystemTime,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// Disk usage percentage
    pub disk_usage_percent: f64,
    /// Network throughput in bytes per second
    pub network_throughput_bps: u64,
    /// Active connections count
    pub active_connections: u32,
    /// Response time in milliseconds
    pub response_time_ms: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
}

/// **COMPREHENSIVE METRICS POINT**
///
/// Comprehensive metrics data point for time series analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Comprehensivemetricspoint
pub struct ComprehensiveMetricsPoint {
    /// Timestamp
    pub timestamp: SystemTime,
    /// CPU metrics
    pub cpu: f64,
    /// Memory metrics
    pub memory: f64,
    /// Disk metrics
    pub disk: f64,
    /// Network metrics
    pub network: f64,
    /// Additional metrics
    pub additional_metrics: HashMap<String, f64>,
}
