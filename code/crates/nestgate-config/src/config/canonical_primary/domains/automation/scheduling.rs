// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **SCHEDULING CONFIGURATION**
///
/// Task scheduling and execution settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Scheduling
pub struct SchedulingConfig {
    /// Enable scheduling
    pub enabled: bool,

    /// Schedule check interval
    pub check_interval: Duration,

    /// Enable cron-style scheduling
    pub cron_enabled: bool,

    /// Maintenance windows (cron format)
    pub maintenance_windows: Vec<String>,

    /// Enable distributed scheduling
    pub distributed: bool,

    /// Maximum scheduled tasks
    pub max_scheduled_tasks: usize,
}

impl Default for SchedulingConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self::development()
    }
}

impl SchedulingConfig {
    /// Creates a development-optimized configuration for task scheduling
    ///
    /// Returns a `SchedulingConfig` with scheduling disabled and simple configuration
    /// suitable for development environments.
    #[must_use]
    pub const fn development() -> Self {
        Self {
            enabled: false,
            check_interval: Duration::from_secs(60),
            cron_enabled: false,
            maintenance_windows: vec![],
            distributed: false,
            max_scheduled_tasks: 50,
        }
    }

    /// Creates a production-hardened configuration for task scheduling
    ///
    /// Returns a `SchedulingConfig` with scheduling enabled, cron support, distributed
    /// scheduling, and maintenance windows configured for production workloads.
    #[must_use]
    pub fn production() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            cron_enabled: true,
            maintenance_windows: vec![
                "0 2 * * *".to_string(),  // 2 AM daily
                "0 14 * * 0".to_string(), // 2 PM Sundays
            ],
            distributed: true,
            max_scheduled_tasks: 500,
        }
    }
}
