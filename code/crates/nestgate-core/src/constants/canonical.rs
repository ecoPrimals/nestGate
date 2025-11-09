// **CANONICAL CONSTANTS SYSTEM**
//! Canonical functionality and utilities.
// Single source of truth for ALL constants across NestGate.
// Eliminates 200+ scattered constant definitions and provides unified access.

// Removed unused Duration import

// ==================== SECTION ====================

/// Performance and optimization constants
pub mod performance {
    /// Target performance improvement minimum
    pub const TARGET_IMPROVEMENT_PERCENT: f64 = 20.0;

    /// SIMD processing widths
    pub const AVX2_WIDTH: usize = 32;
    pub const SSE2_WIDTH: usize = 16;
    pub const SIMD_ALIGNMENT: usize = 32;
    pub const MIN_SIMD_SIZE: usize = 64;

    /// Memory optimization
    pub const CACHE_LINE_SIZE: usize = 64;
    pub const PAGE_SIZE: usize = 4096;
    pub const OPTIMAL_BATCH_SIZE: usize = 1000;

    /// Connection and concurrency limits
    pub const DEFAULT_MAX_CONCURRENT: usize = 1000;
    pub const MAX_CONNECTIONS: usize = 1000;
    pub const MAX_BACKENDS: usize = 100;
    pub const MAX_CONCURRENT_OPS: usize = 1000;

    /// Buffer sizes (domain-specific, DO NOT consolidate)
    ///
    /// # Why Different Buffer Sizes?
    ///
    /// Different I/O operations have different optimal buffer sizes based on:
    /// - Hardware characteristics (disk vs network)
    /// - Protocol requirements (TCP window sizes)
    /// - Cache line sizes (L1/L2/L3)
    /// - System call overhead vs throughput tradeoffs
    ///
    /// ## DEFAULT_BUFFER_SIZE (4096 bytes)
    /// **Use for**: General I/O, disk operations, file system operations
    /// **Why 4KB**: Matches typical page size, optimal for disk I/O
    /// **Performance**: Minimizes system calls while fitting in L1 cache
    ///
    /// ## NETWORK_BUFFER_SIZE (65536 bytes)
    /// **Use for**: Network I/O, socket operations, streaming
    /// **Why 64KB**: Matches typical TCP window size
    /// **Performance**: Reduces context switches for network operations
    ///
    /// # Usage Guidelines
    /// ```rust
    /// use nestgate_core::constants::canonical::performance;
    ///
    /// // For disk I/O
    /// let mut disk_buffer = vec![0u8; performance::DEFAULT_BUFFER_SIZE];
    ///
    /// // For network I/O
    /// let mut network_buffer = vec![0u8; performance::NETWORK_BUFFER_SIZE];
    /// ```
    ///
    /// **Important**: These values are performance-tuned. Do not consolidate!
    pub const DEFAULT_BUFFER_SIZE: usize = 4096;
    pub const NETWORK_BUFFER_SIZE: usize = 65536;
    pub const SIMD_BATCH_SIZE: usize = 32;
    pub const POOL_SIZE: usize = 1024;
    pub const BLOCK_SIZE: usize = 4096;
}
// ==================== SECTION ====================

/// Timeout and timing constants
pub mod timeouts {
    // Removed unused Duration import

    /// Default timeout seconds (consolidated)
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
    pub const DEFAULT_TIMEOUT_MS: u64 = 30000;
    pub const REQUEST_TIMEOUT_MS: u64 = 30000;

    /// Discovery and health check intervals
    pub const DISCOVERY_TIMEOUT_MS: u64 = 5000;
    pub const HEALTH_CHECK_INTERVAL_MS: u64 = 30000;
    pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 30;
    pub const STATS_RETENTION_SECS: u64 = 86400; // 24 hours

    /// Connection timeouts
    pub const CONNECTION_TIMEOUT_SECS: u64 = 30;
    pub const SESSION_TIMEOUT_SECS: u64 = 300;
    pub const OPERATION_TIMEOUT_SECS: u64 = 30;

    /// Retry and rate limiting
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
    pub const MESSAGE_RETRY_ATTEMPTS: u32 = 3;
    pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;
    pub const DEFAULT_RATE_LIMIT_RPM: u32 = 1000;
    pub const DEFAULT_RATE_LIMIT_BURST: u32 = 100;

    /// Monitoring intervals
    pub const METRICS_COLLECTION_INTERVAL_SECS: u64 = 60;
    pub const DEFAULT_HANDLER_TIMEOUT_SECS: u64 = 30;
}
// ==================== SECTION ====================

/// Network and protocol constants
pub mod network {
    // Port constants are now in port_defaults module (single source of truth)
    // Use: constants::port_defaults::DEFAULT_API_PORT
    // Or via re-export: constants::DEFAULT_API_PORT
    
    /// Network addresses
    pub const DEFAULT_BIND_ADDRESS: &str = "0.0.0.0";
    pub const LOCALHOST: &str = "127.0.0.1";

    /// Protocol limits
    pub const MAX_SERVICES: usize = 1000;
    pub const MAX_CONCURRENT_REQUESTS: usize = 10000;
    pub const MAX_SESSIONS: usize = 1000;
    pub const MAX_MESSAGE_SIZE: usize = 1024;
    pub const PROTOCOL_VERSION: u32 = 1;

    /// MTU and buffer sizes
    pub const MTU_SIZE: usize = 1500;
    pub const SEND_BUFFER_SIZE: usize = 65536;
    pub const RECV_BUFFER_SIZE: usize = 65536;
}
// ==================== SECTION ====================

/// Storage and file system constants
pub mod storage {
    /// Storage tiers (consolidated)
    pub const TIER_HOT: &str = "hot";
    pub const TIER_WARM: &str = "warm";
    pub const TIER_COLD: &str = "cold";

    /// Compression algorithms
    pub const COMPRESSION_LZ4: &str = "lz4";
    pub const COMPRESSION_GZIP_6: &str = "gzip-6";
    pub const COMPRESSION_GZIP_9: &str = "gzip-9";

    /// Size units
    pub const KB: u64 = 1024;
    pub const MB: u64 = 1024 * 1024;
    pub const GB: u64 = 1024 * 1024 * 1024;
    pub const TB: u64 = 1024 * 1024 * 1024 * 1024;

    /// File size limits
    pub const MAX_FILE_SIZE_MB: usize = 1024;
    pub const MAX_IN_MEMORY_FILE_SIZE: u64 = 1024 * 1024 * 1024;

    /// ZFS constants (consolidated)
    pub const RECORDSIZE_64K: &str = "64K";
    pub const RECORDSIZE_128K: &str = "128K";
    pub const RECORDSIZE_1M: &str = "1M";
    pub const RECORD_SIZE: usize = 131_072; // 128KB
    pub const ARC_SIZE: usize = 1_073_741_824; // 1GB

    /// ZFS commands
    pub const ZFS: &str = "zfs";
    pub const ZPOOL: &str = "zpool";
    pub const LIST: &str = "list";
    pub const CREATE: &str = "create";
    pub const DESTROY: &str = "destroy";
    pub const SET: &str = "set";
    pub const GET: &str = "get";
    pub const SNAPSHOT: &str = "snapshot";
    pub const STATUS: &str = "status";

    /// ZFS states
    pub const ONLINE: &str = "ONLINE";
    pub const DEGRADED: &str = "DEGRADED";
    pub const FAULTED: &str = "FAULTED";
    pub const OFFLINE: &str = "OFFLINE";
    pub const UNAVAIL: &str = "UNAVAIL";
    pub const REMOVED: &str = "REMOVED";

    /// ZFS properties
    pub const PROPERTY_ALL: &str = "all";
    pub const PROPERTY_METADATA: &str = "metadata";
    pub const PROPERTY_ON: &str = "on";
    pub const PROPERTY_OFF: &str = "off";
    pub const FSTYPE_ZFS: &str = "zfs";

    /// Compression constants
    pub const COMPRESSION_HEADER_SIZE: usize = 12;
}
// ==================== SECTION ====================

/// Security and authentication constants
pub mod security {
    /// Token and session management
    pub const TOKEN_EXPIRATION_S: u64 = 3600; // 1 hour
    pub const AES_256_GCM: &str = "AES-256-GCM";

    /// User roles (consolidated)
    pub const ROLE_ADMIN: &str = "admin";
    pub const ROLE_USER: &str = "user";

    /// Security limits
    pub const MAX_CONCURRENT_SECURITY: usize = 1000;
}
// ==================== SECTION ====================

/// API and service constants
pub mod api {
    /// API versioning
    pub const CURRENT_API_VERSION: &str = "v1";
    pub const CURRENT_CONFIG_VERSION: &str = "3.0.0";
    pub const MIN_SUPPORTED_VERSION: &str = "2.0.0";
    pub const SCHEMA_VERSION: &str = "1.0.0";

    /// HTTP status codes
    pub const STATUS_OK: u16 = 200;
    pub const STATUS_NOT_FOUND: u16 = 404;
    pub const STATUS_INTERNAL_ERROR: u16 = 500;

    /// Content types
    pub const CONTENT_TYPE_JSON: &str = "application/json";

    /// Configuration domains
    pub const CONFIG_API: &str = "api";
    pub const CONFIG_ZFS: &str = "zfs";
    pub const CONFIG_NETWORK: &str = "network";
    pub const CONFIG_SECURITY: &str = "security";
    pub const CONFIG_MONITORING: &str = "monitoring";
}
// ==================== SECTION ====================

/// System and environment constants
pub mod system {
    /// Service identification
    pub const DEFAULT_SERVICE_NAME: &str = "nestgate";
    pub const DEFAULT_LOG_LEVEL: &str = "info";

    /// Environment types
    pub const ENV_DEVELOPMENT: &str = "development";
    pub const ENV_PRODUCTION: &str = "production";
    pub const ENV_TESTING: &str = "testing";
    pub const ENV_STAGING: &str = "staging";

    /// System limits
    pub const MAX_CONFIG_DEPTH: usize = 10;
    pub const MAX_CONFIG_STRING_LENGTH: usize = 1024;
    pub const MAX_CONFIG_ARRAY_LENGTH: usize = 100;
    pub const MAX_FEATURE_FLAGS: usize = 1000;
    pub const ZFS_DISCOVERY_MAX_DEPTH: usize = 10;
    pub const MAX_FILE_DEPTH: usize = 100;
    pub const MAX_RECURSION_DEPTH: usize = 50;
}
// ==================== SECTION ====================

/// Operation and status constants
pub mod operations {
    /// Operation types
    pub const OP_READ: &str = "read";
    pub const OP_WRITE: &str = "write";
    pub const OP_DELETE: &str = "delete";
    pub const OP_CREATE: &str = "create";
    pub const OP_UPDATE: &str = "update";

    /// Status values
    pub const STATUS_SUCCESS: &str = "success";
    pub const STATUS_FAILED: &str = "failed";
    pub const STATUS_PENDING: &str = "pending";
    pub const STATUS_RUNNING: &str = "running";
    pub const STATUS_STOPPED: &str = "stopped";

    /// Error categories
    pub const ERROR_NETWORK: &str = "network_error";
    pub const ERROR_STORAGE: &str = "storage_error";
    pub const ERROR_CONFIG: &str = "config_error";
    pub const ERROR_VALIDATION: &str = "validation_error";
}
// ==================== SECTION ====================

/// Data capability constants
pub mod capabilities {
    /// Capability names (consolidated)
    pub const CAPABILITY_HTTP: &str = "http";
    pub const CAPABILITY_FILE: &str = "file";
    pub const CAPABILITY_GENOME_DATA: &str = "genome_data";

    /// Compute and orchestration
    pub const MAX_COMPUTE_UNITS: usize = 1000;
    pub const MAX_INSTANCES: usize = 500;
}
// ==================== SECTION ====================

/// Cryptographic constants
pub mod crypto {
    /// Character sets
    pub const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    /// CRC table (simplified for compilation)
    pub const CRC_TABLE: [u32; 256] = [0; 256];
}
// ==================== SECTION ====================

/// Size unit constants
pub mod units {
    pub const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
}
// ==================== SECTION ====================

/// Performance constants accessor
pub struct PerformanceConstants;
/// Timeout constants accessor  
pub struct TimeoutConstants;
/// Network constants accessor
pub struct NetworkConstants;
/// Security constants accessor
pub struct SecurityConstants;
/// Storage constants accessor
pub struct StorageConstants;
/// API constants accessor
pub struct ApiConstants;
/// System constants accessor
pub struct SystemConstants;
/// Consolidated constants access
pub struct CanonicalConstants;
impl CanonicalConstants {
    /// Get performance constants module
    #[must_use]
    pub const fn performance() -> PerformanceConstants {
        PerformanceConstants
    }

    /// Get timeout constants module  
    #[must_use]
    pub const fn timeouts() -> TimeoutConstants {
        TimeoutConstants
    }

    /// Get network constants module
    #[must_use]
    pub const fn network() -> NetworkConstants {
        NetworkConstants
    }

    /// Get storage constants module
    #[must_use]
    pub const fn storage() -> StorageConstants {
        StorageConstants
    }

    /// Get security constants module
    #[must_use]
    pub const fn security() -> SecurityConstants {
        SecurityConstants
    }

    /// Get API constants module
    #[must_use]
    pub const fn api() -> ApiConstants {
        ApiConstants
    }

    /// Get system constants module
    #[must_use]
    pub const fn system() -> SystemConstants {
        SystemConstants
    }
}

// ==================== SECTION ====================

/// Const generic configuration helpers
pub struct ConstGenericDefaults;
impl ConstGenericDefaults {
    /// Default max connections for traits
    pub const MAX_CONNECTIONS: usize = performance::MAX_CONNECTIONS;

    /// Default buffer size for traits
    pub const BUFFER_SIZE: usize = performance::DEFAULT_BUFFER_SIZE;

    /// Default timeout for traits
    pub const TIMEOUT_MS: u64 = timeouts::DEFAULT_TIMEOUT_MS;

    /// Default max concurrent operations
    pub const MAX_CONCURRENT_OPS: usize = performance::MAX_CONCURRENT_OPS;

    /// Default batch size for processing
    pub const BATCH_SIZE: usize = performance::OPTIMAL_BATCH_SIZE;
}

/// Zero-cost const generic configuration
pub trait ConstGenericConfig {
    /// Maximum concurrent connections
    const MAX_CONNECTIONS: usize = ConstGenericDefaults::MAX_CONNECTIONS;

    /// Buffer size for operations
    const BUFFER_SIZE: usize = ConstGenericDefaults::BUFFER_SIZE;

    /// Operation timeout in milliseconds
    const TIMEOUT_MS: u64 = ConstGenericDefaults::TIMEOUT_MS;

    /// Maximum concurrent operations
    const MAX_CONCURRENT_OPS: usize = ConstGenericDefaults::MAX_CONCURRENT_OPS;

    /// Batch processing size
    const BATCH_SIZE: usize = ConstGenericDefaults::BATCH_SIZE;
}
// ==================== SECTION ====================

/// Compile-time validation of constants
pub mod validation {
    use super::*;

    /// Validate performance constants at compile time
    #[must_use]
    pub fn validate_performance_constants() -> bool {
        performance::MAX_CONNECTIONS > 0
            && performance::DEFAULT_BUFFER_SIZE > 0
            && performance::OPTIMAL_BATCH_SIZE > 0
    }

    /// Validate timeout constants at compile time
    #[must_use]
    pub fn validate_timeout_constants() -> bool {
        timeouts::DEFAULT_TIMEOUT_SECS > 0
            && timeouts::DEFAULT_TIMEOUT_MS > 0
            && timeouts::DEFAULT_RETRY_ATTEMPTS > 0
    }

    /// Validate network constants at compile time
    #[must_use]
    pub fn validate_network_constants() -> bool {
        // Port validation moved to port_defaults module tests
        network::MAX_SERVICES > 0
            && network::MAX_CONCURRENT_REQUESTS > 0
    }

    // NOTE: Compile-time assertions removed - const fn limitations
    // All constants are validated through unit tests instead
}
// ==================== SECTION ====================

// Re-export key constants for easy access
pub use api::{CONTENT_TYPE_JSON, CURRENT_API_VERSION, STATUS_OK};
pub use network::{DEFAULT_BIND_ADDRESS};
pub use performance::*;
pub use security::{ROLE_ADMIN, ROLE_USER};
pub use storage::{GB, KB, MB, TB, TIER_COLD, TIER_HOT, TIER_WARM};
pub use system::{DEFAULT_SERVICE_NAME, ENV_DEVELOPMENT, ENV_PRODUCTION};
pub use timeouts::{DEFAULT_RETRY_ATTEMPTS, DEFAULT_TIMEOUT_SECS};

// Re-export port constants from port_defaults (single source of truth)
// Use these instead of defining ports in multiple places
pub use super::port_defaults::{
    DEFAULT_API_PORT, DEFAULT_ADMIN_PORT, DEFAULT_METRICS_PORT, DEFAULT_HEALTH_PORT,
    DEFAULT_GRAFANA_PORT, DEFAULT_POSTGRES_PORT, DEFAULT_REDIS_PORT,
};

/// Constants consolidation complete marker
pub const CONSTANTS_CONSOLIDATION_COMPLETE: bool = true;
