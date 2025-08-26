// **CANONICAL ZFS SERVICE TRAIT - COMPREHENSIVE UNIFICATION**
//
// This trait provides the complete canonical interface for all ZFS backend implementations.
// It unifies all methods from native, remote, and fail-safe backends into a single consistent API.

// REMOVED: async_trait - using zero-cost native async patterns
use std::collections::HashMap;

use super::types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

/// **CANONICAL UNIVERSAL ZFS SERVICE TRAIT**
///
/// This trait defines the complete interface that all ZFS backend implementations must provide.
/// It includes all methods from native_real, remote, and fail_safe implementations.
/// **CANONICAL MODERNIZATION**: Zero-cost native async patterns
pub trait UniversalZfsService: Send + Sync {
    // ==================== CORE SERVICE METHODS ====================

    /// Get the service name
    fn service_name(&self) -> &str;

    /// Get the service version  
    fn service_version(&self) -> &str;

    /// Perform a health check on the service
    fn health_check(&self) -> impl std::future::Future<Output = UniversalZfsResult<HealthStatus>> + Send;

    /// Get service metrics
    fn get_metrics(&self) -> impl std::future::Future<Output = UniversalZfsResult<ServiceMetrics>> + Send;

    /// Check if the service is available
    fn is_available(&self) -> impl std::future::Future<Output = bool> + Send;

    /// Shutdown the service gracefully
    fn shutdown(&self) -> impl std::future::Future<Output = UniversalZfsResult<()>> + Send;

    // ==================== POOL OPERATIONS ====================

    /// List all pools
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>>;

    /// Create a new pool
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo>;

    /// Get information about a specific pool
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>>;

    /// Destroy a pool
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()>;

    /// Scrub a pool (data integrity check)
    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()>;

    /// Get pool status information
    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String>;

    // ==================== DATASET OPERATIONS ====================

    /// List all datasets
    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>>;

    /// Create a new dataset
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo>;

    /// Get information about a specific dataset
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>>;

    /// Destroy a dataset
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()>;

    /// Set dataset properties
    async fn set_dataset_properties(
        &self,
        dataset_name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()>;

    /// Get dataset properties
    async fn get_dataset_properties(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>>;

    // ==================== SNAPSHOT OPERATIONS ====================

    /// List all snapshots
    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>>;

    /// Create a snapshot
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo>;

    /// List snapshots for a specific dataset
    async fn list_dataset_snapshots(
        &self,
        dataset_name: &str,
    ) -> UniversalZfsResult<Vec<SnapshotInfo>>;

    /// Destroy a snapshot
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()>;

    // ==================== OPTIMIZATION & CONFIGURATION ====================

    /// Optimize ZFS configuration
    async fn optimize(&self) -> UniversalZfsResult<String>;

    /// Get optimization analytics
    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value>;

    /// Predict optimal tier for data
    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String>;

    /// Get current configuration
    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value>;

    /// Update configuration
    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()>;
}
