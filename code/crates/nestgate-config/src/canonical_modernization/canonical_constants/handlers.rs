// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Default handler timeout in seconds (30 seconds)
    pub const DEFAULT_HANDLER_TIMEOUT_SECS: u64 = 30;

    /// Maximum number of concurrent requests per handler
    pub const MAX_CONCURRENT_REQUESTS: usize = 100;

    /// Default rate limit in requests per minute (60 requests/min)
    pub const DEFAULT_RATE_LIMIT_RPM: u32 = 60;

    /// Default rate limit burst size (10 requests)
    pub const DEFAULT_RATE_LIMIT_BURST: u32 = 10;

    /// Default retry attempts for failed operations
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;

    /// Default retry delay in milliseconds
    pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;

    /// Default metrics collection interval in seconds
    pub const METRICS_COLLECTION_INTERVAL_SECS: u64 = 30;

    /// Default health check interval in seconds
    pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 60;

    /// Dashboard refresh interval in seconds
    pub const DASHBOARD_REFRESH_INTERVAL_SECS: u64 = 5;

    /// Performance monitoring interval in seconds
    pub const PERFORMANCE_MONITOR_INTERVAL_SECS: u64 = 10;

    /// Default workspace size limit in bytes (10GB)
    pub const DEFAULT_WORKSPACE_SIZE_LIMIT: u64 = 10 * 1024 * 1024 * 1024; // 10GB

    /// Default JWT token expiration in seconds (24 hours)
    pub const DEFAULT_JWT_EXPIRATION_SECS: u64 = 24 * 60 * 60; // 24 hours
