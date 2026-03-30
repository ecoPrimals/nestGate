// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

    /// Test timeouts
    ///
    /// Default test timeout in seconds (10 seconds)
    pub const TEST_TIMEOUT_SECS: u64 = 10;
    /// Integration test timeout in seconds (60 seconds)
    pub const INTEGRATION_TEST_TIMEOUT_SECS: u64 = 60;

    /// Test data sizes
    ///
    /// Test data size in kilobytes (1 MB)
    pub const TEST_DATA_SIZE_KB: usize = 1024;
    /// Large test data size in megabytes (10 MB)
    pub const LARGE_TEST_DATA_SIZE_MB: usize = 10;

    /// Test iteration counts
    ///
    /// Number of iterations for performance tests (1000 iterations)
    pub const PERFORMANCE_TEST_ITERATIONS: usize = 1000;
    /// Concurrent users for load testing (100 users)
    pub const LOAD_TEST_CONCURRENT_USERS: usize = 100;

    /// Test environment
    ///
    /// Test service name identifier
    pub const TEST_SERVICE_NAME: &str = "nestgate-test";
    /// Test API port number (18080)
    pub const TEST_API_PORT: u16 = 18080;
