// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Alert thresholds and metrics map aliases for the real-time monitor.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use super::super::types::{ZfsDatasetMetrics, ZfsPerformanceMetrics, ZfsPoolMetrics};

// **CANONICAL MODERNIZATION**: Type aliases to fix clippy complexity warnings
/// Shared map type for pool metrics keyed by pool name.
pub type PoolMetricsMap = Arc<RwLock<HashMap<String, ZfsPoolMetrics>>>;
/// Type alias for `DatasetMetricsMap`
pub type DatasetMetricsMap = Arc<RwLock<HashMap<String, ZfsDatasetMetrics>>>;
/// Type alias for `MetricsCacheMap`
pub type MetricsCacheMap = Arc<RwLock<HashMap<String, ZfsPerformanceMetrics>>>;

// Placeholder type until AlertThresholds is available
#[derive(Debug, Default)]
/// Alertthresholds
pub struct AlertThresholds {
    /// Cpu Threshold
    pub cpu_threshold: f32,
    /// Memory Threshold
    pub memory_threshold: f32,
    /// Disk Threshold
    pub disk_threshold: f32,
}

/// Type alias for `AlertThresholdsArc`
pub type AlertThresholdsArc = Arc<RwLock<AlertThresholds>>;
