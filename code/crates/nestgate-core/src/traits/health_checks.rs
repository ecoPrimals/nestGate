// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Health Monitoring Traits for Universal Primal Architecture
//!
//! **MIGRATED FROM**: `traits::health_checks` (November 7, 2025)
//! **CANONICAL**: This is the single source of truth for health monitoring
//! **STATUS**: Production-ready, native async
//!
//! This module provides health monitoring capabilities that can be implemented
//! by any service in the ecosystem for status reporting and diagnostics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

/// Health status levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Detailed health state information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthstate
pub struct HealthState {
    /// Status
    pub status: HealthStatus,
    /// Message
    pub message: Option<String>,
    /// Timestamp
    pub timestamp: SystemTime,
    /// Details
    pub details: HashMap<String, String>,
    /// Metrics
    pub metrics: HashMap<String, f64>,
}

impl Default for HealthState {
    /// Returns the default instance
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
///
/// Implement this trait for components that need to report their health status.
pub trait HealthCheck: Send + Sync {
    /// Perform a health check and return the current state
    fn check_health(&self) -> impl std::future::Future<Output = crate::Result<HealthState>> + Send;

    /// Get the name of this health check
    fn check_name(&self) -> &str;

    /// Get the timeout for this health check
    fn check_timeout(&self) -> Duration {
        Duration::from_secs(30)
    }
}

/// Health monitoring trait for services
///
/// Implement this trait for services that aggregate multiple health checks.
pub trait HealthMonitor: Send + Sync {
    /// Start health monitoring
    fn start_monitoring(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Stop health monitoring
    fn stop_monitoring(&self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Get current overall health status
    fn get_health_status(
        &self,
    ) -> impl std::future::Future<Output = crate::Result<HealthState>> + Send;

    /// Get detailed health information for all components
    fn get_detailed_health(
        &self,
    ) -> impl std::future::Future<Output = crate::Result<HashMap<String, HealthState>>> + Send;

    /// Register a health check
    ///
    /// Note: Due to trait object limitations with RPITIT, implementations should
    /// use `Arc<impl HealthCheck>` or similar instead of `Box<dyn HealthCheck>`
    fn register_health_check<H: HealthCheck + 'static>(
        &self,
        check: H,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    /// Unregister a health check by name
    fn unregister_health_check(
        &self,
        check_name: &str,
    ) -> impl std::future::Future<Output = crate::Result<()>> + Send;
}

/// Health state builder for creating health states
pub struct HealthStateBuilder {
    state: HealthState,
}

impl HealthStateBuilder {
    /// Creates a new instance
    pub fn new(status: HealthStatus) -> Self {
        Self {
            state: HealthState {
                status,
                timestamp: SystemTime::now(),
                ..Default::default()
            },
        }
    }

    /// Message
    pub fn message<S: Into<String>>(mut self, message: S) -> Self {
        self.state.message = Some(message.into());
        self
    }

    /// Detail
    pub fn detail<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.state.details.insert(key.into(), value.into());
        self
    }

    /// Metric
    pub fn metric<K: Into<String>>(mut self, key: K, value: f64) -> Self {
        self.state.metrics.insert(key.into(), value);
        self
    }

    /// Builds the final instance
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
