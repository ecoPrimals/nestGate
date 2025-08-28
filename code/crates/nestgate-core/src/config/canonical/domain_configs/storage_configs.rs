/// Storage Configuration Domain
///
/// Replaces: StorageConfig, BackendConfig, ReplicationConfig, StorageResourceConfig,
/// FilesystemConfig, BlockStorageConfig, ObjectStorageConfig, NetworkFsConfig, and 10+ others
use super::CanonicalDomainConfig;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;

/// **CANONICAL STORAGE CONFIGURATION**
/// Replaces: StorageConfig, BackendConfig, ReplicationConfig, StorageResourceConfig,
/// FilesystemConfig, BlockStorageConfig, ObjectStorageConfig, NetworkFsConfig, and 10+ others
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalStorageConfig {
    /// Storage backend configurations
    pub backends: StorageBackends,
    /// Replication settings
    pub replication: StorageReplication,
    /// Performance tuning
    pub performance: StoragePerformance,
    /// Security settings
    pub security: StorageSecurity,
    /// Monitoring settings
    pub monitoring: StorageMonitoring,
    /// Tier management settings
    pub tiers: StorageTiers,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}

impl CanonicalDomainConfig for CanonicalStorageConfig {
    fn domain() -> &'static str {
        "storage"
    }

    fn validate(&self) -> Result<()> {
        // Validate performance settings
        if self.performance.cache_size_mb == 0 {
            return Err(NestGateError::config_error(
                "cache_size_mb",
                "must be greater than 0",
            ));
        }

        if self.performance.io_threads == 0 {
            return Err(NestGateError::config_error(
                "io_threads",
                "must be greater than 0",
            ));
        }

        Ok(())
    }

    fn merge(mut self, other: Self) -> Self {
        // Merge performance settings
        self.performance.cache_size_mb = other
            .performance
            .cache_size_mb
            .max(self.performance.cache_size_mb);
        self.performance.io_threads = other
            .performance
            .io_threads
            .max(self.performance.io_threads);

        // Merge environment overrides
        self.environment_overrides
            .extend(other.environment_overrides);

        self
    }

    fn from_environment() -> Result<Self> {
        Ok(Self::default())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "backends": {
                    "type": "object",
                    "description": "Storage backend configurations"
                },
                "replication": {
                    "type": "object",
                    "description": "Replication settings"
                },
                "performance": {
                    "type": "object",
                    "description": "Performance tuning settings"
                }
            },
            "required": ["backends", "performance"]
        })
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageBackends {
    pub filesystem: Option<FilesystemBackend>,
    pub block: Option<BlockBackend>,
    pub object: Option<ObjectBackend>,
    pub network: Option<NetworkBackend>,
    pub zfs: Option<ZfsBackend>,
    pub memory: Option<MemoryBackend>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageReplication {
    pub enabled: bool,
    pub sync_mode: ReplicationSyncMode,
    pub replica_count: u32,
    pub conflict_resolution: ConflictResolutionStrategy,
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformance {
    pub cache_size_mb: u64,
    pub io_threads: u32,
    pub prefetch_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSecurity {
    pub enable_encryption: bool,
    pub key_rotation_interval: Duration,
    pub access_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMonitoring {
    pub enable_metrics: bool,
    pub metrics_interval: Duration,
    pub alert_thresholds: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageTiers {
    pub hot_tier: TierConfig,
    pub warm_tier: TierConfig,
    pub cold_tier: TierConfig,
    pub archive_tier: TierConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierConfig {
    pub storage_type: String,
    pub performance_class: String,
    pub retention_policy: Duration,
    pub migration_threshold: f64,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilesystemBackend {
    pub base_path: PathBuf,
    pub permissions: u32,
    pub enable_compression: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBackend {
    pub device_path: String,
    pub block_size: u32,
    pub enable_encryption: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectBackend {
    pub endpoint: String,
    pub bucket: String,
    pub access_key: String,
    pub secret_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkBackend {
    pub protocol: NetworkProtocol,
    pub endpoints: Vec<String>,
    pub mount_options: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsBackend {
    pub pool_name: String,
    pub dataset_prefix: String,
    pub enable_snapshots: bool,
    pub compression: ZfsCompression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryBackend {
    pub max_size_mb: u64,
    pub enable_persistence: bool,
    pub persistence_path: Option<PathBuf>,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReplicationSyncMode {
    Synchronous,
    Asynchronous,
    SemiSynchronous,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConflictResolutionStrategy {
    LastWriteWins,
    FirstWriteWins,
    Manual,
    Merge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Nfs,
    Smb,
    Http,
    Https,
    Ftp,
    Sftp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZfsCompression {
    None,
    Lz4,
    Zstd,
    Gzip,
}

// ==================== SECTION ====================

impl Default for StoragePerformance {
    fn default() -> Self {
        Self {
            cache_size_mb: 512,
            io_threads: 4,
            prefetch_enabled: true,
        }
    }
}

impl Default for StorageSecurity {
    fn default() -> Self {
        Self {
            enable_encryption: true,
            key_rotation_interval: Duration::from_secs(86400), // 24 hours
            access_logging: true,
        }
    }
}

impl Default for StorageMonitoring {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_interval: Duration::from_secs(60),
            alert_thresholds: HashMap::new(),
        }
    }
}

impl Default for StorageTiers {
    fn default() -> Self {
        Self {
            hot_tier: TierConfig {
                storage_type: "ssd".to_string(),
                performance_class: "high".to_string(),
                retention_policy: Duration::from_secs(86400 * 7), // 7 days
                migration_threshold: 0.8,
            },
            warm_tier: TierConfig {
                storage_type: "hdd".to_string(),
                performance_class: "medium".to_string(),
                retention_policy: Duration::from_secs(86400 * 30), // 30 days
                migration_threshold: 0.6,
            },
            cold_tier: TierConfig {
                storage_type: "archive".to_string(),
                performance_class: "low".to_string(),
                retention_policy: Duration::from_secs(86400 * 365), // 1 year
                migration_threshold: 0.3,
            },
            archive_tier: TierConfig {
                storage_type: "tape".to_string(),
                performance_class: "archive".to_string(),
                retention_policy: Duration::from_secs(86400 * 365 * 7), // 7 years
                migration_threshold: 0.1,
            },
        }
    }
}

impl Default for ReplicationSyncMode {
    fn default() -> Self {
        Self::Asynchronous
    }
}

impl Default for ConflictResolutionStrategy {
    fn default() -> Self {
        Self::LastWriteWins
    }
}

impl Default for NetworkProtocol {
    fn default() -> Self {
        Self::Http
    }
}

impl Default for ZfsCompression {
    fn default() -> Self {
        Self::Lz4
    }
}
