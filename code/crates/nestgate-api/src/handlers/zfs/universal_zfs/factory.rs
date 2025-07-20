//! ZFS Service Factory
//!
//! Creates the appropriate ZFS service implementation based on configuration
//! with automatic backend detection and fail-safe wrapping.

use std::sync::Arc;
use tracing::{debug, error, info, warn};

use crate::handlers::zfs::universal_zfs::{
    backends::MockZfsService,
    config::{ZfsBackend, ZfsServiceConfig},
    fail_safe::FailSafeZfsService,
    traits::UniversalZfsService,
    types::UniversalZfsResult,
};

// Type alias to reduce complexity
type ZfsServiceFuture = std::pin::Pin<
    Box<dyn std::future::Future<Output = UniversalZfsResult<Arc<dyn UniversalZfsService>>> + Send>,
>;

/// Factory for creating ZFS services
pub struct ZfsServiceFactory;

impl ZfsServiceFactory {
    /// Create backend service based on configuration
    fn create_backend_service(backend: &ZfsBackend) -> ZfsServiceFuture {
        match backend {
            ZfsBackend::Auto => {
                debug!("Auto-detecting best ZFS backend");
                Box::pin(Self::auto_detect_backend())
            }
            ZfsBackend::Native => {
                debug!("Creating native ZFS backend");
                Box::pin(Self::create_native_service())
            }
            ZfsBackend::Mock => {
                debug!("Creating mock ZFS backend");
                Box::pin(async {
                    Ok(Arc::new(MockZfsService::new()) as Arc<dyn UniversalZfsService>)
                })
            }
            ZfsBackend::Remote(config) => {
                debug!("Creating remote ZFS backend: {}", config.endpoint);
                let config = config.clone();
                Box::pin(async move { Self::create_remote_service(&config).await })
            }
            ZfsBackend::LoadBalanced(backends) => {
                debug!(
                    "Creating load-balanced ZFS backend with {} backends",
                    backends.len()
                );
                let backends = backends.clone();
                Box::pin(async move { Self::create_load_balanced_service(&backends).await })
            }
            ZfsBackend::Failover { primary, fallback } => {
                debug!("Creating failover ZFS backend");
                let primary = primary.clone();
                let fallback = fallback.clone();
                Box::pin(async move { Self::create_failover_service(&primary, &fallback).await })
            }
        }
    }

    /// Create load-balanced service
    fn create_load_balanced_service(backends: &[ZfsBackend]) -> ZfsServiceFuture {
        if backends.is_empty() {
            return Box::pin(async {
                Err(
                    crate::handlers::zfs::universal_zfs::types::UniversalZfsError::configuration(
                        "Load balanced backend must have at least one backend".to_string(),
                    ),
                )
            });
        }

        // For now, just use the first backend
        warn!("Load balancing not yet implemented, using first backend");
        let backend = backends[0].clone();
        Box::pin(async move { Self::create_backend_service(&backend).await })
    }

    /// Create failover service
    fn create_failover_service(primary: &ZfsBackend, _fallback: &ZfsBackend) -> ZfsServiceFuture {
        // For now, just use the primary backend
        warn!("Failover not yet implemented, using primary backend");
        let primary = primary.clone();
        Box::pin(async move { Self::create_backend_service(&primary).await })
    }

    /// Create service based on configuration with proper cloning
    pub async fn create_service(
        config: ZfsServiceConfig,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        info!("Creating ZFS service with backend: {:?}", config.backend);

        // Validate configuration
        if let Err(e) = config.validate() {
            error!("Invalid ZFS service configuration: {}", e);
            return Err(
                crate::handlers::zfs::universal_zfs::types::UniversalZfsError::configuration(e),
            );
        }

        // Create the primary service based on backend configuration
        let primary_service = Self::create_backend_service(&config.backend).await?;

        // Wrap with fail-safe mechanisms if enabled
        let service =
            if config.fail_safe.circuit_breaker.enabled || config.fail_safe.retry_policy.enabled {
                debug!("Wrapping service with fail-safe mechanisms");
                let fail_safe_service =
                    FailSafeZfsService::new(primary_service, config.fail_safe.clone());

                // Add fallback service if configured
                let fail_safe_service = if config.fail_safe.fallback_enabled {
                    debug!("Adding fallback service");
                    let fallback_service = Self::create_fallback_service(&config.backend).await?;
                    fail_safe_service.with_fallback(fallback_service)
                } else {
                    fail_safe_service
                };

                Arc::new(fail_safe_service) as Arc<dyn UniversalZfsService>
            } else {
                primary_service
            };

        info!(
            "Successfully created ZFS service: {}",
            service.service_name()
        );
        Ok(service)
    }

    /// Create a service with automatic backend detection
    pub async fn create_auto_service() -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        let config = ZfsServiceConfig::from_env();
        Self::create_service(config).await
    }

    /// Create a mock service for testing
    pub fn create_mock_service() -> Arc<dyn UniversalZfsService> {
        Arc::new(MockZfsService::new())
    }

    /// Create a mock service with specific failures
    pub fn create_mock_service_with_failures(
        operations: Vec<String>,
    ) -> Arc<dyn UniversalZfsService> {
        Arc::new(MockZfsService::with_failures(operations))
    }

    /// Create a mock service with delays
    pub fn create_mock_service_with_delays() -> Arc<dyn UniversalZfsService> {
        Arc::new(MockZfsService::with_config(
            "mock-zfs-delayed",
            "1.0.0",
            true,
        ))
    }

    /// Auto-detect the best available backend
    async fn auto_detect_backend() -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        debug!("Auto-detecting ZFS backend");

        // Check if ZFS is available natively
        if Self::is_zfs_available().await {
            info!("Native ZFS detected, using native backend");
            return Self::create_native_service().await;
        }

        // Check for remote ZFS services
        if let Some(remote_service) = Self::detect_remote_services().await {
            info!("Remote ZFS service detected, using remote backend");
            return Ok(remote_service);
        }

        // Fall back to mock service
        warn!("No ZFS backend detected, falling back to mock service");
        Ok(Arc::new(MockZfsService::new()))
    }

    /// Check if native ZFS is available
    async fn is_zfs_available() -> bool {
        // Check if ZFS commands are available
        match tokio::process::Command::new("zfs")
            .arg("version")
            .output()
            .await
        {
            Ok(output) => {
                if output.status.success() {
                    debug!("ZFS version command successful");
                    true
                } else {
                    debug!("ZFS version command failed");
                    false
                }
            }
            Err(e) => {
                debug!("ZFS command not found: {}", e);
                false
            }
        }
    }

    /// Create native ZFS service
    async fn create_native_service() -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        // For now, return mock service as native implementation is not ready
        warn!("Native ZFS service not yet implemented, using mock service");
        Ok(Arc::new(MockZfsService::new()))
    }

    /// Create remote ZFS service
    async fn create_remote_service(
        _config: &crate::handlers::zfs::universal_zfs::config::RemoteConfig,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        // For now, return mock service as remote implementation is not ready
        warn!("Remote ZFS service not yet implemented, using mock service");
        Ok(Arc::new(MockZfsService::new()))
    }

    /// Create fallback service
    async fn create_fallback_service(
        backend: &ZfsBackend,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        match backend {
            ZfsBackend::Auto | ZfsBackend::Native => {
                // Use mock service as fallback for native backends
                Ok(Arc::new(MockZfsService::new()))
            }
            ZfsBackend::Mock => {
                // Use a different mock service as fallback
                Ok(Arc::new(MockZfsService::with_config(
                    "fallback-mock",
                    "1.0.0",
                    false,
                )))
            }
            ZfsBackend::Remote(_) => {
                // Use mock service as fallback for remote backends
                Ok(Arc::new(MockZfsService::new()))
            }
            ZfsBackend::LoadBalanced(_) | ZfsBackend::Failover { .. } => {
                // Use simple mock service as fallback for complex backends
                Ok(Arc::new(MockZfsService::new()))
            }
        }
    }

    /// Detect remote ZFS services
    async fn detect_remote_services() -> Option<Arc<dyn UniversalZfsService>> {
        // Check common service discovery endpoints
        let endpoints = vec![
            "http://localhost:8080/api/v1/zfs/health",
            "http://zfs-service:8080/api/v1/zfs/health",
        ];

        for endpoint in endpoints {
            debug!("Checking remote ZFS service at: {}", endpoint);

            // Try to connect to the service
            match tokio::time::timeout(std::time::Duration::from_secs(5), reqwest::get(endpoint))
                .await
            {
                Ok(Ok(response)) => {
                    if response.status().is_success() {
                        info!("Found remote ZFS service at: {}", endpoint);
                        // Create remote service proxy - using mock service for now until RemoteZfsService is fully implemented
                        return Some(Arc::new(MockZfsService::with_config(
                            "remote-proxy",
                            "1.0.0",
                            true,
                        )));
                    }
                }
                Ok(Err(e)) => {
                    debug!("Failed to connect to {}: {}", endpoint, e);
                }
                Err(_) => {
                    debug!("Timeout connecting to {}", endpoint);
                }
            }
        }

        None
    }

    /// Create service for testing with specific configuration
    pub async fn create_test_service(
        backend: ZfsBackend,
        fail_safe_enabled: bool,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        let mut config = ZfsServiceConfig {
            backend,
            ..Default::default()
        };
        config.fail_safe.circuit_breaker.enabled = fail_safe_enabled;
        config.fail_safe.retry_policy.enabled = fail_safe_enabled;
        config.fail_safe.graceful_degradation = fail_safe_enabled;

        Self::create_service(config).await
    }

    /// Get service health for all available backends
    pub async fn get_backend_health() -> Vec<(String, bool)> {
        let mut health_status = Vec::new();

        // Check native ZFS
        let native_available = Self::is_zfs_available().await;
        health_status.push(("native".to_string(), native_available));

        // Check remote services
        let remote_available = Self::detect_remote_services().await.is_some();
        health_status.push(("remote".to_string(), remote_available));

        // Mock is always available
        health_status.push(("mock".to_string(), true));

        health_status
    }
}

/// Helper trait for service configuration
pub trait ServiceConfigBuilder {
    fn with_backend(self, backend: ZfsBackend) -> Self;
    fn with_fail_safe(self, enabled: bool) -> Self;
    fn with_graceful_degradation(self, enabled: bool) -> Self;
    fn with_circuit_breaker(self, enabled: bool) -> Self;
    fn with_retry_policy(self, enabled: bool) -> Self;
}

impl ServiceConfigBuilder for ZfsServiceConfig {
    fn with_backend(mut self, backend: ZfsBackend) -> Self {
        self.backend = backend;
        self
    }

    fn with_fail_safe(mut self, enabled: bool) -> Self {
        self.fail_safe.circuit_breaker.enabled = enabled;
        self.fail_safe.retry_policy.enabled = enabled;
        self
    }

    fn with_graceful_degradation(mut self, enabled: bool) -> Self {
        self.fail_safe.graceful_degradation = enabled;
        self
    }

    fn with_circuit_breaker(mut self, enabled: bool) -> Self {
        self.fail_safe.circuit_breaker.enabled = enabled;
        self
    }

    fn with_retry_policy(mut self, enabled: bool) -> Self {
        self.fail_safe.retry_policy.enabled = enabled;
        self
    }
}
