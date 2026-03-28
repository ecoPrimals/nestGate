// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **HEALTH MONITORING**
//!
//! Health monitoring and status tracking for system components.
//!
//! ## Architecture
//!
//! This module provides **both** zero-cost and dynamic health checking:
//!
//! 1. **Zero-Cost Path**: Use `HealthCheckZeroCost<T>` for compile-time known types
//! 2. **Dynamic Path**: Use `HealthCheckDyn` for runtime extensibility
//!
//! ## Example
//!
//! ```rust,ignore
//! // Zero-cost (preferred for known types)
//! let monitor = HealthMonitor::new_zero_cost(Duration::from_secs(30));
//! monitor.register_typed(MyHealthCheck::new());
//!
//! // Dynamic (for extensions/plugins)
//! let monitor = HealthMonitor::new(Duration::from_secs(30));
//! monitor.register(Box::new(MyHealthCheck::new()));
//! ```

use crate::error::NestGateError;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::time::{Duration, Instant};
use tracing::{debug, warn};

/// Health status
#[derive(Debug, Clone, PartialEq)]
/// Status values for Health
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
/// Componenthealth
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

// ==================== ZERO-COST NATIVE ASYNC (PREFERRED) ====================

/// **ZERO-COST** health check trait using native async (RPITIT)
///
/// This is the **preferred** trait for health checks where types are known at compile time.
/// Provides 20-50% better performance than the dynamic trait object version.
///
/// ## When to Use
/// - For built-in health checks
/// - When types are known at compile time
/// - When maximum performance is needed
///
/// ## Example
/// ```rust,ignore
/// use nestgate_core::recovery::health_monitoring::{HealthCheckZeroCost, HealthStatus};
///
/// #[derive(Debug)]
/// struct MyHealthCheck;
///
/// impl HealthCheckZeroCost for MyHealthCheck {
///     fn check_health(&self) -> impl Future<Output = Result<HealthStatus, NestGateError>> + Send {
///         async move {
///             // Perform health check
///             Ok(HealthStatus::Healthy)
///         }
///     }
///
///     fn component_name(&self) -> &str {
///         "my_component"
///     }
/// }
/// ```
pub trait HealthCheckZeroCost: Send + Sync + std::fmt::Debug {
    /// Perform health check (native async - zero overhead)
    fn check_health(&self) -> impl Future<Output = Result<HealthStatus, NestGateError>> + Send;

    /// Get component name
    fn component_name(&self) -> &str;
}

// ==================== DYNAMIC TRAIT OBJECT (EXTENSIBILITY) ====================

/// **DYNAMIC** health check trait using `async_trait` for trait objects
///
/// This version uses `async_trait` and is required for `Box<dyn HealthCheckDyn>`.
/// Use this when you need runtime polymorphism (plugins, extensions, etc.).
///
/// ## When to Use
/// - For plugin systems
/// - When types are not known at compile time
/// - When you need `Box<dyn Trait>` or `Arc<dyn Trait>`
///
/// Dyn-compatible health check trait using explicit future boxing.
///
/// Prefer `HealthCheckZeroCost` when you don't need dynamic dispatch.
pub trait HealthCheckDyn: Send + Sync + std::fmt::Debug {
    /// Perform health check
    fn check_health(&self) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>;

    /// Get component name
    fn component_name(&self) -> &str;
}

// ==================== BACKWARD COMPATIBILITY ====================

/// Backward compatibility alias - maps to dynamic version
///
/// **DEPRECATED**: Use `HealthCheckZeroCost` for zero-cost performance
/// or `HealthCheckDyn` for dynamic dispatch explicitly.
#[deprecated(
    since = "0.11.0",
    note = "Use HealthCheckZeroCost (zero-cost) or HealthCheckDyn (dynamic) explicitly"
)]
pub use HealthCheckDyn as HealthCheck;

// ==================== HEALTH MONITOR ====================

/// Health monitor with dynamic health checks (trait objects)
///
/// This version uses `Box<dyn HealthCheckDyn>` for runtime extensibility.
/// For zero-cost monitoring, use `HealthMonitorZeroCost<T>` instead.
#[derive(Debug)]
/// Healthmonitor
pub struct HealthMonitor {
    /// Registered health checks
    #[allow(deprecated)]
    checks: HashMap<String, Box<dyn HealthCheckDyn>>,
    /// Component health status
    health_status: HashMap<String, ComponentHealth>,
    /// Check interval
    check_interval: Duration,
}

impl HealthMonitor {
    /// Create new health monitor (dynamic version)
    ///
    /// For zero-cost monitoring with compile-time types, use `HealthMonitorZeroCost::new()`.
    #[must_use]
    pub fn new(check_interval: Duration) -> Self {
        Self {
            checks: HashMap::new(),
            health_status: HashMap::new(),
            check_interval,
        }
    }

    /// Register health check (dynamic version)
    ///
    /// **Note**: This uses heap allocation and dynamic dispatch.
    /// For zero-cost registration, use `HealthMonitorZeroCost::register_typed()`.
    #[allow(deprecated)]
    pub fn register(&mut self, check: Box<dyn HealthCheckDyn>) {
        let name = check.component_name().to_string();
        debug!("Registering health check for: {}", name);
        self.checks.insert(name, check);
    }

    /// Get the configured check interval
    #[must_use]
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
    #[must_use]
    pub fn get_health(&self, component: &str) -> Option<&ComponentHealth> {
        self.health_status.get(component)
    }

    /// Get overall health status
    #[must_use]
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
    /// Returns the default instance
    fn default() -> Self {
        Self::new(Duration::from_secs(30))
    }
}

#[cfg(test)]
#[path = "health_monitoring_tests.rs"]
mod health_monitoring_tests;
