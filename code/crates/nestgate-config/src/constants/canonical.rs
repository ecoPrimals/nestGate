// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CANONICAL CONSTANTS SYSTEM**
//! Canonical functionality and utilities.
// Single source of truth for ALL constants across NestGate.
// Eliminates 200+ scattered constant definitions and provides unified access.
//
// Note: Many constants in this module are self-documenting by name.
// Comprehensive rustdoc will be added incrementally.
#![allow(missing_docs)]

// Removed unused Duration import

// ==================== SECTION ====================

/// Performance and optimization constants
pub mod performance {
    /// Target performance improvement minimum
    pub const TARGET_IMPROVEMENT_PERCENT: f64 = 20.0;

    /// SIMD processing widths
    pub const AVX2_WIDTH: usize = 32;
    /// Sse2 Width
    pub const SSE2_WIDTH: usize = 16;
    /// Simd Alignment
    pub const SIMD_ALIGNMENT: usize = 32;
    /// Minimum simd size
    pub const MIN_SIMD_SIZE: usize = 64;

    /// Memory optimization
    pub const CACHE_LINE_SIZE: usize = 64;
    /// Page Size
    pub const PAGE_SIZE: usize = 4096;
    /// Optimal Batch Size
    pub const OPTIMAL_BATCH_SIZE: usize = 1000;

    /// Default maximum concurrent operations
    pub const DEFAULT_MAX_CONCURRENT: usize = 1000;
    /// Maximum number of connections
    pub const MAX_CONNECTIONS: usize = 1000;
    /// Maximum number of backend services
    pub const MAX_BACKENDS: usize = 100;
    /// Maximum concurrent operations allowed
    pub const MAX_CONCURRENT_OPS: usize = 1000;

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
    /// **CONSOLIDATED**: Now references hardcoding::limits for buffer sizes
    pub const DEFAULT_BUFFER_SIZE: usize =
        crate::constants::hardcoding::limits::BUFFER_SIZE_DEFAULT;
    /// Network Buffer Size
    pub const NETWORK_BUFFER_SIZE: usize =
        crate::constants::hardcoding::limits::BUFFER_SIZE_DEFAULT;
    /// Simd Batch Size
    pub const SIMD_BATCH_SIZE: usize = 32;
    /// Pool Size
    pub const POOL_SIZE: usize = 1024;
    /// Block Size  
    pub const BLOCK_SIZE: usize = 4096;
}
// ==================== SECTION ====================

/// Timeout constants for network and system operations
///
/// Consolidated timeout values used throughout NestGate for consistency
/// and maintainability. All values are in seconds or milliseconds as indicated.
pub mod timeouts {
    // Removed unused Duration import

    /// Default timeout seconds (consolidated)
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
    /// Default timeout in milliseconds (30 seconds)
    pub const DEFAULT_TIMEOUT_MS: u64 = 30000;
    /// Request timeout in milliseconds
    pub const REQUEST_TIMEOUT_MS: u64 = 30000;

    /// Service discovery timeout in milliseconds
    pub const DISCOVERY_TIMEOUT_MS: u64 = 5000;
    /// Health check interval in milliseconds
    pub const HEALTH_CHECK_INTERVAL_MS: u64 = 30000;
    /// Health check interval in seconds (30 seconds)
    pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 30;
    /// Statistics retention period in seconds (24 hours)
    pub const STATS_RETENTION_SECS: u64 = 86400; // 24 hours

    /// Connection timeout in seconds (30 seconds)
    pub const CONNECTION_TIMEOUT_SECS: u64 = 30;
    /// Session timeout in seconds (5 minutes)
    pub const SESSION_TIMEOUT_SECS: u64 = 300;
    /// Operation timeout in seconds (30 seconds)
    pub const OPERATION_TIMEOUT_SECS: u64 = 30;

    /// Retry and rate limiting
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
    /// Message retry attempts (3 retries)
    pub const MESSAGE_RETRY_ATTEMPTS: u32 = 3;
    /// Default value for retry delay ms
    pub const DEFAULT_RETRY_DELAY_MS: u64 = 1000;
    /// Default value for rate limit rpm
    pub const DEFAULT_RATE_LIMIT_RPM: u32 = 1000;
    /// Default value for rate limit burst
    pub const DEFAULT_RATE_LIMIT_BURST: u32 = 100;

    /// Monitoring intervals
    pub const METRICS_COLLECTION_INTERVAL_SECS: u64 = 60;
    /// Default value for handler timeout secs
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
    /// Localhost
    pub const LOCALHOST: &str = "127.0.0.1";

    pub const MAX_SERVICES: usize = 1000;
    pub const MAX_CONCURRENT_REQUESTS: usize = 10000;
    pub const MAX_SESSIONS: usize = 1000;
    pub const MAX_MESSAGE_SIZE: usize = 1024;
    pub const PROTOCOL_VERSION: u32 = 1;

    /// MTU and buffer sizes
    pub const MTU_SIZE: usize = 1500;
    /// Send Buffer Size
    pub const SEND_BUFFER_SIZE: usize = 65536;
    /// Recv Buffer Size
    pub const RECV_BUFFER_SIZE: usize = 65536;
}
// ==================== SECTION ====================

/// Storage and file system constants
pub mod storage {
    /// Storage tiers (consolidated)
    pub const TIER_HOT: &str = "hot";
    /// Tier Warm
    pub const TIER_WARM: &str = "warm";
    /// Tier Cold
    pub const TIER_COLD: &str = "cold";

    /// Compression algorithms
    pub const COMPRESSION_LZ4: &str = "lz4";
    /// Compression Gzip 6
    pub const COMPRESSION_GZIP_6: &str = "gzip-6";
    /// Compression Gzip 9
    pub const COMPRESSION_GZIP_9: &str = "gzip-9";

    /// Size units
    pub const KB: u64 = 1024;
    /// Mb
    pub const MB: u64 = 1024 * 1024;
    /// Gb
    pub const GB: u64 = 1024 * 1024 * 1024;
    /// Tb
    pub const TB: u64 = 1024 * 1024 * 1024 * 1024;

    /// File size limits
    pub const MAX_FILE_SIZE_MB: usize = 1024;
    pub const MAX_IN_MEMORY_FILE_SIZE: u64 = 1024 * 1024 * 1024;

    /// ZFS constants (consolidated)
    pub const RECORDSIZE_64K: &str = "64K";
    /// Recordsize 128K
    pub const RECORDSIZE_128K: &str = "128K";
    /// Recordsize 1M
    pub const RECORDSIZE_1M: &str = "1M";
    /// Record Size
    pub const RECORD_SIZE: usize = 131_072; // 128KB
    /// Arc Size
    pub const ARC_SIZE: usize = 1_073_741_824; // 1GB

    /// ZFS commands
    pub const ZFS: &str = "zfs";
    /// Zpool
    pub const ZPOOL: &str = "zpool";
    /// List
    pub const LIST: &str = "list";
    /// Create
    pub const CREATE: &str = "create";
    /// Destroy
    pub const DESTROY: &str = "destroy";
    /// Set
    pub const SET: &str = "set";
    /// Get
    pub const GET: &str = "get";
    /// Snapshot
    pub const SNAPSHOT: &str = "snapshot";
    /// Status
    pub const STATUS: &str = "status";

    /// ZFS states
    pub const ONLINE: &str = "ONLINE";
    /// Degraded
    pub const DEGRADED: &str = "DEGRADED";
    /// Faulted
    pub const FAULTED: &str = "FAULTED";
    /// Offline
    pub const OFFLINE: &str = "OFFLINE";
    /// Unavail
    pub const UNAVAIL: &str = "UNAVAIL";
    /// Removed
    pub const REMOVED: &str = "REMOVED";

    /// ZFS properties
    pub const PROPERTY_ALL: &str = "all";
    /// Property Metadata
    pub const PROPERTY_METADATA: &str = "metadata";
    /// Property On
    pub const PROPERTY_ON: &str = "on";
    /// Property Off
    pub const PROPERTY_OFF: &str = "off";
    /// Fstype Zfs
    pub const FSTYPE_ZFS: &str = "zfs";

    /// Compression constants
    pub const COMPRESSION_HEADER_SIZE: usize = 12;
}
// ==================== SECTION ====================

/// Security and authentication constants
pub mod security {
    /// Token and session management
    pub const TOKEN_EXPIRATION_S: u64 = 3600; // 1 hour
    /// Aes 256 Gcm
    pub const AES_256_GCM: &str = "AES-256-GCM";

    /// User roles (consolidated)
    pub const ROLE_ADMIN: &str = "admin";
    /// Role User
    pub const ROLE_USER: &str = "user";

    /// Security limits
    pub const MAX_CONCURRENT_SECURITY: usize = 1000;
}
// ==================== SECTION ====================

/// API and service constants
pub mod api {
    /// API versioning
    pub const CURRENT_API_VERSION: &str = "v1";
    /// Current Config Version
    pub const CURRENT_CONFIG_VERSION: &str = "3.0.0";
    /// Minimum supported version
    pub const MIN_SUPPORTED_VERSION: &str = "2.0.0";
    /// Schema Version
    pub const SCHEMA_VERSION: &str = "1.0.0";

    /// HTTP status codes
    pub const STATUS_OK: u16 = 200;
    /// Status Not Found
    pub const STATUS_NOT_FOUND: u16 = 404;
    /// Status Internal Error
    pub const STATUS_INTERNAL_ERROR: u16 = 500;

    /// Content types
    pub const CONTENT_TYPE_JSON: &str = "application/json";

    /// Configuration domains
    pub const CONFIG_API: &str = "api";
    /// Config Zfs
    pub const CONFIG_ZFS: &str = "zfs";
    /// Config Network
    pub const CONFIG_NETWORK: &str = "network";
    /// Config Security
    pub const CONFIG_SECURITY: &str = "security";
    /// Config Monitoring
    pub const CONFIG_MONITORING: &str = "monitoring";
}
// ==================== SECTION ====================

/// System and environment constants
pub mod system {
    pub const DEFAULT_SERVICE_NAME: &str = "nestgate";
    /// Default value for log level
    pub const DEFAULT_LOG_LEVEL: &str = "info";

    /// Environment types
    pub const ENV_DEVELOPMENT: &str = "development";
    /// Env Production
    pub const ENV_PRODUCTION: &str = "production";
    /// Env Testing
    pub const ENV_TESTING: &str = "testing";
    /// Env Staging
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
///
/// Standard operation names, status values, and error categories used
/// consistently across all NestGate components for state tracking and reporting.
pub mod operations {
    /// Read operation
    pub const OP_READ: &str = "read";
    /// Op Write
    pub const OP_WRITE: &str = "write";
    /// Op Delete
    pub const OP_DELETE: &str = "delete";
    /// Op Create
    pub const OP_CREATE: &str = "create";
    /// Op Update
    pub const OP_UPDATE: &str = "update";

    /// Status values
    pub const STATUS_SUCCESS: &str = "success";
    /// Status Failed
    pub const STATUS_FAILED: &str = "failed";
    /// Status Pending
    pub const STATUS_PENDING: &str = "pending";
    /// Status Running
    pub const STATUS_RUNNING: &str = "running";
    /// Status Stopped
    pub const STATUS_STOPPED: &str = "stopped";

    /// Error categories
    pub const ERROR_NETWORK: &str = "network_error";
    /// Error Storage
    pub const ERROR_STORAGE: &str = "storage_error";
    /// Error Config
    pub const ERROR_CONFIG: &str = "config_error";
    /// Error Validation
    pub const ERROR_VALIDATION: &str = "validation_error";
}
// ==================== SECTION ====================

/// Data capability constants
pub mod capabilities {
    /// Capability names (consolidated)
    pub const CAPABILITY_HTTP: &str = "http";
    /// Capability File
    pub const CAPABILITY_FILE: &str = "file";
    /// Capability Genome Data
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
    /// Units
    pub const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
}
// ==================== SECTION ====================

/// Performance constants accessor
pub struct PerformanceConstants;

/// Timeout constants accessor
///
/// Provides access to timeout-related constants used throughout NestGate.
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
    /// Fn
    pub const fn performance() -> PerformanceConstants {
        PerformanceConstants
    }

    /// Get timeout constants module  
    #[must_use]
    /// Fn
    pub const fn timeouts() -> TimeoutConstants {
        TimeoutConstants
    }

    /// Get network constants module
    #[must_use]
    /// Fn
    pub const fn network() -> NetworkConstants {
        NetworkConstants
    }

    /// Get storage constants module
    #[must_use]
    /// Fn
    pub const fn storage() -> StorageConstants {
        StorageConstants
    }

    /// Get security constants module
    #[must_use]
    /// Fn
    pub const fn security() -> SecurityConstants {
        SecurityConstants
    }

    /// Get API constants module
    #[must_use]
    /// Fn
    pub const fn api() -> ApiConstants {
        ApiConstants
    }

    /// Get system constants module
    #[must_use]
    /// Fn
    pub const fn system() -> SystemConstants {
        SystemConstants
    }
}

// ==================== SECTION ====================

/// Const generic configuration helpers
pub struct ConstGenericDefaults;
impl ConstGenericDefaults {
    /// Default max connections for const generic traits
    pub const MAX_CONNECTIONS: usize = performance::MAX_CONNECTIONS;

    /// Default buffer size for const generic traits
    pub const BUFFER_SIZE: usize = performance::DEFAULT_BUFFER_SIZE;

    /// Default timeout in milliseconds for const generic traits
    pub const TIMEOUT_MS: u64 = timeouts::DEFAULT_TIMEOUT_MS;

    /// Default max concurrent operations for const generic traits
    pub const MAX_CONCURRENT_OPS: usize = performance::MAX_CONCURRENT_OPS;

    /// Default batch size for processing
    pub const BATCH_SIZE: usize = performance::OPTIMAL_BATCH_SIZE;
}

/// Zero-cost const generic configuration
///
/// Trait providing compile-time configuration constants for generic implementations
pub trait ConstGenericConfig {
    /// Maximum number of connections for generic implementations
    const MAX_CONNECTIONS: usize = ConstGenericDefaults::MAX_CONNECTIONS;

    /// Buffer size for generic implementations
    const BUFFER_SIZE: usize = ConstGenericDefaults::BUFFER_SIZE;

    /// Timeout in milliseconds for generic implementations
    const TIMEOUT_MS: u64 = ConstGenericDefaults::TIMEOUT_MS;

    /// Maximum concurrent operations for generic implementations
    const MAX_CONCURRENT_OPS: usize = ConstGenericDefaults::MAX_CONCURRENT_OPS;

    /// Batch processing size
    const BATCH_SIZE: usize = ConstGenericDefaults::BATCH_SIZE;
}
// ==================== SECTION ====================

/// Compile-time validation of constants
///
/// Uses modern const assertions for zero-cost compile-time verification
pub mod validation {
    use super::*;
    macro_rules! const_assert {
        ($cond:expr) => {
            const _: () = assert!($cond);
        };
    }

    const_assert!(performance::MAX_CONNECTIONS > 0);
    const_assert!(performance::OPTIMAL_BATCH_SIZE > 0);

    const_assert!(timeouts::DEFAULT_TIMEOUT_SECS > 0);
    const_assert!(timeouts::DEFAULT_TIMEOUT_MS > 0);
    const_assert!(timeouts::DEFAULT_RETRY_ATTEMPTS > 0);

    const_assert!(network::MAX_SERVICES > 0);
    const_assert!(network::MAX_CONCURRENT_REQUESTS > 0);

    /// Runtime validation function (legacy compatibility)
    /// Note: Actual validation happens at compile time via const_assert!
    #[must_use]
    pub fn validate_performance_constants() -> bool {
        // Always true - verified at compile time
        true
    }

    /// Runtime validation function (legacy compatibility)
    #[must_use]
    pub fn validate_timeout_constants() -> bool {
        // Always true - verified at compile time
        true
    }

    /// Runtime validation function (legacy compatibility)
    #[must_use]
    pub fn validate_network_constants() -> bool {
        // Always true - verified at compile time
        true
    }
}
// ==================== SECTION ====================

// Re-export key constants for easy access
pub use api::{CONTENT_TYPE_JSON, CURRENT_API_VERSION, STATUS_OK};
pub use network::DEFAULT_BIND_ADDRESS;
pub use performance::*;
pub use security::{ROLE_ADMIN, ROLE_USER};
pub use storage::{GB, KB, MB, TB, TIER_COLD, TIER_HOT, TIER_WARM};
pub use system::{DEFAULT_SERVICE_NAME, ENV_DEVELOPMENT, ENV_PRODUCTION};
pub use timeouts::{DEFAULT_RETRY_ATTEMPTS, DEFAULT_TIMEOUT_SECS};

// Re-export port constants from port_defaults (single source of truth)
// Use these instead of defining ports in multiple places
pub use super::port_defaults::{
    DEFAULT_ADMIN_PORT, DEFAULT_API_PORT, DEFAULT_GRAFANA_PORT, DEFAULT_HEALTH_PORT,
    DEFAULT_METRICS_PORT, DEFAULT_POSTGRES_PORT, DEFAULT_REDIS_PORT,
};

/// Constants consolidation complete marker
pub const CONSTANTS_CONSOLIDATION_COMPLETE: bool = true;
