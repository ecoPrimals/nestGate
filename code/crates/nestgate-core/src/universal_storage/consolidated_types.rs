use crate::unified_enums::UnifiedTierType;
use crate::Result;
/// **CONSOLIDATED STORAGE TYPES MODULE**
/// Single source of truth for ALL storage-related types across `NestGate`
///
/// This module consolidates and replaces fragmented storage types from:
/// - `universal_storage/types.rs`
/// - `interface/storage_types.rs`  
/// - mcp/types/storage.rs
/// - `temporal_storage.rs` (storage types)
/// - `hardware_tuning.rs` (`StorageDevice`, `StorageType`)
/// - management.rs (`StorageResources`, `BiomeStorage`)
/// - Various API handler storage structs
///
/// **PROBLEM SOLVED**: Single authoritative source for all storage operations
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== SECTION ====================

/// **THE** Universal Storage Type - replaces all `StorageType` enums
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UniversalStorageType {
    /// Local file system storage
    Local,
    /// Network file system (NFS)
    Nfs { version: NfsVersion },
    /// Server Message Block (SMB/CIFS)
    Smb { version: SmbVersion },
    /// Object storage (S3-compatible)
    Object,
    /// Block storage
    Block,
    /// ZFS-based storage
    Zfs,
    /// Database storage
    Database,
    /// In-memory storage
    Memory,
    /// Cache storage
    Cache,
    /// Cloud storage
    Cloud { provider: CloudProvider },
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}
/// NFS protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NfsVersion {
    V3,
    V4,
    V41,
    V42,
}
/// SMB protocol versions  
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SmbVersion {
    V2,
    V3,
    V31,
}
/// Cloud storage providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CloudProvider {
    AWS { region: String },
    Azure { subscription_id: String },
    GCP { project_id: String },
    Custom { endpoint: String },
}
// ==================== SECTION ====================

/// **THE** Universal Storage Resource - consolidates all storage resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalStorageResource {
    /// Unique resource identifier
    pub resource_id: String,
    /// Human-readable name
    pub name: String,
    /// Storage type
    pub storage_type: UniversalStorageType,
    /// Resource type (dataset, pool, volume, etc.)
    pub resource_type: StorageResourceType,
    /// Storage path or location
    /// Resource size in bytes
    pub size_bytes: u64,
    /// Available space in bytes
    pub available_bytes: u64,
    /// Used space in bytes
    pub used_bytes: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modified timestamp
    pub modified_at: DateTime<Utc>,
    /// Last accessed timestamp
    pub accessed_at: Option<DateTime<Utc>>,
    /// Storage tier
    pub tier: UnifiedTierType,
    /// Resource capabilities
    pub capabilities: Vec<StorageCapability>,
    /// Performance metrics
    pub performance: StoragePerformanceMetrics,
    /// Resource metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Resource tags
    pub tags: Vec<String>,
    /// Access permissions
    pub permissions: StoragePermissions,
    /// Health status
    pub health_status: StorageHealthStatus,
}
/// Storage resource types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageResourceType {
    /// Storage pool
    Pool,
    /// Dataset within a pool
    Dataset,
    /// Volume (block device)
    Volume,
    /// Snapshot
    Snapshot,
    /// Backup
    Backup,
    /// Cache
    Cache,
    /// Custom resource type
    Custom(String),
}
/// Storage capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageCapability {
    ReadWrite,
    ReadOnly,
    Streaming,
    Replication,
    Snapshots,
    Compression,
    Deduplication,
    Encryption,
    Versioning,
    Backup,
    Restore,
    Monitoring,
    Custom(String),
}
/// Storage permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePermissions {
    /// Owner permissions
    pub owner: Vec<String>,
    /// Group permissions
    pub group: Vec<String>,
    /// Other permissions
    pub other: Vec<String>,
    /// Access control list
    pub acl: HashMap<String, Vec<String>>,
}
/// Storage health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageHealthStatus {
    Healthy,
    Warning,
    Critical,
    Offline,
    Maintenance,
    Unknown,
}
// ==================== SECTION ====================

/// Storage performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceMetrics {
    /// Read operations per second
    pub read_ops_per_sec: f64,
    /// Write operations per second
    pub write_ops_per_sec: f64,
    /// Read throughput in bytes per second
    pub read_bytes_per_sec: u64,
    /// Write throughput in bytes per second
    pub write_bytes_per_sec: u64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Queue depth
    pub queue_depth: u32,
    /// Utilization percentage (0-100)
    pub utilization_percent: f64,
    /// Last updated timestamp
    pub last_updated: DateTime<Utc>,
}
/// Storage I/O metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageIoMetrics {
    /// Total read operations
    pub total_reads: u64,
    /// Total write operations
    pub total_writes: u64,
    /// Total bytes read
    pub total_bytes_read: u64,
    /// Total bytes written
    pub total_bytes_written: u64,
    /// Total errors
    pub total_errors: u64,
    /// Average response time
    pub avg_response_time: Duration,
}
// ==================== SECTION ====================

/// Universal storage request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniversalStorageRequest {
    Read {
        range: Option<std::ops::Range<u64>>,
    },
    Write {
        data: Vec<u8>,
        overwrite: bool,
    },
    Delete {
        recursive: bool,
    },
    List {
        recursive: bool,
        filter: Option<String>,
    },
    CreateResource {
        config: Box<StorageResourceConfig>,
    },
    GetMetadata {},
    SetMetadata {
        metadata: HashMap<String, serde_json::Value>,
    },
    Snapshot {
        name: String,
    },
    Restore {},
    Stream {
        range: Option<std::ops::Range<u64>>,
    },
    Monitor {
        events: Vec<StorageEventType>,
    },
}
/// Universal storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UniversalStorageResponse {
    ReadResponse {
        data: Vec<u8>,
        metadata: Option<StorageMetadata>,
    },
    WriteResponse {
        bytes_written: u64,
        checksum: Option<String>,
    },
    DeleteResponse {
        deleted_items: u64,
    },
    ListResponse {
        items: Vec<StorageItem>,
    },
    CreateResponse {},
    MetadataResponse {
        metadata: StorageMetadata,
    },
    SnapshotResponse {
        snapshot_id: String,
        created_at: DateTime<Utc>,
    },
    RestoreResponse {
        restored_bytes: u64,
        restored_items: u64,
    },
    StreamResponse {
        stream_id: String,
        chunk_size: usize,
    },
    MonitorResponse {
        monitor_id: String,
        events: Vec<StorageEvent>,
    },
    Error {
        error: String,
        error_code: String,
    },
}
/// Storage resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceConfig {
    /// Resource name
    pub name: String,
    /// Storage type
    pub storage_type: UniversalStorageType,
    /// Resource type
    pub resource_type: StorageResourceType,
    /// Initial size (if applicable)
    pub initial_size: Option<u64>,
    /// Storage tier preference
    pub tier: Option<UnifiedTierType>,
    /// Capabilities to enable
    pub capabilities: Vec<StorageCapability>,
    /// Configuration options
    pub options: HashMap<String, serde_json::Value>,
    /// Performance requirements
    pub performance_requirements: Option<StoragePerformanceRequirements>,
}
/// Storage performance requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoragePerformanceRequirements {
    /// Minimum read IOPS
    pub min_read_iops: Option<u32>,
    /// Minimum write IOPS
    pub min_write_iops: Option<u32>,
    /// Minimum throughput in bytes per second
    pub min_throughput_bps: Option<u64>,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: Option<f64>,
    /// Required availability percentage
    pub required_availability: Option<f64>,
}
// ==================== SECTION ====================

/// Storage event types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageEventType {
    Created,
    Modified,
    Deleted,
    Moved,
    Accessed,
    PermissionsChanged,
    HealthChanged,
    CapacityChanged,
    PerformanceAlert,
    Error,
}
/// Storage event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEvent {
    /// Event ID
    pub event_id: String,
    /// Event type
    pub event_type: StorageEventType,
    /// Resource path
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Related resource ID
    pub resource_id: Option<String>,
}
// ==================== SECTION ====================

/// Storage item (file or directory)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageItem {
    /// Item name
    pub name: String,
    /// Full path
    /// Item type
    pub item_type: StorageItemType,
    /// Size in bytes
    pub size: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub modified_at: DateTime<Utc>,
    /// Last access timestamp
    pub accessed_at: Option<DateTime<Utc>>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Checksum
    pub checksum: Option<String>,
    /// Extended metadata
    pub metadata: HashMap<String, serde_json::Value>,
}
/// Storage item types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StorageItemType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Pipe,
    Socket,
    Unknown,
}
/// Storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetadata {
    /// Content type
    pub content_type: Option<String>,
    /// Content encoding
    pub content_encoding: Option<String>,
    /// Content language
    pub content_language: Option<String>,
    /// Cache control
    pub cache_control: Option<String>,
    /// `ETag`
    pub etag: Option<String>,
    /// Custom metadata
    pub custom: HashMap<String, String>,
    /// System metadata
    pub system: HashMap<String, serde_json::Value>,
}
// ==================== SECTION ====================

/// **THE** Universal Storage Backend trait
/// Consolidates all storage backend interfaces
pub trait UniversalStorageBackend: Send + Sync {
    /// Handle a storage request
    fn handle_request(
        &self,
        request: UniversalStorageRequest,
    ) -> impl std::future::Future<Output = Result<UniversalStorageResponse>> + Send;
    /// Get backend type
    fn backend_type(&self) -> UniversalStorageType;

    /// Get backend capabilities
    fn capabilities(&self) -> Vec<StorageCapability>;

    /// Check if backend is available
    fn is_available(&self) -> impl std::future::Future<Output = bool> + Send;

    /// Perform health check
    fn health_check(&self)
        -> impl std::future::Future<Output = Result<StorageHealthStatus>> + Send;

    /// Get performance metrics
    fn get_metrics(
        &self,
    ) -> impl std::future::Future<Output = Result<StoragePerformanceMetrics>> + Send;

    /// Initialize backend with configuration
    fn initialize(
        &mut self,
        config: StorageResourceConfig,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Shutdown backend gracefully
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
}

// ==================== SECTION ====================

impl Default for UniversalStorageType {
    fn default() -> Self {
        Self::Local
    }
}

impl Default for StorageResourceType {
    fn default() -> Self {
        Self::Dataset
    }
}

impl Default for StorageHealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for StoragePermissions {
    fn default() -> Self {
        Self {
            owner: vec!["read".to_string(), "write".to_string()],
            group: vec!["read".to_string()],
            other: vec![],
            acl: HashMap::new(),
        }
    }
}

impl Default for StoragePerformanceMetrics {
    fn default() -> Self {
        Self {
            read_ops_per_sec: 0.0,
            write_ops_per_sec: 0.0,
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            avg_latency_ms: 0.0,
            queue_depth: 0,
            utilization_percent: 0.0,
            last_updated: Utc::now(),
        }
    }
}

// ==================== SECTION ====================

impl UniversalStorageType {
    /// Check if storage type supports a capability
    #[must_use]
    pub const fn supports_capability(&self, capability: &StorageCapability) -> bool {
        matches!(
            (self, capability),
            (Self::Zfs, StorageCapability::Snapshots | StorageCapability::Compression | StorageCapability::Deduplication)
                | (Self::Object, StorageCapability::Versioning)
                | (Self::Memory | Self::Cache, StorageCapability::ReadWrite)
        )
    }

    /// Get default capabilities for storage type
    #[must_use]
    pub const fn default_capabilities(&self) -> Vec<StorageCapability> {
        match self {
            Self::Zfs => vec![
                StorageCapability::ReadWrite,
                StorageCapability::Snapshots,
                StorageCapability::Compression,
                StorageCapability::Deduplication,
                StorageCapability::Replication,
            ],
            Self::Object => vec![
                StorageCapability::ReadWrite,
                StorageCapability::Versioning,
                StorageCapability::Backup,
            ],
            Self::Memory => vec![StorageCapability::ReadWrite, StorageCapability::Streaming],
            _ => vec![StorageCapability::ReadWrite],
        }
    }
}

impl StorageResourceConfig {
    /// Create a new storage resource configuration
    #[must_use]
    pub fn new(name: String, storage_type: UniversalStorageType) -> Self {
        Self {
            name,
            storage_type: storage_type.clone(),
            resource_type: StorageResourceType::Dataset,
            initial_size: None,
            tier: None,
            capabilities: storage_type.default_capabilities(),
            options: HashMap::new(),
            performance_requirements: None,
        }
    }

    /// Set storage tier
    #[must_use]
    pub fn with_tier(mut self, tier: UnifiedTierType) -> Self {
        self.tier = Some(tier);
        self
    }

    /// Set initial size
    #[must_use]
    pub fn with_size(mut self, size: u64) -> Self {
        self.initial_size = Some(size);
        self
    }

    /// Add capability
    #[must_use]
    pub fn with_capability(mut self, capability: StorageCapability) -> Self {
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
        self
    }
}

/// Storage request structure for handling storage operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageRequest {
    pub operation: String,
    pub path: Option<String>,
    pub data: Option<Vec<u8>>,
    pub metadata: HashMap<String, String>,
}

/// Storage response structure for returning operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResponse {
    pub success: bool,
    pub data: Option<Vec<u8>>,
    pub metadata: HashMap<String, String>,
    pub error: Option<String>,
}
