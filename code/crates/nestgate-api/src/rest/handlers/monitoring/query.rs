// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Query types for monitoring HTTP handlers.

use serde::Deserialize;

/// Query parameters for historical metrics
#[derive(Debug, Deserialize)]
/// Metricshistoryquery
pub struct MetricsHistoryQuery {
    /// Start time (ISO 8601)
    pub start: Option<String>,
    /// End time (ISO 8601)
    pub end: Option<String>,
    /// Interval (e.g., "5m", "1h", "1d")
    pub interval: Option<String>,
    /// Specific metrics to include
    pub metrics: Option<Vec<String>>,
}
