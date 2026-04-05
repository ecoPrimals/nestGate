// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Maximum number of ZFS pools
    pub const MAX_POOLS: usize = 256;

    /// Maximum number of datasets across all pools
    pub const MAX_DATASETS: usize = 10000;

    /// Maximum number of snapshots
    pub const MAX_SNAPSHOTS: usize = 100_000;

    /// Maximum number of RAID backends
    pub const MAX_RAID_BACKENDS: usize = 8;

    /// Maximum number of copy-on-write operations
    pub const MAX_COW_OPERATIONS: usize = 1000;

    /// ZFS discovery maximum recursion depth
    pub const ZFS_DISCOVERY_MAX_DEPTH: usize = 10;
