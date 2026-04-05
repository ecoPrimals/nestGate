// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Storage limits, rate limiting, and health check configuration.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Capacity and operational limits for a storage backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLimitsConfig {
    /// Maximum storage capacity in bytes
    pub max_capacity: Option<u64>,
    /// Maximum individual file size in bytes
    pub max_file_size: Option<u64>,
    /// Maximum number of files
    pub max_files: Option<u64>,
    /// Rate limiting
    pub rate_limits: RateLimitsConfig,
}

impl Default for StorageLimitsConfig {
    fn default() -> Self {
        Self {
            max_capacity: None,
            max_file_size: Some(1_073_741_824), // 1 GiB
            max_files: None,
            rate_limits: RateLimitsConfig::default(),
        }
    }
}

/// Rate limiting thresholds for storage operations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RateLimitsConfig {
    /// Maximum reads per second
    pub reads_per_second: Option<u32>,
    /// Maximum writes per second
    pub writes_per_second: Option<u32>,
    /// Bandwidth limit (bytes per second)
    pub bandwidth_limit: Option<u64>,
}

/// Health check configuration for monitoring backend availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageHealthCheckConfig {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Health check endpoint
    pub endpoint: Option<String>,
    /// Consecutive failures before marking unhealthy
    pub failure_threshold: u32,
    /// Consecutive successes before marking healthy
    pub recovery_threshold: u32,
}

impl Default for StorageHealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            endpoint: None,
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}
