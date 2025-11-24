//! ZFS Health Monitoring Types
//!
//! Types for monitoring service health and collecting metrics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// Health status of a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Service name
    pub service_name: String,
    /// Current status
    pub status: ServiceStatus,
    /// Health check results
    pub checks: Vec<HealthCheck>,
    /// Last check timestamp
    pub last_check: SystemTime,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Service status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ServiceStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service status is unknown
    Unknown,
}

/// Individual health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Check name
    pub name: String,
    /// Check passed
    pub passed: bool,
    /// Check message
    pub message: Option<String>,
}

/// Service metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    /// Service name
    pub service_name: String,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Total requests
    pub requests_total: u64,
    /// Failed requests
    pub requests_failed: u64,
    /// Error rate (percentage)
    pub error_rate: f64,
    /// Average latency in milliseconds
    pub latency_avg: f64,
    /// P95 latency in milliseconds
    pub latency_p95: f64,
    /// P99 latency in milliseconds
    pub latency_p99: f64,
    /// Custom metrics
    pub custom_metrics: HashMap<String, f64>,
}

impl Default for ServiceMetrics {
    fn default() -> Self {
        Self {
            service_name: String::new(),
            timestamp: SystemTime::now(),
            requests_total: 0,
            requests_failed: 0,
            error_rate: 0.0,
            latency_avg: 0.0,
            latency_p95: 0.0,
            latency_p99: 0.0,
            custom_metrics: HashMap::new(),
        }
    }
}

impl ServiceMetrics {
    /// Create new service metrics
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            ..Default::default()
        }
    }

    /// Update error rate
    pub fn update_error_rate(&mut self) {
        if self.requests_total > 0 {
            self.error_rate = (self.requests_failed as f64 / self.requests_total as f64) * 100.0;
        }
    }
}
