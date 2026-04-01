// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Default metrics collection interval in seconds (60 seconds / 1 minute)
    pub const METRICS_INTERVAL_SECS: u64 = 60;

    /// Health check interval in seconds
    pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 30;

    /// Alert thresholds
    ///
    /// CPU usage alert threshold (80%)
    pub const CPU_ALERT_THRESHOLD: f64 = 80.0;
    /// Memory usage alert threshold (85%)
    pub const MEMORY_ALERT_THRESHOLD: f64 = 85.0;
    /// Disk usage alert threshold (90%)
    pub const DISK_ALERT_THRESHOLD: f64 = 90.0;

    /// Retention periods (days)
    ///
    /// Metrics retention period (30 days)
    pub const METRICS_RETENTION_DAYS: u32 = 30;
    /// Logs retention period (7 days)
    pub const LOGS_RETENTION_DAYS: u32 = 7;
    /// Alerts retention period (90 days)
    pub const ALERTS_RETENTION_DAYS: u32 = 90;
