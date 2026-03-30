// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// - Single source of truth for all constants
// - Consistent naming patterns
// - Domain-organized hierarchy
// - Environment-driven configuration
// - Compile-time optimization support

// Duration imports moved to specific modules where needed

/// **PERFORMANCE CONSTANTS**
///
/// Performance tuning and optimization constants
/// **CONSOLIDATED**: Now references hardcoding module for common values
pub mod performance {
    include!("performance.rs");
}

/// **NETWORK CONSTANTS**
///
/// Network configuration and timeout constants
pub mod network {
    include!("network.rs");
}

/// **STORAGE CONSTANTS**
///
/// Constants for storage operations and file size classifications
pub mod storage {
    include!("storage.rs");
}

/// **SECURITY CONSTANTS**
///
/// Security configuration constants
pub mod security {
    include!("security.rs");
}

/// **SYSTEM CONSTANTS**
///
/// System-level configuration constants
pub mod system {
    include!("system.rs");
}

/// **LIMITS CONSTANTS**
///
/// System limits and boundaries
pub mod limits {
    include!("limits.rs");
}

/// **API CONSTANTS**
///
/// API-specific constants including status codes and content types
pub mod api {
    include!("api.rs");
}

/// **MONITORING CONSTANTS**
///
/// Monitoring and metrics constants
pub mod monitoring {
    include!("monitoring.rs");
}

/// **TESTING CONSTANTS**
///
/// Test configuration constants
pub mod testing {
    include!("testing.rs");
}

/// **ZFS CONSTANTS**
///
/// ZFS-specific constants for pool management and operations
pub mod zfs {
    include!("zfs.rs");
}

/// **HANDLER CONSTANTS**
///
/// Constants for API handler configurations
pub mod handlers {
    include!("handlers.rs");
}

/// **TIMEOUT CONSTANTS**
///
/// Comprehensive timeout constants for all operations
pub mod timeouts {
    include!("timeouts.rs");
}

/// **SIMD AND OPTIMIZATION CONSTANTS**
///
/// Constants for SIMD operations and performance optimization
pub mod simd {
    include!("simd.rs");
}

/// **ZERO-COST ARCHITECTURE CONSTANTS**
///
/// Constants for zero-cost architecture patterns and generic parameters
pub mod zero_cost {
    include!("zero_cost.rs");
}

/// **SERVICE LIMITS CONSTANTS**
///
/// Constants for service limits and capacity planning
pub mod service_limits {
    include!("service_limits.rs");
}

/// **ZFS OPERATION CONSTANTS**
///
/// Constants specific to ZFS operations and limits
pub mod zfs_operations {
    include!("zfs_operations.rs");
}

/// **CACHE CONSTANTS**
///
/// Constants for caching systems and TTL values
pub mod cache {
    include!("cache.rs");
}

/// **DEVELOPMENT ENVIRONMENT CONSTANTS**
///
/// Constants for development and smart defaults
pub mod development {
    include!("development.rs");
}

// ==================== SECTION ====================

/// Macro for compile-time constant access
#[macro_export]
macro_rules! const_access {
    (network::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::network::$const
    };
    (storage::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::storage::$const
    };
    (security::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::security::$const
    };
    (performance::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::performance::$const
    };
    (system::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::system::$const
    };
    (api::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::api::$const
    };
    (testing::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::testing::$const
    };
    (monitoring::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::monitoring::$const
    };
    (zfs::$const:ident) => {
        $crate::canonical_modernization::canonical_constants::zfs::$const
    };
}
// ==================== SECTION ====================

/// **CANONICAL CONSTANTS** - Main constants container
pub struct CanonicalConstants;
impl CanonicalConstants {
    /// Network constants
    pub const NETWORK: NetworkConstants = NetworkConstants;
    /// Storage constants
    pub const STORAGE: StorageConstants = StorageConstants;
    /// Security constants
    pub const SECURITY: SecurityConstants = SecurityConstants;
    /// Performance constants
    pub const PERFORMANCE: PerformanceConstants = PerformanceConstants;
    /// System constants
    pub const SYSTEM: SystemConstants = SystemConstants;
    /// API constants
    pub const API: ApiConstants = ApiConstants;
}

/// Network constants container
pub struct NetworkConstants;
/// Storage constants container
pub struct StorageConstants;
/// Security constants container
pub struct SecurityConstants;
/// Performance constants container
pub struct PerformanceConstants;
/// System constants container
pub struct SystemConstants;
/// API constants container
pub struct ApiConstants;
// ==================== SECTION ====================

/// Zero-cost configuration marker
pub struct ZeroCostConfig;
/// Const generic configuration marker (renamed to avoid conflict)
pub struct ConstGenericConfig;
// ==================== SECTION ====================

// Network constants (avoiding duplicate imports)
pub use network::{DEFAULT_API_PORT, DEFAULT_BIND_ADDRESS, LOCALHOST, REQUEST_TIMEOUT_SECS};

// Storage constants
pub use storage::{
    COMPRESSION_GZIP_6, COMPRESSION_GZIP_9, COMPRESSION_LZ4, TIER_COLD, TIER_HOT, TIER_WARM,
};
pub use storage::{GB, KB, MB, TB};

// Security constants (avoiding duplicate imports)
pub use security::{AES_256_GCM, TOKEN_EXPIRATION_S};
pub use security::{ROLE_ADMIN, ROLE_USER};

// System constants
pub use system::{DEFAULT_LOG_LEVEL, DEFAULT_SERVICE_NAME, DEFAULT_TIMEOUT_SECS};
pub use system::{ENV_DEVELOPMENT, ENV_PRODUCTION};

// API constants
pub use api::CONTENT_TYPE_JSON;
pub use api::{CURRENT_API_VERSION, STATUS_INTERNAL_ERROR, STATUS_NOT_FOUND, STATUS_OK};

// ZFS constants
pub use zfs::{CREATE, DESTROY, GET, LIST, SET, SNAPSHOT, STATUS, ZFS, ZPOOL};
pub use zfs::{DEGRADED, FAULTED, OFFLINE, ONLINE, REMOVED, UNAVAIL};
pub use zfs::{FSTYPE_ZFS, RECORDSIZE_1M, RECORDSIZE_64K, RECORDSIZE_128K};
pub use zfs::{PROPERTY_ALL, PROPERTY_METADATA, PROPERTY_OFF, PROPERTY_ON};

// Limits constants
pub use limits::{MAX_FILE_DEPTH, MAX_RECURSION_DEPTH, ZFS_DISCOVERY_MAX_DEPTH};

// Handler constants
pub use handlers::{DEFAULT_HANDLER_TIMEOUT_SECS, MAX_CONCURRENT_REQUESTS};
pub use handlers::{DEFAULT_RATE_LIMIT_BURST, DEFAULT_RATE_LIMIT_RPM};
pub use handlers::{DEFAULT_RETRY_ATTEMPTS, DEFAULT_RETRY_DELAY_MS};
pub use handlers::{HEALTH_CHECK_INTERVAL_SECS, METRICS_COLLECTION_INTERVAL_SECS};

// Timeout constants (avoiding duplicates with network module)
pub use timeouts::{CONNECTION_TIMEOUT, HEALTH_CHECK_TIMEOUT, OPERATION_TIMEOUT, REQUEST_TIMEOUT};
pub use timeouts::{CONNECTION_TIMEOUT_SECS, HEALTH_CHECK_TIMEOUT_SECS};
pub use timeouts::{DISCOVERY_TIMEOUT_SECS, OPERATION_TIMEOUT_SECS, SESSION_TIMEOUT_SECS};
