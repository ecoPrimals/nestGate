//! Universal ZFS Service Traits
//!
//! Defines the core abstraction for ZFS operations that can be implemented
//! by multiple backends (native, mock, remote) with consistent interfaces.

use async_trait::async_trait;
use std::collections::HashMap;
use std::time::Duration;

use super::types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsResult,
};

/// Universal ZFS Service abstraction
///
/// This trait defines all ZFS operations in a backend-agnostic way.
/// Implementations can use real ZFS commands, mock responses, or remote services.
#[async_trait]
pub trait UniversalZfsService: Send + Sync {
    /// Service identification
    fn service_name(&self) -> &str;
    fn service_version(&self) -> &str;

    /// Health and status operations
    async fn health_check(&self) -> UniversalZfsResult<HealthStatus>;
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics>;
    async fn is_available(&self) -> bool;

    /// Pool operations
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>>;
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>>;
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo>;
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()>;
    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()>;
    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String>;

    /// Dataset operations
    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>>;
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>>;
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo>;
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()>;
    async fn get_dataset_properties(
        &self,
        name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>>;
    async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()>;

    /// Snapshot operations
    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>>;
    async fn list_dataset_snapshots(&self, dataset: &str) -> UniversalZfsResult<Vec<SnapshotInfo>>;
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo>;
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()>;

    /// Advanced operations
    async fn optimize(&self) -> UniversalZfsResult<String>;
    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value>;
    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String>;

    /// Configuration and control
    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value>;
    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()>;
    async fn shutdown(&self) -> UniversalZfsResult<()>;
}

/// Health monitoring trait for services
#[async_trait]
pub trait HealthMonitor: Send + Sync {
    async fn check_health(&self) -> UniversalZfsResult<HealthStatus>;
    async fn get_last_health_check(&self) -> Option<HealthStatus>;
    async fn start_monitoring(&self, interval: Duration) -> UniversalZfsResult<()>;
    async fn stop_monitoring(&self) -> UniversalZfsResult<()>;
}

/// Metrics collection trait
#[async_trait]
pub trait MetricsCollector: Send + Sync {
    async fn collect_metrics(&self) -> UniversalZfsResult<ServiceMetrics>;
    async fn reset_metrics(&self) -> UniversalZfsResult<()>;
    async fn get_historical_metrics(
        &self,
        duration: Duration,
    ) -> UniversalZfsResult<Vec<ServiceMetrics>>;
}

/// Configuration management trait
#[async_trait]
pub trait ConfigurationManager: Send + Sync {
    async fn get_config(&self) -> UniversalZfsResult<serde_json::Value>;
    async fn update_config(&self, config: serde_json::Value) -> UniversalZfsResult<()>;
    async fn validate_config(&self, config: &serde_json::Value) -> UniversalZfsResult<bool>;
    async fn reload_config(&self) -> UniversalZfsResult<()>;
}

/// Service discovery trait for remote services
#[async_trait]
pub trait ServiceDiscovery: Send + Sync {
    async fn discover_services(&self) -> UniversalZfsResult<Vec<String>>;
    async fn register_service(&self, endpoint: &str) -> UniversalZfsResult<()>;
    async fn unregister_service(&self, endpoint: &str) -> UniversalZfsResult<()>;
    async fn get_healthy_services(&self) -> UniversalZfsResult<Vec<String>>;
}
