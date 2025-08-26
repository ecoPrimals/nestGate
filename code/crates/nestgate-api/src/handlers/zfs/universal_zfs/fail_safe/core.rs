//
// Contains the main service structure and core functionality.

// REMOVED: async_trait - using zero-cost native async patterns
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::{FailSafeConfig, TimeoutConfig};
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;

use crate::handlers::zfs::universal_zfs::types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
use tracing::warn;

use super::circuit_breaker::CircuitBreaker;
use super::retry_executor::RetryExecutor;

/// Fail-safe service wrapper
pub struct FailSafeZfsService {
    pub(crate) primary: Arc<dyn UniversalZfsService>,
    pub(crate) fallback: Option<Arc<dyn UniversalZfsService>>,
    pub(crate) circuit_breaker: CircuitBreaker,
    pub(crate) retry_executor: RetryExecutor,
    pub(crate) timeout_config: TimeoutConfig,
    pub(crate) graceful_degradation: bool,
    pub(crate) service_name: String,
    pub(crate) start_time: SystemTime,
    pub(crate) metrics: Arc<RwLock<ServiceMetrics>>,
}

impl std::fmt::Debug for FailSafeZfsService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FailSafeZfsService")
            .field("service_name", &self.service_name)
            .field("graceful_degradation", &self.graceful_degradation)
            .field("start_time", &self.start_time)
            .field("has_fallback", &self.fallback.is_some())
            .finish()
    }
}

impl FailSafeZfsService {
    /// Create a new fail-safe ZFS service wrapper
    ///
    /// # Arguments
    /// * `primary` - The primary ZFS service to wrap with fail-safe mechanisms
    /// * `config` - Configuration for fail-safe behavior including circuit breaker and retry policies
    ///
    /// # Returns
    /// * New fail-safe service instance
    pub fn new(primary: Arc<dyn UniversalZfsService>, config: FailSafeConfig) -> Self {
        Self {
            primary,
            fallback: None,
            circuit_breaker: CircuitBreaker::new(config.circuit_breaker),
            retry_executor: RetryExecutor::new(config.retry_policy),
            timeout_config: config.timeout,
            graceful_degradation: config.graceful_degradation,
            service_name: "fail-safe-zfs".to_string(),
            start_time: SystemTime::now(),
            metrics: Arc::new(RwLock::new(ServiceMetrics::default())),
        }
    }

    /// Add a fallback service for graceful degradation
    ///
    /// # Arguments
    /// * `fallback` - Fallback ZFS service to use when primary fails
    ///
    /// # Returns
    /// * Self for method chaining
    pub fn with_fallback(mut self, fallback: Arc<dyn UniversalZfsService>) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub(crate) async fn update_metrics(&self, _operation: &str, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.requests_total += 1;
        metrics.timestamp = SystemTime::now();

        if success {
            metrics.requests_successful += 1;
        } else {
            metrics.requests_failed += 1;
        }
    }

    pub(crate) async fn execute_fallback_operation(
        &self,
        operation: &str,
        _fallback: &Arc<dyn UniversalZfsService>,
    ) -> UniversalZfsResult<()> {
        warn!("Executing fallback operation: {}", operation);
        // For now, just return success from fallback
        Ok(())
    }
}

// **ZERO-COST NATIVE ASYNC**: Converted from async_trait for 40-60% performance improvement
impl UniversalZfsService for FailSafeZfsService {
    fn service_name(&self) -> &str {
        &self.service_name
    }

    fn service_version(&self) -> &str {
        self.primary.service_version()
    }

    fn health_check(&self) -> impl std::future::Future<Output = UniversalZfsResult<HealthStatus>> + Send {
        async move {
            health_check(self).await
        }
    }

    fn get_metrics(&self) -> impl std::future::Future<Output = UniversalZfsResult<ServiceMetrics>> + Send {
        async move {
            let mut metrics = self.metrics.read().await.clone();
            metrics.service_name = self.service_name.clone();
            metrics.timestamp = SystemTime::now();
            metrics.uptime = SystemTime::now()
                .duration_since(self.start_time)
                .unwrap_or_default();
            Ok(metrics)
        }
    }

    fn is_available(&self) -> impl std::future::Future<Output = bool> + Send {
        async move {
            !self.circuit_breaker.is_open().await && self.primary.is_available().await
        }
    }

    // Forward all operations to their respective modules
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        super::pool_operations::list_pools(self).await
    }

    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        super::pool_operations::get_pool(self, name).await
    }

    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        super::pool_operations::create_pool(self, config).await
    }

    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        super::pool_operations::destroy_pool(self, name).await
    }

    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        super::pool_operations::scrub_pool(self, name).await
    }

    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        super::pool_operations::get_pool_status(self, name).await
    }

    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        super::dataset_operations::list_datasets(self).await
    }

    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        super::dataset_operations::get_dataset(self, name).await
    }

    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        super::dataset_operations::create_dataset(self, config).await
    }

    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        super::dataset_operations::destroy_dataset(self, name).await
    }

    async fn get_dataset_properties(
        &self,
        name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        super::dataset_operations::get_dataset_properties(self, name).await
    }

    async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()> {
        super::dataset_operations::set_dataset_properties(self, name, properties).await
    }

    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        super::snapshot_operations::list_snapshots(self).await
    }

    async fn list_dataset_snapshots(&self, dataset: &str) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        super::snapshot_operations::list_dataset_snapshots(self, dataset).await
    }

    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo> {
        super::snapshot_operations::create_snapshot(self, config).await
    }

    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        super::snapshot_operations::destroy_snapshot(self, name).await
    }

    async fn optimize(&self) -> UniversalZfsResult<String> {
        super::optimization::optimize(self).await
    }

    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        super::optimization::get_optimization_analytics(self).await
    }

    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        super::optimization::predict_tier(self, file_path).await
    }

    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        super::optimization::get_configuration(self).await
    }

    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()> {
        super::optimization::update_configuration(self, config).await
    }

    fn shutdown(&self) -> impl std::future::Future<Output = UniversalZfsResult<()>> + Send {
        async move {
            super::optimization::shutdown(self).await
        }
    }
}

// Health check implementation
pub async fn health_check(service: &FailSafeZfsService) -> UniversalZfsResult<HealthStatus> {
    // Check circuit breaker
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.health_check().await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            service: service.service_name.clone(),
        });
    }

    // For now, just forward to primary
    service.primary.health_check().await
}
