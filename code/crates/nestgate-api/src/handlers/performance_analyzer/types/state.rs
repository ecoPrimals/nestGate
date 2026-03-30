// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Analyzer runtime state.

use chrono::{DateTime, Utc};

use super::config::PerformanceAnalysisConfig;

/// Performance analyzer state
#[derive(Debug, Clone, Default)]
/// Performanceanalyzerstate
pub struct PerformanceAnalyzerState {
    /// Whether analysis is currently running
    pub is_running: bool,
    /// Last analysis timestamp
    pub last_analysis: Option<DateTime<Utc>>,
    /// Total analyses performed
    pub total_analyses: u64,
    /// Configuration
    pub config: PerformanceAnalysisConfig,
}
