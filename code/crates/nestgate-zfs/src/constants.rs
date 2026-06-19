// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS-Specific Constants
//!
//! This module provides ZFS-specific constants that are not yet integrated
//! into the canonical constants system, as well as commonly used byte size
//! multipliers for consistency across the codebase.
//!
//! Storage tier capacity limits, failover timeouts, and AI confidence thresholds
//! are compile-time defaults in [`defaults`] and can be overridden at runtime via
//! `NESTGATE_ZFS_*` environment variables (see accessor functions below).

use std::sync::LazyLock;

// ==================== SIZE MULTIPLIERS ====================

/// Bytes per kilobyte (1 KB = 1024 bytes)
pub const BYTES_PER_KB: u64 = 1024;

/// Bytes per megabyte (1 MB = 1024 KB)
pub const BYTES_PER_MB: u64 = 1024 * 1024;

/// Bytes per gigabyte (1 GB = 1024 MB)
pub const BYTES_PER_GB: u64 = 1024 * 1024 * 1024;

/// Bytes per terabyte (1 TB = 1024 GB)
pub const BYTES_PER_TB: u64 = 1024 * 1024 * 1024 * 1024;

/// Bytes per petabyte (1 PB = 1024 TB)
pub const BYTES_PER_PB: u64 = 1024 * 1024 * 1024 * 1024 * 1024;

/// Kilobytes per megabyte
pub const KB_PER_MB: f64 = 1024.0;

/// Megabytes per gigabyte
pub const MB_PER_GB: f64 = 1024.0;

// ==================== ZFS RECORD SIZES ====================

/// ZFS compression setting for disabled compression
pub const COMPRESSION_OFF: &str = "off";

/// ZFS record size: 64KB (good for databases, small files)
pub const RECORDSIZE_64K: &str = "64K";

/// ZFS record size: 128KB (default, balanced performance)
pub const RECORDSIZE_128K: &str = "128K";

/// ZFS record size: 1MB (good for large files, sequential access)
pub const RECORDSIZE_1M: &str = "1M";

// ==================== COMPILE-TIME DEFAULTS ====================

/// Compile-time default values for env-overridable ZFS settings.
pub mod defaults {
    /// Hot tier default maximum size in GB
    pub const HOT_TIER_MAX_SIZE_GB: u64 = 1000;

    /// Hot tier default minimum free space in GB
    pub const HOT_TIER_MIN_FREE_GB: u64 = 100;

    /// Hot tier default warning threshold percentage
    pub const HOT_TIER_WARNING_THRESHOLD: f64 = 80.0;

    /// Warm tier default maximum size in GB
    pub const WARM_TIER_MAX_SIZE_GB: u64 = 10000;

    /// Warm tier default minimum free space in GB
    pub const WARM_TIER_MIN_FREE_GB: u64 = 1000;

    /// Warm tier default warning threshold percentage
    pub const WARM_TIER_WARNING_THRESHOLD: f64 = 85.0;

    /// Cold tier default maximum size in GB
    pub const COLD_TIER_MAX_SIZE_GB: u64 = 100000;

    /// Cold tier default minimum free space in GB
    pub const COLD_TIER_MIN_FREE_GB: u64 = 10000;

    /// Cold tier default warning threshold percentage
    pub const COLD_TIER_WARNING_THRESHOLD: f64 = 90.0;

    /// Default node failure timeout in seconds (3 minutes)
    pub const NODE_FAILURE_TIMEOUT_SECS: u64 = 180;

    /// Default AI confidence cache timeout in minutes (2 hours)
    pub const AI_CONFIDENCE_CACHE_TIMEOUT_MINS: u64 = 120;

    /// Default AI tier evaluation timeout in minutes (3 hours)
    pub const AI_TIER_EVALUATION_TIMEOUT_MINS: u64 = 180;

    /// Development HTTP port fallback when `NESTGATE_DEV_PORT` is unset
    pub const DEV_HTTP_FALLBACK: u16 = 8080;
}

/// Returns the value from env var `var` if set and parseable, else `default`.
fn env_or_default<T: std::str::FromStr>(var: &str, default: T) -> T {
    std::env::var(var)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

static HOT_TIER_MAX_SIZE_GB: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_HOT_TIER_MAX_GB",
        defaults::HOT_TIER_MAX_SIZE_GB,
    )
});

static HOT_TIER_MIN_FREE_GB: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_HOT_TIER_MIN_FREE_GB",
        defaults::HOT_TIER_MIN_FREE_GB,
    )
});

static HOT_TIER_WARNING_THRESHOLD: LazyLock<f64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_HOT_TIER_WARNING_THRESHOLD",
        defaults::HOT_TIER_WARNING_THRESHOLD,
    )
});

static WARM_TIER_MAX_SIZE_GB: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_WARM_TIER_MAX_GB",
        defaults::WARM_TIER_MAX_SIZE_GB,
    )
});

static WARM_TIER_MIN_FREE_GB: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_WARM_TIER_MIN_FREE_GB",
        defaults::WARM_TIER_MIN_FREE_GB,
    )
});

static WARM_TIER_WARNING_THRESHOLD: LazyLock<f64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_WARM_TIER_WARNING_THRESHOLD",
        defaults::WARM_TIER_WARNING_THRESHOLD,
    )
});

static COLD_TIER_MAX_SIZE_GB: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_COLD_TIER_MAX_GB",
        defaults::COLD_TIER_MAX_SIZE_GB,
    )
});

static COLD_TIER_MIN_FREE_GB: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_COLD_TIER_MIN_FREE_GB",
        defaults::COLD_TIER_MIN_FREE_GB,
    )
});

static COLD_TIER_WARNING_THRESHOLD: LazyLock<f64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_COLD_TIER_WARNING_THRESHOLD",
        defaults::COLD_TIER_WARNING_THRESHOLD,
    )
});

static NODE_FAILURE_TIMEOUT_SECS: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_NODE_FAILURE_TIMEOUT_SECS",
        defaults::NODE_FAILURE_TIMEOUT_SECS,
    )
});

static AI_CONFIDENCE_CACHE_TIMEOUT_MINS: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_AI_CONFIDENCE_CACHE_TIMEOUT_MINS",
        defaults::AI_CONFIDENCE_CACHE_TIMEOUT_MINS,
    )
});

static AI_TIER_EVALUATION_TIMEOUT_MINS: LazyLock<u64> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_AI_TIER_EVALUATION_TIMEOUT_MINS",
        defaults::AI_TIER_EVALUATION_TIMEOUT_MINS,
    )
});

static DEV_HTTP_FALLBACK: LazyLock<u16> = LazyLock::new(|| {
    env_or_default(
        "NESTGATE_ZFS_DEV_HTTP_FALLBACK",
        defaults::DEV_HTTP_FALLBACK,
    )
});

/// Hot tier maximum size in GB (`NESTGATE_ZFS_HOT_TIER_MAX_GB`).
#[must_use]
pub fn hot_tier_max_size_gb() -> u64 {
    *HOT_TIER_MAX_SIZE_GB
}

/// Hot tier minimum free space in GB (`NESTGATE_ZFS_HOT_TIER_MIN_FREE_GB`).
#[must_use]
pub fn hot_tier_min_free_gb() -> u64 {
    *HOT_TIER_MIN_FREE_GB
}

/// Hot tier warning threshold percentage (`NESTGATE_ZFS_HOT_TIER_WARNING_THRESHOLD`).
#[must_use]
pub fn hot_tier_warning_threshold() -> f64 {
    *HOT_TIER_WARNING_THRESHOLD
}

/// Warm tier maximum size in GB (`NESTGATE_ZFS_WARM_TIER_MAX_GB`).
#[must_use]
pub fn warm_tier_max_size_gb() -> u64 {
    *WARM_TIER_MAX_SIZE_GB
}

/// Warm tier minimum free space in GB (`NESTGATE_ZFS_WARM_TIER_MIN_FREE_GB`).
#[must_use]
pub fn warm_tier_min_free_gb() -> u64 {
    *WARM_TIER_MIN_FREE_GB
}

/// Warm tier warning threshold percentage (`NESTGATE_ZFS_WARM_TIER_WARNING_THRESHOLD`).
#[must_use]
pub fn warm_tier_warning_threshold() -> f64 {
    *WARM_TIER_WARNING_THRESHOLD
}

/// Cold tier maximum size in GB (`NESTGATE_ZFS_COLD_TIER_MAX_GB`).
#[must_use]
pub fn cold_tier_max_size_gb() -> u64 {
    *COLD_TIER_MAX_SIZE_GB
}

/// Cold tier minimum free space in GB (`NESTGATE_ZFS_COLD_TIER_MIN_FREE_GB`).
#[must_use]
pub fn cold_tier_min_free_gb() -> u64 {
    *COLD_TIER_MIN_FREE_GB
}

/// Cold tier warning threshold percentage (`NESTGATE_ZFS_COLD_TIER_WARNING_THRESHOLD`).
#[must_use]
pub fn cold_tier_warning_threshold() -> f64 {
    *COLD_TIER_WARNING_THRESHOLD
}

/// Node failure timeout in seconds (`NESTGATE_ZFS_NODE_FAILURE_TIMEOUT_SECS`).
#[must_use]
pub fn node_failure_timeout_secs() -> u64 {
    *NODE_FAILURE_TIMEOUT_SECS
}

/// AI confidence cache timeout in minutes (`NESTGATE_ZFS_AI_CONFIDENCE_CACHE_TIMEOUT_MINS`).
#[must_use]
pub fn ai_confidence_cache_timeout_mins() -> u64 {
    *AI_CONFIDENCE_CACHE_TIMEOUT_MINS
}

/// AI tier evaluation timeout in minutes (`NESTGATE_ZFS_AI_TIER_EVALUATION_TIMEOUT_MINS`).
#[must_use]
pub fn ai_tier_evaluation_timeout_mins() -> u64 {
    *AI_TIER_EVALUATION_TIMEOUT_MINS
}

/// Development HTTP port fallback (`NESTGATE_ZFS_DEV_HTTP_FALLBACK`).
#[must_use]
pub fn dev_http_fallback() -> u16 {
    *DEV_HTTP_FALLBACK
}

// ==================== TEST CONSTANTS ====================

/// Test dataset size: 100 MB in bytes
pub const TEST_DATASET_100MB: u64 = 100 * BYTES_PER_MB;

/// Test dataset size: 1 GB in bytes
pub const TEST_DATASET_1GB: u64 = BYTES_PER_GB;

/// Test transfer rate: 1 MB/s in bytes/sec
pub const TEST_TRANSFER_RATE_1MB_S: f64 = BYTES_PER_MB as f64;

/// Test transfer rate: 10 MB/s in bytes/sec
pub const TEST_TRANSFER_RATE_10MB_S: f64 = 10.0 * BYTES_PER_MB as f64;

// ==================== RE-EXPORTS ====================

// Re-export commonly used constants from canonical system
pub use nestgate_core::canonical_modernization::canonical_constants::storage::*;
