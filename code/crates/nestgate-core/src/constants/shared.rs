//! Shared Constants Module
//!
//! This module contains constants that are used across multiple modules
//! to eliminate duplication and provide a single source of truth.

/// Module version for compatibility tracking across the codebase
pub const MODULE_VERSION: &str = "0.6.0";

/// Default timeout in milliseconds for network operations
pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;

/// Default buffer size in bytes for I/O operations
pub const DEFAULT_BUFFER_SIZE: usize = 8192;

/// Default maximum number of concurrent connections
pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;

/// Default retry attempts for failed operations
pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;

/// Default backoff multiplier for exponential backoff (milliseconds)
pub const DEFAULT_BACKOFF_MS: u64 = 1000;

/// Default health check interval in seconds
pub const DEFAULT_HEALTH_CHECK_INTERVAL_SECS: u64 = 30;

/// Default log rotation size in megabytes
pub const DEFAULT_LOG_ROTATION_MB: usize = 100;

/// Default cache TTL in seconds
pub const DEFAULT_CACHE_TTL_SECS: u64 = 3600;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_constants() {
        assert_eq!(MODULE_VERSION, "0.6.0");
        assert_eq!(DEFAULT_TIMEOUT_MS, 30_000);
        assert_eq!(DEFAULT_BUFFER_SIZE, 8192);
        assert_eq!(DEFAULT_MAX_CONNECTIONS, 1000);
    }
}
