//! Health monitoring traits for Universal Primal Architecture
//!
//! This module provides health monitoring capabilities that can be implemented
//! by any service in the ecosystem for status reporting and diagnostics.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Health status levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Detailed health state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthState {
    pub status: HealthStatus,
    pub message: Option<String>,
    pub timestamp: SystemTime,
    pub details: HashMap<String, String>,
    pub metrics: HashMap<String, f64>,
}

impl Default for HealthState {
    fn default() -> Self {
        Self {
            status: HealthStatus::Unknown,
            message: None,
            timestamp: SystemTime::now(),
            details: HashMap::new(),
            metrics: HashMap::new(),
        }
    }
}

/// Health check trait for individual components
#[async_trait]
pub trait HealthCheck: Send + Sync {
    /// Perform a health check and return the current state
    async fn check_health(&self) -> crate::Result<HealthState>;

    /// Get the name of this health check
    fn check_name(&self) -> &str;

    /// Get the timeout for this health check
    fn check_timeout(&self) -> Duration {
        Duration::from_secs(30)
    }
}

/// Health monitoring trait for services
#[async_trait]
pub trait HealthMonitor: Send + Sync {
    /// Start health monitoring
    async fn start_monitoring(&self) -> crate::Result<()>;

    /// Stop health monitoring
    async fn stop_monitoring(&self) -> crate::Result<()>;

    /// Get current overall health status
    async fn get_health_status(&self) -> crate::Result<HealthState>;

    /// Get detailed health information for all components
    async fn get_detailed_health(&self) -> crate::Result<HashMap<String, HealthState>>;

    /// Register a health check
    async fn register_health_check(&self, check: Box<dyn HealthCheck>) -> crate::Result<()>;

    /// Unregister a health check by name
    async fn unregister_health_check(&self, check_name: &str) -> crate::Result<()>;
}

/// Health state builder for creating health states
pub struct HealthStateBuilder {
    state: HealthState,
}

impl HealthStateBuilder {
    pub fn new(status: HealthStatus) -> Self {
        Self {
            state: HealthState {
                status,
                timestamp: SystemTime::now(),
                ..Default::default()
            },
        }
    }

    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.state.message = Some(message.into());
        self
    }

    pub fn detail<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.state.details.insert(key.into(), value.into());
        self
    }

    pub fn metric<K: Into<String>>(mut self, key: K, value: f64) -> Self {
        self.state.metrics.insert(key.into(), value);
        self
    }

    pub fn build(self) -> HealthState {
        self.state
    }
}

impl HealthState {
    /// Create a new healthy state
    pub fn healthy() -> HealthStateBuilder {
        HealthStateBuilder::new(HealthStatus::Healthy)
    }

    /// Create a new degraded state
    pub fn degraded() -> HealthStateBuilder {
        HealthStateBuilder::new(HealthStatus::Degraded)
    }

    /// Create a new unhealthy state
    pub fn unhealthy() -> HealthStateBuilder {
        HealthStateBuilder::new(HealthStatus::Unhealthy)
    }

    /// Create a new unknown state
    pub fn unknown() -> HealthStateBuilder {
        HealthStateBuilder::new(HealthStatus::Unknown)
    }

    /// Check if this state is considered healthy
    pub fn is_healthy(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy)
    }

    /// Check if this state indicates problems
    pub fn has_issues(&self) -> bool {
        matches!(
            self.status,
            HealthStatus::Degraded | HealthStatus::Unhealthy
        )
    }
}
