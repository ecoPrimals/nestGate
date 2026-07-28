// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Types for performance analytics HTTP handlers.

use std::collections::HashMap;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

/// State management for performance analysis operations.
#[derive(Debug, Clone, Default)]
pub struct PerformanceAnalyzerState {
    /// Current analysis configuration
    pub config: AnalysisConfig,
    /// Last analysis timestamp
    pub last_analysis: Option<SystemTime>,
}

/// Configuration for performance analysis.
#[derive(Debug, Clone, Default)]
pub struct AnalysisConfig {
    /// Analysis interval in seconds
    pub interval_seconds: u64,
    /// Whether to enable predictive analysis
    pub predictive_enabled: bool,
}

/// Response structure for performance metrics data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsResponse {
    /// Current system metrics
    pub metrics: HashMap<String, f64>,
    /// Timestamp when metrics were collected
    pub timestamp: SystemTime,
}

/// Performance alert information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAlert {
    /// Alert identifier
    pub id: String,
    /// Alert message
    pub message: String,
    /// Alert severity level
    pub severity: String,
    /// Timestamp when alert was generated
    pub timestamp: SystemTime,
}

/// Performance optimization recommendation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {
    /// Recommendation identifier
    pub id: String,
    /// Recommendation title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// Expected impact
    pub impact: String,
    /// Priority level
    pub priority: u32,
}
