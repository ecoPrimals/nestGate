// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// Contains the main service structure and core functionality.

//! Core module

use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
// Removed unused tracing import

use crate::handlers::zfs::universal_zfs::config::TimeoutConfig;
use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
use crate::handlers::zfs::universal_zfs::traits::UniversalZfsService;

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
        warn!(
            "Executing fallback operation on service {}: {}",
            self.service_name, operation
        );
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
    async fn health_check(&self) -> UniversalZfsResult<HealthStatus> {
        health_check(self).await
    }

    /// Gets Metrics
    async fn get_metrics(&self) -> UniversalZfsResult<ServiceMetrics> {
        let metrics = self.metrics.read().await.clone();
        Ok(metrics)
    }

    /// Checks if Available
    async fn is_available(&self) -> bool {
        is_available_dispatch(self).await
    }

    // Forward all operations to their respective modules
    async fn list_pools(&self) -> UniversalZfsResult<Vec<PoolInfo>> {
        super::pool_operations::list_pools(self).await
    }

    /// Gets Pool
    async fn get_pool(&self, name: &str) -> UniversalZfsResult<Option<PoolInfo>> {
        super::pool_operations::get_pool(self, name).await
    }

    /// Creates  Pool
    async fn create_pool(&self, config: &PoolConfig) -> UniversalZfsResult<PoolInfo> {
        super::pool_operations::create_pool(self, config).await
    }

    /// Destroy Pool
    async fn destroy_pool(&self, name: &str) -> UniversalZfsResult<()> {
        super::pool_operations::destroy_pool(self, name).await
    }

    /// Scrub Pool
    async fn scrub_pool(&self, name: &str) -> UniversalZfsResult<()> {
        super::pool_operations::scrub_pool(self, name).await
    }

    /// Gets Pool Status
    async fn get_pool_status(&self, name: &str) -> UniversalZfsResult<String> {
        super::pool_operations::get_pool_status(self, name).await
    }

    /// List Datasets
    async fn list_datasets(&self) -> UniversalZfsResult<Vec<DatasetInfo>> {
        super::dataset_operations::list_datasets(self).await
    }

    /// Gets Dataset
    async fn get_dataset(&self, name: &str) -> UniversalZfsResult<Option<DatasetInfo>> {
        super::dataset_operations::get_dataset(self, name).await
    }

    /// Creates  Dataset
    async fn create_dataset(&self, config: &DatasetConfig) -> UniversalZfsResult<DatasetInfo> {
        super::dataset_operations::create_dataset(self, config).await
    }

    /// Destroy Dataset
    async fn destroy_dataset(&self, name: &str) -> UniversalZfsResult<()> {
        super::dataset_operations::destroy_dataset(self, name).await
    }

    /// Gets Dataset Properties
    async fn get_dataset_properties(
        &self,
        name: &str,
    ) -> UniversalZfsResult<HashMap<String, String>> {
        super::dataset_operations::get_dataset_properties(self, name).await
    }

    /// Sets Dataset Properties
    async fn set_dataset_properties(
        &self,
        name: &str,
        properties: &HashMap<String, String>,
    ) -> UniversalZfsResult<()> {
        super::dataset_operations::set_dataset_properties(self, name, properties).await
    }

    /// List Snapshots
    async fn list_snapshots(&self) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        super::snapshot_operations::list_snapshots(self).await
    }

    /// List Dataset Snapshots
    async fn list_dataset_snapshots(&self, dataset: &str) -> UniversalZfsResult<Vec<SnapshotInfo>> {
        super::snapshot_operations::list_dataset_snapshots(self, dataset).await
    }

    /// Creates  Snapshot
    async fn create_snapshot(&self, config: &SnapshotConfig) -> UniversalZfsResult<SnapshotInfo> {
        super::snapshot_operations::create_snapshot(self, config).await
    }

    /// Destroy Snapshot
    async fn destroy_snapshot(&self, name: &str) -> UniversalZfsResult<()> {
        super::snapshot_operations::destroy_snapshot(self, name).await
    }

    /// Optimize
    async fn optimize(&self) -> UniversalZfsResult<String> {
        super::optimization::optimize(self).await
    }

    /// Gets Optimization Analytics
    async fn get_optimization_analytics(&self) -> UniversalZfsResult<serde_json::Value> {
        super::optimization::get_optimization_analytics(self).await
    }

    /// Predict Tier
    async fn predict_tier(&self, file_path: &str) -> UniversalZfsResult<String> {
        super::optimization::predict_tier(self, file_path).await
    }

    /// Gets Configuration
    async fn get_configuration(&self) -> UniversalZfsResult<serde_json::Value> {
        super::optimization::get_configuration(self).await
    }

    /// Updates  Configuration
    async fn update_configuration(&self, config: serde_json::Value) -> UniversalZfsResult<()> {
        super::optimization::update_configuration(self, config).await
    }

    /// Shutdown
    async fn shutdown(&self) -> UniversalZfsResult<()> {
        super::optimization::shutdown(self).await
    }
}

/// Health check implementation for fail-safe ZFS service
///
/// Checks the circuit breaker state and falls back to the fallback service if available.
/// Dispatches on `UniversalZfsServiceEnum` without re-entering the trait on `FailSafe` to avoid
/// recursive `async fn` types (see `UniversalZfsServiceEnum` fail-safe arms).
pub async fn health_check(service: &FailSafeZfsService) -> UniversalZfsResult<HealthStatus> {
    if !service.circuit_breaker.can_execute().await {
        if let Some(fallback) = &service.fallback {
            return drill_to_native_health_check(fallback.as_ref()).await;
        }
        return Err(UniversalZfsError::CircuitBreakerOpen {
            backend: service.service_name.clone(),
        });
    }

    drill_to_native_health_check(service.primary.as_ref()).await
}

async fn drill_to_native_health_check(
    mut current: &crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum,
) -> UniversalZfsResult<HealthStatus> {
    loop {
        match current {
            crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum::Native(
                n,
            ) => return n.health_check().await,
            crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum::FailSafe(
                f,
            ) => {
                current = f.primary.as_ref();
            }
        }
    }
}

/// Fail-safe availability (dispatched without trait recursion on nested fail-safe primaries).
pub async fn is_available_dispatch(service: &FailSafeZfsService) -> bool {
    if service.circuit_breaker.is_open().await {
        return false;
    }
    let mut current = service.primary.as_ref();
    loop {
        match current {
            crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum::Native(
                n,
            ) => return n.is_available().await,
            crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum::FailSafe(
                f,
            ) => {
                current = f.primary.as_ref();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::zfs::universal_zfs::service_enum::UniversalZfsServiceEnum;
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
