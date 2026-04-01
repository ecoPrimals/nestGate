// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Trends and high-level component summaries.

use serde::{Deserialize, Serialize};

/// Performance trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancetrend
pub enum PerformanceTrend {
    /// Performance is improving
    Improving,
    /// Performance is stable
    Stable,
    /// Performance is degrading
    Degrading,
    /// Not enough data to determine trend
    Unknown,
}

/// Component performance analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Componentanalysis
pub struct ComponentAnalysis {
    /// Component name
    pub component_name: String,
    /// Current usage percentage
    pub current_usage: f64,
    /// Performance trend
    pub trend: PerformanceTrend,
    /// Detected anomalies
    pub anomalies: Vec<String>,
    /// Performance recommendations
    pub recommendations: Vec<String>,
}

/// Performance trends analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancetrends
pub struct PerformanceTrends {
    /// CPU usage trend
    pub cpu_trend: PerformanceTrend,
    /// Memory usage trend
    pub memory_trend: PerformanceTrend,
    /// Disk I/O trend
    pub disk_trend: PerformanceTrend,
    /// Network I/O trend
    pub network_trend: PerformanceTrend,
    /// ZFS performance trend
    pub zfs_trend: PerformanceTrend,
}
