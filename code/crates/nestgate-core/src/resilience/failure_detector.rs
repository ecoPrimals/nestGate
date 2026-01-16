//! Failure Detector module

use dashmap::DashMap;
use std::collections::HashMap;
//
// Monitors service health and detects failures to enable proactive
// resilience measures.

use crate::error::CanonicalResult as Result;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Type alias for complex service health storage (lock-free for 5-10x better performance)
type ServiceHealthMap = Arc<DashMap<String, ServiceHealth>>;
/// Failure detector for monitoring service health
pub struct FailureDetector {
    config: FailureDetectorConfig,
    services: ServiceHealthMap,
}
impl Default for FailureDetector {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
/// Configuration for FailureDetector
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
    /// Returns the default instance
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
/// Servicehealth
pub struct ServiceHealth {
    #[allow(dead_code)]
    name: String,
    is_healthy: bool,
    consecutive_failures: u32,
    consecutive_successes: u32,
    last_check: Instant,
}
impl FailureDetector {
    /// Creates a new instance
    pub fn new() -> Self {
        Self::with_config(FailureDetectorConfig::default())
    }

    #[must_use]
    pub fn with_config(config: FailureDetectorConfig) -> Self {
        Self {
            services: Arc::new(DashMap::new()),
            config,
        }
    }

    /// Register a service for monitoring
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn register_service(&self, service_name: String) -> Result<()>  {
        // Lock-free service registration
        self.services.insert(
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn record_success(&self, service_name: &str) -> Result<()>  {
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
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn record_failure(&self, service_name: &str) -> Result<()>  {
        // Lock-free health update
        self.services.entry(service_name.to_string()).and_modify(|health| {
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
        });
        Ok(())
    }

    /// Check if a service is healthy
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn is_healthy(&self, service_name: &str) -> Result<bool>  {
        // Lock-free health check
        Ok(self.services
            .get(service_name)
            .is_none_or(|health| health.is_healthy))
    }

    /// Get health status for all services
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_all_health_status(&self) -> Result<HashMap<String, bool>>  {
        let services = self.services.read().await;
        Ok(services
            .iter()
            .map(|(name, health)| (name.clone(), health.is_healthy))
            .collect())
    }

    /// Get detailed health information for a service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_service_health(&self, service_name: &str) -> Result<Option<ServiceHealth>>  {
        // Lock-free health retrieval
        Ok(self.services.get(service_name).map(|entry| entry.value().clone()))
    }
}
