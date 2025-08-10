//! Environment Configuration
//!
//! ✅ **MODERNIZED**: Capability-based environment variable naming
//! ❌ **DEPRECATED**: Legacy primal-specific environment variables

use serde::{Deserialize, Serialize};

/// Environment configuration with capability-based variables
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    /// Service configuration
    pub service: ServiceEnvironment,
    /// Capability discovery configuration
    pub discovery: DiscoveryEnvironment,
    /// Storage configuration
    pub storage: StorageEnvironment,
    /// Network configuration
    pub network: NetworkEnvironment,
    /// Security configuration
    pub security: SecurityEnvironment,
}

/// Service environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEnvironment {
    /// Service ID (replaces hardcoded primal names)
    pub service_id: String,
    /// Service port
    pub port: u16,
    /// Bind address
    pub bind_address: String,
    /// Log level
    pub log_level: String,
}

impl Default for ServiceEnvironment {
    fn default() -> Self {
        Self {
            service_id: std::env::var("NESTGATE_SERVICE_ID")
                .unwrap_or_else(|_| format!("nestgate-{}", uuid::Uuid::new_v4().simple())),
            port: std::env::var("NESTGATE_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8080),
            bind_address: std::env::var("NESTGATE_BIND_ADDRESS")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            log_level: std::env::var("NESTGATE_LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
        }
    }
}

/// Capability discovery environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryEnvironment {
    /// Universal discovery endpoint
    pub discovery_url: Option<String>,
    /// Orchestration capability discovery URL
    pub orchestration_discovery_url: Option<String>,
    /// Security capability discovery URL
    pub security_discovery_url: Option<String>,
    /// AI capability discovery URL
    pub ai_discovery_url: Option<String>,
    /// Compute capability discovery URL
    pub compute_discovery_url: Option<String>,
    /// Discovery timeout
    pub discovery_timeout: u64,
}

impl Default for DiscoveryEnvironment {
    fn default() -> Self {
        Self {
            discovery_url: std::env::var("ECOSYSTEM_DISCOVERY_URL").ok(),
            orchestration_discovery_url: std::env::var("ORCHESTRATION_DISCOVERY_URL").ok(),
            security_discovery_url: std::env::var("SECURITY_DISCOVERY_URL").ok(),
            ai_discovery_url: std::env::var("AI_DISCOVERY_URL").ok(),
            compute_discovery_url: std::env::var("COMPUTE_DISCOVERY_URL").ok(),
            discovery_timeout: std::env::var("DISCOVERY_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
        }
    }
}

/// Storage environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEnvironment {
    /// ZFS pool name
    pub zfs_pool: Option<String>,
    /// Storage root path
    pub storage_root: String,
    /// Backup configuration
    pub backup_enabled: bool,
    /// Snapshot configuration
    pub snapshot_interval: u64,
}

impl Default for StorageEnvironment {
    fn default() -> Self {
        Self {
            zfs_pool: std::env::var("NESTGATE_ZFS_POOL").ok(),
            storage_root: std::env::var("NESTGATE_STORAGE_ROOT")
                .unwrap_or_else(|_| "/var/lib/nestgate".to_string()),
            backup_enabled: std::env::var("NESTGATE_BACKUP_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            snapshot_interval: std::env::var("NESTGATE_SNAPSHOT_INTERVAL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600), // 1 hour
        }
    }
}

/// Network environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEnvironment {
    /// API base URL
    pub api_url: String,
    /// WebSocket URL
    pub websocket_url: String,
    /// Enable TLS
    pub tls_enabled: bool,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Key path
    pub key_path: Option<String>,
}

impl Default for NetworkEnvironment {
    fn default() -> Self {
        Self {
            api_url: std::env::var("NESTGATE_API_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            websocket_url: std::env::var("NESTGATE_WEBSOCKET_URL")
                .unwrap_or_else(|_| "ws://localhost:8080/ws".to_string()),
            tls_enabled: std::env::var("NESTGATE_TLS_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            cert_path: std::env::var("NESTGATE_CERT_PATH").ok(),
            key_path: std::env::var("NESTGATE_KEY_PATH").ok(),
        }
    }
}

/// Security environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEnvironment {
    /// Enable authentication
    pub auth_enabled: bool,
    /// JWT secret
    pub jwt_secret: Option<String>,
    /// Security capability endpoint
    pub security_endpoint: Option<String>,
    /// API key for external security services
    pub security_api_key: Option<String>,
}

impl Default for SecurityEnvironment {
    fn default() -> Self {
        Self {
            auth_enabled: std::env::var("NESTGATE_AUTH_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            jwt_secret: std::env::var("NESTGATE_JWT_SECRET").ok(),
            security_endpoint: std::env::var("SECURITY_CAPABILITY_ENDPOINT").ok(),
            security_api_key: std::env::var("SECURITY_API_KEY").ok(),
        }
    }
}
