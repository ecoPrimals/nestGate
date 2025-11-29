//! Health and Metrics Type Definitions
//!
//! Service health status and performance metrics types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical Service Health Status
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicehealth
pub struct ServiceHealth {
    /// Status
    pub status: HealthStatus,
    /// Uptime
    pub uptime: Duration,
    /// Last Check
    pub last_check: std::time::SystemTime,
}

/// Health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Status values for Health
pub enum HealthStatus {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

impl Default for ServiceHealth {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            uptime: Duration::from_secs(0),
            last_check: std::time::SystemTime::now(),
        }
    }
}

impl ServiceHealth {
    /// Create a new healthy service status
    #[must_use]
    pub fn healthy() -> Self {
        Self {
            status: HealthStatus::Healthy,
            uptime: Duration::from_secs(0),
            last_check: std::time::SystemTime::now(),
        }
    }

    /// Check if service is healthy
    #[must_use]
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy)
    }

    /// Check if service is degraded
    #[must_use]
    pub fn is_degraded(&self) -> bool {
        matches!(self.status, HealthStatus::Degraded)
    }
}

/// Canonical Service Metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicemetrics
pub struct ServiceMetrics {
    /// Requests Per Second
    pub requests_per_second: f64,
    /// Average Latency Ms
    pub average_latency_ms: f64,
    /// Error Rate
    pub error_rate: f64,
    /// Cpu Usage
    pub cpu_usage: f64,
    /// Memory Usage in megabytes
    pub memory_usage_mb: u64,
}

impl Default for ServiceMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            requests_per_second: 0.0,
            average_latency_ms: 0.0,
            error_rate: 0.0,
            cpu_usage: 0.0,
            memory_usage_mb: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_health_default() {
        let health = ServiceHealth::default();
        assert_eq!(health.status, HealthStatus::Unknown);
        assert_eq!(health.uptime, Duration::from_secs(0));
    }

    #[test]
    fn test_service_health_healthy() {
        let health = ServiceHealth::healthy();
        assert_eq!(health.status, HealthStatus::Healthy);
        assert!(health.is_healthy());
        assert!(!health.is_degraded());
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Degraded, HealthStatus::Unhealthy);
    }

    #[test]
    fn test_service_health_is_healthy() {
        let mut health = ServiceHealth {
            status: HealthStatus::Healthy,
            ..Default::default()
        };
        assert!(health.is_healthy());

        health.status = HealthStatus::Degraded;
        assert!(!health.is_healthy());

        health.status = HealthStatus::Unhealthy;
        assert!(!health.is_healthy());
    }

    #[test]
    fn test_service_health_is_degraded() {
        let mut health = ServiceHealth {
            status: HealthStatus::Degraded,
            ..Default::default()
        };
        assert!(health.is_degraded());

        health.status = HealthStatus::Healthy;
        assert!(!health.is_degraded());

        health.status = HealthStatus::Unhealthy;
        assert!(!health.is_degraded());
    }

    #[test]
    fn test_service_health_serialization() {
        let health = ServiceHealth::healthy();
        let json = serde_json::to_string(&health).expect("Failed to serialize");
        let deserialized: ServiceHealth =
            serde_json::from_str(&json).expect("Failed to deserialize");
        assert_eq!(health.status, deserialized.status);
    }

    #[test]
    fn test_service_metrics_default() {
        let metrics = ServiceMetrics::default();
        assert_eq!(metrics.requests_per_second, 0.0);
        assert_eq!(metrics.average_latency_ms, 0.0);
        assert_eq!(metrics.error_rate, 0.0);
        assert_eq!(metrics.cpu_usage, 0.0);
        assert_eq!(metrics.memory_usage_mb, 0);
    }

    #[test]
    fn test_service_metrics_custom_values() {
        let metrics = ServiceMetrics {
            requests_per_second: 1500.0,
            average_latency_ms: 23.5,
            error_rate: 0.001,
            cpu_usage: 45.2,
            memory_usage_mb: 512,
        };

        assert_eq!(metrics.requests_per_second, 1500.0);
        assert_eq!(metrics.average_latency_ms, 23.5);
        assert_eq!(metrics.error_rate, 0.001);
        assert_eq!(metrics.cpu_usage, 45.2);
        assert_eq!(metrics.memory_usage_mb, 512);
    }

    #[test]
    fn test_service_metrics_serialization() {
        let metrics = ServiceMetrics {
            requests_per_second: 100.0,
            average_latency_ms: 50.0,
            error_rate: 0.01,
            cpu_usage: 25.0,
            memory_usage_mb: 256,
        };

        let json = serde_json::to_string(&metrics).expect("Failed to serialize");
        let deserialized: ServiceMetrics =
            serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(
            metrics.requests_per_second,
            deserialized.requests_per_second
        );
        assert_eq!(metrics.average_latency_ms, deserialized.average_latency_ms);
        assert_eq!(metrics.error_rate, deserialized.error_rate);
        assert_eq!(metrics.cpu_usage, deserialized.cpu_usage);
        assert_eq!(metrics.memory_usage_mb, deserialized.memory_usage_mb);
    }

    #[test]
    fn test_health_status_serialization() {
        let statuses = vec![
            HealthStatus::Healthy,
            HealthStatus::Degraded,
            HealthStatus::Unhealthy,
            HealthStatus::Unknown,
        ];

        for status in statuses {
            let json = serde_json::to_string(&status).expect("Failed to serialize");
            let deserialized: HealthStatus =
                serde_json::from_str(&json).expect("Failed to deserialize");
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_service_health_with_uptime() {
        let mut health = ServiceHealth::healthy();
        health.uptime = Duration::from_secs(3600); // 1 hour

        assert_eq!(health.uptime.as_secs(), 3600);
        assert!(health.is_healthy());
    }
}
