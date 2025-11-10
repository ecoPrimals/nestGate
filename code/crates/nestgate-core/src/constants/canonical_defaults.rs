//! **CANONICAL DEFAULTS**
//!
//! Default values for canonical configuration and network settings.

pub mod network {
    //! Network-related canonical defaults

    /// Default API base URL
    pub const DEFAULT_API_BASE_URL: &str = "http://localhost:8080";

    /// Default WebSocket URL  
    pub const DEFAULT_WEBSOCKET_URL: &str = "ws://localhost:8080/ws";

    /// Default metrics URL
    pub const DEFAULT_METRICS_URL: &str = "http://localhost:9090";

    /// Default web UI URL
    pub const DEFAULT_WEB_UI_URL: &str = "http://localhost:3000";

    /// Localhost constant
    pub const LOCALHOST: &str = "127.0.0.1";

    /// Default bind address
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";

    // Port constants moved to port_defaults.rs (single source of truth)
    // Re-exported here for backward compatibility
    pub use super::super::port_defaults::{
        DEFAULT_ADMIN_PORT as DEFAULT_INTERNAL_PORT, DEFAULT_API_PORT, DEFAULT_METRICS_PORT,
    };

    /// Build API URL from environment or default
    #[must_use]
    pub fn build_api_url() -> String {
        std::env::var("NESTGATE_API_URL").unwrap_or_else(|_| DEFAULT_API_BASE_URL.to_string())
    }

    /// Build WebSocket URL from environment or default
    #[must_use]
    pub fn build_websocket_url() -> String {
        std::env::var("NESTGATE_WS_URL").unwrap_or_else(|_| DEFAULT_WEBSOCKET_URL.to_string())
    }

    /// Build metrics URL from environment or default
    #[must_use]
    pub fn build_metrics_url() -> String {
        std::env::var("NESTGATE_METRICS_URL").unwrap_or_else(|_| DEFAULT_METRICS_URL.to_string())
    }

    /// Build generic endpoint from environment
    #[must_use]
    pub fn build_endpoint() -> String {
        build_api_url()
    }

    pub mod limits {
        //! Network limits and constraints

        /// Maximum concurrent requests
        pub const MAX_CONCURRENT_REQUESTS: usize = 1000;

        /// Maximum request size in bytes
        pub const MAX_REQUEST_SIZE: usize = 1024 * 1024; // 1MB

        /// Connection timeout in milliseconds
        pub const CONNECTION_TIMEOUT_MS: u64 = 5000;
    }
}

pub mod performance {
    //! Performance-related defaults
    //!
    //! # Buffer Size Guidelines
    //!
    //! **IMPORTANT**: Do NOT consolidate these buffer sizes - they are intentionally different!
    //!
    //! ## Network Buffer (8192 bytes)
    //! Optimized for network I/O operations:
    //! - Balances memory usage vs system call overhead
    //! - Smaller than canonical::performance::NETWORK_BUFFER_SIZE (64KB) for memory efficiency
    //! - Good for moderate throughput scenarios
    //!
    //! ## Default Buffer (4096 bytes)
    //! Optimized for disk/general I/O:
    //! - Matches typical filesystem page size
    //! - Optimal for disk operations
    //! - Fits in L1/L2 cache for fast access

    /// Network buffer size (8KB) - optimized for network I/O
    ///
    /// **Use for**: Network operations, socket I/O, moderate throughput
    /// **Performance**: Balances memory vs system call overhead
    pub const NETWORK_BUFFER_SIZE: usize = 8192;

    /// Default buffer size (4KB) - optimized for disk I/O
    ///
    /// **Use for**: File operations, disk I/O, general buffering
    /// **Performance**: Matches page size, optimal for filesystem operations
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;

    /// Maximum connections
    pub const MAX_CONNECTIONS: usize = 1000;
}

pub mod concurrency {
    //! Concurrency and threading defaults

    /// Default thread pool size
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 8;

    /// Default maximum connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;

    /// Default worker count
    pub const DEFAULT_WORKER_COUNT: usize = 4;

    /// Default queue size
    pub const DEFAULT_QUEUE_SIZE: usize = 10000;

    /// Default maximum parallel tests
    pub const DEFAULT_MAX_PARALLEL_TESTS: usize = 4;
}

pub mod sizes {
    //! Size and capacity defaults

    /// Default buffer size (4KB)
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;

    /// Default cache size (128MB in bytes)
    pub const DEFAULT_CACHE_SIZE: u64 = 128 * 1024 * 1024;

    /// Default page size (4KB)
    pub const DEFAULT_PAGE_SIZE: usize = 4096;

    /// Default record size
    pub const DEFAULT_RECORD_SIZE: usize = 256;

    /// Default file size limit (100MB)
    pub const DEFAULT_FILE_SIZE_LIMIT: u64 = 100 * 1024 * 1024;

    /// Default memory limit (1GB)
    pub const DEFAULT_MEMORY_LIMIT: u64 = 1024 * 1024 * 1024;
}

pub mod timeouts {
    //! Timeout constants
    use std::time::Duration;

    /// Default timeout in milliseconds
    pub const DEFAULT_TIMEOUT_MS: u64 = 5000;

    /// Connection timeout
    pub const CONNECTION_TIMEOUT_MS: u64 = 3000;

    /// Request timeout
    pub const REQUEST_TIMEOUT_MS: u64 = 10000;

    /// Default connection timeout as Duration
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

    /// Default request timeout as Duration
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(60);

    /// Default health check timeout as Duration
    pub const DEFAULT_HEALTH_TIMEOUT: Duration = Duration::from_secs(10);

    /// Default ZFS operation timeout as Duration
    pub const DEFAULT_ZFS_TIMEOUT: Duration = Duration::from_secs(300);

    /// Default test timeout as Duration
    pub const DEFAULT_TEST_TIMEOUT: Duration = Duration::from_secs(30);
}
