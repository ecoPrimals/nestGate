//! Canonical Unified Traits for `NestGate`
//! This module provides the unified trait system that consolidates all `NestGate` functionality.
//! Trait definitions and implementations.
//! These traits use `impl Future` returns which may trigger `clippy::type_complexity` warnings
//! but represent the modern async Rust patterns and are more efficient than boxed futures.

#![allow(clippy::type_complexity)]

use std::future::Future;

// **CANONICAL UNIFIED TRAIT SYSTEM**
// This is THE single source of truth for ALL traits across NestGate,
// replacing and consolidating 50+ scattered trait definitions.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

// Removed unused imports for pedantic perfection
use crate::unified_enums::service_types::UnifiedServiceType;

// ==================== THE CANONICAL SERVICE TRAIT ====================

/// **THE** canonical service trait that replaces ALL service traits
/// This is the single source of truth for all `NestGate` services
pub trait CanonicalService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Service health status type
    type Health: Clone + Send + Sync + 'static;

    /// Service metrics type
    type Metrics: Clone + Send + Sync + 'static;

    /// Service error type
    type Error: Send + Sync + std::error::Error + 'static;
    // ==================== CORE SERVICE OPERATIONS ====================

    /// Start the service - native async
    fn start(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Stop the service - native async
    fn stop(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Check service health - native async
    fn is_healthy(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Health, Self::Error>> + Send;

    /// Get service metrics - native async
    fn get_metrics(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Metrics, Self::Error>> + Send;

    /// Get service capabilities - native async
    fn capabilities(
        &self,
    ) -> impl Future<Output = std::result::Result<ServiceCapabilities, Self::Error>> + Send;

    /// Validate configuration - native async
    fn validate_config(
        &self,
        config: &Self::Config,
    ) -> impl Future<Output = std::result::Result<Vec<String>, Self::Error>> + Send;

    // ==================== ADDITIONAL SERVICE METHODS ====================

    /// Get service identifier - PEDANTIC ADDITION
    fn service_id(&self) -> &str {
        "unknown"
    }

    /// Get service type - PEDANTIC ADDITION
    fn service_type(&self) -> UnifiedServiceType {
        UnifiedServiceType::Generic
    }

    /// Initialize service with config - PEDANTIC ADDITION
    fn initialize(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            let _ = config; // Use config parameter
            Ok(())
        }
    }

    /// Health check method - PEDANTIC ADDITION
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Health, Self::Error>> + Send {
        async move {
            // PEDANTIC: Use is_healthy method instead of default()
            self.is_healthy().await
        }
    }

    /// Shutdown method - PEDANTIC ADDITION  
    fn shutdown(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default graceful shutdown
            Ok(())
        }
    }

    /// Restart method - PEDANTIC ADDITION
    fn restart(&self) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default restart implementation
            Ok(())
        }
    }

    /// Update configuration method - PEDANTIC ADDITION
    fn update_config(
        &self,
        _config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send {
        async move {
            // Default config update implementation
            Ok(())
        }
    }
}

// ==================== THE CANONICAL PROVIDER TRAIT ====================

/// **THE** canonical provider trait that replaces ALL provider traits
/// This is the single source of truth for all `NestGate` providers
pub trait CanonicalProvider<T>: Send + Sync + 'static {
    /// Provider configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Provider error type
    type Error: Send + Sync + std::error::Error + 'static;

    /// Provider metadata type
    type Metadata: Clone + Send + Sync + 'static;
    // ==================== CORE PROVIDER OPERATIONS ====================

    /// Provide service instance - native async
    fn provide(
        &self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<T, Self::Error>> + Send;

    /// Configure provider - native async
    fn configure(
        &mut self,
        config: Self::Config,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get provider metadata - native async
    fn metadata(
        &self,
    ) -> impl Future<Output = std::result::Result<Self::Metadata, Self::Error>> + Send;

    /// Health check - native async
    fn health_check(
        &self,
    ) -> impl Future<Output = std::result::Result<ProviderHealth, Self::Error>> + Send;

    /// Get provider capabilities - native async
    fn capabilities(
        &self,
    ) -> impl Future<Output = std::result::Result<ProviderCapabilities, Self::Error>> + Send;
}

// ==================== CANONICAL STORAGE TRAIT ====================

/// **THE** canonical storage trait that replaces ALL storage traits
///
/// This trait consolidates and replaces:
/// - `UnifiedStorageBackend` (from `unified_storage_traits.rs`)
/// - `CanonicalStorageBackend` (from `canonical_storage.rs`)  
/// - `StorageBackend` (from backends/mod.rs)
/// - `ZeroCostUnifiedStorageBackend` (from `zero_cost_unified_storage_traits.rs`)
/// - `EnterpriseStorageCapabilities` (from enterprise/traits.rs)
///
/// **PERFORMANCE**: Native async throughout - zero `async_trait` overhead
/// **COMPLETENESS**: Comprehensive interface covering all storage operations
/// **UNIFICATION**: Single source of truth for all storage implementations
pub trait CanonicalStorage: CanonicalService {
    /// Storage item type - can be bytes, structured data, or custom types
    type Item: Clone + Send + Sync + 'static;

    /// Storage key type - flexible key system (string, path, custom)
    type Key: Clone + Send + Sync + 'static;

    /// Storage metadata type - extensible metadata system
    type Metadata: Clone + Send + Sync + 'static;

    /// Backend-specific configuration type
    type BackendConfig: Clone + Send + Sync + 'static;

    // ==================== CORE STORAGE OPERATIONS ====================

    /// Read data from storage - native async
    fn read(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<Self::Item>, Self::Error>> + Send;

    /// Write data to storage - native async
    fn write(
        &self,
        key: Self::Key,
        item: Self::Item,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Delete data from storage - native async
    fn delete(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    /// List storage keys with optional prefix - native async
    fn list(
        &self,
        prefix: Option<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<Self::Key>, Self::Error>> + Send;

    /// Check if key exists - native async
    fn exists(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<bool, Self::Error>> + Send;

    // ==================== METADATA OPERATIONS ====================

    /// Get metadata for a storage item - native async
    fn get_metadata(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<Self::Metadata>, Self::Error>> + Send;

    /// Set metadata for a storage item - native async
    fn set_metadata(
        &self,
        key: Self::Key,
        metadata: Self::Metadata,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== BATCH OPERATIONS ====================

    /// Batch read multiple items - native async
    fn batch_read(
        &self,
        keys: Vec<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<(Self::Key, Option<Self::Item>)>, Self::Error>>
           + Send;

    /// Batch write multiple items - native async
    fn batch_write(
        &self,
        items: Vec<(Self::Key, Self::Item)>,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Batch delete multiple items - native async
    fn batch_delete(
        &self,
        keys: Vec<Self::Key>,
    ) -> impl Future<Output = std::result::Result<Vec<(Self::Key, bool)>, Self::Error>> + Send;

    // ==================== ADVANCED OPERATIONS ====================

    /// Copy item to new location - native async
    fn copy(
        &self,
        source: Self::Key,
        destination: Self::Key,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Move item to new location - native async  
    fn move_item(
        &self,
        source: Self::Key,
        destination: Self::Key,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get storage usage statistics - native async
    fn usage_stats(
        &self,
    ) -> impl Future<Output = std::result::Result<StorageUsageStats, Self::Error>> + Send;

    // ==================== BACKEND MANAGEMENT ====================

    /// Get backend type identifier
    fn backend_type(&self) -> StorageBackendType;

    /// Get backend capabilities
    fn capabilities(&self) -> Vec<StorageCapability>;

    /// Initialize backend with configuration - native async
    fn initialize_backend(
        &mut self,
        config: Self::BackendConfig,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Shutdown backend gracefully - native async
    fn shutdown_backend(
        &mut self,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    // ==================== OPTIONAL ADVANCED FEATURES ====================

    /// Create a snapshot (if supported) - native async
    fn create_snapshot(
        &self,
        name: &str,
    ) -> impl Future<Output = std::result::Result<Option<SnapshotInfo>, Self::Error>> + Send {
        async move {
            let _ = name;
            Ok(None) // Default: not supported
        }
    }

    /// Stream read large data (if supported) - native async  
    fn stream_read(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<DataStream>, Self::Error>> + Send {
        async move {
            let _ = key;
            Ok(None) // Default: not supported
        }
    }

    /// Stream write large data (if supported) - native async
    fn stream_write(
        &self,
        key: Self::Key,
    ) -> impl Future<Output = std::result::Result<Option<WriteStream>, Self::Error>> + Send {
        async move {
            let _ = key;
            Ok(None) // Default: not supported
        }
    }
}

// ==================== CANONICAL NETWORK TRAIT ====================

/// **THE** canonical network trait that replaces ALL network service traits
pub trait CanonicalNetwork: CanonicalService {
    /// Request type
    type Request: Clone + Send + Sync + 'static;

    /// Response type
    type Response: Clone + Send + Sync + 'static;
    // ==================== NETWORK OPERATIONS ====================

    /// Handle network request - native async
    fn handle_request(
        &self,
        request: Self::Request,
    ) -> impl Future<Output = std::result::Result<Self::Response, Self::Error>> + Send;

    /// Establish connection - native async
    fn connect(
        &self,
        endpoint: &str,
    ) -> impl Future<Output = std::result::Result<ConnectionHandle, Self::Error>> + Send;

    /// Close connection - native async
    fn disconnect(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;

    /// Get connection status - native async
    fn connection_status(
        &self,
        handle: ConnectionHandle,
    ) -> impl Future<Output = std::result::Result<ConnectionStatus, Self::Error>> + Send;

    /// List active connections - native async
    fn list_connections(
        &self,
    ) -> impl Future<Output = std::result::Result<Vec<ConnectionHandle>, Self::Error>> + Send;
}

// ==================== CANONICAL SECURITY TRAIT ====================

/// **THE** canonical security trait that replaces ALL security service traits
pub trait CanonicalSecurity: CanonicalService {
    /// Authentication token type
    type Token: Clone + Send + Sync + 'static;

    /// User identity type
    type Identity: Clone + Send + Sync + 'static;
    // ==================== SECURITY OPERATIONS ====================

    /// Authenticate user - native async
    fn authenticate(
        &self,
        credentials: SecurityCredentials,
    ) -> impl Future<Output = std::result::Result<Self::Token, Self::Error>> + Send;

    /// Authorize operation - native async
    /// Validate token - native async
    fn validate_token(
        &self,
        token: Self::Token,
    ) -> impl Future<Output = std::result::Result<Self::Identity, Self::Error>> + Send;

    /// Revoke token - native async
    fn revoke_token(
        &self,
        token: Self::Token,
    ) -> impl Future<Output = std::result::Result<(), Self::Error>> + Send;
}

// ==================== ADDITIONAL CANONICAL TRAITS ====================

/// MCP protocol trait
pub trait CanonicalMcp: CanonicalService {}
/// Automation trait
pub trait CanonicalAutomation: CanonicalService {}
/// Zero-cost service marker
pub trait ZeroCostService: CanonicalService {}
/// Service factory
pub trait CanonicalServiceFactory<T: CanonicalService> {
    fn create_service(
        &self,
        config: T::Config,
    ) -> impl Future<Output = std::result::Result<T, crate::NestGateError>> + Send;
}
/// Provider factory
pub trait CanonicalProviderFactory<T, P: CanonicalProvider<T>> {
    fn create_provider(
        &self,
        config: P::Config,
    ) -> impl Future<Output = std::result::Result<P, crate::NestGateError>> + Send;
}
// ==================== SUPPORTING TYPES ====================

/// Service capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize)] // PEDANTIC: Added Default derive
pub struct ServiceCapabilities {
    pub can_scale: bool,
    pub can_migrate: bool,
    pub can_backup: bool,
    pub supported_protocols: Vec<String>,
}
/// Provider health status
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Default derive
pub struct ProviderHealth {
    pub is_healthy: bool,
    pub last_check: SystemTime,
    pub health: String,
}
impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            is_healthy: false,
            last_check: SystemTime::now(),
            health: String::new(),
        }
    }
}

/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ProviderCapabilities {
    pub supported_types: Vec<UnifiedServiceType>,
    pub max_instances: Option<u32>,
}
/// Storage usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageUsageStats {
    /// Total storage capacity in bytes
    pub total_capacity: u64,
    /// Used storage in bytes
    pub used_space: u64,
    /// Available storage in bytes
    pub available_space: u64,
    /// Number of stored items
    pub item_count: u64,
    /// Last updated timestamp
    pub last_updated: std::time::SystemTime,
}
/// Connection handle
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConnectionHandle(pub u64);
/// Connection status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Active,
    Idle,
    Closed,
    Error(String),
}
/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}
/// Security credentials
#[derive(Debug, Clone)]
pub struct SecurityCredentials {
    pub username: String,
    pub password: String,
}
/// Cron schedule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CronSchedule {
    pub expression: String,
}
/// Schedule ID
#[derive(Debug, Clone, Serialize, Deserialize)] // PEDANTIC: Added Serialize/Deserialize derives
pub struct ScheduleId {
    pub id: String,
}
/// Schedule info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleInfo {
    pub id: ScheduleId,
    pub schedule: CronSchedule,
    pub next_run: Option<SystemTime>,
}

// ==================== CANONICAL STORAGE TYPES ====================

/// Storage backend type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageBackendType {
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

/// Storage capability enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StorageCapability {
    /// Basic CRUD operations
    BasicOperations,
    /// Batch operations support
    BatchOperations,
    /// Metadata operations
    Metadata,
    /// Streaming data support
    Streaming,
    /// Snapshot support
    Snapshots,
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
    /// Custom capability
    Custom(String),
}

/// Snapshot information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotInfo {
    /// Snapshot identifier
    pub id: String,
    /// Snapshot name
    pub name: String,
    /// Creation timestamp
    pub created_at: std::time::SystemTime,
    /// Snapshot size in bytes
    pub size: u64,
}

/// Data stream for reading large objects
pub struct DataStream {
    // Implementation would contain actual stream
    _phantom: std::marker::PhantomData<()>,
}

/// Write stream for writing large objects
pub struct WriteStream {
    // Implementation would contain actual stream
    _phantom: std::marker::PhantomData<()>,
}
