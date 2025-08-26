use nestgate_core::zero_cost::ZeroCostServiceHealth;
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

/// Zero-cost orchestration health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostOrchestrationHealth {
    /// Overall service health
    pub status: ZeroCostServiceHealth,
    /// Health check timestamp
    pub timestamp: SystemTime,
    /// Health check duration
    pub check_duration: Duration,
    /// Additional health metrics
    pub metrics: HealthMetrics,
    /// Health check errors
    pub errors: Vec<String>,
}

/// Health metrics for detailed monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Response time in milliseconds
    pub response_time_ms: f64,
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Active connections count
    pub active_connections: u32,
    /// Connection pool health
    pub connection_pool_health: f64,
}

impl Default for ZeroCostOrchestrationHealth {
    fn default() -> Self {
        Self {
            status: ZeroCostServiceHealth::Unknown {
                reason: "Service health not yet determined".to_string(),
            },
            timestamp: SystemTime::now(),
            check_duration: Duration::from_millis(0),
            metrics: HealthMetrics::default(),
            errors: Vec::new(),
        }
    }
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            response_time_ms: 0.0,
            cpu_usage: 0.0,
            memory_usage: 0,
            active_connections: 0,
            connection_pool_health: 1.0,
        }
    }
}

/// Health checker for orchestration client
pub struct HealthChecker {
    /// Health check configuration
    config: HealthCheckConfig,
    /// Last health check result
    last_health: Option<ZeroCostOrchestrationHealth>,
}

/// Health check configuration
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
    /// Healthy threshold
    pub healthy_threshold: u32,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            unhealthy_threshold: 3,
            healthy_threshold: 2,
        }
    }
}

impl HealthChecker {
    /// Create new health checker
    pub fn new(config: HealthCheckConfig) -> Self {
        Self {
            config,
            last_health: None,
        }
    }

    /// Perform health check
    pub async fn perform_health_check(
        &mut self,
        base_url: &str,
    ) -> Result<ZeroCostOrchestrationHealth> {
        let start_time = SystemTime::now();

        let mut health = ZeroCostOrchestrationHealth {
            timestamp: start_time,
            ..Default::default()
        };

        // Perform HTTP health check
        let client = reqwest::Client::builder()
            .timeout(self.config.timeout)
            .build()
            .map_err(|e| {
                NestGateError::network_error(
                    &format!("Failed to create health check client: {}", e),
                    "health_check",
                    Some(base_url),
                )
            })?;

        let health_url = format!("{}/health", base_url);

        match client.get(&health_url).send().await {
            Ok(response) => {
                let response_time = start_time.elapsed().unwrap_or_default();
                health.check_duration = response_time;
                health.metrics.response_time_ms = response_time.as_millis() as f64;

                if response.status().is_success() {
                    health.status = ZeroCostServiceHealth::Healthy {
                        last_check: SystemTime::now(),
                        metrics: None,
                    };
                } else {
                    health.status = ZeroCostServiceHealth::Degraded {
                        reason: "Service experiencing issues".to_string(),
                        severity: 5,
                        last_check: SystemTime::now(),
                    };
                    health
                        .errors
                        .push(format!("HTTP status: {}", response.status()));
                }
            }
            Err(e) => {
                health.status = ZeroCostServiceHealth::Unhealthy {
                    error: "Service health check failed".to_string(),
                    last_check: SystemTime::now(),
                    recovery_hint: Some("Check service configuration".to_string()),
                };
                health.errors.push(format!("Health check failed: {}", e));
                health.check_duration = start_time.elapsed().unwrap_or_default();
            }
        }

        // Collect additional metrics
        self.collect_system_metrics(&mut health.metrics).await;

        self.last_health = Some(health.clone());
        Ok(health)
    }

    /// Get last health check result
    pub fn last_health(&self) -> Option<&ZeroCostOrchestrationHealth> {
        self.last_health.as_ref()
    }

    /// Check if service is healthy
    pub fn is_healthy(&self) -> bool {
        self.last_health
            .as_ref()
            .map(|h| matches!(h.status, ZeroCostServiceHealth::Healthy { .. }))
            .unwrap_or(false)
    }

    /// Get health status
    pub fn health_status(&self) -> ZeroCostServiceHealth {
        self.last_health
            .as_ref()
            .map(|h| h.status.clone())
            .unwrap_or(ZeroCostServiceHealth::Unknown {
                reason: "No health data available".to_string(),
            })
    }

    /// Collect system metrics
    async fn collect_system_metrics(&self, metrics: &mut HealthMetrics) {
        // Collect basic system metrics
        // In a real implementation, this would use system monitoring libraries

        // Simulate CPU usage
        metrics.cpu_usage = 15.5;

        // Simulate memory usage (64 MB)
        metrics.memory_usage = 64 * 1024 * 1024;

        // Simulate active connections
        metrics.active_connections = 42;

        // Connection pool health (0.0 to 1.0)
        metrics.connection_pool_health = 0.95;
    }
}

impl ZeroCostOrchestrationHealth {
    /// Check if the health status indicates a healthy service
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, ZeroCostServiceHealth::Healthy { .. })
    }

    /// Check if the health status indicates a degraded service
    pub fn is_degraded(&self) -> bool {
        matches!(self.status, ZeroCostServiceHealth::Degraded { .. })
    }

    /// Check if the health status indicates an unhealthy service
    pub fn is_unhealthy(&self) -> bool {
        matches!(self.status, ZeroCostServiceHealth::Unhealthy { .. })
    }

    /// Get health status as string
    pub fn status_string(&self) -> &'static str {
        match &self.status {
            ZeroCostServiceHealth::Healthy { .. } => "healthy",
            ZeroCostServiceHealth::Degraded { .. } => "degraded",
            ZeroCostServiceHealth::Unhealthy { .. } => "unhealthy",
            ZeroCostServiceHealth::Unknown { .. } => "unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_check() {
        let health = ZeroCostOrchestrationHealth {
            status: ZeroCostServiceHealth::Healthy,
            ..Default::default()
        };

        assert!(health.is_healthy());
        assert!(!health.is_degraded());
        assert!(!health.is_unhealthy());
        assert_eq!(health.status_string(), "healthy");
    }

    #[test]
    fn test_health_checker_creation() {
        let config = HealthCheckConfig::default();
        let checker = HealthChecker::new(config);

        assert!(!checker.is_healthy());
        assert_eq!(checker.health_status(), ZeroCostServiceHealth::Unknown);
    }
}
