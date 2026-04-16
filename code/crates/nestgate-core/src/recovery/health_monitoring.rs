// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// **DYNAMIC** health check trait using explicit future boxing for trait objects.
///
/// Required for `Box<dyn HealthCheckDyn>` / `Arc<dyn HealthCheckDyn>`.
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
    fn check_health(
        &self,
    ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>;

    /// Get component name
    fn component_name(&self) -> &str;
}

// ==================== HEALTH MONITOR ====================

/// Health monitor with dynamic health checks (trait objects)
///
/// This version uses `Box<dyn HealthCheckDyn>` for runtime extensibility.
/// For zero-cost monitoring, use `HealthMonitorZeroCost<T>` instead.
#[derive(Debug)]
/// Healthmonitor
pub struct HealthMonitor {
    /// Registered health checks
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
    pub fn register(&mut self, check: Box<dyn HealthCheckDyn>) {
        let name = check.component_name().to_string();
        debug!("Registering health check for: {}", name);
        self.checks.insert(name, check);
    }

    /// Get the configured check interval
    #[must_use]
    pub const fn get_check_interval(&self) -> Duration {
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
                HealthStatus::Warning | HealthStatus::Unknown => has_warning = true,
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
mod tests {
    use super::*;
    use crate::error::NestGateError;
    use std::future::Future;
    use std::pin::Pin;

    #[derive(Debug)]
    struct StaticCheck {
        name: &'static str,
        status: HealthStatus,
    }

    impl HealthCheckDyn for StaticCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
        {
            let s = self.status.clone();
            Box::pin(async move { Ok(s) })
        }

        fn component_name(&self) -> &str {
            self.name
        }
    }

    #[derive(Debug)]
    struct FailingCheck {
        name: &'static str,
    }

    impl HealthCheckDyn for FailingCheck {
        fn check_health(
            &self,
        ) -> Pin<Box<dyn Future<Output = Result<HealthStatus, NestGateError>> + Send + '_>>
        {
            Box::pin(
                async move { Err(NestGateError::validation_error("simulated health failure")) },
            )
        }

        fn component_name(&self) -> &str {
            self.name
        }
    }

    #[tokio::test]
    async fn register_and_run_checks() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(1));
        monitor.register(Box::new(StaticCheck {
            name: "c1",
            status: HealthStatus::Healthy,
        }));
        let results = monitor.check_all().await;
        assert_eq!(results.len(), 1);
        let h = monitor.get_health("c1").expect("component health");
        assert_eq!(h.status, HealthStatus::Healthy);
        assert_eq!(monitor.overall_health(), HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn failing_check_becomes_unhealthy_status() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(1));
        monitor.register(Box::new(FailingCheck { name: "bad" }));
        monitor.check_all().await;
        let h = monitor.get_health("bad").expect("recorded");
        assert_eq!(h.status, HealthStatus::Unhealthy);
        assert_eq!(monitor.overall_health(), HealthStatus::Unhealthy);
    }

    #[tokio::test]
    async fn warning_and_healthy_yields_overall_warning() {
        let mut monitor = HealthMonitor::new(Duration::from_secs(1));
        monitor.register(Box::new(StaticCheck {
            name: "w",
            status: HealthStatus::Warning,
        }));
        monitor.register(Box::new(StaticCheck {
            name: "ok",
            status: HealthStatus::Healthy,
        }));
        monitor.check_all().await;
        assert_eq!(monitor.overall_health(), HealthStatus::Warning);
    }

    #[tokio::test]
    async fn overall_unknown_when_no_checks_ran() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));
        assert_eq!(monitor.overall_health(), HealthStatus::Unknown);
    }

    #[test]
    fn check_interval_accessor() {
        let d = Duration::from_millis(500);
        let m = HealthMonitor::new(d);
        assert_eq!(m.get_check_interval(), d);
    }
}
