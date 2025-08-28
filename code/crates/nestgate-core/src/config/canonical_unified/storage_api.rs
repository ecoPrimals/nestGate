//
// Storage and API configuration structures for the canonical unified configuration system.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

use crate::canonical_modernization::canonical_constants::{
    storage::{GB},
};

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Storage backend type
    pub backend_type: String,
    /// Storage tiers configuration
    pub tiers: StorageTiersConfig,
    /// Compression configuration
    pub compression: CompressionConfig,
    /// Encryption configuration
    pub encryption: StorageEncryptionConfig,
    /// Backup configuration
    pub backup: BackupConfig,
    /// Replication configuration
    pub replication: ReplicationConfig,
    /// Performance tuning
    pub performance: StoragePerformanceConfig,
    /// Cache configuration
    pub cache: CacheStorageConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStorageConfig {
    /// Cache directory path
    pub cache_directory: String,
    /// Cache size in bytes
    pub cache_size_bytes: u64,
    /// Maximum cache entries
    pub max_entries: u64,
    /// Whether cold tier is unlimited
    pub cold_tier_unlimited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct StorageTiersConfig {
    /// Hot tier configuration
    pub hot: TierConfig,
    /// Warm tier configuration
    pub warm: TierConfig,
    /// Cold tier configuration
    pub cold: TierConfig,
    /// Archive tier configuration
    pub archive: TierConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    /// Storage path
    pub path: PathBuf,
    /// Maximum size (bytes)
    pub max_size_bytes: u64,
    /// Compression level (0-9)
    pub compression_level: u8,
    /// Retention policy
    pub retention_days: u32,
    /// Access frequency threshold
    pub access_frequency_threshold: u32,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ApiConfig {
    /// REST API configuration
    pub rest: RestApiConfig,
    /// Streaming configuration
    pub streaming: StreamingConfig,
    /// Server-sent events configuration
    pub sse: SseConfig,
    /// WebSocket configuration
    pub websocket: WebSocketConfig,
    /// Authentication handlers
    pub auth_handlers: AuthHandlerConfig,
    /// Dashboard configuration
    pub dashboard: DashboardConfig,
    /// Load testing configuration
    pub load_testing: LoadTestingConfig,
    /// Workspace configuration
    pub workspace: WorkspaceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestApiConfig {
    /// API version
    pub version: String,
    /// Base path prefix
    pub base_path: String,
    /// Enable API documentation
    pub enable_docs: bool,
    /// Enable CORS
    pub enable_cors: bool,
    /// CORS allowed origins
    pub cors_origins: Vec<String>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ZfsConfig {
    /// Pool configurations
    pub pools: Vec<PoolConfig>,
    /// Dataset configurations
    pub datasets: Vec<DatasetConfig>,
    /// Snapshot configuration
    pub snapshots: SnapshotConfig,
    /// Performance configuration
    pub performance: ZfsPerformanceConfig,
    /// Failsafe configuration
    pub failsafe: FailSafeConfig,
    /// Tiering configuration
    pub tiering: TieringConfig,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageEncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BackupConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub retention_days: u32,
    pub destination: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReplicationConfig {
    pub enabled: bool,
    pub targets: Vec<String>,
    pub sync_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StoragePerformanceConfig {
    pub read_cache_size: u64,
    pub write_cache_size: u64,
    pub io_threads: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StreamingConfig {
    pub enabled: bool,
    pub buffer_size: usize,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SseConfig {
    pub enabled: bool,
    pub heartbeat_interval: Duration,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebSocketConfig {
    pub enabled: bool,
    pub max_connections: u32,
    pub message_size_limit: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthHandlerConfig {
    pub enabled: bool,
    pub session_timeout: Duration,
    pub max_sessions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DashboardConfig {
    pub enabled: bool,
    pub refresh_interval: Duration,
    pub max_data_points: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadTestingConfig {
    pub enabled: bool,
    pub max_concurrent_tests: u32,
    pub default_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceConfig {
    pub enabled: bool,
    pub base_path: PathBuf,
    pub max_workspaces: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PoolConfig {
    pub name: String,
    pub devices: Vec<String>,
    pub raid_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DatasetConfig {
    pub name: String,
    pub pool: String,
    pub quota: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SnapshotConfig {
    pub enabled: bool,
    pub interval: Duration,
    pub retention_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZfsPerformanceConfig {
    pub arc_max: Option<u64>,
    pub prefetch_disable: bool,
    pub sync_disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FailSafeConfig {
    pub enabled: bool,
    pub health_check_interval: Duration,
    pub recovery_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TieringConfig {
    pub enabled: bool,
    pub hot_threshold: u64,
    pub cold_threshold: u64,
}

// ==================== SECTION ====================

impl Default for CacheStorageConfig {
    fn default() -> Self {
        Self {
            cache_directory: "/tmp/nestgate-cache".to_string(),
            cache_size_bytes: GB,
            max_entries: 10000,
            cold_tier_unlimited: false,
        }
    }
}

impl Default for TierConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("/tmp/nestgate-storage"),
            max_size_bytes: 1024 * 1024 * 1024, // 1GB
            compression_level: 6,
            retention_days: 30,
            access_frequency_threshold: 10,
        }
    }
}


impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend_type: "filesystem".to_string(),
            tiers: StorageTiersConfig::default(),
            compression: CompressionConfig::default(),
            encryption: StorageEncryptionConfig::default(),
            backup: BackupConfig::default(),
            replication: ReplicationConfig::default(),
            performance: StoragePerformanceConfig::default(),
            cache: CacheStorageConfig::default(),
        }
    }
}

impl Default for RestApiConfig {
    fn default() -> Self {
        Self {
            version: "v1".to_string(),
            base_path: "/api".to_string(),
            enable_docs: true,
            enable_cors: true,
            cors_origins: vec!["*".to_string()],
        }
    }
}


 