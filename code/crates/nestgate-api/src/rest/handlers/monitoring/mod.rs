// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Pure data layer handlers for system monitoring and performance metrics.
// These handlers provide clean access to monitoring data without any
// authentication or user management overhead.

//! Monitoring module — health checks, metrics, system info, Prometheus helpers.

mod health;
mod metrics;
mod prometheus;
mod query;

pub use health::get_alerts;
pub use metrics::{get_metrics, get_metrics_history};
pub use prometheus::calculate_real_zfs_cache_hit_ratio;
pub use query::MetricsHistoryQuery;

#[cfg(test)]
mod tests;
