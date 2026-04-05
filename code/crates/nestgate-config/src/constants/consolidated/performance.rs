// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Connection pools, timeouts, retries, buffers, and concurrency limits.

use std::sync::{Arc, OnceLock};

use super::defaults::env_or_parse;

/// Performance tuning constants
#[derive(Debug, Clone)]
/// Performanceconstants
pub struct PerformanceConstants {
    // Connections
    max_connections: usize,
    connection_pool_size: usize,

    // Timeouts (milliseconds)
    connection_timeout_ms: u64,
    request_timeout_ms: u64,
    idle_timeout_ms: u64,
    keepalive_interval_ms: u64,

    // Retries
    max_retry_attempts: u32,
    retry_delay_ms: u64,
    retry_backoff_multiplier: f32,

    // Buffer sizes
    network_buffer_size: usize,
    disk_buffer_size: usize,
    memory_pool_size: usize,

    // Concurrency
    worker_threads: usize,
    async_tasks_limit: usize,
}

impl Default for PerformanceConstants {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            // Connections
            max_connections: env_or_parse("NESTGATE_MAX_CONNECTIONS", 1000),
            connection_pool_size: env_or_parse("NESTGATE_POOL_SIZE", 100),

            // Timeouts
            connection_timeout_ms: env_or_parse("NESTGATE_CONN_TIMEOUT_MS", 5000),
            request_timeout_ms: env_or_parse("NESTGATE_REQ_TIMEOUT_MS", 30000),
            idle_timeout_ms: env_or_parse("NESTGATE_IDLE_TIMEOUT_MS", 300_000),
            keepalive_interval_ms: env_or_parse("NESTGATE_KEEPALIVE_MS", 60000),

            // Retries
            max_retry_attempts: env_or_parse("NESTGATE_MAX_RETRIES", 3),
            retry_delay_ms: env_or_parse("NESTGATE_RETRY_DELAY_MS", 1000),
            retry_backoff_multiplier: env_or_parse("NESTGATE_RETRY_BACKOFF", 2.0),

            // Buffers
            network_buffer_size: env_or_parse("NESTGATE_NET_BUFFER", 8192),
            disk_buffer_size: env_or_parse("NESTGATE_DISK_BUFFER", 4096),
            memory_pool_size: env_or_parse("NESTGATE_MEM_POOL", 1024 * 1024),

            // Concurrency
            worker_threads: env_or_parse(
                "NESTGATE_WORKERS",
                std::thread::available_parallelism().map_or(4, std::num::NonZero::get),
            ),
            async_tasks_limit: env_or_parse("NESTGATE_ASYNC_LIMIT", 10000),
        }
    }
}

impl PerformanceConstants {
    /// Get or initialize the global performance constants
    pub fn get() -> Arc<Self> {
        static INSTANCE: OnceLock<Arc<PerformanceConstants>> = OnceLock::new();
        INSTANCE.get_or_init(|| Arc::new(Self::default())).clone()
    }

    // Connection getters

    /// Returns the maximum number of concurrent connections allowed
    #[must_use]
    pub const fn max_connections(&self) -> usize {
        self.max_connections
    }
    /// Connection Pool Size
    #[must_use]
    pub const fn connection_pool_size(&self) -> usize {
        self.connection_pool_size
    }

    // Timeout getters (in Duration)

    /// Returns the connection timeout duration
    #[must_use]
    pub const fn connection_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.connection_timeout_ms)
    }
    /// Request Timeout
    #[must_use]
    pub const fn request_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.request_timeout_ms)
    }
    /// Idle Timeout
    #[must_use]
    pub const fn idle_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.idle_timeout_ms)
    }
    /// Keepalive Interval
    #[must_use]
    pub const fn keepalive_interval(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.keepalive_interval_ms)
    }

    // Retry getters

    /// Returns the maximum number of retry attempts for failed operations
    #[must_use]
    pub const fn max_retry_attempts(&self) -> u32 {
        self.max_retry_attempts
    }
    /// Retry Delay
    #[must_use]
    pub const fn retry_delay(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.retry_delay_ms)
    }
    /// Retry Backoff Multiplier
    #[must_use]
    pub const fn retry_backoff_multiplier(&self) -> f32 {
        self.retry_backoff_multiplier
    }

    // Buffer getters

    /// Returns the network buffer size in bytes
    #[must_use]
    pub const fn network_buffer_size(&self) -> usize {
        self.network_buffer_size
    }
    /// Disk Buffer Size
    #[must_use]
    pub const fn disk_buffer_size(&self) -> usize {
        self.disk_buffer_size
    }
    /// Memory Pool Size
    #[must_use]
    pub const fn memory_pool_size(&self) -> usize {
        self.memory_pool_size
    }

    // Concurrency getters

    /// Returns the number of worker threads for the async runtime
    #[must_use]
    pub const fn worker_threads(&self) -> usize {
        self.worker_threads
    }
    /// Async Tasks Limit
    #[must_use]
    pub const fn async_tasks_limit(&self) -> usize {
        self.async_tasks_limit
    }
}
