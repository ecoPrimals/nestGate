// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **METRICS TYPES**
//!
//! Performance metrics reporting and query types.

use nestgate_core::diagnostics::SystemMetrics;
use serde::{Deserialize, Serialize};

/// Metrics Report Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsReportPayload {
    /// Node ID
    pub node_id: String,
    /// Metrics
    pub metrics: SystemMetrics,
}

/// Metrics Query Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsQueryPayload {
    /// Node ID
    pub node_id: Option<String>,
    /// Time range
    pub time_range: Option<TimeRange>,
    /// Metric type
    pub metric_type: Option<MetricType>,
}

/// Time Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    /// Start time
    pub start: std::time::SystemTime,
    /// End time
    pub end: std::time::SystemTime,
}

/// Metric Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    Cpu,
    Memory,
    Disk,
    Network,
    All,
}
