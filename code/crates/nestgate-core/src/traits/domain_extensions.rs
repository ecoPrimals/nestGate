// **DOMAIN-SPECIFIC TRAIT EXTENSIONS**
//! Domain-specific extensions and traits for the `NestGate` system.
// This module provides domain-specific extensions to the canonical trait system.

use std::collections::HashMap;
use std::future::Future;

// Use canonical trait system
use crate::traits::canonical::CanonicalService;

use crate::Result;

/// ZFS service extension trait
pub trait ZfsServiceExtension: CanonicalService {
    /// ZFS-specific configuration type
    type ZfsConfig: Clone + Send + Sync + 'static;

    /// ZFS pool information type
    type PoolInfo: Clone + Send + Sync + 'static;

    /// ZFS dataset information type
    type DatasetInfo: Clone + Send + Sync + 'static;
    /// List ZFS pools
    fn list_pools(&self) -> impl Future<Output = Result<Vec<Self::PoolInfo>>> + Send;

    /// Create ZFS dataset
    fn create_dataset(
        &self,
        pool: &str,
        dataset: &str,
        config: Self::ZfsConfig,
    ) -> impl Future<Output = Result<Self::DatasetInfo>> + Send;

    /// Delete ZFS dataset
    fn delete_dataset(&self, pool: &str, dataset: &str) -> impl Future<Output = Result<()>> + Send;

    /// Get ZFS pool status
    fn pool_status(&self, pool: &str) -> impl Future<Output = Result<Self::PoolInfo>> + Send;

    /// Scrub ZFS pool
    fn scrub_pool(&self, pool: &str) -> impl Future<Output = Result<()>> + Send;

    /// Create ZFS snapshot
    fn create_snapshot(
        &self,
        dataset: &str,
        snapshot: &str,
    ) -> impl Future<Output = Result<()>> + Send;

    /// List ZFS snapshots
    fn list_snapshots(&self, dataset: &str) -> impl Future<Output = Result<Vec<String>>> + Send;

    /// Rollback to snapshot
    fn rollback_snapshot(
        &self,
        dataset: &str,
        snapshot: &str,
    ) -> impl Future<Output = Result<()>> + Send;
}

/// Storage service extension trait
pub trait StorageServiceExtension: CanonicalService {
    /// Storage item type
    type StorageItem: Clone + Send + Sync + 'static;

    /// Storage metadata type
    type StorageMetadata: Clone + Send + Sync + 'static;
    /// List storage items
    fn list_items(&self, path: &str)
        -> impl Future<Output = Result<Vec<Self::StorageItem>>> + Send;

    /// Get storage item metadata
    fn get_metadata(
        &self,
        path: &str,
    ) -> impl Future<Output = Result<Self::StorageMetadata>> + Send;

    /// Move storage item
    fn move_item(&self, from: &str, to: &str) -> impl Future<Output = Result<()>> + Send;

    /// Copy storage item
    fn copy_item(&self, from: &str, to: &str) -> impl Future<Output = Result<()>> + Send;

    /// Get storage usage
    fn get_usage(&self) -> impl Future<Output = Result<u64>> + Send;

    /// Get available space
    fn get_available_space(&self) -> impl Future<Output = Result<u64>> + Send;
}

/// `EcoPrimal` service extension trait
pub trait EcoPrimalExtension: CanonicalService {
    /// Primal metadata type
    type PrimalMetadata: Clone + Send + Sync + 'static;
    /// Register with ecosystem
    fn register_with_ecosystem(&self) -> impl Future<Output = Result<()>> + Send;

    /// Discover other primals
    fn discover_primals(&self) -> impl Future<Output = Result<Vec<String>>> + Send;

    /// Get primal capabilities
    fn get_primal_capabilities(
        &self,
    ) -> impl Future<Output = Result<HashMap<String, String>>> + Send;

    /// Connect to primal
    fn connect_to_primal(&self, primal_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Disconnect from primal
    fn disconnect_from_primal(&self, primal_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Send message to primal
    fn send_to_primal(
        &self,
        primal_id: &str,
        message: &str,
    ) -> impl Future<Output = Result<String>> + Send;

    /// Get primal metadata
    fn get_primal_metadata(
        &self,
        primal_id: &str,
    ) -> impl Future<Output = Result<Self::PrimalMetadata>> + Send;
}

/// Registry service extension trait
pub trait RegistryServiceExtension: CanonicalService {
    /// Registry entry type
    type RegistryEntry: Clone + Send + Sync + 'static;
    /// Register service
    fn register_service(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl Future<Output = Result<()>> + Send;

    /// Unregister service
    fn unregister_service(&self, service_id: &str) -> impl Future<Output = Result<()>> + Send;

    /// Discover services
    fn discover_services(
        &self,
        service_type: Option<&str>,
    ) -> impl Future<Output = Result<Vec<Self::RegistryEntry>>> + Send;

    /// Get service metadata
    fn get_service_metadata(
        &self,
        service_id: &str,
    ) -> impl Future<Output = Result<HashMap<String, String>>> + Send;

    /// Update service metadata
    fn update_service_metadata(
        &self,
        service_id: &str,
        metadata: HashMap<String, String>,
    ) -> impl Future<Output = Result<()>> + Send;
}

/// Network service extension trait
pub trait NetworkServiceExtension: CanonicalService {
    /// Network connection type
    type Connection: Clone + Send + Sync + 'static;

    /// Network endpoint type
    type Endpoint: Clone + Send + Sync + 'static;
    /// Establish connection
    fn connect(
        &self,
        endpoint: Self::Endpoint,
    ) -> impl Future<Output = Result<Self::Connection>> + Send;

    /// Close connection
    fn disconnect(&self, connection: Self::Connection) -> impl Future<Output = Result<()>> + Send;

    /// Send data
    fn send(
        &self,
        connection: &Self::Connection,
        data: &[u8],
    ) -> impl Future<Output = Result<()>> + Send;

    /// Receive data
    fn receive(
        &self,
        connection: &Self::Connection,
    ) -> impl Future<Output = Result<Vec<u8>>> + Send;

    /// List active connections
    fn list_connections(&self) -> impl Future<Output = Result<Vec<Self::Connection>>> + Send;

    /// Get connection status
    fn connection_status(
        &self,
        connection: &Self::Connection,
    ) -> impl Future<Output = Result<String>> + Send;
}

/// Generic service extension with type parameter
pub fn extend_service<T>(service: T) -> Result<T>
where
    T: CanonicalService,
{
    // Extension logic would go here
    Ok(service)
}
