// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

use std::future::Future;

/// **THE** base trait for all services in the NestGate ecosystem
///
/// This trait provides common functionality that all services must implement:
/// - Lifecycle management (start, stop, health checks)
/// - Configuration management
/// - Metrics and observability
///
/// **Native Async**: All methods use `impl Future` for zero-cost abstractions
///
/// # Examples
///
/// ```rust,ignore
/// use nestgate_core::traits::canonical_hierarchy::CanonicalService;
///
/// pub struct MyService {
///     config: MyConfig,
/// }
///
/// impl CanonicalService for MyService {
///     type Config = MyConfig;
///     type Health = MyHealth;
///     type Metrics = MyMetrics;
///     type Error = MyError;
///
///     async fn start(&mut self) -> Result<(), Self::Error> {
///         // Initialize service
///         Ok(())
///     }
///
///     async fn stop(&mut self) -> Result<(), Self::Error> {
///         // Cleanup
///         Ok(())
///     }
///
///     async fn health(&self) -> Result<Self::Health, Self::Error> {
///         // Health check
///         Ok(MyHealth::default())
///     }
///
///     fn config(&self) -> &Self::Config {
///         &self.config
///     }
///
///     async fn metrics(&self) -> Result<Self::Metrics, Self::Error> {
///         // Collect metrics
///         Ok(MyMetrics::default())
///     }
///
///     fn name(&self) -> &str {
///         "my-service"
///     }
///
///     fn version(&self) -> &str {
///         env!("CARGO_PKG_VERSION")
///     }
/// }
/// ```
pub trait CanonicalService: Send + Sync + 'static {
    /// Service configuration type
    type Config: Clone + Send + Sync + 'static;

    /// Health status type
    type Health: Clone + Send + Sync + 'static;

    /// Metrics type
    type Metrics: Clone + Send + Sync + 'static;

    /// Error type
    type Error: Send + Sync + std::error::Error + 'static;

    // ==================== LIFECYCLE ====================

    /// Start the service
    fn start(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Stop the service gracefully
    fn stop(&mut self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Check service health
    fn health(&self) -> impl Future<Output = Result<Self::Health, Self::Error>> + Send;

    // ==================== CONFIGURATION ====================

    /// Get current configuration
    fn config(&self) -> &Self::Config;

    /// Update configuration (if supported)
    ///
    /// Default implementation is not provided - must be overridden if dynamic
    /// configuration updates are supported. If not supported, don't implement.
    fn update_config(
        &mut self,
        _config: Self::Config,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send
    where
        Self: Sized,
        Self::Error: From<crate::NestGateError>,
    {
        async move {
            // Default: Not supported
            // Implementations should override this method to support dynamic configuration
            Err(crate::NestGateError::not_implemented(
                "update_config not implemented - this service does not support dynamic configuration updates",
            )
            .into())
        }
    }

    // ==================== OBSERVABILITY ====================

    /// Get service metrics
    fn metrics(&self) -> impl Future<Output = Result<Self::Metrics, Self::Error>> + Send;

    /// Get service name
    fn name(&self) -> &str;

    /// Get service version
    fn version(&self) -> &str;
}
