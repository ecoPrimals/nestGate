//! **UNIFIED CONSTANTS SYSTEM**
//!
//! This module provides the single source of truth for ALL constants across NestGate,
//! replacing scattered constants, magic numbers, and hardcoded values throughout the codebase.
//!
//! **CONSOLIDATES AND REPLACES**:
//! - 200+ scattered constants across all crates
//! - Hardcoded values like "127.0.0.1", "8080", timeouts
//! - Magic numbers in performance calculations
//! - Duplicate DEFAULT_* patterns across modules
//!
//! **DESIGN PRINCIPLES**:
//! - Domain-organized for logical grouping
//! - Typed constants where appropriate
//! - Documentation for all values
//! - Environment-aware where needed

use std::time::Duration;

// ==================== SECTION ====================

pub mod network {
    use super::*;

    /// Default API server host
    pub const DEFAULT_HOST: &str = "127.0.0.1";
    
    /// Default API server port
    pub const DEFAULT_API_PORT: u16 = 8080;
    
    /// Default internal communication port
    pub const DEFAULT_INTERNAL_PORT: u16 = 8081;
    
    /// Maximum concurrent connections - use canonical constant
    pub use crate::constants::canonical::performance::MAX_CONNECTIONS;
    
    /// Default connection timeout
    pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Network buffer size - use canonical constant  
    pub use crate::constants::canonical::performance::DEFAULT_BUFFER_SIZE as BUFFER_SIZE;
    
    /// Maximum packet size
    pub const MAX_PACKET_SIZE: usize = 65536;
    
    /// Default health check interval
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(60);
    
    /// Circuit breaker failure threshold
    pub const CIRCUIT_BREAKER_THRESHOLD: u32 = 5;
    
    /// Load balancer default algorithm
    pub const DEFAULT_LB_ALGORITHM: &str = "round_robin";
    
    /// Default discovery endpoint
    pub const DEFAULT_DISCOVERY_ENDPOINT: &str = "http://localhost:8083/discovery";
}

// ==================== SECTION ====================

pub mod storage {
    use super::*;

    /// Default storage backend
    pub const DEFAULT_BACKEND: &str = "filesystem";
    
    /// Default data directory
    pub const DEFAULT_DATA_DIR: &str = "./data";
    
    /// Maximum file size (1GB)
    pub const MAX_FILE_SIZE: u64 = 1024 * 1024 * 1024;
    
    /// Default cache size (100MB)
    pub const DEFAULT_CACHE_SIZE: u64 = 100 * 1024 * 1024;
    
    /// Default cache TTL
    pub const DEFAULT_CACHE_TTL: Duration = Duration::from_secs(3600);
    
    /// Default backup retention (30 days)
    pub const DEFAULT_BACKUP_RETENTION: u32 = 30;
    
    /// Default compression level
    pub const DEFAULT_COMPRESSION_LEVEL: u8 = 6;
    
    /// Default record size
    pub const DEFAULT_RECORD_SIZE: u32 = 128 * 1024; // 128KB
    
    /// Storage tier thresholds
    pub mod tiers {
        /// Hot tier access threshold (accesses per day)
        pub const HOT_TIER_THRESHOLD: u32 = 100;
        
        /// Warm tier access threshold (accesses per week)
        pub const WARM_TIER_THRESHOLD: u32 = 10;
        
        /// Cold tier size limit (1TB)
        pub const COLD_TIER_SIZE_LIMIT: u64 = 1024 * 1024 * 1024 * 1024;
    }
}

// ==================== SECTION ====================

pub mod zfs {
    use super::*;

    /// Default ZFS pool name
    pub const DEFAULT_POOL_NAME: &str = "nestgate";
    
    /// Default ARC size (512MB)
    pub const DEFAULT_ARC_SIZE: u64 = 512 * 1024 * 1024;
    
    /// Default dataset quota (10GB)
    pub const DEFAULT_DATASET_QUOTA: u64 = 10 * 1024 * 1024 * 1024;
    
    /// Default snapshot retention (7 days)
    pub const DEFAULT_SNAPSHOT_RETENTION: u32 = 7;
    
    /// ZFS operation timeout
    pub const OPERATION_TIMEOUT: Duration = Duration::from_secs(300);
    
    /// Pool health check interval
    pub const POOL_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(300);
    
    /// Default scrub schedule
    pub const DEFAULT_SCRUB_SCHEDULE: &str = "0 2 * * 0"; // Weekly at 2 AM
    
    /// Maximum pool operations per second
    pub const MAX_OPERATIONS_PER_SEC: u32 = 100;
    
    /// ZFS prefetch distance
    pub const PREFETCH_DISTANCE: u32 = 8;
}

// ==================== SECTION ====================

pub mod security {
    use super::*;

    /// Default token expiry time
    pub const DEFAULT_TOKEN_EXPIRY: Duration = Duration::from_secs(3600);
    
    /// Default session timeout
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(1800);
    
    /// Default key rotation interval (30 days)
    pub const DEFAULT_KEY_ROTATION: Duration = Duration::from_secs(30 * 24 * 3600);
    
    /// Default encryption algorithm
    pub const DEFAULT_ENCRYPTION_ALGORITHM: &str = "AES-256-GCM";
    
    /// Default key size (bits)
    pub const DEFAULT_KEY_SIZE: u32 = 256;
    
    /// Rate limiting defaults
    pub const DEFAULT_RATE_LIMIT_RPM: u32 = 60;
    pub const DEFAULT_RATE_LIMIT_BURST: u32 = 10;
    
    /// Security scan interval
    pub const SECURITY_SCAN_INTERVAL: Duration = Duration::from_secs(24 * 3600);
}

// ==================== SECTION ====================

pub mod performance {
    use super::*;

    /// Default worker thread count
    pub const DEFAULT_WORKER_THREADS: usize = 8;
    
    /// Default blocking thread count  
    pub const DEFAULT_BLOCKING_THREADS: usize = 16;
    
    /// Default I/O buffer size
    pub const DEFAULT_IO_BUFFER_SIZE: usize = 8192;
    
    /// Default async I/O queue depth
    pub const DEFAULT_QUEUE_DEPTH: u32 = 32;
    
    /// Default memory limit (1GB)
    pub const DEFAULT_MEMORY_LIMIT: u64 = 1024 * 1024 * 1024;
    
    /// Default buffer pool size
    pub const DEFAULT_BUFFER_POOL_SIZE: usize = 1000;
    
    /// Performance monitoring interval
    pub const MONITORING_INTERVAL: Duration = Duration::from_secs(60);
}

// ==================== SECTION ====================

pub mod api {
    use super::*;

    /// Default API request timeout
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Maximum request body size (10MB)
    pub const MAX_REQUEST_BODY_SIZE: usize = 10 * 1024 * 1024;
    
    /// Default dashboard refresh interval
    pub const DASHBOARD_REFRESH_INTERVAL: Duration = Duration::from_secs(5);
    
    /// Maximum load test duration
    pub const MAX_LOAD_TEST_DURATION: Duration = Duration::from_secs(300);
    
    /// Default CORS allowed origins
    pub const DEFAULT_CORS_ORIGINS: &[&str] = &["*"];
    
    /// Default CORS allowed methods
    pub const DEFAULT_CORS_METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE", "OPTIONS"];
}

// ==================== SECTION ====================

pub mod mcp {
    use super::*;

    /// Default MCP server port
    pub const DEFAULT_MCP_PORT: u16 = 8082;
    
    /// MCP protocol version
    pub const PROTOCOL_VERSION: &str = "1.0.0";
    
    /// Maximum MCP message size (1MB)
    pub const MAX_MESSAGE_SIZE: usize = 1024 * 1024;
    
    /// MCP connection timeout
    pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(10);
    
    /// Maximum concurrent MCP connections
    pub const MAX_MCP_CONNECTIONS: usize = 100;
}

// ==================== SECTION ====================

pub mod automation {
    use super::*;

    /// Maximum concurrent workflows
    pub const MAX_CONCURRENT_WORKFLOWS: usize = 50;
    
    /// Default workflow timeout
    pub const DEFAULT_WORKFLOW_TIMEOUT: Duration = Duration::from_secs(600);
    
    /// Scheduler tick interval
    pub const SCHEDULER_TICK_INTERVAL: Duration = Duration::from_millis(100);
    
    /// Maximum scheduled tasks
    pub const MAX_SCHEDULED_TASKS: usize = 1000;
}

// ==================== SECTION ====================

pub mod monitoring {
    use super::*;

    /// Default metrics collection interval
    pub const METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);
    
    /// Default metrics retention period (7 days)
    pub const METRICS_RETENTION_PERIOD: Duration = Duration::from_secs(7 * 24 * 3600);
    
    /// Default log rotation size (100MB)
    pub const DEFAULT_LOG_ROTATION_SIZE: u64 = 100 * 1024 * 1024;
    
    /// Default log file count
    pub const DEFAULT_LOG_FILE_COUNT: u32 = 10;
    
    /// Default tracing sample rate
    pub const DEFAULT_TRACING_SAMPLE_RATE: f64 = 0.1;
    
    /// Alert evaluation interval
    pub const ALERT_EVALUATION_INTERVAL: Duration = Duration::from_secs(60);
}

// ==================== SECTION ====================

pub mod system {
    use super::*;

    /// Default instance name
    pub const DEFAULT_INSTANCE_NAME: &str = "NestGate Instance";
    
    /// Default configuration directory
    pub const DEFAULT_CONFIG_DIR: &str = "./config";
    
    /// Maximum configuration file size (10MB)
    pub const MAX_CONFIG_FILE_SIZE: u64 = 10 * 1024 * 1024;
    
    /// Configuration validation timeout
    pub const CONFIG_VALIDATION_TIMEOUT: Duration = Duration::from_secs(10);
    
    /// Default retry attempts
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3;
    
    /// Default retry base delay
    pub const DEFAULT_RETRY_BASE_DELAY: Duration = Duration::from_millis(100);
    
    /// Default retry maximum delay
    pub const DEFAULT_RETRY_MAX_DELAY: Duration = Duration::from_secs(60);
}

// ==================== SECTION ====================

pub mod integration {
    use super::*;

    /// Default integration timeout
    pub const DEFAULT_INTEGRATION_TIMEOUT: Duration = Duration::from_secs(30);
    
    /// Maximum integration retries
    pub const MAX_INTEGRATION_RETRIES: u32 = 3;
    
    /// Integration health check interval
    pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(120);
    
    /// Default service discovery timeout
    pub const SERVICE_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(10);
}

// ==================== SECTION ====================

pub mod validation {
    /// Minimum instance name length
    pub const MIN_INSTANCE_NAME_LENGTH: usize = 3;
    
    /// Maximum instance name length
    pub const MAX_INSTANCE_NAME_LENGTH: usize = 64;
    
    /// Valid port range minimum
    pub const MIN_PORT: u16 = 1024;
    
    /// Valid port range maximum
    pub const MAX_PORT: u16 = 65535;
    
    /// Minimum memory allocation (1MB)
    pub const MIN_MEMORY_ALLOCATION: u64 = 1024 * 1024;
    
    /// Maximum cache size (10GB)
    pub const MAX_CACHE_SIZE: u64 = 10 * 1024 * 1024 * 1024;
}

// ==================== SECTION ====================

pub mod features {
    /// Enable zero-cost optimizations
    pub const ZERO_COST_OPTIMIZATIONS: bool = cfg!(feature = "zero-cost-optimizations");
    
    /// Enable advanced monitoring
    pub const ADVANCED_MONITORING: bool = cfg!(feature = "advanced-monitoring");
    
    /// Enable security scanning
    pub const SECURITY_SCANNING: bool = cfg!(feature = "security-scanning");
    
    /// Enable experimental features
    pub const EXPERIMENTAL_FEATURES: bool = cfg!(feature = "experimental-features");
    
    /// Enable development features
    pub const DEVELOPMENT_FEATURES: bool = cfg!(debug_assertions);
}

// ==================== SECTION ====================

pub mod version {
    /// NestGate version
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
    
    /// Modernization version
    pub const MODERNIZATION_VERSION: &str = "3.0.0";
    
    /// Configuration schema version
    pub const CONFIG_SCHEMA_VERSION: &str = "3.0.0";
    
    /// API version
    pub const API_VERSION: &str = "v1";
    
    /// Build information
    pub const BUILD_INFO: &str = concat!(
        "NestGate v", env!("CARGO_PKG_VERSION"),
        " unified modernization build"
    );
}

// ==================== SECTION ====================

pub mod migration {
    /// Modernization completion status
    pub const MODERNIZATION_COMPLETE: bool = false; // Set to true when actually complete
    
    /// Performance improvement percentage achieved
    pub const PERFORMANCE_IMPROVEMENT_PERCENT: u8 = 0; // Update as improvements are made
    
    /// Technical debt elimination percentage
    pub const DEBT_ELIMINATION_PERCENT: u8 = 0; // Update as debt is eliminated
    
    /// Configuration unification percentage  
    pub const CONFIG_UNIFICATION_PERCENT: u8 = 0; // Update as configs are unified
} 