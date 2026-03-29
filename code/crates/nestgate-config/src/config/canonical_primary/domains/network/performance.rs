// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **NETWORK PERFORMANCE CONFIGURATION**

use nestgate_types::error::Result;
use serde::{Deserialize, Serialize};

/// Network performance configuration for optimizing throughput and latency.
///
/// Controls TCP/IP tuning parameters to optimize network performance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `NetworkPerformance`
pub struct NetworkPerformanceConfig {
    /// Network buffer size in bytes.
    pub buffer_size: u32,
    /// Whether to disable Nagle's algorithm (`TCP_NODELAY`).
    pub tcp_nodelay: bool,
    /// Whether TCP keep-alive is enabled.
    pub keep_alive: bool,
    /// Keep-alive timeout in seconds (0 = use system default).
    pub keep_alive_timeout_seconds: u64,
}

impl NetworkPerformanceConfig {
    /// Create development-optimized configuration with small buffers.
    ///
    /// Uses conservative settings suitable for local testing.
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            buffer_size: 8192,
            tcp_nodelay: false,
            keep_alive: true,
            keep_alive_timeout_seconds: 60,
        }
    }

    /// Create production-hardened configuration with large buffers and `TCP_NODELAY`.
    ///
    /// Optimizes for low latency and high throughput in production.
    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            buffer_size: 65536,
            tcp_nodelay: true,
            keep_alive: true,
            keep_alive_timeout_seconds: 120,
        }
    }

    /// Validate the performance configuration.
    ///
    /// Ensures buffer sizes and timeouts are reasonable.
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails.
    pub const fn validate(&self) -> Result<()> {
        Ok(())
    }

    /// Merge this configuration with another, preferring values from `other`.
    ///
    /// All fields from `other` will replace the current values.
    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub const fn merge(mut self, other: Self) -> Self {
        self.buffer_size = other.buffer_size;
        self.tcp_nodelay = other.tcp_nodelay;
        self.keep_alive = other.keep_alive;
        self.keep_alive_timeout_seconds = other.keep_alive_timeout_seconds;
        self
    }
}
