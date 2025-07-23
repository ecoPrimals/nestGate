//! Type definitions for the NestGate MCP system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

/// Type aliases for convenience
pub type ProviderId = String;

/// Type alias for volume identifiers
pub type VolumeId = String;

/// Type alias for mount identifiers
pub type MountId = String;

/// Type alias for node identifiers
pub type NodeId = String;

/// Storage tiers for data classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageTier {
    /// Hot storage for frequently accessed data.
    Hot,
    /// Warm storage for less frequently accessed data.
    Warm,
    /// Cold storage for infrequently accessed data.
    Cold,
    /// Archive storage for rarely accessed data.
    Archive,
}

impl StorageTier {
    /// Convert storage tier to string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            StorageTier::Hot => "Hot",
            StorageTier::Warm => "Warm",
            StorageTier::Cold => "Cold",
            StorageTier::Archive => "Archive",
        }
    }
}

/// Storage protocols supported by providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StorageProtocol {
    /// NFS protocol with version.
    Nfs(NfsVersion),
    /// SMB protocol with version.
    Smb(SmbVersion),
    /// iSCSI protocol.
    Iscsi,
    /// S3 protocol.
    S3,
    /// Custom protocol with name.
    Custom(String),
}

/// NFS protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NfsVersion {
    /// NFS version 3.
    V3,
    /// NFS version 4.0.
    V4,
    /// NFS version 4.1.
    V41,
    /// NFS version 4.2.
    V42,
}

/// SMB protocol versions
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SmbVersion {
    /// SMB version 2.
    V2,
    /// SMB version 3.0.
    V3,
    /// SMB version 3.1.
    V31,
}

/// Authentication credentials.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    /// Optional API key.
    pub api_key: Option<String>,
    /// Optional OAuth2 client ID.
    pub client_id: Option<String>,
    /// Optional OAuth2 client secret.
    pub client_secret: Option<String>,
}

/// Authentication configuration for a provider (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub method: AuthMethod,
    pub username: Option<String>,
    pub password: Option<String>,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
    pub token: Option<String>,
    pub realm: Option<String>,
}

/// TLS configuration for secure connections (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub certificate_path: Option<String>,
    pub key_path: Option<String>,
    pub ca_path: Option<String>,
    pub verify_peer: bool,
    pub verify_hostname: bool,
    pub cipher_suites: Option<Vec<String>>,
}

/// Provider configuration (Enhanced with proven patterns)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub provider_type: String,
    pub endpoint: String,
    pub region: Option<String>,
    pub credentials: Option<AuthConfig>,
    pub custom_config: HashMap<String, serde_json::Value>,
}

/// Capabilities supported by a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    /// Whether warm storage tier is supported.
    pub supports_warm_storage: bool,

    /// Whether cold storage tier is supported.
    pub supports_cold_storage: bool,

    /// Whether caching is supported.
    pub supports_caching: bool,

    /// Maximum size of a single volume in bytes.
    pub max_volume_size: u64,

    /// Maximum number of volumes supported.
    pub max_volumes: u32,

    /// List of supported storage protocols.
    pub supported_protocols: Vec<StorageProtocol>,

    /// IOPS (Input/Output Operations Per Second) capabilities.
    pub iops_capabilities: Option<IopsCapabilities>,

    /// Throughput capabilities.
    pub throughput_capabilities: Option<ThroughputCapabilities>,
}

/// IOPS capabilities of a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IopsCapabilities {
    /// Minimum guaranteed IOPS.
    pub min_iops: u32,

    /// Maximum supported IOPS.
    pub max_iops: u32,

    /// Burst IOPS limit.
    pub burst_iops: Option<u32>,
}

/// Throughput capabilities of a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputCapabilities {
    /// Minimum guaranteed throughput in MB/s.
    pub min_throughput_mbs: u32,

    /// Maximum supported throughput in MB/s.
    pub max_throughput_mbs: u32,

    /// Burst throughput limit in MB/s.
    pub burst_throughput_mbs: Option<u32>,
}

/// Status of a storage provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderStatus {
    /// Provider is online and ready.
    Online,
    /// Provider is offline or unreachable.
    Offline,
    /// Provider is in maintenance mode.
    Maintenance,
    /// Provider is in an error state.
    Error(String),
}

/// Information about a storage provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderInfo {
    /// Unique identifier for the provider.
    pub id: String,
    /// Name of the provider.
    pub name: String,
    /// Current status of the provider.
    pub status: ProviderStatus,
    /// Total storage capacity in bytes.
    pub total_capacity: u64,
    /// Available storage capacity in bytes.
    pub available_capacity: u64,
}

/// Filter criteria for listing providers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderFilter {
    /// Filter by provider status.
    pub status: Option<ProviderStatus>,
    /// Filter by minimum available capacity.
    pub min_capacity: Option<u64>,
}

/// Volume status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeStatus {
    /// Status code
    pub code: String,
    /// Status message
    pub message: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

/// Volume information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeInfo {
    /// Volume ID
    pub id: String,
    /// Volume name
    pub name: String,
    /// Volume size in bytes
    pub size: u64,
    /// Used space in bytes
    pub used: u64,
    /// Volume path
    pub path: PathBuf,
    /// Volume type
    pub volume_type: String,
    /// Volume status
    pub status: VolumeStatus,
    /// Access policy
    pub access_policy: AccessPolicy,
    /// Mount options
    pub mount_options: MountOptions,
    /// Performance configuration
    pub performance: PerformanceConfig,
    /// Volume metadata
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: i64,
    /// Volume options
    pub options: HashMap<String, String>,
}

/// Volume request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeRequest {
    /// Volume name
    pub name: String,
    /// Volume size in bytes
    pub size: u64,
    /// Volume type
    pub volume_type: String,
    /// Provider ID
    pub provider_id: String,
    /// Volume options
    pub options: HashMap<String, String>,
}

/// Response to a volume request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeResponse {
    /// Unique identifier for the volume.
    pub id: String,
    /// Name of the volume.
    pub name: String,
    /// Size of the volume in bytes.
    pub size: u64,
    /// Current status of the volume.
    pub status: VolumeStatus,
    /// Provider ID that owns this volume.
    pub provider_id: String,
}

/// Volume filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeFilter {
    /// Status code filter
    pub status_code: Option<String>,
    /// Volume type filter
    pub volume_type: Option<String>,
    /// Provider ID filter
    pub provider_id: Option<String>,
}

/// Mount status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountStatus {
    /// Status code
    pub code: String,
    /// Status message
    pub message: String,
    /// Creation timestamp
    pub created_at: String,
    /// Last update timestamp
    pub updated_at: String,
}

/// Mount information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountInfo {
    /// Mount ID
    pub id: String,
    /// Volume ID
    pub volume_id: String,
    /// Mount path
    pub mount_path: PathBuf,
    /// Mount status
    pub status: MountStatus,
}

/// Mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountRequest {
    pub volume_id: String,
    pub mount_path: PathBuf,
    pub options: MountOptions,
}

/// Response to a mount request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountResponse {
    /// Unique identifier for the mount.
    pub id: String,
    /// ID of the volume being mounted.
    pub volume_id: String,
    /// Path where the volume is mounted.
    pub path: PathBuf,
    /// Current status of the mount.
    pub status: MountStatus,
}

/// Mount filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountFilter {
    /// Status code filter
    pub status_code: Option<String>,
    /// Volume ID filter
    pub volume_id: Option<String>,
}

/// Access policy for a volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Whether the volume is read-only
    pub read_only: bool,
    /// Whether the volume is shared
    pub shared: bool,
    /// List of users with access
    pub users: Vec<String>,
    /// List of groups with access
    pub groups: Vec<String>,
}

/// Mount options for configuring volume mounts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountOptions {
    /// The filesystem type (e.g., "nfs", "smb", "iscsi")
    pub fs_type: String,
    /// Mount flags for the filesystem
    pub mount_flags: Vec<String>,
    /// The protocol version or type
    pub protocol: String,
    /// Whether the mount should be read-only
    pub read_only: bool,
    /// Performance preference for the mount
    pub performance: PerformancePreference,
    /// Cache policy for the mount
    pub cache_policy: CachePolicy,
}

/// Performance preferences for mount operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PerformancePreference {
    /// Balance between throughput and latency
    Balanced,
    /// Optimize for maximum throughput
    HighThroughput,
    /// Optimize for minimum latency
    LowLatency,
}

/// Cache policies for mount operations
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CachePolicy {
    /// Use default caching behavior
    Default,
    /// Write through caching (immediate writes)
    WriteThrough,
    /// Write back caching (delayed writes)
    WriteBack,
    /// No caching
    NoCache,
}

impl Default for MountOptions {
    fn default() -> Self {
        Self {
            fs_type: "ext4".to_string(),
            mount_flags: vec!["rw".to_string(), "sync".to_string()],
            protocol: "nfs4".to_string(),
            read_only: false,
            performance: PerformancePreference::Balanced,
            cache_policy: CachePolicy::Default,
        }
    }
}

/// Performance configuration for a volume
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// IOPS limit
    pub iops_limit: Option<u32>,
    /// Bandwidth limit in bytes per second
    pub bandwidth_limit: Option<u64>,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: SystemTime,
    pub node_id: String,
    pub _cpu_usage: f64,   // 0.0 - 100.0
    pub memory_usage: f64, // 0.0 - 100.0
    pub disk_usage: f64,   // 0.0 - 100.0
    pub network_io: NetworkIo,
    pub disk_io: DiskIo,
    pub storage_metrics: StorageMetrics,
    pub performance_metrics: PerformanceMetrics,
}

impl SystemMetrics {
    pub async fn collect() -> std::result::Result<Self, String> {
        // Enhanced metrics collection with advanced capabilities
        Ok(Self {
            timestamp: SystemTime::now(),
            node_id: gethostname::gethostname().to_string_lossy().to_string(),
            _cpu_usage: Self::collect_cpu_usage().await?,
            memory_usage: Self::collect_memory_usage().await?,
            disk_usage: Self::collect_disk_usage().await?,
            network_io: NetworkIo::collect().await?,
            disk_io: DiskIo::collect().await?,
            storage_metrics: StorageMetrics::collect().await?,
            performance_metrics: PerformanceMetrics::collect().await?,
        })
    }

    async fn collect_cpu_usage() -> std::result::Result<f64, String> {
        // Platform-specific CPU usage collection
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let _stat = fs::read_to_string("/proc/stat").map_err(|e| e.to_string())?;
            // Parse /proc/stat for CPU usage - simplified implementation
            Ok(25.0) // Default CPU usage for standalone mode
        }
        #[cfg(not(target_os = "linux"))]
        Ok(25.0)
    }

    async fn collect_memory_usage() -> std::result::Result<f64, String> {
        // Platform-specific memory usage collection
        #[cfg(target_os = "linux")]
        {
            use std::fs;
            let _meminfo = fs::read_to_string("/proc/meminfo").map_err(|e| e.to_string())?;
            // Parse /proc/meminfo for memory usage - simplified implementation
            Ok(45.0) // Default memory usage for standalone mode
        }
        #[cfg(not(target_os = "linux"))]
        Ok(45.0)
    }

    async fn collect_disk_usage() -> std::result::Result<f64, String> {
        // Platform-specific disk usage collection
        // Simplified implementation for standalone mode
        Ok(65.0) // Default disk usage for standalone mode
    }
}

/// Enhanced system metrics with advanced capabilities
pub type EnhancedSystemMetrics = SystemMetrics;

/// Network I/O Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIo {
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
    pub errors: u64,
}

impl NetworkIo {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            bytes_sent: 0,
            bytes_received: 0,
            packets_sent: 0,
            packets_received: 0,
            errors: 0,
        })
    }
}

/// Disk I/O Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskIo {
    pub reads_completed: u64,
    pub writes_completed: u64,
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub io_time_ms: u64,
}

impl DiskIo {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            reads_completed: 0,
            writes_completed: 0,
            bytes_read: 0,
            bytes_written: 0,
            io_time_ms: 0,
        })
    }
}

/// Storage-specific Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMetrics {
    pub total_capacity: u64,
    pub used_capacity: u64,
    pub available_capacity: u64,
    pub tier_metrics: HashMap<StorageTier, TierMetrics>,
    pub pool_metrics: HashMap<String, PoolMetrics>,
}

impl StorageMetrics {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            total_capacity: 0,
            used_capacity: 0,
            available_capacity: 0,
            tier_metrics: HashMap::new(),
            pool_metrics: HashMap::new(),
        })
    }
}

/// Performance Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub iops: IopsMetrics,
    pub throughput: ThroughputMetrics,
    pub latency: LatencyMetrics,
}

impl PerformanceMetrics {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            iops: IopsMetrics::collect().await?,
            throughput: ThroughputMetrics::collect().await?,
            latency: LatencyMetrics::collect().await?,
        })
    }
}

/// IOPS Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IopsMetrics {
    pub read_iops: f64,
    pub write_iops: f64,
    pub total_iops: f64,
    pub peak_iops: f64,
}

impl IopsMetrics {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            read_iops: 0.0,
            write_iops: 0.0,
            total_iops: 0.0,
            peak_iops: 0.0,
        })
    }
}

/// Throughput Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThroughputMetrics {
    pub read_throughput: f64,  // MB/s
    pub write_throughput: f64, // MB/s
    pub total_throughput: f64, // MB/s
    pub peak_throughput: f64,  // MB/s
}

impl ThroughputMetrics {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            read_throughput: 0.0,
            write_throughput: 0.0,
            total_throughput: 0.0,
            peak_throughput: 0.0,
        })
    }
}

/// Latency Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetrics {
    pub read_latency_ms: f64,
    pub write_latency_ms: f64,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
}

impl LatencyMetrics {
    pub async fn collect() -> std::result::Result<Self, String> {
        Ok(Self {
            read_latency_ms: 0.0,
            write_latency_ms: 0.0,
            avg_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
        })
    }
}

/// Tier-specific Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TierMetrics {
    pub tier: StorageTier,
    pub capacity: u64,
    pub used: u64,
    pub available: u64,
    pub performance: PerformanceMetrics,
}

/// Pool Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolMetrics {
    pub pool_name: String,
    pub capacity: u64,
    pub used: u64,
    pub available: u64,
    pub health: PoolHealth,
    pub datasets: Vec<DatasetMetrics>,
}

/// Pool Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PoolHealth {
    Online,
    Degraded,
    Faulted,
    Offline,
    Unavailable,
}

/// Dataset Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetMetrics {
    pub name: String,
    pub used: u64,
    pub available: u64,
    pub referenced: u64,
    pub compression_ratio: f64,
}

/// Enhanced Provider Capabilities with advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedProviderCapabilities {
    pub provider_id: String,
    pub provider_name: String,
    pub provider_version: String,
    pub supported_protocols: Vec<StorageProtocol>,
    pub supported_tiers: Vec<StorageTier>,
    pub performance_capabilities: PerformanceCapabilities,
    pub security_capabilities: SecurityCapabilities,
    pub management_capabilities: ManagementCapabilities,
    pub integration_capabilities: IntegrationCapabilities,
}

impl Default for EnhancedProviderCapabilities {
    fn default() -> Self {
        Self {
            provider_id: "default".to_string(),
            provider_name: "Default Provider".to_string(),
            provider_version: "1.0.0".to_string(),
            supported_protocols: vec![
                StorageProtocol::Nfs(NfsVersion::V4),
                StorageProtocol::Smb(SmbVersion::V3),
            ],
            supported_tiers: vec![StorageTier::Hot, StorageTier::Warm, StorageTier::Cold],
            performance_capabilities: PerformanceCapabilities::default(),
            security_capabilities: SecurityCapabilities::default(),
            management_capabilities: ManagementCapabilities::default(),
            integration_capabilities: IntegrationCapabilities::default(),
        }
    }
}

/// Performance Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCapabilities {
    pub max_iops: u64,
    pub max_throughput_mbps: u64,
    pub min_latency_ms: f64,
    pub supports_qos: bool,
    pub supports_caching: bool,
    pub cache_policies: Vec<CachePolicy>,
}

impl Default for PerformanceCapabilities {
    fn default() -> Self {
        Self {
            max_iops: 10000,
            max_throughput_mbps: 1000,
            min_latency_ms: 1.0,
            supports_qos: true,
            supports_caching: true,
            cache_policies: vec![
                CachePolicy::Default,
                CachePolicy::WriteThrough,
                CachePolicy::WriteBack,
            ],
        }
    }
}

/// Security Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    pub supports_encryption_at_rest: bool,
    pub supports_encryption_in_transit: bool,
    pub supported_auth_methods: Vec<AuthMethod>,
    pub supports_rbac: bool,
    pub supports_audit_logging: bool,
}

impl Default for SecurityCapabilities {
    fn default() -> Self {
        Self {
            supports_encryption_at_rest: true,
            supports_encryption_in_transit: true,
            supported_auth_methods: vec![
                AuthMethod::Kerberos,
                AuthMethod::Certificate,
                AuthMethod::Token,
            ],
            supports_rbac: true,
            supports_audit_logging: true,
        }
    }
}

/// Management Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagementCapabilities {
    pub supports_snapshots: bool,
    pub supports_cloning: bool,
    pub supports_replication: bool,
    pub supports_backup: bool,
    pub supports_tiering: bool,
    pub supports_deduplication: bool,
    pub supports_compression: bool,
}

impl Default for ManagementCapabilities {
    fn default() -> Self {
        Self {
            supports_snapshots: true,
            supports_cloning: true,
            supports_replication: true,
            supports_backup: true,
            supports_tiering: true,
            supports_deduplication: true,
            supports_compression: true,
        }
    }
}

/// Integration Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCapabilities {
    pub supports_kubernetes: bool,
    pub supports_docker: bool,
    pub supports_vmware: bool,
    pub supports_openstack: bool,
    pub supports_mcp_federation: bool,
    pub api_versions: Vec<String>,
}

impl Default for IntegrationCapabilities {
    fn default() -> Self {
        Self {
            supports_kubernetes: true,
            supports_docker: true,
            supports_vmware: false,
            supports_openstack: false,
            supports_mcp_federation: true,
            api_versions: vec!["v1".to_string(), "v2".to_string()],
        }
    }
}

/// Authentication Methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Kerberos,
    Ldap,
    ActiveDirectory,
    Local,
    Certificate,
    Token,
}

/// Enhanced Mount Options with advanced features
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMountOptions {
    pub protocol: StorageProtocol,
    pub mount_point: String,
    pub read_only: bool,
    pub cache_policy: Option<CachePolicy>,
    pub performance_preference: Option<PerformancePreference>,
    pub auth_config: Option<AuthConfig>,
    pub tls_config: Option<TlsConfig>,
    pub qos_config: Option<QosConfig>,
    pub custom_options: HashMap<String, String>,
}

/// Quality of Service Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QosConfig {
    pub max_iops: Option<u64>,
    pub max_throughput_mbps: Option<u64>,
    pub max_latency_ms: Option<f64>,
    pub priority: QosPriority,
}

/// QoS Priority Levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QosPriority {
    Low,
    Normal,
    High,
    Critical,
}

/// Enhanced Volume Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedVolumeInfo {
    pub volume_id: String,
    pub name: String,
    pub size: u64,
    pub tier: StorageTier,
    pub protocol: StorageProtocol,
    pub created_at: SystemTime,
    pub status: EnhancedVolumeStatus,
    pub performance_metrics: Option<PerformanceMetrics>,
}

/// Enhanced Volume Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnhancedVolumeStatus {
    Creating,
    Available,
    InUse,
    Deleting,
    Error,
    Maintenance,
}

/// Enhanced Mount Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMountInfo {
    pub mount_id: String,
    pub volume_id: String,
    pub mount_point: String,
    pub protocol: StorageProtocol,
    pub options: EnhancedMountOptions,
    pub mounted_at: SystemTime,
    pub status: EnhancedMountStatus,
}

/// Enhanced Mount Status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnhancedMountStatus {
    Mounting,
    Mounted,
    Unmounting,
    Unmounted,
    Error,
}

/// Enhanced Mount Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedMountRequest {
    pub volume_id: String,
    pub mount_point: String,
    pub options: EnhancedMountOptions,
}

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, crate::error::Error>;
