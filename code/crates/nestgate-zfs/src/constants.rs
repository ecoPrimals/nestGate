// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS-Specific Constants
//!
//! This module provides ZFS-specific constants that are not yet integrated
//! into the canonical constants system, as well as commonly used byte size
//! multipliers for consistency across the codebase.

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

// ==================== STORAGE TIER CAPACITY DEFAULTS ====================

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

// ==================== FAILOVER TIMEOUTS ====================

/// Default node failure timeout in seconds (3 minutes)
pub const NODE_FAILURE_TIMEOUT_SECS: u64 = 180;

// ==================== AI CONFIDENCE TIMEOUTS ====================

/// Default AI confidence cache timeout in minutes (2 hours)
pub const AI_CONFIDENCE_CACHE_TIMEOUT_MINS: u64 = 120;

/// Default AI tier evaluation timeout in minutes (3 hours)
pub const AI_TIER_EVALUATION_TIMEOUT_MINS: u64 = 180;

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
