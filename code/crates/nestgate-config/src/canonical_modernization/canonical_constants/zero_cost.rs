// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Default maximum concurrent operations
    pub const DEFAULT_MAX_CONCURRENT: usize = 1000;

    /// Default buffer size for zero-cost operations
    pub const DEFAULT_BUFFER_SIZE: usize = 65536;

    /// Default maximum file size in MB
    pub const DEFAULT_MAX_FILE_SIZE_MB: usize = 1024;

    /// Default operation timeout in seconds
    pub const DEFAULT_OPERATION_TIMEOUT_SECS: u64 = 30;

    /// Default maximum pool size
    pub const DEFAULT_POOL_SIZE: usize = 1024;

    /// Default maximum backends for storage
    pub const DEFAULT_MAX_BACKENDS: usize = 100;

    /// Default discovery timeout in milliseconds
    pub const DEFAULT_DISCOVERY_TIMEOUT_MS: u64 = 5000;

    /// Default health check interval in milliseconds
    pub const DEFAULT_HEALTH_CHECK_INTERVAL_MS: u64 = 30000;
