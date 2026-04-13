// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Observability module

use nestgate_types::error::NestGateError;
use std::collections::HashMap;
//
// Comprehensive monitoring, metrics, and tracing infrastructure for NestGate.
// Provides unified observability across all system components.

/// Health check infrastructure for monitoring system and component health status.
pub mod health_checks;
/// Metrics collection and aggregation for performance monitoring and analysis.
pub mod metrics;
/// Distributed tracing configuration for request flow and performance analysis.
pub mod tracing_config;

#[cfg(test)]
mod health_checks_tests;
#[cfg(test)]
mod observability_comprehensive_tests;

// Re-export key observability components
pub use health_checks::{HealthChecker, HealthStatus, SystemHealth};
pub use metrics::{MetricsCollector, MetricsRegistry, PerformanceMetrics};
pub use tracing_config::{TracingConfig, init_tracing};

use nestgate_types::Result;
use std::sync::Arc;
use std::time::Duration;

/// Central observability coordinator that manages metrics, health checks, and tracing.
///
/// Provides a unified interface for all observability concerns including:
/// - Metrics collection and aggregation
/// - Health status monitoring
/// - Distributed tracing coordination
/// - System-wide observability configuration
pub struct ObservabilityManager {
    metrics: Arc<MetricsRegistry>,
    health_checker: Arc<HealthChecker>,
    config: ObservabilityConfig,
}
// Configuration for observability features
#[derive(Debug, Clone)]
/// Configuration for Observability
pub struct ObservabilityConfig {
    /// Enable metrics collection
    pub metrics_enabled: bool,
    /// Enable health checks
    pub health_checks_enabled: bool,
    /// Enable distributed tracing
    pub tracing_enabled: bool,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Maximum metrics history to keep
    pub max_metrics_history: usize,
}
impl Default for ObservabilityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            metrics_enabled: true,
            health_checks_enabled: true,
            tracing_enabled: true,
            metrics_interval: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(60),
            max_metrics_history: 1000,
        }
    }
}

impl ObservabilityManager {
    /// Create a new observability manager
    #[must_use]
    pub fn new(config: ObservabilityConfig) -> Self {
        let metrics = Arc::new(MetricsRegistry::new());
        let health_checker = Arc::new(HealthChecker::new());

        Self {
            metrics,
            health_checker,
            config,
        }
    }

    /// Initialize observability systems
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing observability systems");

        if self.config.tracing_enabled {
            let tracing_config = TracingConfig::default();
            init_tracing(&tracing_config)?;
            tracing::info!("Distributed tracing initialized");
        }

        if self.config.metrics_enabled {
            self.start_metrics_collection();
            tracing::info!("Metrics collection started");
        }

        if self.config.health_checks_enabled {
            self.start_health_monitoring();
            tracing::info!("Health monitoring started");
        }

        tracing::info!("Observability systems fully initialized");
        Ok(())
    }

    /// Start metrics collection background task
    fn start_metrics_collection(&self) {
        let metrics = Arc::clone(&self.metrics);
        let interval = self.config.metrics_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = metrics.collect_system_metrics().await {
                    tracing::warn!("Failed to collect system metrics: {}", e);
                }
            }
        });
    }

    /// Start health monitoring background task
    fn start_health_monitoring(&self) {
        let health_checker = Arc::clone(&self.health_checker);
        let interval = self.config.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = health_checker.run_health_checks() {
                    tracing::warn!("Health check failed: {}", e);
                }
            }
        });
    }

    /// Get current system metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_metrics(&self) -> Result<PerformanceMetrics> {
        self.metrics.get_current_metrics().await
    }

    /// Get system health status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_health(&self) -> Result<SystemHealth> {
        self.health_checker.get_system_health()
    }

    /// Record a custom metric
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn record_metric(
        &self,
        name: &str,
        value: f64,
        _tags: HashMap<String, String>,
    ) -> Result<()> {
        self.metrics.record_custom_metric(name, value).await
    }

    /// Get metrics history for analysis
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_metrics_history(&self, duration: Duration) -> Result<Vec<PerformanceMetrics>> {
        self.metrics.get_metrics_history(duration).await
    }
}

// Global observability instance
static OBSERVABILITY: std::sync::OnceLock<Arc<ObservabilityManager>> = std::sync::OnceLock::new();

/// Initialize global observability system with the provided configuration.
///
/// This function sets up the global observability manager and starts background
/// metrics collection and health checking. Should be called once at application startup.
///
/// # Errors
///
/// Returns an error if observability has already been initialized.
pub fn init_observability(config: ObservabilityConfig) -> Result<()> {
    let manager = Arc::new(ObservabilityManager::new(config));
    OBSERVABILITY.set(manager.clone()).map_err(|_| {
        NestGateError::configuration_error("observability", "Observability already initialized")
    })?;

    // Initialize in background
    let init_manager = Arc::clone(&manager);
    tokio::spawn(async move {
        if let Err(e) = init_manager.initialize() {
            tracing::error!("Failed to initialize observability: {}", e);
        }
    });

    Ok(())
}

/// Get the global observability manager instance.
///
/// Returns `None` if observability has not been initialized via `init_observability()`.
pub fn get_observability() -> Option<Arc<ObservabilityManager>> {
    OBSERVABILITY.get().cloned()
}

/// Record a metric using the global observability manager.
///
/// If observability is not initialized, logs a warning and returns Ok.
/// This allows graceful degradation when observability is not configured.
///
/// # Errors
///
/// Returns an error if the metric recording fails (e.g., invalid metric name or value).
pub async fn record_metric(name: &str, value: f64) -> Result<()> {
    if let Some(obs) = get_observability() {
        obs.record_metric(name, value, HashMap::new()).await
    } else {
        tracing::warn!(
            "Observability not initialized, metric '{}' not recorded",
            name
        );
        Ok(())
    }
}

/// Get current system health status from all monitored components.
///
/// Returns a comprehensive health report including status for all registered
/// health checkers. If observability is not initialized, returns a default
/// unhealthy status.
///
/// # Errors
///
/// Returns an error if health status cannot be retrieved from the system.
pub fn get_system_health() -> Result<SystemHealth> {
    get_observability().map_or_else(
        || {
            Err(NestGateError::configuration_error(
                "health_check",
                "Observability not initialized",
            ))
        },
        |obs| obs.get_health(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_observability_manager_creation() {
        let config = ObservabilityConfig::default();
        let manager = ObservabilityManager::new(config);

        // Should be able to get metrics and health
        assert!(manager.get_metrics().await.is_ok());
        assert!(manager.get_health().is_ok());
    }

    #[tokio::test]
    async fn test_global_observability() {
        let config = ObservabilityConfig::default();

        // Initialize global observability
        assert!(init_observability(config).is_ok());

        // Should be able to get the global instance
        assert!(get_observability().is_some());

        // Should be able to record metrics
        assert!(record_metric("test_metric", 42.0).await.is_ok());
    }
}
