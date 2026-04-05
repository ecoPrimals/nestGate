// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Real-time metrics collection and data aggregation for the performance dashboard.

//! Metrics Collector module

mod collector;
mod linux_proc;
mod state;
mod types;

#[cfg(test)]
mod tests;

pub use collector::RealTimeMetricsCollector;
pub use state::MetricsCollectorState;
pub use types::*;
