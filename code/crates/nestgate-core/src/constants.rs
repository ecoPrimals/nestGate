//! Core System Constants - SOVEREIGN SCIENCE Standard
//!
//! This module contains all system constants to achieve 100% hardcoding elimination.
//! No hardcoded values are permitted anywhere in the codebase.

use std::env;
use std::time::Duration;

/// Service name constants - centralized and configurable
pub mod service_defaults {
    pub const DEFAULT_API_SERVICE: &str = "nestgate-api";
    pub const DEFAULT_UI_SERVICE: &str = "nestgate-ui";
    pub const DEFAULT_POSTGRES_SERVICE: &str = "postgres-db";
    pub const DEFAULT_REDIS_SERVICE: &str = "redis-cache";
    pub const DEFAULT_SONGBIRD_SERVICE: &str = "songbird-orchestrator";
}

/// Timeout constants - all durations centralized for configuration
pub mod timeout_defaults {
    use std::time::Duration;

    // Network timeouts
    pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_VALIDATION_TIMEOUT: Duration = Duration::from_secs(30);

    // Service lifecycle timeouts
    pub const DEFAULT_SERVICE_START_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_SERVICE_STOP_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(10);

    // Monitoring intervals
    pub const DEFAULT_HEALTH_CHECK_INTERVAL: Duration = Duration::from_secs(30);
    pub const DEFAULT_METRICS_COLLECTION_INTERVAL: Duration = Duration::from_secs(60);
    pub const DEFAULT_PERFORMANCE_MONITORING_INTERVAL: Duration = Duration::from_secs(10);

    // Cache and session timeouts
    pub const DEFAULT_SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
    pub const DEFAULT_CACHE_DECAY_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
    pub const DEFAULT_CERT_VALIDITY_DURATION: Duration = Duration::from_secs(3600); // 1 hour

    // ZFS operation timeouts
    pub const DEFAULT_ZFS_OPERATION_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    pub const DEFAULT_POOL_CREATION_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
    pub const DEFAULT_SNAPSHOT_INTERVAL: Duration = Duration::from_secs(60); // 1 minute
    pub const DEFAULT_SNAPSHOT_RETENTION_CHECK: Duration = Duration::from_secs(3600); // 1 hour

    // Lifecycle management
    pub const DEFAULT_LIFECYCLE_EVALUATION_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
    pub const DEFAULT_TIER_TRANSITION_MIN_DURATION: Duration = Duration::from_secs(60); // 1 minute
    pub const DEFAULT_DELETION_SCHEDULE_DELAY: Duration = Duration::from_secs(86400); // 24 hours

    // Performance analysis windows
    pub const DEFAULT_TREND_ANALYSIS_WINDOW: Duration = Duration::from_secs(3600); // 1 hour
    pub const DEFAULT_BASELINE_UPDATE_INTERVAL: Duration = Duration::from_secs(86400); // 24 hours
    pub const DEFAULT_OPTIMIZATION_INTERVAL: Duration = Duration::from_secs(60); // 1 minute
    pub const DEFAULT_BOTTLENECK_DETECTION_INTERVAL: Duration = Duration::from_secs(30); // 30 seconds

    // Test-specific timeouts (for test environments)
    pub const TEST_SHORT_TIMEOUT: Duration = Duration::from_secs(10);
    pub const TEST_MEDIUM_TIMEOUT: Duration = Duration::from_secs(30);
    pub const TEST_LONG_TIMEOUT: Duration = Duration::from_secs(60);
    pub const TEST_CHAOS_DURATION: Duration = Duration::from_secs(10);
    pub const TEST_E2E_WORKFLOW_TIMEOUT: Duration = Duration::from_secs(300);
}

/// Schedule frequency constants
pub mod schedule_defaults {
    use std::time::Duration;

    pub const DAILY_DURATION: Duration = Duration::from_secs(86400); // 24 hours
    pub const WEEKLY_DURATION: Duration = Duration::from_secs(604800); // 7 days
    pub const MONTHLY_DURATION: Duration = Duration::from_secs(2592000); // 30 days (approximate)
    pub const HOURLY_DURATION: Duration = Duration::from_secs(3600); // 1 hour
}

/// Age-based constants for lifecycle management
pub mod age_defaults {
    use std::time::Duration;

    pub const ONE_HOUR_AGE: Duration = Duration::from_secs(3600);
    pub const ONE_DAY_AGE: Duration = Duration::from_secs(86400);
    pub const ONE_WEEK_AGE: Duration = Duration::from_secs(604800);
    pub const TWO_HOURS_AGE: Duration = Duration::from_secs(7200);
    pub const FIVE_MINUTES_AGE: Duration = Duration::from_secs(300);
    pub const TEN_MINUTES_AGE: Duration = Duration::from_secs(600);
    pub const THREE_MINUTES_AGE: Duration = Duration::from_secs(180);

    // Additional comprehensive time periods for SOVEREIGN SCIENCE compliance
    pub const ZERO_DURATION: Duration = Duration::from_secs(0);
    pub const THIRTY_MINUTES_AGE: Duration = Duration::from_secs(1800);
    pub const SIX_HOURS_AGE: Duration = Duration::from_secs(21600);
    pub const THREE_DAYS_AGE: Duration = Duration::from_secs(259200);
    pub const ONE_MONTH_AGE: Duration = Duration::from_secs(2592000);
    pub const THREE_MONTHS_AGE: Duration = Duration::from_secs(7776000);
    pub const ONE_YEAR_AGE: Duration = Duration::from_secs(31536000);

    // Specific durations found in codebase
    pub const NINETY_SECONDS: Duration = Duration::from_secs(90);
    pub const FOUR_SECONDS: Duration = Duration::from_secs(4);
    pub const EIGHT_SECONDS: Duration = Duration::from_secs(8);
    pub const ONE_HOUR_ONE_MINUTE: Duration = Duration::from_secs(3661);
    pub const TWENTY_FIVE_HOURS: Duration = Duration::from_secs(90061);
}

/// Migration and failover constants
pub mod migration_defaults {
    use std::time::Duration;

    pub const DEFAULT_MIGRATION_TIMEOUT: Duration = Duration::from_secs(30);
    pub const DEFAULT_MIGRATION_CHECK_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour
    pub const DEFAULT_HEARTBEAT_STALENESS: Duration = Duration::from_secs(300); // 5 minutes
}

/// Default uptime and system constants
pub mod system_defaults {
    use std::time::Duration;

    pub const DEFAULT_SYSTEM_UPTIME: Duration = Duration::from_secs(3600); // 1 hour
    pub const DEFAULT_AI_CACHE_DURATION: Duration = Duration::from_secs(86400); // 24 hours
}

/// Test environment configuration constants
pub mod test_defaults {
    use std::time::Duration;

    pub const TEST_METRICS_TREND_WINDOW: Duration = Duration::from_secs(300); // 5 minutes for tests
    pub const TEST_BASELINE_UPDATE_INTERVAL: Duration = Duration::from_secs(3600); // 1 hour for tests
    pub const TEST_MAX_COMPLETION_TIME: Duration = Duration::from_secs(60); // Tests should complete within 1 minute
    pub const TEST_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10); // Shutdown within 10 seconds
    pub const TEST_INTEGRATION_TIMEOUT: Duration = Duration::from_secs(30); // Integration tests timeout

    // Additional test constants referenced by test files
    pub const TEST_SHORT_TIMEOUT: Duration = Duration::from_secs(10);
    pub const TEST_MEDIUM_TIMEOUT: Duration = Duration::from_secs(30);
    pub const TEST_LONG_TIMEOUT: Duration = Duration::from_secs(60);
}

/// Environment variable keys for dynamic configuration
pub mod env_keys {
    pub const API_SERVICE_NAME: &str = "NESTGATE_API_SERVICE";
    pub const UI_SERVICE_NAME: &str = "NESTGATE_UI_SERVICE";
    pub const CONNECTION_TIMEOUT: &str = "NESTGATE_CONNECTION_TIMEOUT";
    pub const REQUEST_TIMEOUT: &str = "NESTGATE_REQUEST_TIMEOUT";
    pub const SESSION_TIMEOUT: &str = "NESTGATE_SESSION_TIMEOUT";
    pub const HEALTH_CHECK_INTERVAL: &str = "NESTGATE_HEALTH_CHECK_INTERVAL";
    pub const METRICS_INTERVAL: &str = "NESTGATE_METRICS_INTERVAL";
}

/// Helper functions for configuration resolution
pub mod config_helpers {
    use std::env;
    use std::time::Duration;

    /// Get service name from environment or default
    pub fn get_api_service_name() -> String {
        env::var(super::env_keys::API_SERVICE_NAME)
            .unwrap_or_else(|_| super::service_defaults::DEFAULT_API_SERVICE.to_string())
    }

    /// Get service name from environment or default
    pub fn get_ui_service_name() -> String {
        env::var(super::env_keys::UI_SERVICE_NAME)
            .unwrap_or_else(|_| super::service_defaults::DEFAULT_UI_SERVICE.to_string())
    }

    /// Get timeout from environment or default
    pub fn get_connection_timeout() -> Duration {
        env::var(super::env_keys::CONNECTION_TIMEOUT)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(super::timeout_defaults::DEFAULT_CONNECTION_TIMEOUT)
    }

    /// Get timeout from environment or default
    pub fn get_request_timeout() -> Duration {
        env::var(super::env_keys::REQUEST_TIMEOUT)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(super::timeout_defaults::DEFAULT_REQUEST_TIMEOUT)
    }

    /// Get session timeout from environment or default
    pub fn get_session_timeout() -> Duration {
        env::var(super::env_keys::SESSION_TIMEOUT)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(super::timeout_defaults::DEFAULT_SESSION_TIMEOUT)
    }
}

/// Network Security Constants
pub mod security_defaults {
    // Default IP addresses for secure configuration
    pub const LOCALHOST_IPV4: &str = "127.0.0.1";
    pub const LOCALHOST_IPV6: &str = "::1";
    pub const UNSPECIFIED_IPV4: &str = "0.0.0.0";

    // Network ranges for security validation
    pub const LINK_LOCAL_RANGE: &str = "169.254.0.0/16";
    pub const MULTICAST_RANGE: &str = "224.0.0.0/4";
    pub const PRIVATE_RANGE_A: &str = "10.0.0.0/8";
    pub const PRIVATE_RANGE_B: &str = "172.16.0.0/12";
    pub const PRIVATE_RANGE_C: &str = "192.168.0.0/16";
    pub const BROADCAST_IP: &str = "255.255.255.255";
}

/// Songbird Service Constants
pub mod songbird_defaults {
    pub const DEFAULT_SONGBIRD_SERVICE: &str = "http://songbird-orchestrator:8000";
}

/// NestGate Core Constants
/// All configurable values centralized here to eliminate hardcoding
/// Network Constants
pub mod network {
    use std::env;

    /// Default API port (configurable via NESTGATE_API_PORT)
    pub fn api_port() -> u16 {
        env::var("NESTGATE_API_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080)
    }

    /// Default UI port (configurable via NESTGATE_UI_PORT)
    pub fn ui_port() -> u16 {
        env::var("NESTGATE_UI_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000)
    }

    /// Default secure port (configurable via NESTGATE_SECURE_PORT)
    pub fn secure_port() -> u16 {
        env::var("NESTGATE_SECURE_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8443)
    }

    /// Default Prometheus port (configurable via NESTGATE_PROMETHEUS_PORT)
    pub fn prometheus_port() -> u16 {
        env::var("NESTGATE_PROMETHEUS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(9090)
    }

    /// Default orchestrator port (configurable via NESTGATE_ORCHESTRATOR_PORT)
    pub fn orchestrator_port() -> u16 {
        env::var("NESTGATE_ORCHESTRATOR_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8000)
    }

    /// Default NFS port (configurable via NESTGATE_NFS_PORT)
    pub fn nfs_port() -> u16 {
        env::var("NESTGATE_NFS_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(2049)
    }

    /// Default BearDog port (configurable via NESTGATE_BEARDOG_PORT)
    pub fn beardog_port() -> u16 {
        env::var("NESTGATE_BEARDOG_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080)
    }

    /// Default discovery port (configurable via NESTGATE_DISCOVERY_PORT)
    pub fn discovery_port() -> u16 {
        env::var("NESTGATE_DISCOVERY_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3001)
    }

    /// Default SMB port (configurable via NESTGATE_SMB_PORT)
    pub fn smb_port() -> u16 {
        env::var("NESTGATE_SMB_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(445)
    }
}

/// Timeout Constants
pub mod timeouts {
    use super::Duration;
    use std::env;

    /// Default request timeout (configurable via NESTGATE_REQUEST_TIMEOUT_SECS)
    pub fn request_timeout() -> Duration {
        Duration::from_secs(
            env::var("NESTGATE_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30), // 30 seconds default
        )
    }

    /// Default connection timeout (configurable via NESTGATE_CONNECTION_TIMEOUT_SECS)
    pub fn connection_timeout() -> Duration {
        let secs = env::var("NESTGATE_CONNECTION_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);
        Duration::from_secs(secs)
    }

    /// Default operation timeout (configurable via NESTGATE_OPERATION_TIMEOUT_SECS)
    pub fn operation_timeout() -> Duration {
        let secs = env::var("NESTGATE_OPERATION_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300);
        Duration::from_secs(secs)
    }

    /// Default monitoring interval (configurable via NESTGATE_MONITORING_INTERVAL_SECS)
    pub fn monitoring_interval() -> Duration {
        let secs = env::var("NESTGATE_MONITORING_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(60);
        Duration::from_secs(secs)
    }

    /// Default heartbeat interval (configurable via NESTGATE_HEARTBEAT_INTERVAL_SECS)
    pub fn heartbeat_interval() -> Duration {
        let secs = env::var("NESTGATE_HEARTBEAT_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10);
        Duration::from_secs(secs)
    }

    /// Default retry interval (configurable via NESTGATE_RETRY_INTERVAL_SECS)
    pub fn retry_interval() -> Duration {
        let secs = env::var("NESTGATE_RETRY_INTERVAL_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);
        Duration::from_secs(secs)
    }

    /// Default test timeout (configurable via NESTGATE_TEST_TIMEOUT_SECS)
    pub fn test_timeout() -> Duration {
        let secs = env::var("NESTGATE_TEST_TIMEOUT_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(5);
        Duration::from_secs(secs)
    }
}

/// Authentication Constants
pub mod auth {
    use std::env;
    use std::time::Duration;

    /// Default session duration (configurable via NESTGATE_SESSION_DURATION_SECS)
    pub fn session_duration() -> Duration {
        Duration::from_secs(
            env::var("NESTGATE_SESSION_DURATION_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600), // 1 hour default
        )
    }

    /// Token prefix (configurable via NESTGATE_TOKEN_PREFIX)
    pub fn token_prefix() -> String {
        env::var("NESTGATE_TOKEN_PREFIX").unwrap_or_else(|_| "nestgate".to_string())
    }
}

/// Service Names
pub mod services {
    use std::env;

    /// API service name (configurable via NESTGATE_API_SERVICE_NAME)
    pub fn api_service_name() -> String {
        env::var("NESTGATE_API_SERVICE_NAME").unwrap_or_else(|_| "nestgate-api".to_string())
    }

    /// UI service name (configurable via NESTGATE_UI_SERVICE_NAME)
    pub fn ui_service_name() -> String {
        env::var("NESTGATE_UI_SERVICE_NAME").unwrap_or_else(|_| "nestgate-ui".to_string())
    }

    /// Orchestrator service name (configurable via NESTGATE_ORCHESTRATOR_SERVICE_NAME)
    pub fn orchestrator_service_name() -> String {
        env::var("NESTGATE_ORCHESTRATOR_SERVICE_NAME")
            .unwrap_or_else(|_| "songbird-orchestrator".to_string())
    }

    /// Database service name (configurable via NESTGATE_DB_SERVICE_NAME)
    pub fn database_service_name() -> String {
        env::var("NESTGATE_DB_SERVICE_NAME").unwrap_or_else(|_| "postgres-db".to_string())
    }

    /// Cache service name (configurable via NESTGATE_CACHE_SERVICE_NAME)
    pub fn cache_service_name() -> String {
        env::var("NESTGATE_CACHE_SERVICE_NAME").unwrap_or_else(|_| "redis-cache".to_string())
    }

    /// BearDog service name (configurable via NESTGATE_BEARDOG_SERVICE_NAME)
    pub fn beardog_service_name() -> String {
        env::var("NESTGATE_BEARDOG_SERVICE_NAME").unwrap_or_else(|_| "beardog".to_string())
    }
}

/// Network Addresses
pub mod addresses {
    use std::env;
    use std::net::{IpAddr, Ipv4Addr};

    /// Default bind address (configurable via NESTGATE_BIND_ADDRESS)
    pub fn bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS").unwrap_or_else(|_| "0.0.0.0".to_string())
    }

    /// Localhost address (configurable via NESTGATE_LOCALHOST)
    pub fn localhost() -> IpAddr {
        env::var("NESTGATE_LOCALHOST")
            .ok()
            .and_then(|addr| addr.parse().ok())
            .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
    }

    /// Default gateway for test networks (configurable via NESTGATE_TEST_GATEWAY)
    pub fn test_gateway() -> String {
        env::var("NESTGATE_TEST_GATEWAY").unwrap_or_else(|_| "192.168.100.1".to_string())
    }

    /// Default test network range (configurable via NESTGATE_TEST_NETWORK)
    pub fn test_network_range() -> String {
        env::var("NESTGATE_TEST_NETWORK").unwrap_or_else(|_| "192.168.100.0/24".to_string())
    }

    /// Default test host address (configurable via NESTGATE_TEST_HOST)
    pub fn test_host_address() -> String {
        env::var("NESTGATE_TEST_HOST").unwrap_or_else(|_| "192.168.100.100".to_string())
    }
}

/// Test Configuration
pub mod test {
    use super::Duration;
    use std::env;

    /// Default test duration (configurable via NESTGATE_TEST_DURATION_SECS)
    pub fn test_duration() -> Duration {
        let secs = env::var("NESTGATE_TEST_DURATION_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(30);
        Duration::from_secs(secs)
    }

    /// Test fixture username (configurable via NESTGATE_TEST_USERNAME)
    pub fn test_username() -> String {
        env::var("NESTGATE_TEST_USERNAME").unwrap_or_else(|_| "testuser".to_string())
    }

    /// Test fixture token prefix (configurable via NESTGATE_TEST_TOKEN_PREFIX)
    pub fn test_token_prefix() -> String {
        env::var("NESTGATE_TEST_TOKEN_PREFIX").unwrap_or_else(|_| "test_token".to_string())
    }
}

/// Time Constants
pub mod time {
    use super::Duration;

    /// One second
    pub const SECOND: Duration = Duration::from_secs(1);

    /// One minute
    pub const MINUTE: Duration = Duration::from_secs(60);

    /// One day duration (configurable via NESTGATE_TIME_DAY_SECS)
    pub fn day() -> Duration {
        Duration::from_secs(
            std::env::var("NESTGATE_TIME_DAY_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(86400), // 24 hours default
        )
    }

    /// One week duration (configurable via NESTGATE_TIME_WEEK_SECS)
    pub fn week() -> Duration {
        Duration::from_secs(
            std::env::var("NESTGATE_TIME_WEEK_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(604800), // 7 days default
        )
    }

    /// One hour duration (configurable via NESTGATE_TIME_HOUR_SECS)
    pub fn hour() -> Duration {
        Duration::from_secs(
            std::env::var("NESTGATE_TIME_HOUR_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600), // 1 hour default
        )
    }

    // Keep const versions for backward compatibility
    pub const DAY: Duration = Duration::from_secs(86400);
    pub const WEEK: Duration = Duration::from_secs(604800);
    pub const HOUR: Duration = Duration::from_secs(3600);
}

/// Default values for configuration when environment variables are not set
pub mod defaults {
    /// Default configuration file path
    pub const CONFIG_FILE: &str = "nestgate.toml";

    /// Default log level
    pub const LOG_LEVEL: &str = "info";

    /// Default data directory
    pub const DATA_DIR: &str = "./data";

    /// Default temp directory
    pub const TEMP_DIR: &str = "/tmp";

    /// Default maximum file size (100MB)
    pub const MAX_FILE_SIZE: usize = 100 * 1024 * 1024;
}

/// SMB Configuration Constants
pub mod smb {
    use std::env;

    /// Default SMB workgroup (configurable via NESTGATE_SMB_WORKGROUP)
    pub fn workgroup() -> String {
        env::var("NESTGATE_SMB_WORKGROUP").unwrap_or_else(|_| "WORKGROUP".to_string())
    }

    /// Default SMB server string (configurable via NESTGATE_SMB_SERVER_STRING)
    pub fn server_string() -> String {
        env::var("NESTGATE_SMB_SERVER_STRING")
            .unwrap_or_else(|_| "NestGate Samba Server".to_string())
    }

    /// Default SMB password chat (configurable via NESTGATE_SMB_PASSWORD_CHAT)
    pub fn password_chat() -> String {
        env::var("NESTGATE_SMB_PASSWORD_CHAT").unwrap_or_else(|_| {
            "*Enter\\snew\\s*\\spassword:* %n\\n *Retype\\snew\\s*\\spassword:* %n\\n *password\\supdated\\ssuccessfully* .".to_string()
        })
    }

    /// Default SMB log file pattern (configurable via NESTGATE_SMB_LOG_FILE)
    pub fn log_file() -> String {
        env::var("NESTGATE_SMB_LOG_FILE").unwrap_or_else(|_| "/var/log/samba/log.%m".to_string())
    }

    /// Default SMB max log size (configurable via NESTGATE_SMB_MAX_LOG_SIZE)
    pub fn max_log_size() -> u32 {
        env::var("NESTGATE_SMB_MAX_LOG_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000)
    }

    /// Default SMB panic action (configurable via NESTGATE_SMB_PANIC_ACTION)
    pub fn panic_action() -> String {
        env::var("NESTGATE_SMB_PANIC_ACTION")
            .unwrap_or_else(|_| "/usr/share/samba/panic-action %d".to_string())
    }

    /// Default SMB passwd program (configurable via NESTGATE_SMB_PASSWD_PROGRAM)
    pub fn passwd_program() -> String {
        env::var("NESTGATE_SMB_PASSWD_PROGRAM").unwrap_or_else(|_| "/usr/bin/passwd %u".to_string())
    }
}

/// Discovery interval for services
pub fn discovery_interval() -> Duration {
    env::var("NESTGATE_DISCOVERY_INTERVAL")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .map(Duration::from_secs)
        .unwrap_or(schedule_defaults::HOURLY_DURATION)
}

/// biomeOS integration constants
pub mod biomeos_defaults {
    /// Default ZFS pool name for biomeOS volumes
    pub const DEFAULT_BIOMEOS_POOL: &str = "nestpool";

    /// Default dataset path prefix for biomeOS volumes
    pub const DEFAULT_BIOMEOS_DATASET_PREFIX: &str = "biomeos";

    /// Default mount path prefix for biomeOS volumes
    pub const DEFAULT_BIOMEOS_MOUNT_PREFIX: &str = "/biomeos";

    /// Default Primal service ID prefix
    pub const DEFAULT_PRIMAL_SERVICE_PREFIX: &str = "primal-nestgate";

    /// Template resource sizes
    pub mod template_sizes {
        /// Small scratch space for temporary operations
        pub const SCRATCH_SPACE_SIZE: &str = "10Gi";

        /// Standard model cache size
        pub const MODEL_CACHE_SIZE: &str = "50Gi";

        /// Standard results storage size
        pub const RESULTS_STORAGE_SIZE: &str = "100Gi";

        /// Large training data storage size
        pub const TRAINING_DATA_SIZE: &str = "500Gi";
    }

    /// Service capability strings
    pub mod capabilities {
        pub const ZFS_POOLS: &str = "zfs-pools";
        pub const TIERED_STORAGE: &str = "tiered-storage";
        pub const SNAPSHOTS: &str = "snapshots";
        pub const VOLUME_PROVISIONING: &str = "volume-provisioning";
        pub const ENCRYPTION: &str = "encryption";
        pub const FEDERATION: &str = "federation";
        pub const COMPRESSION: &str = "compression";
        pub const DEDUPLICATION: &str = "deduplication";
    }

    /// Integration and discovery constants
    pub mod integrations {
        /// Universal service discovery capability
        pub const UNIVERSAL_SERVICE_DISCOVERY: &str = "universal_service_discovery";

        /// Capability-based integration patterns
        pub const CAPABILITY_SECURITY: &str = "security";
        pub const CAPABILITY_ORCHESTRATION: &str = "orchestration";
        pub const CAPABILITY_DISCOVERY: &str = "discovery";
        pub const CAPABILITY_COMPUTE: &str = "compute";
        pub const CAPABILITY_INTELLIGENCE: &str = "intelligence";

        /// Universal protocol support
        pub const PROTOCOL_HTTP: &str = "http";
        pub const PROTOCOL_WEBSOCKET: &str = "websocket";
        pub const PROTOCOL_GRPC: &str = "grpc";
        pub const PROTOCOL_CUSTOM: &str = "custom";

        /// Service categories
        pub const CATEGORY_STORAGE: &str = "storage";
        pub const CATEGORY_SECURITY: &str = "security";
        pub const CATEGORY_COMPUTE: &str = "compute";
        pub const CATEGORY_NETWORK: &str = "network";
        pub const CATEGORY_ORCHESTRATION: &str = "orchestration";
        pub const CATEGORY_INTELLIGENCE: &str = "intelligence";
        pub const CATEGORY_CUSTOM: &str = "custom";
    }
}

/// String constants to avoid repeated allocations
pub mod strings {
    use std::sync::Arc;

    /// Default service configuration strings
    pub const DEFAULT_SERVICE_NAME: &str = "nestgate";
    pub const DEFAULT_SERVICE_VERSION: &str = "1.0.0";
    pub const DEFAULT_SERVICE_DESCRIPTION: &str = "NestGate Universal Storage Primal";
    pub const DEFAULT_BIND_INTERFACE: &str = "127.0.0.1";

    /// Common protocol strings
    pub const LOCALHOST: &str = "localhost";
    pub const LOCALHOST_IP: &str = "127.0.0.1";
    pub const DEFAULT_PROFILE: &str = "balanced";
    pub const DEFAULT_BACKEND: &str = "filesystem";
    pub const DEFAULT_PATH: &str = "/tmp/nestgate";

    /// Security and crypto strings
    pub const FALLBACK_HASH_ALGORITHM: &str = "FALLBACK_HASH";
    pub const FALLBACK_KEY_ID: &str = "fallback_key";
    pub const DEFAULT_TRUST_ANCHOR: &str = "system";
    pub const DEFAULT_TIMEZONE: &str = "UTC";
    pub const DEFAULT_LICENSE: &str = "AGPL-3.0";

    /// Certificate and subject strings
    pub const DEFAULT_CERT_SUBJECT: &str = "CN=NestGate";
    pub const DEFAULT_CERT_ISSUER: &str = "CN=NestGate CA";
    pub const DEFAULT_CERT_CN: &str = "NestGate";
    pub const UNKNOWN_CN: &str = "Unknown";

    /// Common key usage strings
    pub const DIGITAL_SIGNATURE: &str = "digitalSignature";
    pub const KEY_ENCIPHERMENT: &str = "keyEncipherment";
    pub const SERVER_AUTH: &str = "serverAuth";
    pub const CLIENT_AUTH: &str = "clientAuth";

    /// Common permission strings
    pub const PERMISSION_READ: &str = "read";
    pub const PERMISSION_WRITE: &str = "write";
    pub const PERMISSION_HARDWARE_TUNING: &str = "hardware_tuning";

    /// OS and architecture strings
    pub const UNKNOWN_OS: &str = "unknown";
    pub const MACOS: &str = "macOS";
    pub const WINDOWS: &str = "Windows";

    /// Common CPU scaling options
    pub const CPU_SCALING_ONDEMAND: &str = "ondemand";
    pub const CPU_SCALING_PERFORMANCE: &str = "performance";
    pub const CPU_SCALING_POWERSAVE: &str = "powersave";

    /// Lazy shared strings for high-frequency usage
    pub fn default_service_name() -> Arc<str> {
        Arc::from(DEFAULT_SERVICE_NAME)
    }

    pub fn default_service_version() -> Arc<str> {
        Arc::from(DEFAULT_SERVICE_VERSION)
    }

    pub fn localhost_ip() -> Arc<str> {
        Arc::from(LOCALHOST_IP)
    }

    pub fn default_profile() -> Arc<str> {
        Arc::from(DEFAULT_PROFILE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_service_defaults() {
        assert_eq!(service_defaults::DEFAULT_API_SERVICE, "nestgate-api");
        assert_eq!(service_defaults::DEFAULT_UI_SERVICE, "nestgate-ui");
        assert_eq!(service_defaults::DEFAULT_POSTGRES_SERVICE, "postgres-db");
        assert_eq!(service_defaults::DEFAULT_REDIS_SERVICE, "redis-cache");
    }

    #[test]
    fn test_timeout_constants() {
        assert_eq!(
            timeout_defaults::DEFAULT_CONNECTION_TIMEOUT,
            Duration::from_secs(30)
        );
        assert_eq!(
            timeout_defaults::DEFAULT_SESSION_TIMEOUT,
            Duration::from_secs(3600)
        );
        assert_eq!(
            timeout_defaults::DEFAULT_ZFS_OPERATION_TIMEOUT,
            Duration::from_secs(300)
        );
    }

    #[test]
    fn test_schedule_constants() {
        assert_eq!(
            schedule_defaults::DAILY_DURATION,
            Duration::from_secs(86400)
        );
        assert_eq!(
            schedule_defaults::WEEKLY_DURATION,
            Duration::from_secs(604800)
        );
        assert_eq!(
            schedule_defaults::HOURLY_DURATION,
            Duration::from_secs(3600)
        );
    }

    #[test]
    fn test_config_helpers() {
        // Ensure clean environment for test
        std::env::remove_var("NESTGATE_API_SERVICE");
        std::env::remove_var("NESTGATE_API_SERVICE_NAME");

        let api_service = config_helpers::get_api_service_name();
        assert_eq!(api_service, "nestgate-api"); // Default when env var not set

        let timeout = config_helpers::get_connection_timeout();
        assert_eq!(timeout, Duration::from_secs(30)); // Default when env var not set
    }

    #[test]
    fn test_environment_variable_resolution() {
        std::env::set_var("NESTGATE_API_SERVICE", "custom-api");
        let api_service = config_helpers::get_api_service_name();
        assert_eq!(api_service, "custom-api");
        std::env::remove_var("NESTGATE_API_SERVICE");

        // Ensure environment is clean for other tests
        std::env::remove_var("NESTGATE_API_SERVICE_NAME");
    }
}
