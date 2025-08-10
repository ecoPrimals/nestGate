//! Unified Storage Traits - THE Canonical Storage Interface System
//!
//! This module consolidates all fragmented storage trait definitions into a single,
//! comprehensive trait hierarchy that serves as the definitive storage interface
//! for the entire NestGate ecosystem.
//!
//! **CONSOLIDATES**:
//! - backends/mod.rs::StorageBackend (basic file operations)
//! - consolidated_types.rs::UniversalStorageBackend (request/response pattern)
//! - types.rs::UniversalStorageBackend (streaming + monitoring)
//! - traits.rs::StorageProvider + StorageCapability (provider pattern)
//!
//! **PROVIDES**:
//! - Single canonical storage trait hierarchy
//! - Comprehensive operation support (CRUD, streaming, monitoring, health)
//! - Unified metadata and error handling
//! - Factory pattern for backend creation
//! - Zero-cost abstractions where possible

use crate::error::{NestGateError, Result};
use crate::unified_enums::{UnifiedHealthStatus, UnifiedServiceState};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ==================== CORE STORAGE TRAITS ====================

/// **THE** Unified Storage Backend trait - consolidates all storage interfaces
/// This is the canonical trait that all storage backends must implement
#[async_trait]
pub trait UnifiedStorageBackend: Send + Sync {
    // ===== BASIC OPERATIONS =====
    /// Read data from storage
    async fn read(&self, path: &str) -> Result<Vec<u8>>;

    /// Write data to storage
    async fn write(&self, path: &str, data: &[u8]) -> Result<()>;

    /// Delete from storage
    async fn delete(&self, path: &str) -> Result<()>;

    /// Check if path exists
    async fn exists(&self, path: &str) -> Result<bool>;

    /// List items at path
    async fn list(&self, path: &str) -> Result<Vec<UnifiedStorageItem>>;

    /// Get metadata for item
    async fn metadata(&self, path: &str) -> Result<UnifiedStorageMetadata>;

    // ===== ADVANCED OPERATIONS =====
    /// Handle complex storage requests
    async fn handle_request(
        &self,
        request: UnifiedStorageRequest,
    ) -> Result<UnifiedStorageResponse>;

    /// Stream data for real-time operations
    async fn stream_data(&self, request: StreamRequest) -> Result<DataStream>;

    /// Monitor changes for real-time synchronization
    async fn monitor_changes(&self, path: &str) -> Result<ChangeStream>;

    // ===== BACKEND MANAGEMENT =====
    /// Get backend type identifier
    fn backend_type(&self) -> UnifiedStorageType;

    /// Get backend capabilities
    fn capabilities(&self) -> Vec<UnifiedStorageCapability>;

    /// Check if backend is available
    async fn is_available(&self) -> bool;

    /// Perform comprehensive health check
    async fn health_check(&self) -> Result<UnifiedStorageHealth>;

    /// Get performance metrics
    async fn get_metrics(&self) -> Result<UnifiedStorageMetrics>;

    /// Initialize backend with configuration
    async fn initialize(&mut self, config: UnifiedStorageConfig) -> Result<()>;

    /// Shutdown backend gracefully
    async fn shutdown(&mut self) -> Result<()>;
}

/// Unified Storage Provider trait for ecosystem integration
#[async_trait]
pub trait UnifiedStorageProvider: Send + Sync {
    /// Provider identification
    fn provider_name(&self) -> &str;
    fn provider_version(&self) -> &str;

    /// Capability discovery
    async fn can_handle(&self, storage_type: &UnifiedStorageType) -> bool;
    async fn discover_backends(&self) -> Result<Vec<BackendInfo>>;

    /// Provider lifecycle
    async fn initialize(&mut self) -> Result<()>;
    async fn health_check(&self) -> Result<UnifiedProviderHealth>;
    async fn shutdown(&mut self) -> Result<()>;

    /// Backend creation
    async fn create_backend(
        &self,
        config: UnifiedStorageConfig,
    ) -> Result<Box<dyn UnifiedStorageBackend>>;
}

// ==================== UNIFIED DATA TYPES ====================

/// Unified storage type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnifiedStorageType {
    /// Local filesystem storage
    FileSystem,
    /// In-memory storage
    Memory,
    /// Object storage (S3-compatible)
    ObjectStorage,
    /// Block storage
    BlockStorage,
    /// Network filesystem (NFS, SMB, etc.)
    NetworkFileSystem,
    /// ZFS storage
    Zfs,
    /// Distributed storage
    Distributed,
    /// Custom storage type
    Custom(String),
}

/// Unified storage capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnifiedStorageCapability {
    /// Basic read/write operations
    BasicOperations,
    /// Streaming data support
    Streaming,
    /// Real-time monitoring
    Monitoring,
    /// Metadata operations
    Metadata,
    /// Atomic operations
    Atomic,
    /// Versioning support
    Versioning,
    /// Encryption support
    Encryption,
    /// Compression support
    Compression,
    /// Replication support
    Replication,
    /// Backup support
    Backup,
    /// Custom capability
    Custom(String),
}

/// Unified storage item representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageItem {
    pub path: String,
    pub item_type: UnifiedStorageItemType,
    pub size: Option<u64>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub modified: Option<chrono::DateTime<chrono::Utc>>,
    pub accessed: Option<chrono::DateTime<chrono::Utc>>,
    pub metadata: UnifiedStorageMetadata,
}

/// Unified storage item types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnifiedStorageItemType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharDevice,
    Pipe,
    Socket,
    Unknown,
}

/// Unified storage metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageMetadata {
    // Core metadata
    pub size: u64,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
    pub accessed: Option<chrono::DateTime<chrono::Utc>>,

    // Content metadata
    pub content_type: Option<String>,
    pub content_encoding: Option<String>,
    pub content_language: Option<String>,
    pub cache_control: Option<String>,
    pub etag: Option<String>,

    // System metadata
    pub permissions: Option<String>,
    pub owner: Option<String>,
    pub group: Option<String>,
    pub checksum: Option<String>,

    // Custom metadata
    pub custom: HashMap<String, String>,
    pub system: HashMap<String, serde_json::Value>,
}

/// Unified storage health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageHealth {
    pub status: UnifiedHealthStatus,
    pub message: String,
    pub details: HashMap<String, String>,
    pub metrics: UnifiedStorageMetrics,
    pub last_check: chrono::DateTime<chrono::Utc>,
}

/// Unified storage performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageMetrics {
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub error_rate: f64,
    pub throughput_mbps: f64,
    pub storage_utilization: f64,
    pub custom_metrics: HashMap<String, f64>,
}

/// Unified provider health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedProviderHealth {
    pub is_healthy: bool,
    pub status: UnifiedServiceState,
    pub message: String,
    pub backend_count: usize,
    pub healthy_backends: usize,
    pub metrics: HashMap<String, String>,
}

// ==================== REQUEST/RESPONSE TYPES ====================

/// Unified storage request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedStorageRequest {
    Read {
        path: String,
        range: Option<std::ops::Range<u64>>,
    },
    Write {
        path: String,
        data: Vec<u8>,
        metadata: Box<Option<UnifiedStorageMetadata>>,
    },
    Delete {
        path: String,
    },
    List {
        path: String,
        recursive: bool,
    },
    Metadata {
        path: String,
    },
    Exists {
        path: String,
    },
    Copy {
        source: String,
        destination: String,
    },
    Move {
        source: String,
        destination: String,
    },
    CreateDirectory {
        path: String,
    },
    Custom {
        operation: String,
        parameters: HashMap<String, serde_json::Value>,
    },
}

/// Unified storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnifiedStorageResponse {
    Data {
        data: Vec<u8>,
        metadata: Option<UnifiedStorageMetadata>,
    },
    Items {
        items: Vec<UnifiedStorageItem>,
    },
    Metadata {
        metadata: UnifiedStorageMetadata,
    },
    Success {
        message: String,
    },
    Boolean {
        value: bool,
    },
    Custom {
        result: serde_json::Value,
    },
}

/// Stream request for real-time operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamRequest {
    pub path: String,
    pub operation: StreamOperation,
    pub buffer_size: Option<usize>,
    pub timeout: Option<std::time::Duration>,
}

/// Stream operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StreamOperation {
    Read,
    Write,
    Monitor,
    Custom(String),
}

/// Data stream type alias
pub type DataStream = tokio::sync::mpsc::Receiver<Result<Vec<u8>>>;

/// Change stream type alias  
pub type ChangeStream = tokio::sync::mpsc::Receiver<Result<StorageChange>>;

/// Storage change event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageChange {
    pub path: String,
    pub change_type: ChangeType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: Option<UnifiedStorageMetadata>,
}

/// Change types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Created,
    Modified,
    Deleted,
    Moved { from: String },
    Copied { from: String },
}

// ==================== CONFIGURATION TYPES ====================

/// Unified storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedStorageConfig {
    pub backend_type: UnifiedStorageType,
    pub connection: ConnectionConfig,
    pub performance: PerformanceConfig,
    pub security: SecurityConfig,
    pub custom: HashMap<String, serde_json::Value>,
}

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub endpoint: Option<String>,
    pub timeout: std::time::Duration,
    pub retry_attempts: u32,
    pub pool_size: Option<usize>,
}

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    pub buffer_size: usize,
    pub concurrent_operations: usize,
    pub cache_enabled: bool,
    pub compression_enabled: bool,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
    pub authentication: Option<AuthenticationConfig>,
    pub permissions: Option<PermissionConfig>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub method: AuthMethod,
    pub credentials: HashMap<String, String>,
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    None,
    ApiKey,
    Bearer,
    Basic,
    Custom(String),
}

/// Permission configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionConfig {
    pub read: bool,
    pub write: bool,
    pub delete: bool,
    pub admin: bool,
}

/// Backend information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendInfo {
    pub name: String,
    pub backend_type: UnifiedStorageType,
    pub capabilities: Vec<UnifiedStorageCapability>,
    pub health: UnifiedHealthStatus,
    pub endpoint: Option<String>,
    pub metadata: HashMap<String, String>,
}

// ==================== FACTORY PATTERN ====================

/// Unified backend factory for creating storage backends
pub struct UnifiedBackendFactory {
    providers: Arc<RwLock<HashMap<UnifiedStorageType, Box<dyn UnifiedStorageProvider>>>>,
}

impl Default for UnifiedBackendFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl UnifiedBackendFactory {
    /// Create a new backend factory
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a storage provider
    pub async fn register_provider(
        &self,
        storage_type: UnifiedStorageType,
        provider: Box<dyn UnifiedStorageProvider>,
    ) -> Result<()> {
        let mut providers = self.providers.write().await;
        providers.insert(storage_type, provider);
        Ok(())
    }

    /// Create a backend of the specified type
    pub async fn create_backend(
        &self,
        config: UnifiedStorageConfig,
    ) -> Result<Box<dyn UnifiedStorageBackend>> {
        let providers = self.providers.read().await;
        let provider =
            providers
                .get(&config.backend_type)
                .ok_or_else(|| NestGateError::Storage {
                    operation: "create_backend".to_string(),
                    details: format!(
                        "Storage provider not found for type: {:?}",
                        config.backend_type
                    ),
                })?;

        provider.create_backend(config).await
    }

    /// List available backend types
    pub async fn available_types(&self) -> Vec<UnifiedStorageType> {
        let providers = self.providers.read().await;
        providers.keys().cloned().collect()
    }
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for UnifiedStorageType {
    fn default() -> Self {
        Self::FileSystem
    }
}

impl Default for UnifiedStorageMetadata {
    fn default() -> Self {
        let now = chrono::Utc::now();
        Self {
            size: 0,
            created: now,
            modified: now,
            accessed: Some(now),
            content_type: None,
            content_encoding: None,
            content_language: None,
            cache_control: None,
            etag: None,
            permissions: None,
            owner: None,
            group: None,
            checksum: None,
            custom: HashMap::new(),
            system: HashMap::new(),
        }
    }
}

impl Default for UnifiedStorageConfig {
    fn default() -> Self {
        Self {
            backend_type: UnifiedStorageType::default(),
            connection: ConnectionConfig {
                endpoint: None,
                timeout: std::time::Duration::from_secs(30),
                retry_attempts: 3,
                pool_size: Some(10),
            },
            performance: PerformanceConfig {
                buffer_size: 8192,
                concurrent_operations: 10,
                cache_enabled: true,
                compression_enabled: false,
            },
            security: SecurityConfig {
                encryption_enabled: false,
                authentication: None,
                permissions: None,
            },
            custom: HashMap::new(),
        }
    }
}

impl Default for UnifiedStorageMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            throughput_mbps: 0.0,
            storage_utilization: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

// ==================== UTILITY FUNCTIONS ====================

/// Create a default storage configuration for a given type
pub fn create_default_config(storage_type: UnifiedStorageType) -> UnifiedStorageConfig {
    UnifiedStorageConfig {
        backend_type: storage_type,
        ..Default::default()
    }
}

/// Validate storage configuration
pub fn validate_config(config: &UnifiedStorageConfig) -> Result<()> {
    // Basic validation
    if config.connection.timeout.is_zero() {
        return Err(NestGateError::validation_error(
            "connection_timeout",
            "Connection timeout cannot be zero",
            Some("0".to_string()),
        ));
    }

    if config.performance.buffer_size == 0 {
        return Err(NestGateError::validation_error(
            "buffer_size",
            "Buffer size cannot be zero",
            Some("0".to_string()),
        ));
    }

    if config.performance.concurrent_operations == 0 {
        return Err(NestGateError::validation_error(
            "max_concurrent_operations",
            "Concurrent operations cannot be zero",
            Some("0".to_string()),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_storage_type_default() {
        assert_eq!(
            UnifiedStorageType::default(),
            UnifiedStorageType::FileSystem
        );
    }

    #[test]
    fn test_unified_storage_config_validation() {
        let config = UnifiedStorageConfig::default();
        assert!(validate_config(&config).is_ok());

        let mut invalid_config = config.clone();
        invalid_config.connection.timeout = std::time::Duration::from_secs(0);
        assert!(validate_config(&invalid_config).is_err());
    }

    #[test]
    fn test_create_default_config() {
        let config = create_default_config(UnifiedStorageType::Memory);
        assert_eq!(config.backend_type, UnifiedStorageType::Memory);
        assert!(validate_config(&config).is_ok());
    }
}
