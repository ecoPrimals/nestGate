// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Maximum recursion depth for ZFS discovery operations
    pub const ZFS_DISCOVERY_MAX_DEPTH: usize = 10;

    /// Maximum file depth for nested directories
    pub const MAX_FILE_DEPTH: usize = 100;

    /// Maximum recursion depth for general operations
    pub const MAX_RECURSION_DEPTH: usize = 50;

    /// Maximum number of ZFS pools
    pub const MAX_POOLS: usize = 64;

    /// Maximum number of datasets per pool
    pub const MAX_DATASETS: usize = 1000;

    /// Maximum number of concurrent operations
    pub const MAX_CONCURRENT_OPERATIONS: usize = 100;

    /// Maximum number of optimization operations
    pub const MAX_OPTIMIZATIONS: usize = 50;
