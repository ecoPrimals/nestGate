// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

    /// Default buffer size - **CONSOLIDATED** to `hardcoding::limits`
    pub const DEFAULT_BUFFER_SIZE_BYTES: usize =
        crate::constants::hardcoding::limits::BUFFER_SIZE_DEFAULT;

    /// Maximum buffer size - **CONSOLIDATED** to `hardcoding::limits`
    pub const MAX_BUFFER_SIZE_BYTES: usize = crate::constants::hardcoding::limits::BUFFER_SIZE_MAX;

    /// Maximum number of concurrent operations
    pub const MAX_CONCURRENT_OPERATIONS: usize = 1000;

    /// Default batch size for bulk operations
    pub const DEFAULT_BATCH_SIZE: usize = 1000;

    /// Maximum number of retry attempts - **CONSOLIDATED** to `hardcoding::limits`
    pub const MAX_RETRY_ATTEMPTS: u32 = crate::constants::hardcoding::limits::MAX_RETRIES;

    /// Default performance monitoring interval (seconds)
    pub const PERFORMANCE_MONITOR_INTERVAL_SECS: u64 = 30;

    /// Default thread pool size
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 4;

    /// Default batch size
    pub const BATCH_SIZE: usize = 100;

    /// Maximum number of concurrent requests (zero-cost architecture)
    pub const MAX_CONCURRENT_REQUESTS: usize = 10000;

    /// Maximum number of concurrent connections - **CONSOLIDATED** to `hardcoding::limits`
    pub const MAX_CONCURRENT_CONNECTIONS: usize =
        crate::constants::hardcoding::limits::MAX_CONNECTIONS;

    /// Default request timeout - **CONSOLIDATED** to `hardcoding::timeouts`
    pub const REQUEST_TIMEOUT_MS: u64 = crate::constants::hardcoding::timeouts::REQUEST_MS;

    /// Cache line size for memory optimization
    pub const CACHE_LINE_SIZE: usize = 64;

    /// Default memory pool size
    pub const DEFAULT_POOL_SIZE: usize = 1024;

    /// Maximum file size in megabytes
    pub const MAX_FILE_SIZE_MB: usize = 1024;
