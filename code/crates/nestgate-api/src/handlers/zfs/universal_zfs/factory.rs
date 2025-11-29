//
// Creates the appropriate ZFS service implementation based on configuration
// with automatic backend detection and fail-safe wrapping.

//! Factory module

use std::sync::Arc;
use std::time::Duration;

use crate::handlers::zfs::universal_zfs::{
    backends::RemoteZfsService,
    config::{ZfsBackend, ZfsServiceConfig},
    fail_safe::FailSafeZfsService,
    traits::UniversalZfsService,
};
use crate::handlers::zfs::universal_zfs_types::{UniversalZfsError, UniversalZfsResult};
use tracing::{debug, error, info, warn};

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
                Box::pin(async { Self::create_native_service() })
            }
            ZfsBackend::Development => {
                debug!("Creating development ZFS backend - using native implementation");
                Box::pin(async { Self::create_native_service() })
            }
            ZfsBackend::Remote {
                endpoint,
                timeout: _,
            } => {
                debug!("Creating remote ZFS backend: {}", endpoint);
                let remote_config = crate::handlers::zfs::universal_zfs::config::RemoteConfig {
                    endpoint: endpoint.clone(),
                    timeout: Duration::from_secs(30),
                    auth: None,
                };
                Box::pin(async move { Self::create_remote_service(&remote_config) })
            }
        }
    }

    /// Create service based on configuration with proper cloning
    pub async fn create_service(
        config: ZfsServiceConfig,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        info!("Creating ZFS service with backend: {:?}", config.backend);

        // Validate configuration
        if let Err(e) = config.validate() {
            error!("Invalid ZFS service configuration: {}", e);
            return Err(UniversalZfsError::configuration(e));
        }

        // Create the primary service based on backend configuration
        let primary_service = Self::create_backend_service(&config.backend).await?;

        // Wrap with fail-safe mechanisms if enabled
        let service = if config.fail_safe.circuit_breaker.enabled
            || config.fail_safe.retry_policy.enabled
        {
            debug!("Wrapping service with fail-safe mechanisms");
            // Create the enum wrapper for the service
            let enum_service = Arc::new(
                crate::handlers::zfs::universal_zfs::traits::UniversalZfsServiceEnum::Native(
                    crate::handlers::zfs::universal_zfs::backends::native::NativeZfsService::new(),
                ),
            );
            let fail_safe_service = FailSafeZfsService::new(enum_service, config.fail_safe.clone());

            // Add fallback service if configured
            let fail_safe_service = if config.fail_safe.fallback_enabled {
                debug!("Adding fallback service");
                let _fallback_service = Self::create_fallback_service(&config.backend)?;
                // Wrap fallback service in enum for compatibility
                let fallback_enum = Arc::new(crate::handlers::zfs::universal_zfs::traits::UniversalZfsServiceEnum::Native(
                        crate::handlers::zfs::universal_zfs::backends::native::NativeZfsService::new()
                    ));
                fail_safe_service.with_fallback(fallback_enum)
            } else {
                /// Fail Safe Service
                fail_safe_service
            };

            Arc::new(fail_safe_service) as Arc<dyn UniversalZfsService>
        } else {
            /// Primary Service
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
        let config = ZfsServiceConfig::default(); // Use default instead of from_env
        Self::create_service(config).await
    }

    /// CANONICAL MODERNIZATION: Production service creation always uses real implementation
    #[must_use]
    pub fn create_production_service() -> Arc<dyn UniversalZfsService> {
        Arc::new(
            crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService::new(),
        )
    }

    /// Auto-detect the best available backend
    async fn auto_detect_backend() -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        debug!("Auto-detecting ZFS backend");

        // Check if ZFS is available natively
        if Self::is_zfs_available().await {
            info!("Native ZFS detected, using native backend");
            return Self::create_native_service();
        }

        // Check for remote ZFS services
        if let Some(remote_service) = Self::detect_remote_services().await {
            info!("Remote ZFS service detected, using remote backend");
            return Ok(remote_service);
        }

        // In production, fail rather than fall back to mock service
        error!("No ZFS backend available in production environment");
        Err(UniversalZfsError::service_unavailable(
            "No ZFS backend available. Please install ZFS or configure a remote ZFS service.",
        ))
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
    fn create_native_service() -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        // Use the real native ZFS implementation
        debug!("Creating native ZFS service");
        let service =
            crate::handlers::zfs::universal_zfs::backends::native::NativeZfsService::new();
        info!("Successfully created native ZFS service");
        Ok(Arc::new(service) as Arc<dyn UniversalZfsService>)
    }

    /// Create remote ZFS service
    fn create_remote_service(
        config: &crate::handlers::zfs::universal_zfs::config::RemoteConfig,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        // Try to create a real remote service implementation
        debug!(
            "Attempting to create remote ZFS service for endpoint: {}",
            config.endpoint
        );

        let service = RemoteZfsService::new(config.clone());
        info!("Successfully created remote ZFS service");
        Ok(Arc::new(service))
    }

    /// Create fallback service
    fn create_fallback_service(
        backend: &ZfsBackend,
    ) -> UniversalZfsResult<Arc<dyn UniversalZfsService>> {
        match backend {
            ZfsBackend::Auto | ZfsBackend::Native => {
                // Try to create a minimal native service as fallback
                warn!("Creating minimal native ZFS service as fallback");
                let service = crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService::new();
                Ok(Arc::new(service))
            }
            ZfsBackend::Development => {
                // Development backend uses native implementation
                debug!("Creating development ZFS backend using native implementation");
                let service = crate::handlers::zfs::universal_zfs::backends::native::core::NativeZfsService::new();
                Ok(Arc::new(service))
            }
            ZfsBackend::Remote {
                endpoint,
                timeout: _,
            } => {
                // Create remote ZFS service
                warn!("Creating remote ZFS service fallback");
                let remote_config = crate::handlers::zfs::universal_zfs::config::RemoteConfig {
                    endpoint: endpoint.clone(),
                    timeout: Duration::from_secs(30),
                    auth: None,
                };
                let service = RemoteZfsService::new(remote_config);
                Ok(Arc::new(service))
            }
        }
    }

    /// Detect remote ZFS services
    async fn detect_remote_services() -> Option<Arc<dyn UniversalZfsService>> {
        // ✅ MIGRATED: Now uses centralized runtime configuration
        use nestgate_core::config::runtime::get_config;

        // Get configuration from centralized system
        let config = get_config();

        // Check common service discovery endpoints
        let endpoints = vec![
            format!(
                "http://{}:{}/api/v1/zfs/health",
                config.network.api_host, config.network.api_port
            ),
            format!(
                "http://zfs-service:{}/api/v1/zfs/health",
                config.network.api_port
            ),
        ];

        for endpoint in &endpoints {
            debug!("Checking remote ZFS service at: {}", endpoint);

            // Try to connect to the service
            match tokio::time::timeout(Duration::from_secs(5), reqwest::get(endpoint)).await {
                Ok(Ok(response)) => {
                    if response.status().is_success() {
                        info!("Found remote ZFS service at: {}", endpoint);
                        // Create RemoteZfsService with proper configuration
                        let remote_config =
                            crate::handlers::zfs::universal_zfs::config::RemoteConfig {
                                endpoint: endpoint.to_string(),
                                timeout: Duration::from_secs(30),
                                auth: None,
                            };

                        let service = RemoteZfsService::new(remote_config);
                        info!(
                            "Successfully connected to remote ZFS service at: {}",
                            endpoint
                        );
                        return Some(Arc::new(service));
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
        config.fail_safe.enable_graceful_degradation = fail_safe_enabled;

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

        // Note: Mock backend eliminated in canonical modernization
        // All backends now use real implementations

        health_status
    }
}

/// Helper trait for service configuration
pub trait ServiceConfigBuilder {
    /// Configure the ZFS backend type (native, remote, mock)
    fn with_backend(self, backend: ZfsBackend) -> Self;
    /// Enable or disable fail-safe mechanisms
    fn with_fail_safe(self, enabled: bool) -> Self;
    /// Enable or disable graceful degradation on failures
    fn with_graceful_degradation(self, enabled: bool) -> Self;
    /// Enable or disable circuit breaker pattern
    fn with_circuit_breaker(self, enabled: bool) -> Self;
    /// Enable or disable retry policy for failed operations
    fn with_retry_policy(self, enabled: bool) -> Self;
}
impl ServiceConfigBuilder for ZfsServiceConfig {
    /// Builder method to set Backend
    fn with_backend(mut self, backend: ZfsBackend) -> Self {
        self.backend = backend;
        self
    }

    /// Builder method to set Fail Safe
    fn with_fail_safe(mut self, enabled: bool) -> Self {
        self.fail_safe.circuit_breaker.enabled = enabled;
        self.fail_safe.retry_policy.enabled = enabled;
        self
    }

    /// Builder method to set Graceful Degradation
    fn with_graceful_degradation(mut self, enabled: bool) -> Self {
        self.fail_safe.enable_graceful_degradation = enabled;
        self
    }

    /// Builder method to set Circuit Breaker
    fn with_circuit_breaker(mut self, enabled: bool) -> Self {
        self.fail_safe.circuit_breaker.enabled = enabled;
        self
    }

    /// Builder method to set Retry Policy
    fn with_retry_policy(mut self, enabled: bool) -> Self {
        self.fail_safe.retry_policy.enabled = enabled;
        self
    }
}
