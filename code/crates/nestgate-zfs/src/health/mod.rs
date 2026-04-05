// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// This module will be fully implemented in Week 2

//! Health module — types, command-output reporting, and the ZFS health monitor.

mod monitoring;
mod reporting;
mod types;

pub use monitoring::ZfsHealthMonitor;
pub use types::{
    Alert, AlertLevel, BackgroundTasks, HealthDataMap, HealthReport, HealthStatus, HealthStatusMap,
    MonitoringTasks,
};

#[cfg(test)]
mod tests;
