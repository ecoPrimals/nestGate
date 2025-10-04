//! **HEALTH MONITORING**
//!
//! Health monitoring and status tracking for system components.

use crate::error::NestGateError;
use async_trait::async_trait;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    /// Component is healthy
    Healthy,
    /// Component has warnings
    Warning,
    /// Component is unhealthy
    Unhealthy,
    /// Component status unknown
    Unknown,
}

/// Component health information
#[derive(Debug, Clone)]
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Current health status
    pub status: HealthStatus,
    /// Last check timestamp
    pub last_check: Instant,
    /// Health message
    pub message: String,
    /// Check duration
    pub check_duration: Duration,
}

/// Health check trait
#[async_trait]
pub trait HealthCheck: Send + Sync + std::fmt::Debug {
    /// Perform health check
    async fn check_health(&self) -> Result<HealthStatus, NestGateError>;

    /// Get component name
    fn component_name(&self) -> &str;
}

/// Health monitor
#[derive(Debug)]
pub struct HealthMonitor {
    /// Registered health checks
    checks: HashMap<String, Box<dyn HealthCheck>>,
    /// Component health status
    health_status: HashMap<String, ComponentHealth>,
    /// Check interval
    check_interval: Duration,
}

impl HealthMonitor {
    /// Create new health monitor
    #[must_use]
    pub fn new(check_interval: Duration) -> Self {
        Self {
            checks: HashMap::new(),
            health_status: HashMap::new(),
            check_interval,
        }
    }

    /// Register health check
    pub fn register(&mut self, check: Box<dyn HealthCheck>) {
        let name = check.component_name().to_string();
        debug!("Registering health check for: {}", name);
        self.checks.insert(name, check);
    }

    /// Get the configured check interval
    pub fn get_check_interval(&self) -> Duration {
        self.check_interval
    }

    /// Run all health checks
    pub async fn check_all(&mut self) -> HashMap<String, ComponentHealth> {
        let mut results = HashMap::new();

        for (name, check) in &self.checks {
            let start = Instant::now();
            let status = match check.check_health().await {
                Ok(status) => status,
                Err(e) => {
                    warn!("Health check failed for {}: {}", name, e);
                    HealthStatus::Unhealthy
                }
            };
            let duration = start.elapsed();

            let health = ComponentHealth {
                name: name.clone(),
                status: status.clone(),
                last_check: Instant::now(),
                message: format!("Status: {status:?}"),
                check_duration: duration,
            };

            results.insert(name.clone(), health.clone());
            self.health_status.insert(name.clone(), health);
        }

        results
    }

    /// Get health status for component
    pub fn get_health(&self, component: &str) -> Option<&ComponentHealth> {
        self.health_status.get(component)
    }

    /// Get overall health status
    pub fn overall_health(&self) -> HealthStatus {
        if self.health_status.is_empty() {
            return HealthStatus::Unknown;
        }

        let mut has_warning = false;
        for health in self.health_status.values() {
            match health.status {
                HealthStatus::Unhealthy => return HealthStatus::Unhealthy,
                HealthStatus::Warning => has_warning = true,
                HealthStatus::Unknown => has_warning = true,
                HealthStatus::Healthy => {}
            }
        }

        if has_warning {
            HealthStatus::Warning
        } else {
            HealthStatus::Healthy
        }
    }
}

impl Default for HealthMonitor {
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}
