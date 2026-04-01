// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Default service name
    pub const DEFAULT_SERVICE_NAME: &str = "nestgate";

    /// Default timeout
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;

    /// Default log level
    pub const DEFAULT_LOG_LEVEL: &str = "info";

    /// Environment types
    pub const ENV_DEVELOPMENT: &str = "development";
    /// Staging environment identifier
    pub const ENV_STAGING: &str = "staging";
    /// Production environment identifier
    pub const ENV_PRODUCTION: &str = "production";
    /// Testing environment identifier
    pub const ENV_TESTING: &str = "testing";

    /// System limits
    ///
    /// Maximum memory usage in megabytes
    pub const MAX_MEMORY_MB: u64 = 4096;
    /// Maximum number of CPU cores to use
    pub const MAX_CPU_CORES: u32 = 8;
