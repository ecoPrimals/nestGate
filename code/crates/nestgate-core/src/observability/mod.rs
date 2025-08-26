use crate::NestGateError;
use std::collections::HashMap;
//
// Comprehensive monitoring, metrics, and tracing infrastructure for NestGate.
// Provides unified observability across all system components.

pub mod health_checks;
pub mod metrics;
pub mod tracing_config;

// Re-export key observability components
pub use health_checks::{HealthChecker, HealthStatus, SystemHealth};
pub use metrics::{MetricsCollector, MetricsRegistry, PerformanceMetrics};
pub use tracing_config::{init_tracing, TracingConfig};

use crate::{Result, NestGateError};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Central observability coordinator
pub struct ObservabilityManager {
    metrics: Arc<MetricsRegistry>,
    health_checker: Arc<HealthChecker>,
    config: ObservabilityConfig,
}

/// Configuration for observability features
#[derive(Debug, Clone)]
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
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("🔍 Initializing observability systems");

        if self.config.tracing_enabled {
            let tracing_config = TracingConfig::default();
            init_tracing(tracing_config)?;
            tracing::info!("✅ Distributed tracing initialized");
        }

        if self.config.metrics_enabled {
            self.start_metrics_collection().await?;
            tracing::info!("✅ Metrics collection started");
        }

        if self.config.health_checks_enabled {
            self.start_health_monitoring().await?;
            tracing::info!("✅ Health monitoring started");
        }

        tracing::info!("🎯 Observability systems fully initialized");
        Ok(())
    }

    /// Start metrics collection background task
    async fn start_metrics_collection(&self) -> Result<()> {
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

        Ok(())
    }

    /// Start health monitoring background task
    async fn start_health_monitoring(&self) -> Result<()> {
        let health_checker = Arc::clone(&self.health_checker);
        let interval = self.config.health_check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                if let Err(e) = health_checker.run_health_checks().await {
                    tracing::warn!("Health check failed: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Get current system metrics
    pub async fn get_metrics(&self) -> Result<PerformanceMetrics> {
        self.metrics.get_current_metrics().await
    }

    /// Get system health status
    pub async fn get_health(&self) -> Result<SystemHealth> {
        self.health_checker.get_system_health().await
    }

    /// Record a custom metric
    pub async fn record_metric(
        &self,
        name: &str,
        value: f64,
        _tags: HashMap<String, String>,
    ) -> Result<()> {
        self.metrics.record_custom_metric(name, value).await
    }

    /// Get metrics history for analysis
    pub async fn get_metrics_history(&self, duration: Duration) -> Result<Vec<PerformanceMetrics>> {
        self.metrics.get_metrics_history(duration).await
    }
}

/// Global observability instance
static OBSERVABILITY: std::sync::OnceLock<Arc<ObservabilityManager>> = std::sync::OnceLock::new();

/// Initialize global observability
pub fn init_observability(config: ObservabilityConfig) -> Result<()> {
    let manager = Arc::new(ObservabilityManager::new(config));

    OBSERVABILITY.set(manager.clone()).map_err(|_| {
        NestGateError::configuration_error(
            "Observability already initialized".to_string(),
            Some("observability".to_string()),
        )
    })?;

    // Initialize in background
    let init_manager = Arc::clone(&manager);
    tokio::spawn(async move {
        if let Err(e) = init_manager.initialize().await {
            tracing::error!("Failed to initialize observability: {}", e);
        }
    });

    Ok(())
}

/// Get global observability manager
pub fn get_observability() -> Option<Arc<ObservabilityManager>> {
    OBSERVABILITY.get().cloned()
}

/// Record a metric using the global observability manager
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

/// Get current system health
pub async fn get_system_health() -> Result<SystemHealth> {
    if let Some(obs) = get_observability() {
        obs.get_health().await
    } else {
        Err(NestGateError::configuration_error(
            "Observability not initialized".to_string(),
            Some("health_check".to_string()),
        ))
    }
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
        assert!(manager.get_health().await.is_ok());
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
