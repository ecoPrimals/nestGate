// **MIGRATED DOMAIN CONSTANTS MODULE**
//! Domain Constants functionality and utilities.
// This module now uses the canonical constants system instead of scattered
//! domain-specific constants. All constants are now consolidated into the
//! unified canonical system.

// Re-export from canonical constants system
pub use crate::constants::canonical_defaults::{
    concurrency::{
        DEFAULT_MAX_CONNECTIONS, DEFAULT_MAX_PARALLEL_TESTS, DEFAULT_QUEUE_SIZE,
        DEFAULT_THREAD_POOL_SIZE, DEFAULT_WORKER_COUNT,
    },
    environment::{
        DEFAULT_CONFIG_DIR, DEFAULT_ENVIRONMENT, DEFAULT_INSTANCE_NAME, DEFAULT_LOG_DIR,
        DEFAULT_LOG_LEVEL, DEFAULT_SERVICE_NAME,
    },
    network::{
        build_api_url, build_grpc_url, build_web_ui_url, DEFAULT_API_PORT, DEFAULT_BIND_ADDRESS,
        DEFAULT_HEALTH_PORT, DEFAULT_INTERNAL_PORT, DEFAULT_METRICS_PORT, LOCALHOST,
    },
    performance::{
        DEFAULT_BENCHMARK_ITERATIONS, DEFAULT_CPU_CORES, DEFAULT_DISK_THRESHOLD,
        DEFAULT_MEMORY_THRESHOLD, DEFAULT_WARMUP_ITERATIONS,
    },
    security::{
        DEFAULT_MAX_LOGIN_ATTEMPTS, DEFAULT_RATE_LIMIT_BURST, DEFAULT_RATE_LIMIT_RPM,
        DEFAULT_SESSION_TIMEOUT, DEFAULT_TOKEN_EXPIRATION,
    },
    sizes::{
        DEFAULT_BUFFER_SIZE, DEFAULT_CACHE_SIZE, DEFAULT_FILE_SIZE_LIMIT, DEFAULT_MEMORY_LIMIT,
        DEFAULT_PAGE_SIZE, DEFAULT_RECORD_SIZE,
    },
    storage::{
        DEFAULT_BACKEND, DEFAULT_BACKUP_RETENTION_DAYS, DEFAULT_COMPRESSION_LEVEL,
        DEFAULT_DATA_DIR, DEFAULT_ZFS_POOL,
    },
    test::{
        DEFAULT_MOCK_DELAY, DEFAULT_TEST_DATA_DIR, DEFAULT_TEST_DB_URL, DEFAULT_TEST_RETRIES,
        DEFAULT_TEST_STORAGE_DIR,
    },
    timeouts::{
        DEFAULT_CONNECTION_TIMEOUT, DEFAULT_HEALTH_TIMEOUT, DEFAULT_REQUEST_TIMEOUT,
        DEFAULT_TEST_TIMEOUT, DEFAULT_ZFS_TIMEOUT,
    },
};

/// Domain-specific timeout constants
pub mod timeouts {
    use std::time::Duration;

    /// Request timeout in seconds
    pub const REQUEST_TIMEOUT_SECS: u64 = 30;

    /// Connection timeout
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

    /// Health check timeout
    pub const HEALTH_TIMEOUT: Duration = Duration::from_secs(5);
}
// ==================== MIGRATION ALIASES ====================

// ==================== DOMAIN-SPECIFIC CONSTANT GROUPS ====================

/// Network-related constants for domain operations
pub mod network {
    use std::env;

    // ✅ SOVEREIGNTY-COMPLIANT CONSTANTS - Environment-driven
    pub const DEFAULT_API_PORT: u16 = 8080;
    pub const DEFAULT_WEBSOCKET_PORT: u16 = 8081;
    pub const DEFAULT_HEALTH_PORT: u16 = 8082;
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
    pub const LOCALHOST: &str = "localhost";

    // ✅ ADDED MISSING URL CONSTANTS - Capability-based endpoints
    pub const DEFAULT_API_BASE_URL: &str = "http://localhost:8080";
    pub const DEFAULT_WEB_UI_URL: &str = "http://localhost:3000";
    pub const DEFAULT_GRPC_URL: &str = "http://localhost:9090";

    // Import all constants for backward compatibility
    pub use self::{
        DEFAULT_API_BASE_URL as API_BASE_URL, DEFAULT_API_PORT as API_PORT,
        DEFAULT_BIND_ADDRESS as BIND_ADDRESS, DEFAULT_GRPC_URL as GRPC_URL,
        DEFAULT_HEALTH_PORT as HEALTH_PORT, DEFAULT_WEBSOCKET_PORT as WEBSOCKET_PORT,
        DEFAULT_WEB_UI_URL as WEB_UI_URL,
    };

    /// Build discovery endpoint using canonical constants
    #[must_use]
    pub const fn discovery_endpoint() -> String {
        env::var("DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| format!("http://{}:{}/discovery", "localhost", 8083)
    }

    /// Build API endpoint using canonical constants
    #[must_use]
    pub const fn api_endpoint() -> String {
        format!("{LOCALHOST}:{DEFAULT_API_PORT}")
    }
}
/// Storage domain constants (consolidated)
pub mod storage {
    pub use super::{
        DEFAULT_BACKEND as BACKEND, DEFAULT_BACKUP_RETENTION_DAYS as BACKUP_RETENTION_DAYS,
        DEFAULT_BUFFER_SIZE as BUFFER_SIZE, DEFAULT_CACHE_SIZE as CACHE_SIZE,
        DEFAULT_COMPRESSION_LEVEL as COMPRESSION_LEVEL, DEFAULT_DATA_DIR as DATA_DIR,
        DEFAULT_RECORD_SIZE as RECORD_SIZE, DEFAULT_ZFS_POOL as ZFS_POOL,
    };
}
/// Security domain constants (consolidated)
pub mod security {
    pub use super::{
        DEFAULT_MAX_LOGIN_ATTEMPTS as MAX_LOGIN_ATTEMPTS,
        DEFAULT_RATE_LIMIT_BURST as RATE_LIMIT_BURST, DEFAULT_RATE_LIMIT_RPM as RATE_LIMIT_RPM,
        DEFAULT_SESSION_TIMEOUT as SESSION_TIMEOUT, DEFAULT_TOKEN_EXPIRATION as TOKEN_EXPIRATION,
    };
}
/// Performance domain constants (consolidated)
pub mod performance {
    pub use super::{
        DEFAULT_BENCHMARK_ITERATIONS as BENCHMARK_ITERATIONS, DEFAULT_CPU_CORES as CPU_CORES,
        DEFAULT_DISK_THRESHOLD as DISK_THRESHOLD, DEFAULT_MAX_CONNECTIONS as MAX_CONNECTIONS,
        DEFAULT_MEMORY_THRESHOLD as MEMORY_THRESHOLD, DEFAULT_THREAD_POOL_SIZE as THREAD_POOL_SIZE,
        DEFAULT_WARMUP_ITERATIONS as WARMUP_ITERATIONS, DEFAULT_WORKER_COUNT as WORKER_COUNT,
    };
}
/// Test domain constants (consolidated)
pub mod test {
    pub use super::{
        DEFAULT_MAX_PARALLEL_TESTS as MAX_PARALLEL, DEFAULT_MOCK_DELAY as MOCK_DELAY,
        DEFAULT_TEST_DATA_DIR as DATA_DIR, DEFAULT_TEST_DB_URL as DB_URL,
        DEFAULT_TEST_RETRIES as RETRIES, DEFAULT_TEST_STORAGE_DIR as STORAGE_DIR,
        DEFAULT_TEST_TIMEOUT as TIMEOUT,
    };
}
// ==================== CONVENIENCE FUNCTIONS ====================

/// Get canonical API port (environment-aware)
#[must_use]
pub const fn get_api_port() -> u16 {
    std::env::var("NESTGATE_API_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(DEFAULT_API_PORT)
}
/// Get canonical bind address (environment-aware)
#[must_use]
pub const fn get_bind_address() -> String {
    std::env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| DEFAULT_BIND_ADDRESS.to_string())
}
/// Get canonical data directory (environment-aware)
#[must_use]
pub const fn get_data_dir() -> String {
    std::env::var("NESTGATE_DATA_DIR").unwrap_or_else(|_| DEFAULT_DATA_DIR.to_string())
}
/// Build canonical discovery endpoint using environment-aware constants
#[must_use]
pub const fn build_discovery_endpoint() -> String {
    format!("http://{get_bind_address(}:8083/discovery"))
}
