//! Health Checks module

use std::collections::HashMap;
//
// Comprehensive health monitoring for all system components.

use crate::error::Result;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;

// Type alias to resolve clippy::type_complexity warning
type HealthProviderMap = Arc<RwLock<HashMap<String, Box<dyn HealthCheckProvider + Send + Sync>>>>;

/// Health status for individual components
#[derive(Debug, Clone, PartialEq)]
/// Status values for Health
pub enum HealthStatus {
    /// Component is healthy and operational
    Healthy,
    /// Component has warnings but is operational
    Warning,
    /// Component is unhealthy and may not be operational
    Unhealthy,
    /// Component status is unknown
    Unknown,
}
/// System-wide health information
#[derive(Debug, Clone)]
/// Systemhealth
pub struct SystemHealth {
    /// Overall system status
    pub overall_status: HealthStatus,
    /// Individual component health statuses
    pub components: HashMap<String, ComponentHealth>,
    /// Timestamp of health check
    pub timestamp: SystemTime,
    /// Overall health score (0.0 to 1.0)
    pub health_score: f64,
}
/// Health information for a specific component
#[derive(Debug, Clone)]
/// Componenthealth
pub struct ComponentHealth {
    /// Component health status
    pub status: HealthStatus,
    /// Human-readable status message
    pub message: String,
    /// Last successful check timestamp
    pub last_success: Option<SystemTime>,
    /// Last failure timestamp
    pub last_failure: Option<SystemTime>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
/// Health checker for system monitoring
pub struct HealthChecker {
    components: HealthProviderMap,
}
/// Trait for components that can provide health information
pub trait HealthCheckProvider {
    /// Perform health check for this component
    fn check_health(&self) -> Result<ComponentHealth>;
    /// Get component name
    fn component_name(&self) -> &str;
}

impl HealthChecker {
    /// Create a new health checker
    #[must_use]
    pub fn new() -> Self {
        Self {
            components: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a health check provider
    pub async fn register_provider(
        &self,
        name: String,
        provider: Box<dyn HealthCheckProvider + Send + Sync>,
    ) {
        self.components.write().await.insert(name, provider);
    }

    /// Run health checks for all registered components
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn run_health_checks(&self) -> Result<SystemHealth> {
        let mut component_health = HashMap::new();
        let mut healthy_count = 0;
        let mut total_count = 0;

        for (name, provider) in self.components.read().await.iter() {
            total_count += 1;

            match provider.check_health() {
                Ok(health) => {
                    if health.status == HealthStatus::Healthy {
                        healthy_count += 1;
                    }
                    component_health.insert(name.clone(), health);
                }
                Err(e) => {
                    tracing::warn!("Health check failed for component {}: {}", name, e);
                    component_health.insert(
                        name.clone(),
                        ComponentHealth {
                            status: HealthStatus::Unhealthy,
                            message: format!("Health check failed: {e}"),
                            last_success: None,
                            last_failure: Some(SystemTime::now()),
                            metadata: HashMap::new(),
                        },
                    );
                }
            }
        }

        // Calculate overall health
        let health_score = if total_count > 0 {
            f64::from(healthy_count) / f64::from(total_count)
        } else {
            1.0 // No components registered, assume healthy
        };

        let overall_status = if health_score >= 0.8 {
            HealthStatus::Healthy
        } else if health_score >= 0.5 {
            HealthStatus::Warning
        } else {
            HealthStatus::Unhealthy
        };

        Ok(SystemHealth {
            overall_status,
            components: component_health,
            timestamp: SystemTime::now(),
            health_score,
        })
    }

    /// Get system health status
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn get_system_health(&self) -> Result<SystemHealth> {
        self.run_health_checks().await
    }
}

impl Default for HealthChecker {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

/// Basic system health check provider
pub struct SystemHealthProvider;
impl HealthCheckProvider for SystemHealthProvider {
    /// Check Health
    fn check_health(&self) -> Result<ComponentHealth> {
        // Basic system checks
        let uptime_check = std::fs::read_to_string("/proc/uptime").is_ok();
        let memory_check = std::fs::read_to_string("/proc/meminfo").is_ok();

        let status = if uptime_check && memory_check {
            HealthStatus::Healthy
        } else {
            HealthStatus::Warning
        };

        Ok(ComponentHealth {
            status,
            message: "System basic checks completed".to_string(),
            last_success: Some(SystemTime::now()),
            last_failure: None,
            metadata: HashMap::new(),
        })
    }

    /// Component Name
    fn component_name(&self) -> &str {
        "system"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockHealthProvider {
        name: String,
        status: HealthStatus,
    }

    impl HealthCheckProvider for MockHealthProvider {
        /// Check Health
        fn check_health(&self) -> Result<ComponentHealth> {
            Ok(ComponentHealth {
                status: self.status.clone(),
                message: "Mock health check".to_string(),
                last_success: Some(SystemTime::now()),
                last_failure: None,
                metadata: HashMap::new(),
            })
        }

        /// Component Name
        fn component_name(&self) -> &str {
            &self.name
        }
    }

    #[tokio::test]
    async fn test_health_checker() -> crate::Result<()> {
        let checker = HealthChecker::new();

        // Add healthy component
        checker
            .register_provider(
                "test1".to_string(),
                Box::new(MockHealthProvider {
                    name: "test1".to_string(),
                    status: HealthStatus::Healthy,
                }),
            )
            .await;

        // Add unhealthy component
        checker
            .register_provider(
                "test2".to_string(),
                Box::new(MockHealthProvider {
                    name: "test2".to_string(),
                    status: HealthStatus::Unhealthy,
                }),
            )
            .await;

        let health = checker.get_system_health().await?;
        assert_eq!(health.overall_status, HealthStatus::Warning);
        assert_eq!(health.health_score, 0.5);
        assert_eq!(health.components.len(), 2);
        Ok(())
    }
}
