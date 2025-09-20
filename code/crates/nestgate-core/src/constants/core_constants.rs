// **CORE CONSTANTS**
//! Core Constants functionality and utilities.
// Core system constants for NestGate operations.
// These are legacy constants - use canonical constants for new code.

// Import canonical constants to avoid duplication
pub use crate::constants::canonical::{
    network::{DEFAULT_API_PORT as DEFAULT_PORT, LOCALHOST, DEFAULT_BIND_ADDRESS},
    performance::{MAX_CONNECTIONS, DEFAULT_BUFFER_SIZE, CACHE_LINE_SIZE, AVX2_WIDTH, PAGE_SIZE, OPTIMAL_BATCH_SIZE},
    system::{DEFAULT_TIMEOUT_SECS},
    limits::{MAX_CONFIG_DEPTH, MAX_CONFIG_STRING_LENGTH, MAX_CONFIG_ARRAY_LENGTH, MAX_FEATURE_FLAGS},
    api::{CURRENT_CONFIG_VERSION, MIN_SUPPORTED_VERSION, SCHEMA_VERSION},
};

// Legacy constants for backward compatibility - migrated to canonical constants
pub use crate::constants::LOCALHOST as DEFAULT_HOST;
pub use crate::constants::DEFAULT_BIND_ADDRESS as ANY;

/// Discovery endpoint - uses canonical endpoint building
pub const fn get_discovery_endpoint() -> String {
    use crate::constants::{LOCALHOST, DEFAULT_API_PORT};
    std::env::var("NESTGATE_DISCOVERY_ENDPOINT")
        .unwrap_or_else(|_| format!("http://{}:{}/discovery", LOCALHOST, DEFAULT_API_PORT)
}
// Port constants (legacy - use canonical network constants)
pub mod ports {
    pub const HTTP: u16 = 80;
    pub const HTTPS: u16 = 443;
}

// Performance constants (legacy - use canonical performance constants)  
pub use crate::constants::MAX_CONCURRENT_REQUESTS;

/// Maximum in-memory file size - uses canonical size constants
pub const fn get_max_in_memory_file_size() -> u64 {
    use crate::constants::canonical::storage::{MB};
    std::env::var("NESTGATE_MAX_IN_MEMORY_FILE_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100 * MB)
}
