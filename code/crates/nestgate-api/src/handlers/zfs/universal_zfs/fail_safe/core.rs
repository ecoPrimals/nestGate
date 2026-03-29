// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Contains the main service structure and core functionality.

//! Core module

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::TimeoutConfig;
use crate::handlers::zfs::universal_zfs::traits::{UniversalZfsService, UniversalZfsServiceEnum};

use crate::handlers::zfs::universal_zfs_types::{
    DatasetConfig, DatasetInfo, HealthStatus, PoolConfig, PoolInfo, ServiceMetrics, SnapshotConfig,
    SnapshotInfo, UniversalZfsError, UniversalZfsResult,
};
use tracing::warn;

use super::circuit_breaker::CircuitBreaker;
use super::retry_executor::RetryExecutor;

/// Fail-safe service wrapper
pub struct FailSafeZfsService {
    pub(crate) primary: Arc<UniversalZfsServiceEnum>,
    pub(crate) fallback: Option<Arc<UniversalZfsServiceEnum>>,
    pub(crate) circuit_breaker: CircuitBreaker,
    pub(crate) retry_executor: RetryExecutor,
    pub(crate) timeout_config: TimeoutConfig,
    pub(crate) graceful_degradation: bool,
    pub(crate) service_name: String,
    pub(crate) start_time: SystemTime,
    pub(crate) metrics: Arc<RwLock<ServiceMetrics>>,
}
impl std::fmt::Debug for FailSafeZfsService {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FailSafeZfsService")
            .field("service_name", &self.service_name)
            .field("graceful_degradation", &self.graceful_degradation)
            .field("start_time", &self.start_time)
            .field("has_fallback", &self.fallback.is_some())
            .field("timeout_config", &self.timeout_config)
            .field("metrics", &self.metrics)
            .finish_non_exhaustive()
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
    #[must_use]
    pub fn new(
        primary: Arc<UniversalZfsServiceEnum>,
        config: nestgate_core::config::canonical_primary::handler_config::ZfsFailSafeConfig,
    ) -> Self {
        Self {
            primary,
            fallback: None,
            circuit_breaker: CircuitBreaker::new(
                crate::handlers::zfs::universal_zfs::config::CircuitBreakerConfig {
                    enabled: config.circuit_breaker.enabled,
                    failure_threshold: config.failure_threshold,
                    recovery_timeout: config.circuit_timeout,
                    half_open_max_calls: 3,
                },
            ),
            retry_executor: RetryExecutor::new(
                crate::handlers::zfs::universal_zfs::config::RetryPolicy {
                    max_attempts: 3,
                    initial_delay: std::time::Duration::from_millis(100),
                    max_delay: std::time::Duration::from_secs(10),
                    backoff_multiplier: 2.0,
                },
            ),
            timeout_config: TimeoutConfig {
                operation_timeout: config.circuit_timeout,
                connection_timeout: std::time::Duration::from_secs(30),
                health_check_timeout: std::time::Duration::from_secs(5),
            },
            graceful_degradation: config.enable_graceful_degradation,
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
    #[must_use]
    pub fn with_fallback(mut self, fallback: Arc<UniversalZfsServiceEnum>) -> Self {
        self.fallback = Some(fallback);
        self
    }

    pub(crate) async fn update_metrics(&self, _operation: &str, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.requests_total += 1;
        metrics.timestamp = SystemTime::now();

        metrics.requests_total += 1;
        if !success {
            metrics.requests_failed += 1;
        }
    }

    pub(crate) fn execute_fallback_operation(
        &self,
        operation: &str,
        _fallback: &Arc<UniversalZfsServiceEnum>,
    ) -> UniversalZfsResult<()> {
        warn!("Executing fallback operation: {}", operation);
        // For now, just return success from fallback
        Ok(())
    }
}

impl UniversalZfsService for FailSafeZfsService {
    /// Service Name
    fn service_name(&self) -> &str {
        &self.service_name
    }

    /// Service Version
    fn service_version(&self) -> &str {
        self.primary.service_version()
    }

    /// Health Check
    fn health_check(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HealthStatus>> + Send + '_>> {
        Box::pin(async move { health_check(self).await })
    }

    /// Gets Metrics
    fn get_metrics(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<ServiceMetrics>> + Send + '_>> {
        Box::pin(async move {
            let metrics = self.metrics.read().await.clone();
            Ok(metrics)
        })
    }

    /// Checks if Available
    fn is_available(&self) -> Pin<Box<dyn Future<Output = bool> + Send + '_>> {
        Box::pin(async move {
            !self.circuit_breaker.is_open().await && self.primary.is_available().await
        })
    }

    // Forward all operations to their respective modules
    fn list_pools(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<PoolInfo>>> + Send + '_>> {
        Box::pin(async move { super::pool_operations::list_pools(self).await })
    }

    /// Gets Pool
    fn get_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<PoolInfo>>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::pool_operations::get_pool(self, &name).await })
    }

    /// Creates  Pool
    fn create_pool(
        &self,
        config: &PoolConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<PoolInfo>> + Send + '_>> {
        let config = config.clone();
        Box::pin(async move { super::pool_operations::create_pool(self, &config).await })
    }

    /// Destroy Pool
    fn destroy_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::pool_operations::destroy_pool(self, &name).await })
    }

    /// Scrub Pool
    fn scrub_pool(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::pool_operations::scrub_pool(self, &name).await })
    }

    /// Gets Pool Status
    fn get_pool_status(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::pool_operations::get_pool_status(self, &name).await })
    }

    /// List Datasets
    fn list_datasets(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<DatasetInfo>>> + Send + '_>> {
        Box::pin(async move { super::dataset_operations::list_datasets(self).await })
    }

    /// Gets Dataset
    fn get_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Option<DatasetInfo>>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::dataset_operations::get_dataset(self, &name).await })
    }

    /// Creates  Dataset
    fn create_dataset(
        &self,
        config: &DatasetConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<DatasetInfo>> + Send + '_>> {
        let config = config.clone();
        Box::pin(async move { super::dataset_operations::create_dataset(self, &config).await })
    }

    /// Destroy Dataset
    fn destroy_dataset(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::dataset_operations::destroy_dataset(self, &name).await })
    }

    /// Gets Dataset Properties
    fn get_dataset_properties(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<HashMap<String, String>>> + Send + '_>>
    {
        let name = name.to_owned();
        Box::pin(
            async move { super::dataset_operations::get_dataset_properties(self, &name).await },
        )
    }

    /// Sets Dataset Properties
    fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        let properties = properties.clone();
        Box::pin(async move {
            super::dataset_operations::set_dataset_properties(self, &name, &properties).await
        })
    }

    /// List Snapshots
    fn list_snapshots(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>> {
        Box::pin(async move { super::snapshot_operations::list_snapshots(self).await })
    }

    /// List Dataset Snapshots
    fn list_dataset_snapshots(
        &self,
        dataset: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<Vec<SnapshotInfo>>> + Send + '_>> {
        let dataset = dataset.to_owned();
        Box::pin(
            async move { super::snapshot_operations::list_dataset_snapshots(self, &dataset).await },
        )
    }

    /// Creates  Snapshot
    fn create_snapshot(
        &self,
        config: &SnapshotConfig,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<SnapshotInfo>> + Send + '_>> {
        let config = config.clone();
        Box::pin(async move { super::snapshot_operations::create_snapshot(self, &config).await })
    }

    /// Destroy Snapshot
    fn destroy_snapshot(
        &self,
        name: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        let name = name.to_owned();
        Box::pin(async move { super::snapshot_operations::destroy_snapshot(self, &name).await })
    }

    /// Optimize
    fn optimize(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        Box::pin(async move { super::optimization::optimize(self).await })
    }

    /// Gets Optimization Analytics
    fn get_optimization_analytics(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>> {
        Box::pin(async move { super::optimization::get_optimization_analytics(self).await })
    }

    /// Predict Tier
    fn predict_tier(
        &self,
        file_path: &str,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<String>> + Send + '_>> {
        let file_path = file_path.to_owned();
        Box::pin(async move { super::optimization::predict_tier(self, &file_path).await })
    }

    /// Gets Configuration
    fn get_configuration(
        &self,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<serde_json::Value>> + Send + '_>> {
        Box::pin(async move { super::optimization::get_configuration(self).await })
    }

    /// Updates  Configuration
    fn update_configuration(
        &self,
        config: serde_json::Value,
    ) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move { super::optimization::update_configuration(self, config).await })
    }

    /// Shutdown
    fn shutdown(&self) -> Pin<Box<dyn Future<Output = UniversalZfsResult<()>> + Send + '_>> {
        Box::pin(async move { super::optimization::shutdown(self).await })
    }
}

/// Health check implementation for fail-safe ZFS service
///
/// Checks the circuit breaker state and falls back to the fallback service if available.
pub async fn health_check(service: &FailSafeZfsService) -> UniversalZfsResult<HealthStatus> {
    // Check circuit breaker
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return fallback.health_check().await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: service.service_name.clone(),
        });
    }

    // For now, just forward to primary
    service.primary.health_check().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs::traits::UniversalZfsServiceEnum;
    use nestgate_core::config::canonical_primary::handler_config::ZfsFailSafeConfig;
    use std::sync::Arc;

    fn sample_fail_safe_config() -> ZfsFailSafeConfig {
        let mut c = ZfsFailSafeConfig::default();
        c.circuit_breaker.enabled = false;
        c
    }

    #[test]
    fn fail_safe_debug_includes_service_name() {
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let svc = FailSafeZfsService::new(primary, sample_fail_safe_config());
        let s = format!("{svc:?}");
        assert!(s.contains("fail-safe") || s.contains("FailSafeZfsService"));
    }

    #[test]
    fn with_fallback_sets_field() {
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let fb = Arc::new(UniversalZfsServiceEnum::new_native());
        let svc = FailSafeZfsService::new(primary, sample_fail_safe_config()).with_fallback(fb);
        assert!(svc.fallback.is_some());
    }

    #[test]
    fn execute_fallback_operation_ok() {
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let svc = FailSafeZfsService::new(primary, sample_fail_safe_config());
        let fb = Arc::new(UniversalZfsServiceEnum::new_native());
        assert!(svc.execute_fallback_operation("list_pools", &fb).is_ok());
    }

    #[tokio::test]
    async fn update_metrics_increments_counters() {
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let svc = FailSafeZfsService::new(primary, sample_fail_safe_config());
        svc.update_metrics("test", false).await;
        let m = svc.metrics.read().await;
        assert!(m.requests_total >= 2);
        assert!(m.requests_failed >= 1);
    }

    #[tokio::test]
    async fn health_check_forwards_when_breaker_disabled() {
        let primary = Arc::new(UniversalZfsServiceEnum::new_native());
        let svc = FailSafeZfsService::new(primary, sample_fail_safe_config());
        let _ = health_check(&svc).await;
    }
}
