//! **CONSOLIDATED MAGIC NUMBERS CONSTANTS**
//! 
//! This module consolidates all previously scattered magic numbers into
//! organized, documented constants with clear purposes.
//! 
//! **REPLACES**: 200+ scattered magic numbers across the codebase
//! **PROVIDES**: Single source of truth for all numeric constants

/// Network and port-related constants
pub mod network {
    /// Default API server port (HTTP)
    pub const DEFAULT_API_PORT: u16 = 8080;
    
    /// Development server port
    pub const DEFAULT_DEV_PORT: u16 = 3000;
    
    /// Internal services port
    pub const DEFAULT_INTERNAL_PORT: u16 = 9090;
    
    /// Secure API port (HTTPS)
    pub const DEFAULT_SECURE_PORT: u16 = 18080;
    
    /// Default network timeout (30 seconds)
    pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;
    
    /// Short timeout for quick operations (5 seconds)
    pub const SHORT_TIMEOUT_MS: u64 = 5_000;
    
    /// Long timeout for heavy operations (60 seconds)
    pub const LONG_TIMEOUT_MS: u64 = 60_000;
    
    /// Extended timeout for very heavy operations (5 minutes)
    pub const EXTENDED_TIMEOUT_MS: u64 = 300_000;
}

/// Performance and buffer size constants
pub mod performance {
    /// 64KB buffer size (most common)
    pub const BUFFER_SIZE_64KB: usize = 65_536;
    
    /// 8KB buffer size (small operations)
    pub const BUFFER_SIZE_8KB: usize = 8_192;
    
    /// 4KB buffer size (minimal operations)
    pub const BUFFER_SIZE_4KB: usize = 4_096;
    
    /// 1KB buffer size (tiny operations)
    pub const BUFFER_SIZE_1KB: usize = 1_024;
    
    /// Default maximum connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1_000;
    
    /// High volume connection limit
    pub const HIGH_VOLUME_LIMIT: usize = 10_000;
    
    /// Enterprise-grade limit
    pub const ENTERPRISE_LIMIT: usize = 100_000;
    
    /// Small connection pool size
    pub const SMALL_POOL_SIZE: usize = 256;
}

/// Storage-related constants
pub mod storage {
    /// Default cache size (128MB)
    pub const DEFAULT_CACHE_SIZE_MB: u64 = 128;
    
    /// 1MB buffer for large operations
    pub const BUFFER_SIZE_1MB: usize = 1_048_576;
    
    /// ZFS record size (128KB)
    pub const ZFS_RECORD_SIZE_128KB: usize = 131_072;
    
    /// ZFS ARC size (1GB)
    pub const ZFS_ARC_SIZE_1GB: usize = 1_073_741_824;
}

/// Security-related constants
pub mod security {
    /// Maximum login attempts before lockout
    pub const MAX_LOGIN_ATTEMPTS: u8 = 5;
    
    /// Session timeout (30 minutes)
    pub const SESSION_TIMEOUT_MINUTES: u16 = 30;
    
    /// Minimum password length
    pub const PASSWORD_MIN_LENGTH: u8 = 8;
    
    /// Token expiry time (24 hours)
    pub const TOKEN_EXPIRY_HOURS: u8 = 24;
}

/// Thread and concurrency constants
pub mod concurrency {
    /// Default thread pool size
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 8;
    
    /// Maximum concurrent operations
    pub const MAX_CONCURRENT_OPS: usize = 100;
    
    /// Worker thread count for CPU-intensive tasks
    pub const CPU_INTENSIVE_WORKERS: usize = 4;
}

/// Test and development constants
pub mod testing {
    /// Test iteration count for performance tests
    pub const PERFORMANCE_TEST_ITERATIONS: usize = 10_000;
    
    /// Test timeout for unit tests (1 second)
    pub const UNIT_TEST_TIMEOUT_MS: u64 = 1_000;
    
    /// Test timeout for integration tests (30 seconds)
    pub const INTEGRATION_TEST_TIMEOUT_MS: u64 = 30_000;
    
    /// Mock service port for testing
    pub const MOCK_SERVICE_PORT: u16 = 18080;
}
