//! Domain-Specific Constants - Consolidated System
//!
//! This module consolidates all domain-specific constants from across the codebase
//! into organized, maintainable modules. This eliminates duplication and provides
//! a single source of truth for all constants.
//!
//! **CONSOLIDATES**:
//! - ZFS tier and property constants from nestgate-zfs/src/config/tiers.rs
//! - Protocol constants from nestgate-api/src/universal_adapter.rs  
//! - API endpoint constants from multiple modules
//! - Performance constants from various files
//! - Network constants scattered across modules
//!
//! **PROVIDES**:
//! - Organized domain hierarchies (storage, network, api, performance)
//! - Consistent naming conventions
//! - Single source of truth
//! - Easy maintenance and updates

use std::time::Duration;

// ==================== STORAGE DOMAIN CONSTANTS ====================

/// Storage-related constants consolidated from multiple modules
pub mod storage {
    /// Storage tier definitions (consolidated from ZFS and API modules)
    pub mod tiers {
        /// Hot storage tier (high performance, frequently accessed)
        pub const HOT: &str = "hot";
        /// Warm storage tier (medium performance, occasionally accessed)
        pub const WARM: &str = "warm";
        /// Cold storage tier (low performance, rarely accessed)
        pub const COLD: &str = "cold";

        /// All tier names for validation
        pub const ALL_TIERS: &[&str] = &[HOT, WARM, COLD];

        /// Validate tier name
        pub fn is_valid_tier(tier: &str) -> bool {
            ALL_TIERS.contains(&tier)
        }
    }

    /// ZFS property constants (from nestgate-zfs/src/config/tiers.rs)
    pub mod zfs {
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

            /// Compression algorithms
            pub const COMPRESSION_OFF: &str = "off";
            pub const COMPRESSION_LZ4: &str = "lz4";
            pub const COMPRESSION_ZSTD: &str = "zstd";
            pub const COMPRESSION_GZIP_9: &str = "gzip-9";

            /// Record sizes
            pub const RECORDSIZE_64K: &str = "64K";
            pub const RECORDSIZE_128K: &str = "128K";
            pub const RECORDSIZE_1M: &str = "1M";

            /// General values
            pub const ON: &str = "on";
            pub const OFF: &str = "off";
            pub const ALWAYS: &str = "always";
            pub const STANDARD: &str = "standard";

            /// Performance bias
            pub const BIAS_LATENCY: &str = "latency";
            pub const BIAS_THROUGHPUT: &str = "throughput";
        }

        /// ZFS pool names
        pub mod pools {
            pub const DEFAULT: &str = "nestpool";
            pub const PRODUCTION: &str = "nestpool-prod";
        }
    }

    /// Storage protocols
    pub mod protocols {
        pub const NFS: &str = "NFS";
        pub const SMB: &str = "SMB";
        pub const HTTP: &str = "HTTP";
        pub const HTTPS: &str = "HTTPS";
        pub const ZFS: &str = "ZFS";
        pub const S3: &str = "S3";
        pub const BLOCK: &str = "BLOCK";

        /// All supported protocols
        pub const ALL_PROTOCOLS: &[&str] = &[NFS, SMB, HTTP, HTTPS, ZFS, S3, BLOCK];

        /// Validate protocol name
        pub fn is_valid_protocol(protocol: &str) -> bool {
            ALL_PROTOCOLS.contains(&protocol)
        }
    }

    /// Storage sizes for different file types and operations
    pub mod sizes {
        /// File size thresholds
        pub const SMALL_FILE_BYTES: u64 = 1024 * 1024; // 1MB
        pub const MEDIUM_FILE_BYTES: u64 = 10 * 1024 * 1024; // 10MB
        pub const LARGE_FILE_BYTES: u64 = 100 * 1024 * 1024; // 100MB
        pub const HUGE_FILE_BYTES: u64 = 1024 * 1024 * 1024; // 1GB

        /// Block sizes for different operations
        pub const BLOCK_SIZE_4K: u64 = 4 * 1024;
        pub const BLOCK_SIZE_8K: u64 = 8 * 1024;
        pub const BLOCK_SIZE_64K: u64 = 64 * 1024;
        pub const BLOCK_SIZE_1M: u64 = 1024 * 1024;

        /// Default sizes
        pub const DEFAULT_BLOCK_SIZE: u64 = BLOCK_SIZE_64K;
        pub const DEFAULT_FILE_THRESHOLD: u64 = MEDIUM_FILE_BYTES;
    }
}

// ==================== NETWORK DOMAIN CONSTANTS ====================

/// Network-related constants consolidated from multiple modules
pub mod network {
    /// Default port numbers
    pub mod ports {
        // Web services
        pub const HTTP: u16 = 80;
        pub const HTTPS: u16 = 443;
        pub const API_DEFAULT: u16 = 8080;
        pub const WEBSOCKET: u16 = 8080;

        // File system protocols
        pub const NFS: u16 = 2049;
        pub const SMB: u16 = 445;
        pub const FTP: u16 = 21;
        pub const SFTP: u16 = 22;

        // Database ports
        pub const POSTGRES: u16 = 5432;
        pub const MYSQL: u16 = 3306;
        pub const MONGODB: u16 = 27017;
        pub const REDIS: u16 = 6379;

        // NestGate specific
        pub const NESTGATE_API: u16 = 8000;
        pub const NESTGATE_STREAMING: u16 = 8001;
        pub const NESTGATE_HEALTH: u16 = 8002;
        pub const NESTGATE_METRICS: u16 = 8003;
    }

    /// Network addresses and hostnames
    pub mod addresses {
        pub const LOCALHOST: &str = "localhost";
        pub const LOCALHOST_IP: &str = "127.0.0.1";
        pub const ANY_ADDRESS: &str = "0.0.0.0";
        pub const DEFAULT_BIND: &str = "0.0.0.0:8000";
        pub const DEFAULT_DISCOVERY_ENDPOINT: &str = "http://localhost:8083/discovery";
    }

    /// Connection limits and buffers
    pub mod limits {
        pub const MAX_CONNECTIONS: usize = 10000;
        pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;
        pub const MAX_CONCURRENT_REQUESTS: usize = 1000;
        pub const DEFAULT_BUFFER_SIZE: usize = 8192;
        pub const LARGE_BUFFER_SIZE: usize = 65536;
    }
}

// ==================== API DOMAIN CONSTANTS ====================

/// API-related constants consolidated from multiple modules  
pub mod api {
    /// API versioning
    pub mod versions {
        pub const V1: &str = "v1";
        pub const V2: &str = "v2";
        pub const CURRENT: &str = V1;
    }

    /// API endpoint paths (from universal_adapter.rs and other modules)
    pub mod endpoints {
        /// Base API prefix
        pub const BASE: &str = "/api";
        pub const V1_BASE: &str = "/api/v1";

        /// Coordination endpoints
        pub const COORDINATE: &str = "/api/v1/coordinate";
        pub const COORDINATE_STORAGE: &str = "/api/v1/coordinate-storage";
        pub const PROVISION_STORAGE: &str = "/api/v1/provision-storage";
        pub const OPTIMIZE_STORAGE: &str = "/api/v1/optimize-storage";
        pub const SECURE_STORAGE: &str = "/api/v1/secure-storage";

        /// Health and monitoring
        pub const HEALTH: &str = "/health";
        pub const METRICS: &str = "/metrics";
        pub const STATUS: &str = "/status";

        /// Authentication
        pub const AUTH: &str = "/api/v1/auth";
        pub const LOGIN: &str = "/api/v1/auth/login";
        pub const LOGOUT: &str = "/api/v1/auth/logout";
    }

    /// HTTP response messages
    pub mod messages {
        pub const COORDINATION_SUCCESSFUL: &str = "Coordination successful";
        pub const COORDINATION_DISABLED: &str = "Coordination disabled";
        pub const ENDPOINT_NOT_AVAILABLE: &str = "Endpoint not available";
        pub const SERVICE_HEALTHY: &str = "Service is healthy";
        pub const SERVICE_UNHEALTHY: &str = "Service is unhealthy";
        pub const UNAUTHORIZED: &str = "Unauthorized access";
        pub const FORBIDDEN: &str = "Access forbidden";
        pub const NOT_FOUND: &str = "Resource not found";
        pub const INTERNAL_ERROR: &str = "Internal server error";
    }

    /// Content types
    pub mod content_types {
        pub const JSON: &str = "application/json";
        pub const TEXT: &str = "text/plain";
        pub const HTML: &str = "text/html";
        pub const XML: &str = "application/xml";
        pub const BINARY: &str = "application/octet-stream";
    }
}

// ==================== PERFORMANCE DOMAIN CONSTANTS ====================

/// Performance-related constants
pub mod performance {
    /// Buffer sizes for different performance levels
    pub mod buffers {
        pub const SMALL: usize = 4096; // 4KB
        pub const MEDIUM: usize = 8192; // 8KB
        pub const LARGE: usize = 65536; // 64KB
        pub const HUGE: usize = 1048576; // 1MB
        pub const DEFAULT: usize = MEDIUM;
    }

    /// Connection pool sizes
    pub mod pools {
        pub const MIN_CONNECTIONS: usize = 1;
        pub const DEFAULT_CONNECTIONS: usize = 10;
        pub const MAX_CONNECTIONS: usize = 100;
    }

    /// Retry configurations
    pub mod retry {
        pub const DEFAULT_ATTEMPTS: u32 = 3;
        pub const MAX_ATTEMPTS: u32 = 10;
        pub const BACKOFF_MULTIPLIER: f64 = 2.0;
    }
}

// ==================== SECURITY DOMAIN CONSTANTS ====================

/// Security-related constants
pub mod security {
    /// Encryption algorithms
    pub mod encryption {
        pub const AES_256: &str = "AES-256";
        pub const AES_128: &str = "AES-128";
        pub const RSA_2048: &str = "RSA-2048";
        pub const RSA_4096: &str = "RSA-4096";
        pub const DEFAULT: &str = AES_256;
    }

    /// Authentication methods
    pub mod auth {
        pub const BEARER: &str = "Bearer";
        pub const BASIC: &str = "Basic";
        pub const API_KEY: &str = "ApiKey";
        pub const JWT: &str = "JWT";
    }

    /// Security levels
    pub mod levels {
        pub const LOW: &str = "low";
        pub const MEDIUM: &str = "medium";
        pub const HIGH: &str = "high";
        pub const CRITICAL: &str = "critical";
    }
}

// ==================== TIMEOUT DOMAIN CONSTANTS ====================

/// Timeout constants for various operations
pub mod timeouts {
    use super::Duration;

    /// Connection timeouts
    pub const CONNECTION_DEFAULT: Duration = Duration::from_secs(30);
    pub const CONNECTION_SHORT: Duration = Duration::from_secs(5);
    pub const CONNECTION_LONG: Duration = Duration::from_secs(120);

    /// Request timeouts  
    pub const REQUEST_DEFAULT: Duration = Duration::from_secs(30);
    pub const REQUEST_SHORT: Duration = Duration::from_secs(10);
    pub const REQUEST_LONG: Duration = Duration::from_secs(300);
    pub const REQUEST_TIMEOUT_SECS: u64 = 30;

    /// Health check timeouts
    pub const HEALTH_CHECK: Duration = Duration::from_secs(10);
    pub const HEALTH_CHECK_FAST: Duration = Duration::from_secs(3);

    /// Database operation timeouts
    pub const DATABASE_QUERY: Duration = Duration::from_secs(30);
    pub const DATABASE_TRANSACTION: Duration = Duration::from_secs(60);

    /// File operation timeouts
    pub const FILE_READ: Duration = Duration::from_secs(30);
    pub const FILE_WRITE: Duration = Duration::from_secs(60);
    pub const FILE_COPY: Duration = Duration::from_secs(300);

    /// Time unit constants
    pub const SECOND: Duration = Duration::from_secs(1);
    pub const MINUTE: Duration = Duration::from_secs(60);
    pub const HOUR: Duration = Duration::from_secs(3600);
    pub const DAY: Duration = Duration::from_secs(86400);
}

// ==================== SERVICE DOMAIN CONSTANTS ====================

/// Service-related constants
pub mod services {
    /// Service names and identifiers
    pub mod names {
        pub const NESTGATE_CORE: &str = "nestgate-core";
        pub const NESTGATE_API: &str = "nestgate-api";
        pub const NESTGATE_ZFS: &str = "nestgate-zfs";
        pub const NESTGATE_MCP: &str = "nestgate-mcp";
        pub const NESTGATE_NAS: &str = "nestgate-nas";
        pub const NESTGATE_AUTOMATION: &str = "nestgate-automation";
    }

    /// Service metadata
    pub mod metadata {
        pub const ORGANIZATION: &str = "EcoPrimals Foundation";
        pub const MAINTAINER: &str = "NestGate Team";
        pub const MAINTAINER_EMAIL: &str = "team@nestgate.dev";
        pub const LICENSE: &str = "MIT";
        pub const VERSION: &str = "2.1.0";
    }

    /// Service capabilities
    pub mod capabilities {
        pub const STORAGE: &str = "storage";
        pub const API: &str = "api";
        pub const MONITORING: &str = "monitoring";
        pub const SECURITY: &str = "security";
        pub const ORCHESTRATION: &str = "orchestration";
        pub const AI_INTEGRATION: &str = "ai-integration";
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Utility functions for working with domain constants
pub mod utils {
    use super::*;

    /// Get default port for a service type
    pub fn default_port_for_service(service: &str) -> u16 {
        match service {
            services::names::NESTGATE_API => network::ports::NESTGATE_API,
            "storage" => network::ports::NFS,
            "database" => network::ports::POSTGRES,
            "cache" => network::ports::REDIS,
            _ => network::ports::HTTP,
        }
    }

    /// Get compression algorithm for storage tier
    pub fn compression_for_tier(tier: &str) -> &'static str {
        match tier {
            storage::tiers::HOT => storage::zfs::values::COMPRESSION_LZ4,
            storage::tiers::WARM => storage::zfs::values::COMPRESSION_ZSTD,
            storage::tiers::COLD => storage::zfs::values::COMPRESSION_GZIP_9,
            _ => storage::zfs::values::COMPRESSION_LZ4,
        }
    }

    /// Get buffer size for performance level
    pub fn buffer_size_for_performance(level: &str) -> usize {
        match level {
            "high" => performance::buffers::HUGE,
            "medium" => performance::buffers::LARGE,
            "low" => performance::buffers::MEDIUM,
            _ => performance::buffers::DEFAULT,
        }
    }

    /// Validate storage tier
    pub fn validate_tier(tier: &str) -> Result<(), &'static str> {
        if storage::tiers::is_valid_tier(tier) {
            Ok(())
        } else {
            Err("Invalid storage tier")
        }
    }

    /// Validate protocol
    pub fn validate_protocol(protocol: &str) -> Result<(), &'static str> {
        if storage::protocols::is_valid_protocol(protocol) {
            Ok(())
        } else {
            Err("Invalid storage protocol")
        }
    }
}

// ==================== CONVENIENCE RE-EXPORTS ====================

pub use api::versions::CURRENT as CURRENT_API_VERSION;
pub use network::ports::{
    NESTGATE_API as DEFAULT_API_PORT, NESTGATE_HEALTH as DEFAULT_HEALTH_PORT,
};
pub use performance::buffers::DEFAULT as DEFAULT_BUFFER_SIZE;
pub use storage::protocols::{NFS as PROTOCOL_NFS, SMB as PROTOCOL_SMB, ZFS as PROTOCOL_ZFS};
/// Re-export commonly used constants for convenience
pub use storage::tiers::{COLD as TIER_COLD, HOT as TIER_HOT, WARM as TIER_WARM};

// ==================== TEST CONSTANTS ====================

/// Test-only constants for configuration and testing
#[cfg(test)]
pub mod test {
    // Email testing constants
    pub const EXAMPLE_SENDER_EMAIL: &str = "test-sender@example.com";
    pub const EXAMPLE_TEST_EMAIL: &str = "test@example.com";
    pub const EXAMPLE_SMTP_SERVER: &str = "smtp.example.com";

    // Webhook testing constants
    pub const EXAMPLE_WEBHOOK_URL: &str = "https://example.com/webhook";
    pub const EXAMPLE_SLACK_WEBHOOK: &str = "https://hooks.slack.com/services/TEST/WEBHOOK/URL";

    // Security testing constants
    pub const ROLE_CUSTOM: &str = "custom";
    pub const ROLE_POWER_USER: &str = "power_user";
    pub const DESC_CUSTOM: &str = "Custom role for testing";
    pub const DESC_POWER_USER: &str = "Power user role for testing";
    pub const PERM_CUSTOM: &str = "custom:test";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tier_validation() {
        assert!(storage::tiers::is_valid_tier("hot"));
        assert!(storage::tiers::is_valid_tier("warm"));
        assert!(storage::tiers::is_valid_tier("cold"));
        assert!(!storage::tiers::is_valid_tier("invalid"));
    }

    #[test]
    fn test_protocol_validation() {
        assert!(storage::protocols::is_valid_protocol("NFS"));
        assert!(storage::protocols::is_valid_protocol("SMB"));
        assert!(!storage::protocols::is_valid_protocol("INVALID"));
    }

    #[test]
    fn test_utility_functions() {
        assert_eq!(
            utils::default_port_for_service("nestgate-api"),
            network::ports::NESTGATE_API
        );
        assert_eq!(
            utils::compression_for_tier("hot"),
            storage::zfs::values::COMPRESSION_LZ4
        );
        assert_eq!(
            utils::buffer_size_for_performance("high"),
            performance::buffers::HUGE
        );
    }

    #[test]
    fn test_convenience_exports() {
        assert_eq!(TIER_HOT, "hot");
        assert_eq!(PROTOCOL_NFS, "NFS");
        assert_eq!(DEFAULT_API_PORT, 8000);
        assert_eq!(CURRENT_API_VERSION, "v1");
    }
}
