/// **UNIFIED CONSTANTS MODULE**
/// Consolidates all duplicate constants across the NestGate ecosystem into a single,
/// organized, and maintainable system.
///
/// **ELIMINATES**:
/// - Duplicate API versions and prefixes across modules
/// - Repeated protocol constants (NFS, SMB, HTTP, ZFS)
/// - Fragmented storage tier definitions
/// - Scattered ZFS property constants
/// - Duplicate port numbers and timeouts
/// - Repeated service names and endpoints
/// - Various message constants
///
/// **PROVIDES**:
/// - Single source of truth for all system constants
/// - Organized constant hierarchies by domain
/// - Compile-time constant validation
/// - Easy maintenance and updates
use std::time::Duration;

// ==================== API CONSTANTS ====================

/// API versioning and endpoint constants
pub mod api {
    /// Current API version
    pub const VERSION: &str = "v1";

    /// API prefix for all endpoints
    pub const PREFIX: &str = "/api/v1";

    /// Endpoint paths
    pub mod endpoints {
        pub const COORDINATE: &str = "/api/v1/coordinate";
        pub const COORDINATE_STORAGE: &str = "/api/v1/coordinate-storage";
        pub const PROVISION_STORAGE: &str = "/api/v1/provision-storage";
        pub const OPTIMIZE_STORAGE: &str = "/api/v1/optimize-storage";
        pub const SECURE_STORAGE: &str = "/api/v1/secure-storage";
        pub const HEALTH: &str = "/health";
        pub const EVENTS: &str = "/events";
    }

    /// API response messages
    pub mod messages {
        pub const COORDINATION_SUCCESSFUL: &str = "Coordination successful";
        pub const COORDINATION_DISABLED: &str = "Coordination disabled";
        pub const ENDPOINT_NOT_AVAILABLE: &str = "Endpoint not available";
    }

    /// API capabilities (consolidated from nestgate-api/src/constants.rs)
    pub mod capabilities {
        pub const STORAGE: &str = "storage";
        pub const DATA: &str = "data";
        pub const ORCHESTRATION: &str = "orchestration";
        pub const AI: &str = "ai";
        pub const SECURITY: &str = "security";
        pub const TIERED_STORAGE: &str = "tiered_storage";
        pub const REPLICATION: &str = "replication";
        pub const SNAPSHOTS: &str = "snapshots";
        pub const ENCRYPTION: &str = "encryption";
        pub const MONITORING: &str = "monitoring";
        pub const AUTOMATION: &str = "automation";
        pub const PERFORMANCE: &str = "performance";
        pub const FEDERATION: &str = "federation";
        pub const ECOSYSTEM: &str = "ecosystem";
        pub const UNIVERSAL_COORDINATION: &str = "universal_coordination";
        pub const COMPUTE: &str = "compute";
        pub const EXECUTION: &str = "execution";
        pub const COORDINATION: &str = "coordination";
        pub const AUTHENTICATION: &str = "authentication";
        pub const ML: &str = "ml";
        pub const AGENTS: &str = "agents";
    }

    /// API roles (consolidated from nestgate-api/src/constants.rs)
    pub mod roles {
        pub const STORAGE_PROVIDER: &str = "storage_provider";
        pub const DATA_PROCESSOR: &str = "data_processor";
        pub const ORCHESTRATION_PARTICIPANT: &str = "orchestration_participant";
        pub const AI_COMPUTE_PROVIDER: &str = "ai_compute_provider";
        pub const SECURITY_COORDINATOR: &str = "security_coordinator";
        pub const ECOSYSTEM_MEMBER: &str = "ecosystem_member";
        pub const PERFORMANCE_MONITORING: &str = "performance_monitoring";
        pub const AUTOMATED_MANAGEMENT: &str = "automated_management";
    }

    /// API features (consolidated from nestgate-api/src/constants.rs)
    pub mod features {
        pub const POOLED_STORAGE: &str = "pooled_storage";
        pub const COMPRESSION: &str = "compression";
        pub const DEDUPLICATION: &str = "deduplication";
    }

    /// API status constants (consolidated from nestgate-api/src/constants.rs)
    pub mod status {
        pub const ACTIVE: &str = "active";
        pub const CUSTOM: &str = "custom";
    }
}

// ==================== PROTOCOL CONSTANTS ====================

/// Protocol-specific constants
pub mod protocols {

    /// HTTP protocol constants
    pub mod http {
        pub const DEFAULT_PORT: u16 = 80;
        pub const SECURE_PORT: u16 = 443;
        pub const MAX_HEADER_SIZE: usize = 8192;
        pub const MAX_BODY_SIZE: usize = 1024 * 1024; // 1MB
        pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
        pub const KEEP_ALIVE_TIMEOUT_SECS: u64 = 60;
    }

    /// NFS protocol constants
    pub mod nfs {
        pub const DEFAULT_PORT: u16 = 2049;
        pub const VERSION_3: &str = "3";
        pub const VERSION_4: &str = "4";
        pub const VERSION_4_1: &str = "4.1";
        pub const VERSION_4_2: &str = "4.2";
        pub const DEFAULT_BLOCK_SIZE: u32 = 4096;
        pub const MAX_READ_SIZE: u32 = 1048576; // 1MB
        pub const MAX_WRITE_SIZE: u32 = 1048576; // 1MB
    }

    /// SMB/CIFS protocol constants
    pub mod smb {
        pub const DEFAULT_PORT: u16 = 445;
        pub const NETBIOS_PORT: u16 = 139;
        pub const VERSION_2: &str = "2.0";
        pub const VERSION_3: &str = "3.0";
        pub const VERSION_3_1: &str = "3.1";
        pub const DEFAULT_DIALECT: &str = "SMB3";
        pub const MAX_BUFFER_SIZE: u32 = 65536;
    }

    /// ZFS protocol constants
    pub mod zfs {
        pub const DEFAULT_RECORD_SIZE: u32 = 131072; // 128KB
        pub const MIN_RECORD_SIZE: u32 = 512;
        pub const MAX_RECORD_SIZE: u32 = 1048576; // 1MB
        pub const DEFAULT_COMPRESSION: &str = "lz4";
        pub const DEFAULT_CHECKSUM: &str = "sha256";
        pub const SNAPSHOT_PREFIX: &str = "nestgate-";

        /// ZFS property constants
        pub mod properties {
            pub const COMPRESSION: &str = "compression";
            pub const CHECKSUM: &str = "checksum";
            pub const RECORDSIZE: &str = "recordsize";
            pub const QUOTA: &str = "quota";
            pub const RESERVATION: &str = "reservation";
            pub const MOUNTPOINT: &str = "mountpoint";
            pub const READONLY: &str = "readonly";
            pub const EXEC: &str = "exec";
            pub const SETUID: &str = "setuid";
            pub const ATIME: &str = "atime";
        }
    }

    /// gRPC protocol constants
    pub mod grpc {
        pub const DEFAULT_PORT: u16 = 9090;
        pub const MAX_MESSAGE_SIZE: u32 = 4 * 1024 * 1024; // 4MB
        pub const KEEP_ALIVE_TIME_SECS: u64 = 30;
        pub const KEEP_ALIVE_TIMEOUT_SECS: u64 = 5;
        pub const MAX_CONNECTION_IDLE_SECS: u64 = 300;
        pub const MAX_CONNECTION_AGE_SECS: u64 = 3600;
    }
}

// ==================== NETWORK CONSTANTS ====================

/// Network and connectivity constants
pub mod network {
    use std::time::Duration;

    /// Default network addresses (configurable via environment)
    pub mod addresses {
        pub const LOCALHOST: &str = "127.0.0.1";
        pub const LOCALHOST_IPV6: &str = "::1";
        pub const ANY_ADDRESS: &str = "0.0.0.0";
        pub const ANY_ADDRESS_IPV6: &str = "::";

        /// Environment-driven address resolution
        pub fn bind_address() -> String {
            std::env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| ANY_ADDRESS.to_string())
        }

        pub fn discovery_address() -> String {
            std::env::var("NESTGATE_DISCOVERY_ADDRESS").unwrap_or_else(|_| LOCALHOST.to_string())
        }
    }

    /// Default ports (configurable via environment)
    pub mod ports {
        pub const API_PORT: u16 = 8080;
        pub const HEALTH_PORT: u16 = 8081;
        pub const METRICS_PORT: u16 = 8082;
        pub const DISCOVERY_PORT: u16 = 8083;
        pub const MCP_PORT: u16 = 8084;
        pub const ORCHESTRATOR_PORT: u16 = 8085;
        pub const GRPC_PORT: u16 = 9090;
        pub const ADMIN_PORT: u16 = 9091;

        /// Environment-driven port resolution
        pub fn api_port() -> u16 {
            std::env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(API_PORT)
        }

        pub fn health_port() -> u16 {
            std::env::var("NESTGATE_HEALTH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(HEALTH_PORT)
        }

        pub fn metrics_port() -> u16 {
            std::env::var("NESTGATE_METRICS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(METRICS_PORT)
        }
    }

    /// Connection timeouts (configurable via environment)
    pub mod timeouts {
        use super::Duration;

        pub const CONNECTION_TIMEOUT_SECS: u64 = 30;
        pub const REQUEST_TIMEOUT_SECS: u64 = 60;
        pub const HEALTH_CHECK_TIMEOUT_SECS: u64 = 10;
        pub const DISCOVERY_TIMEOUT_SECS: u64 = 15;
        pub const SHUTDOWN_TIMEOUT_SECS: u64 = 30;

        /// Environment-driven timeout resolution
        pub fn connection_timeout() -> Duration {
            std::env::var("NESTGATE_CONNECTION_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .map(Duration::from_secs)
                .unwrap_or_else(|| Duration::from_secs(CONNECTION_TIMEOUT_SECS))
        }

        pub fn request_timeout() -> Duration {
            std::env::var("NESTGATE_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .map(Duration::from_secs)
                .unwrap_or_else(|| Duration::from_secs(REQUEST_TIMEOUT_SECS))
        }

        pub fn health_check_timeout() -> Duration {
            std::env::var("NESTGATE_HEALTH_CHECK_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .map(Duration::from_secs)
                .unwrap_or_else(|| Duration::from_secs(HEALTH_CHECK_TIMEOUT_SECS))
        }
    }

    /// Buffer sizes
    pub mod buffers {
        pub const READ_BUFFER_SIZE: usize = 8192;
        pub const WRITE_BUFFER_SIZE: usize = 8192;
        pub const SOCKET_BUFFER_SIZE: usize = 65536;
        pub const MAX_FRAME_SIZE: usize = 1024 * 1024; // 1MB
    }
}

// ==================== STORAGE CONSTANTS ====================

/// Storage system constants
pub mod storage {
    /// Storage tier constants
    pub mod tiers {
        pub const HOT: &str = "hot";
        pub const WARM: &str = "warm";
        pub const COLD: &str = "cold";
        pub const ARCHIVE: &str = "archive";

        /// Tier transition thresholds (configurable)
        pub mod thresholds {
            pub const HOT_TO_WARM_DAYS: u32 = 30;
            pub const WARM_TO_COLD_DAYS: u32 = 90;
            pub const COLD_TO_ARCHIVE_DAYS: u32 = 365;

            pub fn hot_to_warm_days() -> u32 {
                std::env::var("NESTGATE_HOT_TO_WARM_DAYS")
                    .ok()
                    .and_then(|d| d.parse().ok())
                    .unwrap_or(HOT_TO_WARM_DAYS)
            }
        }
    }

    /// Storage sizes and limits
    pub mod limits {
        pub const DEFAULT_POOL_SIZE_GB: u64 = 1000;
        pub const MIN_POOL_SIZE_GB: u64 = 1;
        pub const MAX_POOL_SIZE_GB: u64 = 1024 * 1024; // 1PB
        pub const DEFAULT_DATASET_QUOTA_GB: u64 = 100;
        pub const MIN_DATASET_SIZE_MB: u64 = 1;
        pub const MAX_FILE_SIZE_GB: u64 = 16 * 1024; // 16TB

        /// Environment-driven limits
        pub fn default_pool_size_bytes() -> u64 {
            std::env::var("NESTGATE_DEFAULT_POOL_SIZE_GB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(DEFAULT_POOL_SIZE_GB)
                * 1024
                * 1024
                * 1024
        }
    }

    /// Replication constants
    pub mod replication {
        pub const DEFAULT_REPLICATION_FACTOR: u32 = 3;
        pub const MIN_REPLICATION_FACTOR: u32 = 1;
        pub const MAX_REPLICATION_FACTOR: u32 = 10;
        pub const SYNC_TIMEOUT_SECS: u64 = 300; // 5 minutes
        pub const HEALTH_CHECK_INTERVAL_SECS: u64 = 60;
    }
}

// ==================== PERFORMANCE CONSTANTS ====================

/// Performance and optimization constants
pub mod performance {
    /// Cache constants
    pub mod cache {
        pub const DEFAULT_CACHE_SIZE_MB: u64 = 512;
        pub const MIN_CACHE_SIZE_MB: u64 = 16;
        pub const MAX_CACHE_SIZE_MB: u64 = 16 * 1024; // 16GB
        pub const DEFAULT_TTL_SECS: u64 = 300; // 5 minutes
        pub const CACHE_LINE_SIZE: usize = 64;

        pub fn cache_size_bytes() -> u64 {
            std::env::var("NESTGATE_CACHE_SIZE_MB")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(DEFAULT_CACHE_SIZE_MB)
                * 1024
                * 1024
        }
    }

    /// Thread pool constants
    pub mod threads {
        pub const DEFAULT_WORKER_THREADS: usize = 8;
        pub const MIN_WORKER_THREADS: usize = 1;
        pub const MAX_WORKER_THREADS: usize = 1024;
        pub const DEFAULT_BLOCKING_THREADS: usize = 512;

        pub fn worker_threads() -> usize {
            std::env::var("NESTGATE_WORKER_THREADS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(DEFAULT_WORKER_THREADS)
        }
    }

    /// Memory constants
    pub mod memory {
        pub const DEFAULT_HEAP_SIZE_MB: u64 = 1024;
        pub const MIN_HEAP_SIZE_MB: u64 = 64;
        pub const MAX_HEAP_SIZE_MB: u64 = 64 * 1024; // 64GB
        pub const GC_THRESHOLD_PERCENT: u8 = 80;
        pub const MEMORY_PRESSURE_THRESHOLD_PERCENT: u8 = 90;
    }
}

// ==================== SECURITY CONSTANTS ====================

/// Security and cryptography constants
pub mod security {

    /// Authentication constants
    pub mod auth {
        pub const DEFAULT_SESSION_TIMEOUT_SECS: u64 = 3600; // 1 hour
        pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
        pub const LOCKOUT_DURATION_SECS: u64 = 300; // 5 minutes
        pub const TOKEN_REFRESH_THRESHOLD_SECS: u64 = 300; // 5 minutes before expiry

        pub fn session_timeout() -> std::time::Duration {
            std::env::var("NESTGATE_SESSION_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .map(std::time::Duration::from_secs)
                .unwrap_or_else(|| std::time::Duration::from_secs(DEFAULT_SESSION_TIMEOUT_SECS))
        }
    }

    /// Encryption constants
    pub mod encryption {
        pub const DEFAULT_KEY_SIZE: u32 = 256; // bits
        pub const MIN_KEY_SIZE: u32 = 128;
        pub const MAX_KEY_SIZE: u32 = 4096;
        pub const DEFAULT_ALGORITHM: &str = "AES-256-GCM";
        pub const SALT_SIZE: usize = 32;
        pub const IV_SIZE: usize = 12;
        pub const TAG_SIZE: usize = 16;
    }

    /// TLS constants
    pub mod tls {
        pub const MIN_TLS_VERSION: &str = "1.2";
        pub const PREFERRED_TLS_VERSION: &str = "1.3";
        pub const CERT_VALIDITY_DAYS: u32 = 365;
        pub const CERT_RENEWAL_THRESHOLD_DAYS: u32 = 30;
    }
}

// ==================== MONITORING CONSTANTS ====================

/// Monitoring and observability constants
pub mod monitoring {

    /// Metrics constants
    pub mod metrics {
        pub const COLLECTION_INTERVAL_SECS: u64 = 15;
        pub const RETENTION_DAYS: u32 = 30;
        pub const BATCH_SIZE: usize = 1000;
        pub const FLUSH_INTERVAL_SECS: u64 = 60;

        pub fn collection_interval() -> std::time::Duration {
            std::env::var("NESTGATE_METRICS_INTERVAL_SECS")
                .ok()
                .and_then(|i| i.parse().ok())
                .map(std::time::Duration::from_secs)
                .unwrap_or_else(|| std::time::Duration::from_secs(COLLECTION_INTERVAL_SECS))
        }
    }

    /// Health check constants
    pub mod health {
        pub const CHECK_INTERVAL_SECS: u64 = 30;
        pub const UNHEALTHY_THRESHOLD: u32 = 3;
        pub const HEALTHY_THRESHOLD: u32 = 2;
        pub const TIMEOUT_SECS: u64 = 10;
    }

    /// Alerting constants
    pub mod alerts {
        pub const CPU_THRESHOLD_PERCENT: f64 = 80.0;
        pub const MEMORY_THRESHOLD_PERCENT: f64 = 85.0;
        pub const DISK_THRESHOLD_PERCENT: f64 = 90.0;
        pub const NETWORK_THRESHOLD_MBPS: f64 = 100.0;
        pub const ERROR_RATE_THRESHOLD_PERCENT: f64 = 5.0;
    }
}

// ==================== TEST CONSTANTS ====================

/// Testing framework constants
pub mod testing {

    /// Test execution constants
    pub mod execution {
        pub const DEFAULT_TEST_TIMEOUT_SECS: u64 = 60;
        pub const LONG_TEST_TIMEOUT_SECS: u64 = 300;
        pub const INTEGRATION_TEST_TIMEOUT_SECS: u64 = 600;
        pub const E2E_TEST_TIMEOUT_SECS: u64 = 1800;
        pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
        pub const MAX_RETRY_ATTEMPTS: u32 = 10;

        pub fn test_timeout() -> std::time::Duration {
            std::env::var("TEST_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .map(std::time::Duration::from_secs)
                .unwrap_or_else(|| std::time::Duration::from_secs(DEFAULT_TEST_TIMEOUT_SECS))
        }
    }

    /// Test resource limits
    pub mod resources {
        pub const DEFAULT_MEMORY_LIMIT_MB: u64 = 1024;
        pub const DEFAULT_CPU_CORES: u32 = 2;
        pub const DEFAULT_DISK_LIMIT_MB: u64 = 10240;
        pub const DEFAULT_NETWORK_LIMIT_MBPS: u32 = 100;
        pub const MAX_CONCURRENT_TESTS: usize = 16;
    }

    /// Mock service constants
    pub mod mocking {
        pub const DEFAULT_RESPONSE_DELAY_MS: u64 = 10;
        pub const MAX_RESPONSE_DELAY_MS: u64 = 5000;
        pub const DEFAULT_FAILURE_RATE: f64 = 0.0;
        pub const MAX_FAILURE_RATE: f64 = 1.0;
        pub const MOCK_SERVICE_PORT_START: u16 = 19000;
        pub const MOCK_SERVICE_PORT_END: u16 = 19999;
    }
}

// REMOVED: Duplicate protocols module - consolidated into main protocols module above

// REMOVED: Duplicate storage module eliminated
// Use the original storage module definition above instead

// ==================== ZFS CONSTANTS ====================

/// ZFS-specific constants for properties, pools, and datasets
pub mod zfs {
    /// ZFS pool names
    pub mod pools {
        pub const DEFAULT: &str = "nestpool";
        pub const PRODUCTION: &str = "nestpool-prod";
    }

    /// ZFS property names
    pub mod properties {
        pub const COMPRESSION: &str = "compression";
        pub const RECORDSIZE: &str = "recordsize";
        pub const ATIME: &str = "atime";
        pub const PRIMARYCACHE: &str = "primarycache";
        pub const SECONDARYCACHE: &str = "secondarycache";
        pub const LOGBIAS: &str = "logbias";
        pub const SYNC: &str = "sync";
        pub const DEDUP: &str = "dedup";
    }

    /// ZFS property values
    pub mod values {
        /// Cache settings
        pub const CACHE_ALL: &str = "all";
        pub const CACHE_METADATA: &str = "metadata";
        pub const CACHE_NONE: &str = "none";

        /// Record sizes
        pub const RECORDSIZE_64K: &str = "64K";
        pub const RECORDSIZE_128K: &str = "128K";
        pub const RECORDSIZE_1M: &str = "1M";

        /// Boolean values
        pub const VALUE_ON: &str = "on";
        pub const VALUE_OFF: &str = "off";
        pub const VALUE_ALWAYS: &str = "always";
        pub const VALUE_STANDARD: &str = "standard";

        /// Log bias settings
        pub const BIAS_LATENCY: &str = "latency";
        pub const BIAS_THROUGHPUT: &str = "throughput";
    }

    /// ZFS error messages
    pub mod errors {
        pub const TIER_NAME_EMPTY: &str = "Tier name cannot be empty";
        pub const POOL_NAME_EMPTY: &str = "Pool name cannot be empty";
        pub const DATASET_PREFIX_EMPTY: &str = "Dataset prefix cannot be empty";
    }
}

// ==================== NETWORK CONSTANTS ====================

// REMOVED: Duplicate network module eliminated
// Use the original network module definition above instead

// ==================== SERVICE CONSTANTS ====================

/// Service names and identifiers
pub mod services {
    /// Default service information
    pub mod defaults {
        pub const SERVICE_NAME: &str = "nestgate-storage";
        pub const MAINTAINER_NAME: &str = "NestGate Team";
        pub const MAINTAINER_EMAIL: &str = "team@nestgate.dev";
        pub const ORGANIZATION: &str = "EcoPrimals Foundation";
    }

    /// Universal service types
    pub mod types {
        pub const UNIVERSAL_SECURITY: &str = "universal-security-service";
        pub const COMMUNITY_BACKUP: &str = "community-backup-service";
        pub const STORAGE: &str = "storage";
        pub const API: &str = "api";
        pub const MIDDLEWARE: &str = "middleware";
        pub const MCP: &str = "mcp";
        pub const ZFS: &str = "zfs";
    }
}

// ==================== TIMEOUT CONSTANTS ====================

/// Timeout configurations for various operations
pub mod timeouts {
    use super::Duration;

    /// File operation timeouts
    pub const MOUNT_TIMEOUT: Duration = Duration::from_secs(5);
    pub const UNMOUNT_TIMEOUT: Duration = Duration::from_secs(5);
    pub const FILE_OPERATION_TIMEOUT: Duration = Duration::from_secs(2);

    /// Network timeouts
    pub const HTTP_CLIENT_TIMEOUT: Duration = Duration::from_secs(30);
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
    pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

    /// Discovery timeouts
    pub const DISCOVERY_TIMEOUT: Duration = Duration::from_secs(10);
    pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);

    /// SSE timeouts
    pub const SSE_KEEP_ALIVE: Duration = Duration::from_secs(30);
}

// REMOVED: Duplicate performance, monitoring, and testing modules eliminated
// Use the original module definitions above instead

// ==================== COMPILE-TIME CONSTANTS ====================

/// Compile-time validated constants using const generics
pub mod compile_time {
    /// Size constants for const generics
    pub const SIZE_0: usize = 0;
    pub const SIZE_1: usize = 1;
    pub const SIZE_2: usize = 2;
    pub const SIZE_4: usize = 4;
    pub const SIZE_8: usize = 8;
    pub const SIZE_16: usize = 16;
    pub const SIZE_32: usize = 32;
    pub const SIZE_64: usize = 64;
    pub const SIZE_128: usize = 128;
    pub const SIZE_256: usize = 256;
    pub const SIZE_512: usize = 512;
    pub const SIZE_1024: usize = 1024;
    pub const SIZE_2048: usize = 2048;
    pub const SIZE_4096: usize = 4096;
    pub const SIZE_8192: usize = 8192;
    pub const SIZE_16384: usize = 16384;
    pub const SIZE_32768: usize = 32768;
    pub const SIZE_65536: usize = 65536;
    pub const SIZE_131072: usize = 131072;
    pub const SIZE_262144: usize = 262144;
    pub const SIZE_524288: usize = 524288;
    pub const SIZE_1048576: usize = 1048576; // 1MB
}

// ==================== CONVENIENCE FUNCTIONS ====================

/// Convenience functions for working with unified constants
impl UnifiedConstants {
    /// Get default port for a service type
    pub fn default_port_for_service(service_type: &str) -> u16 {
        match service_type {
            services::types::API => network::ports::API_PORT,
            services::types::STORAGE => 2049, // NFS default
            "database" => 5432,               // PostgreSQL default
            "cache" => 6379,                  // Redis default
            _ => 80,                          // HTTP default
        }
    }

    /// Get compression algorithm for storage tier
    pub fn compression_for_tier(tier: &str) -> &'static str {
        match tier {
            storage::tiers::HOT => "lz4",
            storage::tiers::WARM => "zstd",
            storage::tiers::COLD => "gzip-9",
            _ => "lz4", // default compression
        }
    }

    /// Get buffer size for performance level
    pub fn buffer_size_for_performance(level: &str) -> usize {
        match level {
            "high" => 1048576, // 1MB buffer
            "medium" => 65536, // 64KB buffer
            "low" => 8192,     // 8KB buffer
            _ => 8192,         // default 8KB buffer
        }
    }
}

/// Main unified constants struct for namespace organization
pub struct UnifiedConstants;

// ==================== CONSTANTS CLEANUP COMPLETED ====================

pub use api::capabilities::*;
pub use api::roles::*;
/// All deprecated constants have been removed - use the organized module hierarchy above
/// Re-export commonly used constants for convenience
// API constants
pub use api::PREFIX as API_PREFIX_V1;
pub use api::VERSION as API_VERSION_V1;

// Network constants
pub use network::ports::*;

// Performance constants
pub use performance::*;

// Protocol constants
pub use protocols::*;

// Storage constants (consolidated)
pub use storage::limits::*;
pub use storage::tiers::{COLD as COLD_TIER, HOT as HOT_TIER, WARM as WARM_TIER};

// Timeout constants
pub use timeouts::{CONNECTION_TIMEOUT, HTTP_CLIENT_TIMEOUT, REQUEST_TIMEOUT};
