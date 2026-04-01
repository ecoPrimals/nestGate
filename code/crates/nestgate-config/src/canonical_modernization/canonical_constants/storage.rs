// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Small file threshold (1MB)
    pub const SMALL_FILE_BYTES: u64 = 1024 * 1024;

    /// Large file threshold (100MB)
    pub const LARGE_FILE_BYTES: u64 = 100 * 1024 * 1024;

    /// Very large file threshold (1GB)
    pub const VERY_LARGE_FILE_BYTES: u64 = 1024 * 1024 * 1024;

    /// Default block size for storage operations
    pub const DEFAULT_BLOCK_SIZE: usize = 4096;

    /// Maximum filename length in characters
    pub const MAX_FILENAME_LENGTH: usize = 255;

    /// Storage tiers
    ///
    /// Hot tier storage - frequently accessed data
    pub const TIER_HOT: &str = "hot";
    /// Warm tier storage - moderately accessed data
    pub const TIER_WARM: &str = "warm";
    /// Cold tier storage - infrequently accessed data
    pub const TIER_COLD: &str = "cold";

    /// Compression algorithms
    ///
    /// LZ4 compression algorithm - fast, moderate compression
    pub const COMPRESSION_LZ4: &str = "lz4";
    /// GZIP compression algorithm - standard compression
    pub const COMPRESSION_GZIP: &str = "gzip";
    /// GZIP level 6 compression - balanced compression
    pub const COMPRESSION_GZIP_6: &str = "gzip-6";
    /// GZIP level 9 compression - maximum compression
    pub const COMPRESSION_GZIP_9: &str = "gzip-9";
    /// ZSTD compression algorithm - modern, efficient compression
    pub const COMPRESSION_ZSTD: &str = "zstd";

    /// Size constants (bytes)
    ///
    /// Kilobyte (1024 bytes)
    pub const KB: u64 = 1024;
    /// Megabyte (1024 KB)
    pub const MB: u64 = 1024 * KB;
    /// Gigabyte (1024 MB)
    pub const GB: u64 = 1024 * MB;
    /// Terabyte (1024 GB)
    pub const TB: u64 = 1024 * GB;

    /// Default storage limits
    ///
    /// Default maximum file size (100 MB)
    pub const DEFAULT_MAX_FILE_SIZE: u64 = 100 * MB;
    /// Default pool size (10 GB)
    pub const DEFAULT_POOL_SIZE: u64 = 10 * GB;
