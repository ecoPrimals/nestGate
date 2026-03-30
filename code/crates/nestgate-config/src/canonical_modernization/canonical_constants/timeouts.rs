// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

    use std::time::Duration;
    // Basic timeouts (seconds)
    /// Default request timeout in seconds
    pub const REQUEST_TIMEOUT_SECS: u64 = 30;
    /// Default connection timeout in seconds
    pub const CONNECTION_TIMEOUT_SECS: u64 = 10;
    /// Default health check timeout in seconds
    pub const HEALTH_CHECK_TIMEOUT_SECS: u64 = 5;
    /// Default operation timeout in seconds
    pub const OPERATION_TIMEOUT_SECS: u64 = 60;
    /// Default session timeout in seconds (1 hour)
    pub const SESSION_TIMEOUT_SECS: u64 = 3600; // 1 hour

    // Extended timeouts (seconds)
    /// Extended operation timeout in seconds (5 minutes)
    pub const EXTENDED_TIMEOUT_SECS: u64 = 300; // 5 minutes
    /// Very long operation timeout in seconds (1 hour)
    pub const VERY_LONG_TIMEOUT_SECS: u64 = 3600; // 1 hour
    /// Service discovery timeout in seconds
    pub const DISCOVERY_TIMEOUT_SECS: u64 = 30;
    /// Service operation timeout in seconds
    pub const SERVICE_TIMEOUT_SECS: u64 = 60;

    // Storage operation timeouts (seconds)
    /// ZFS pool creation timeout in seconds (5 minutes)
    pub const POOL_CREATION_TIMEOUT_SECS: u64 = 300; // 5 minutes
    /// Snapshot operation timeout in seconds (1 minute)
    pub const SNAPSHOT_TIMEOUT_SECS: u64 = 60; // 1 minute
    /// Backup operation timeout in seconds (1 hour)
    pub const BACKUP_TIMEOUT_SECS: u64 = 3600; // 1 hour
    /// Scrub operation timeout in seconds (24 hours)
    pub const SCRUB_TIMEOUT_SECS: u64 = 86400; // 24 hours

    // Monitoring timeouts (seconds)
    /// Metrics collection timeout in seconds
    pub const METRICS_TIMEOUT_SECS: u64 = 10;
    /// Alert processing timeout in seconds
    pub const ALERT_TIMEOUT_SECS: u64 = 30;

    // Test timeouts (seconds)
    /// Default test timeout in seconds
    pub const TEST_TIMEOUT_SECS: u64 = 10;
    /// Integration test timeout in seconds
    pub const INTEGRATION_TEST_TIMEOUT_SECS: u64 = 60;

    // Timeout limits
    /// Maximum allowed timeout in seconds (5 minutes)
    pub const MAX_TIMEOUT_SECS: u64 = 300; // 5 minutes
    /// Minimum timeout in seconds
    pub const MIN_TIMEOUT_SECS: u64 = 1;

    // Duration constants for convenience
    /// Request timeout as Duration constant
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(REQUEST_TIMEOUT_SECS);
    /// Connection timeout as Duration constant
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(CONNECTION_TIMEOUT_SECS);
    /// Health check timeout as Duration constant
    pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(HEALTH_CHECK_TIMEOUT_SECS);
    /// Operation timeout as Duration constant
    pub const OPERATION_TIMEOUT: Duration = Duration::from_secs(OPERATION_TIMEOUT_SECS);
    /// Extended timeout as Duration
    pub const EXTENDED_TIMEOUT: Duration = Duration::from_secs(EXTENDED_TIMEOUT_SECS);
    /// Very long timeout as Duration
    pub const VERY_LONG_TIMEOUT: Duration = Duration::from_secs(VERY_LONG_TIMEOUT_SECS);
    /// Pool creation timeout as Duration
    pub const POOL_CREATION_TIMEOUT: Duration = Duration::from_secs(POOL_CREATION_TIMEOUT_SECS);
    /// Snapshot timeout as Duration
    pub const SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(SNAPSHOT_TIMEOUT_SECS);
    /// Backup timeout as Duration
    pub const BACKUP_TIMEOUT: Duration = Duration::from_secs(BACKUP_TIMEOUT_SECS);
    /// Scrub timeout as Duration
    pub const SCRUB_TIMEOUT: Duration = Duration::from_secs(SCRUB_TIMEOUT_SECS);
    /// Metrics timeout as Duration
    pub const METRICS_TIMEOUT: Duration = Duration::from_secs(METRICS_TIMEOUT_SECS);
    /// Alert timeout as Duration
    pub const ALERT_TIMEOUT: Duration = Duration::from_secs(ALERT_TIMEOUT_SECS);
