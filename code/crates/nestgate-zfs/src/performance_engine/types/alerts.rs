// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use super::optimization::PerformanceOptimizationResult;

/// Performance alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Type of performance alert
    pub alert_type: AlertType,
    /// Severity level of the alert
    pub severity: AlertSeverity,
    /// Name of the affected ZFS pool
    pub pool_name: String,
    /// Name of the affected dataset (if applicable)
    pub dataset_name: Option<String>,
    /// Human-readable alert description
    pub description: String,
    /// When the alert was triggered
    pub timestamp: SystemTime,
}
/// Alert types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertType {
    /// Performance has degraded below acceptable thresholds
    PerformanceDegradation,
    /// A performance bottleneck has been identified
    BottleneckDetected,
    /// A performance threshold has been exceeded
    ThresholdExceeded,
    /// An optimization attempt has failed
    OptimizationFailed,
}
/// Alert severity levels for performance monitoring
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlertSeverity {
    /// Informational alert - no action required
    Info,
    /// Warning alert - attention recommended
    Warning,
    /// Error alert - action required
    Error,
    /// Critical alert - immediate action required
    Critical,
}
/// Alert response
#[derive(Debug, Clone, Default)]
pub struct AlertResponse {
    /// Whether mitigation was applied successfully
    pub mitigation_applied: bool,
    /// Result of optimization if applied
    pub optimization_result: Option<PerformanceOptimizationResult>,
    /// Whether follow-up action is required
    pub follow_up_required: bool,
}
