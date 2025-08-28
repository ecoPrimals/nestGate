use std::collections::HashMap;
//
// Monitors service health and detects failures to enable proactive
// resilience measures.

use crate::error::CanonicalResult as Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Type alias for complex service health storage
type ServiceHealthMap = Arc<RwLock<HashMap<String, ServiceHealth>>>;

/// Failure detector for monitoring service health
pub struct FailureDetector {
    config: FailureDetectorConfig,
    services: ServiceHealthMap,
}

impl Default for FailureDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct FailureDetectorConfig {
    /// Health check interval
    pub check_interval: Duration,
    /// Failure threshold before marking as unhealthy
    pub failure_threshold: u32,
    /// Recovery threshold before marking as healthy
    pub recovery_threshold: u32,
    /// Timeout for health checks
    pub health_check_timeout: Duration,
}

impl Default for FailureDetectorConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            failure_threshold: 3,
            recovery_threshold: 2,
            health_check_timeout: Duration::from_secs(5),
        }
    }
}

/// Service health information
#[derive(Debug, Clone)]
pub struct ServiceHealth {
    #[allow(dead_code)]
    name: String,
    is_healthy: bool,
    consecutive_failures: u32,
    consecutive_successes: u32,
    last_check: Instant,
}

impl FailureDetector {
    pub fn new() -> Self {
        Self::with_config(FailureDetectorConfig::default())
    }

    pub fn with_config(config: FailureDetectorConfig) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Register a service for monitoring
    pub async fn register_service(&self, service_name: String) -> Result<()> {
        let mut services = self.services.write().await;
        services.insert(
            service_name.clone(),
            ServiceHealth {
                name: service_name,
                is_healthy: true,
                consecutive_failures: 0,
                consecutive_successes: 0,
                last_check: Instant::now(),
            },
        );
        Ok(())
    }

    /// Record a successful operation for a service
    pub async fn record_success(&self, service_name: &str) -> Result<()> {
        let mut services = self.services.write().await;
        if let Some(health) = services.get_mut(service_name) {
            health.consecutive_failures = 0;
            health.consecutive_successes += 1;
            health.last_check = Instant::now();

            // Check if we should mark as healthy
            if !health.is_healthy && health.consecutive_successes >= self.config.recovery_threshold
            {
                health.is_healthy = true;
                tracing::info!(
                    "Service '{}' marked as healthy after {} consecutive successes",
                    service_name,
                    health.consecutive_successes
                );
            }
        }
        Ok(())
    }

    /// Record a failed operation for a service
    pub async fn record_failure(&self, service_name: &str) -> Result<()> {
        let mut services = self.services.write().await;
        if let Some(health) = services.get_mut(service_name) {
            health.consecutive_successes = 0;
            health.consecutive_failures += 1;
            health.last_check = Instant::now();

            // Check if we should mark as unhealthy
            if health.is_healthy && health.consecutive_failures >= self.config.failure_threshold {
                health.is_healthy = false;
                tracing::warn!(
                    "Service '{}' marked as unhealthy after {} consecutive failures",
                    service_name,
                    health.consecutive_failures
                );
            }
        }
        Ok(())
    }

    /// Check if a service is healthy
    pub async fn is_healthy(&self, service_name: &str) -> Result<bool> {
        let services = self.services.read().await;
        Ok(services
            .get(service_name)
            .is_none_or(|health| health.is_healthy))
    }

    /// Get health status for all services
    pub async fn get_all_health_status(&self) -> Result<HashMap<String, bool>> {
        let services = self.services.read().await;
        Ok(services
            .iter()
            .map(|(name, health)| (name.clone(), health.is_healthy))
            .collect())
    }

    /// Get detailed health information for a service
    pub async fn get_service_health(&self, service_name: &str) -> Result<Option<ServiceHealth>> {
        let services = self.services.read().await;
        Ok(services.get(service_name).cloned())
    }
}
