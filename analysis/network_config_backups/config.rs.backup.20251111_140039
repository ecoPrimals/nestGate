//! # Production Service Configuration
//! Configuration types and utilities.
// Configuration structures for production smart services

use std::time::Duration;

/// Configuration for production service
#[derive(Debug, Clone)]
pub struct ProductionServiceConfig {
    /// Maximum concurrent requests
    pub max_concurrent_requests: usize,
    /// Request timeout
    pub request_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Metrics collection interval
    pub metrics_interval: Duration,
    /// Enable performance monitoring
    pub enable_monitoring: bool,
}
impl Default for ProductionServiceConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            request_timeout: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(10),
            metrics_interval: Duration::from_secs(5),
            enable_monitoring: true,
        }
    }
} 