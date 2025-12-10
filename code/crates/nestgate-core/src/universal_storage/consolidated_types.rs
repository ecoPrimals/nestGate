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
/// Types of UniversalStorage
pub enum UniversalStorageType {
    /// Local file system storage
    Local,
    /// Network file system (NFS)
    Nfs {
        /// NFS protocol version
        version: NfsVersion,
    },
    /// Server Message Block (SMB/CIFS)
    Smb {
        /// SMB protocol version
        version: SmbVersion,
    },
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
    Cloud {
        /// Cloud storage provider (AWS, Azure, GCP, or custom)
        provider: CloudProvider,
    },
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}
/// NFS protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Nfsversion
pub enum NfsVersion {
    /// V3
    V3,
    /// V4
    V4,
    /// V41
    V41,
    /// V42
    V42,
}
/// SMB protocol versions  
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Smbversion
pub enum SmbVersion {
    /// V2
    V2,
    /// V3
    V3,
    /// V31
    V31,
}
/// Cloud storage providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Cloudprovider
pub enum CloudProvider {
    /// AWS cloud provider
    AWS {
        /// AWS region identifier (e.g., "us-east-1")
        region: String,
    },
    /// Azure cloud provider
    Azure {
        /// Azure subscription ID
        subscription_id: String,
    },
    /// Google Cloud Platform provider
    GCP {
        /// GCP project ID
        project_id: String,
    },
    /// Custom cloud provider
    Custom {
        /// Custom endpoint URL
        endpoint: String,
    },
}
// ==================== SECTION ====================

/// **THE** Universal Storage Resource - consolidates all storage resource types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Universalstorageresource
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
/// Types of StorageResource
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
/// Storagecapability
pub enum StorageCapability {
    /// Readwrite
    ReadWrite,
    /// Readonly
    ReadOnly,
    /// Streaming
    Streaming,
    /// Replication
    Replication,
    /// Snapshots
    Snapshots,
    /// Compression
    Compression,
    /// Deduplication
    Deduplication,
    /// Encryption
    Encryption,
    /// Versioning
    Versioning,
    /// Backup
    Backup,
    /// Restore operations (data recovery)
    Restore,
    /// Monitoring and observability
    Monitoring,
    /// Custom capability type with arbitrary name
    Custom(String),
}
/// Storage permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagepermissions
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
/// Status values for StorageHealth
pub enum StorageHealthStatus {
    /// Healthy
    Healthy,
    /// Warning
    Warning,
    /// Critical
    Critical,
    /// Offline
    Offline,
    /// Maintenance
    Maintenance,
    /// Unknown
    Unknown,
}
// ==================== SECTION ====================

/// Storage performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageperformancemetrics
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
/// Storageiometrics
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
/// Universalstoragerequest
pub enum UniversalStorageRequest {
    /// Read
    Read {
        /// Optional byte range for partial reads
        range: Option<std::ops::Range<u64>>,
    },
    /// Write
    Write {
        /// Data to write
        data: Vec<u8>,
        /// Whether to overwrite existing data
        overwrite: bool,
    },
    /// Delete
    Delete {
        /// Whether to delete recursively
        recursive: bool,
    },
    /// List resources in storage
    List {
        /// Whether to list recursively
        recursive: bool,
        /// Optional filter pattern
        filter: Option<String>,
    },
    /// Create a new storage resource
    CreateResource {
        /// Resource configuration
        #[allow(deprecated)]
        config: Box<StorageResourceConfig>,
    },
    /// Get resource metadata
    GetMetadata {},
    /// Set resource metadata
    SetMetadata {
        /// Metadata key-value pairs
        metadata: HashMap<String, serde_json::Value>,
    },
    /// Create a snapshot
    Snapshot {
        /// Snapshot name
        name: String,
    },
    /// Restore from snapshot
    Restore {},
    /// Stream data
    Stream {
        /// Optional byte range for streaming
        range: Option<std::ops::Range<u64>>,
    },
    /// Monitor storage events
    Monitor {
        /// Event types to monitor
        events: Vec<StorageEventType>,
    },
}
/// Universal storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Universalstorageresponse
pub enum UniversalStorageResponse {
    /// Readresponse
    ReadResponse {
        /// Data read from storage
        data: Vec<u8>,
        /// Optional metadata about the read operation
        metadata: Option<StorageMetadata>,
    },
    /// Writeresponse
    WriteResponse {
        /// Number of bytes written
        bytes_written: u64,
        /// Optional checksum of written data
        checksum: Option<String>,
    },
    /// Deleteresponse
    DeleteResponse {
        /// Number of items deleted
        deleted_items: u64,
    },
    /// List response with storage items
    ListResponse {
        /// List of storage items
        items: Vec<StorageItem>,
    },
    /// Create response
    CreateResponse {},
    /// Metadata response
    MetadataResponse {
        /// Storage metadata
        metadata: StorageMetadata,
    },
    /// Snapshot response
    SnapshotResponse {
        /// Unique snapshot identifier
        snapshot_id: String,
        /// Timestamp when snapshot was created
        created_at: DateTime<Utc>,
    },
    /// Restore response
    RestoreResponse {
        /// Number of bytes restored
        restored_bytes: u64,
        /// Number of items restored
        restored_items: u64,
    },
    /// Stream response
    StreamResponse {
        /// Unique stream identifier
        stream_id: String,
        /// Size of data chunks in stream
        chunk_size: usize,
    },
    /// Monitor response with event data
    MonitorResponse {
        /// Unique monitor session identifier
        monitor_id: String,
        /// Storage events that occurred
        events: Vec<StorageEvent>,
    },
    /// Error response
    Error {
        /// Error message
        error: String,
        /// Error code for categorization
        error_code: String,
    },
}
/// Storage resource configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::StorageResourceConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::StorageResourceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for StorageResource
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
/// Storageperformancerequirements
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
/// Types of StorageEvent
pub enum StorageEventType {
    /// Created
    Created,
    /// Modified
    Modified,
    /// Deleted
    Deleted,
    /// Moved
    Moved,
    /// Accessed
    Accessed,
    /// Permissionschanged
    PermissionsChanged,
    /// Healthchanged
    HealthChanged,
    /// Capacitychanged
    CapacityChanged,
    /// Performancealert
    PerformanceAlert,
    /// Error
    Error,
}
/// Storage event
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storageevent
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
/// Storageitem
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
/// Types of StorageItem
pub enum StorageItemType {
    /// File
    File,
    /// Directory
    Directory,
    /// Symlink
    Symlink,
    /// Blockdevice
    BlockDevice,
    /// Chardevice
    CharDevice,
    /// Pipe
    Pipe,
    /// Socket
    Socket,
    /// Unknown
    Unknown,
}
/// Storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Storagemetadata
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
/// **DEPRECATED**: Use canonical storage traits instead
#[deprecated(
    since = "0.9.0",
    note = "Use crate::traits::canonical::CanonicalStorage or crate::traits::unified_storage::UnifiedStorage"
)]
/// UniversalStorageBackend trait
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
    #[allow(deprecated)]
    fn initialize(
        &mut self,
        config: StorageResourceConfig,
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Shutdown backend gracefully
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
}

// ==================== SECTION ====================

impl Default for UniversalStorageType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Local
    }
}

impl Default for StorageResourceType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Dataset
    }
}

impl Default for StorageHealthStatus {
    /// Returns the default instance
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for StoragePermissions {
    /// Returns the default instance
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
    /// Returns the default instance
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
    pub fn supports_capability(&self, capability: &StorageCapability) -> bool {
        matches!(
            (self, capability),
            (
                Self::Zfs,
                StorageCapability::Snapshots
                    | StorageCapability::Compression
                    | StorageCapability::Deduplication
            ) | (Self::Object, StorageCapability::Versioning)
                | (Self::Memory | Self::Cache, StorageCapability::ReadWrite)
        )
    }

    /// Get default capabilities for storage type
    #[must_use]
    pub fn default_capabilities(&self) -> Vec<StorageCapability> {
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

#[allow(deprecated)]
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
/// Request parameters for Storage operation
pub struct StorageRequest {
    /// Operation
    pub operation: String,
    /// Path
    pub path: Option<String>,
    /// Data
    pub data: Option<Vec<u8>>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
}

/// Storage response structure for returning operation results
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Storage operation
pub struct StorageResponse {
    /// Success
    pub success: bool,
    /// Data
    pub data: Option<Vec<u8>>,
    /// Additional metadata key-value pairs
    pub metadata: HashMap<String, String>,
    /// Error
    pub error: Option<String>,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Storageresourceconfigcanonical
pub type StorageResourceConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using StorageResourceConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
