// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

    /// Default development host
    pub const DEFAULT_DEV_HOST: &str = crate::constants::hardcoding::addresses::LOCALHOST_IPV4;

    /// Default development port
    pub const DEFAULT_DEV_PORT: u16 = 8080;

    /// Default development timeout
    pub const DEFAULT_DEV_TIMEOUT_SECS: u64 = 30;

    /// Default development retry attempts
    pub const DEFAULT_DEV_RETRY_ATTEMPTS: u32 = 3;

    /// Default development buffer size
    pub const DEFAULT_DEV_BUFFER_SIZE: usize = 8192;

    /// Default development max connections
    pub const DEFAULT_DEV_MAX_CONNECTIONS: usize = 1000;

    /// Test constants for development
    pub mod test_constants {
        /// Default test iterations
        pub const DEFAULT_ITERATIONS: usize = 1000;

        /// Performance test iterations
        pub const PERFORMANCE_ITERATIONS: usize = 10000;

        /// SIMD performance test iterations
        pub const SIMD_ITERATIONS: u32 = 100;

        /// Test compute service URL
        pub const TEST_COMPUTE_SERVICE_URL: &str = ""; // TRIPLE PEDANTIC: Use std::env::var("NESTGATE_API_ENDPOINT") or build_api_url() instead

        /// Unwrap migrator batch size
        pub const UNWRAP_MIGRATOR_BATCH_SIZE: usize = 10;

        /// Target performance improvement percentage
        pub const TARGET_IMPROVEMENT_PERCENT: f64 = 20.0;
    }
