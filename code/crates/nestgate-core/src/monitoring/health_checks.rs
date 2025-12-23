// Health Check System
//! Monitoring and observability functionality.
// Comprehensive health monitoring for all NestGate components including
//! providers, storage backends, system resources, and dependencies.

use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Overall health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Status values for Health
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some non-critical issues detected
    Degraded { warnings: Vec<String> },
    /// Critical issues detected
    Unhealthy { errors: Vec<String> },
    /// Unknown status (startup, maintenance, etc.)
    Unknown,
}
impl HealthStatus {
    /// Check if the status indicates the system is operational
    pub fn is_operational(&self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded { .. })
    }

    /// Get severity level (0 = healthy, 1 = degraded, 2 = unhealthy, 3 = unknown)
    pub fn severity_level(&self) -> u8 {
        match self {
            HealthStatus::Healthy => 0,
            HealthStatus::Degraded { .. } => 1,
            HealthStatus::Unhealthy { .. } => 2,
            HealthStatus::Unknown => 3,
        }
    }
}

/// Health information for a specific component
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Componenthealth
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Component type (provider, storage, system, etc.)
    pub component_type: String,
    /// Current health status
    pub status: HealthStatus,
    /// Last health check timestamp
    pub last_check: SystemTime,
    /// Time taken for the health check
    pub check_duration: Duration,
    /// Additional details about the health status
    pub details: HashMap<String, String>,
    /// Health check history (last N checks)
    pub history: Vec<HealthCheckResult>,
}
/// Result of a health check
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthcheckresult
pub struct HealthCheckResult {
    /// Timestamp of the check
    pub timestamp: SystemTime,
    /// Status at the time of check
    pub status: HealthStatus,
    /// Duration of the check
    pub duration: Duration,
    /// Any error message if check failed
    pub error_message: Option<String>,
}
/// System-wide health information
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Systemhealth
pub struct SystemHealth {
    /// Overall system status
    pub overall_status: HealthStatus,
    /// Individual component healths
    pub components: HashMap<String, ComponentHealth>,
    /// System uptime
    pub uptime: Duration,
    /// Last updated timestamp
    pub last_updated: SystemTime,
    /// Health check summary
    pub summary: HealthSummary,
}
/// Summary of health check results
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthsummary
pub struct HealthSummary {
    /// Total number of components
    pub total_components: usize,
    /// Number of healthy components
    pub healthy_components: usize,
    /// Number of degraded components
    pub degraded_components: usize,
    /// Number of unhealthy components
    pub unhealthy_components: usize,
    /// Number of components with unknown status
    pub unknown_components: usize,
}
/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for HealthCheck
pub struct HealthCheckConfig {
    /// How often to perform health checks
    pub check_interval: Duration,
    /// Timeout for individual health checks
    pub check_timeout: Duration,
    /// Number of health check results to keep in history
    pub history_size: usize,
    /// Whether to perform deep health checks (more thorough but slower)
    pub deep_checks_enabled: bool,
    /// Custom health check endpoints
    pub custom_endpoints: Vec<String>,
}
impl Default for HealthCheckConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            check_timeout: Duration::from_secs(10),
            history_size: 10,
            deep_checks_enabled: false,
            custom_endpoints: Vec::new(),
        }
    }
}

/// Trait for components that can be health checked
/// **CANONICAL MODERNIZATION**: Native async trait without async_trait overhead
pub trait HealthCheckable: Send + Sync + 'static {
    /// Perform a health check on this component
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send;
    /// Get the component name
    fn component_name(&self) -> &str;

    /// Get the component type
    fn component_type(&self) -> &str;

    /// Perform a deep health check (optional, defaults to regular health check)
    fn deep_health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send {
        self.health_check()
    }
}

/// Health check manager
pub struct HealthCheckManager {
    /// Configuration
    config: HealthCheckConfig,
    /// Registered components
    components: Arc<RwLock<HashMap<String, Arc<dyn HealthCheckable>>>>,
    /// Health check results
    health_data: Arc<RwLock<SystemHealth>>,
    /// Manager start time
    start_time: Instant,
}
impl HealthCheckManager {
    /// Create a new health check manager
    #[must_use]
    pub fn new(config: HealthCheckConfig) -> Self {
        info!("🏥 Initializing health check manager");

        let system_health = SystemHealth {
            overall_status: HealthStatus::Unknown,
            components: HashMap::new(),
            uptime: Duration::from_secs(0),
            last_updated: SystemTime::now(),
            summary: HealthSummary {
                total_components: 0,
                healthy_components: 0,
                degraded_components: 0,
                unhealthy_components: 0,
                unknown_components: 0,
            },
        };

        Self {
            config,
            components: Arc::new(RwLock::new(HashMap::new())),
            health_data: Arc::new(RwLock::new(system_health)),
            start_time: Instant::now(),
        }
    }

    /// Register a component for health checking
    pub async fn register_component(&self, component: Arc<dyn HealthCheckable>) {
        let name = component.component_name().to_string();
        let component_type = component.component_type().to_string();

        // Add to components list
        {
            let mut components = self.components.write().await;
            components.insert(name.clone(), component);
        }

        // Initialize health data for the component
        {
            let mut health_data = self.health_data.write().await;
            health_data.components.insert(
                name.clone(),
                ComponentHealth {
                    name: name.clone(),
                    component_type,
                    status: HealthStatus::Unknown,
                    last_check: SystemTime::now(),
                    check_duration: Duration::from_secs(0),
                    details: HashMap::new(),
                    history: Vec::new(),
                },
            );
        }

        info!("📝 Registered component for health checks: {}", name);
    }

    /// Perform health checks on all registered components
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn check_all_components(&self) -> Result<SystemHealth>  {
        debug!("🔍 Performing health checks on all components");

        let components = self.components.read().await;
        let mut check_results = HashMap::new();

        // Perform health checks concurrently
        let mut check_futures = Vec::new();
        for (name, component) in components.iter() {
            let component = Arc::clone(component);
            let name = name.clone();
            let timeout = self.config.check_timeout;
            let deep_checks = self.config.deep_checks_enabled;

            let future = tokio::spawn(async move {
                let start_time = Instant::now();

                let result = tokio::time::timeout(timeout, async move {
                    if deep_checks {
                        component.deep_health_check().await
                    } else {
                        component.health_check().await
                    }
                })
                .await;

                let duration = start_time.elapsed();

                let (status, error_message) = match result {
                    Ok(Ok(status)) => (status, None),
                    Ok(Err(e)) => (
                        HealthStatus::Unhealthy {
                            errors: vec![format!("Health check error: {e}")],
                        },
                        Some(e.to_string()),
                    ),
                    Err(_) => (
                        HealthStatus::Unhealthy {
                            errors: vec!["Health check timeout".to_string()],
                        },
                        Some("Timeout".to_string()),
                    ),
                };

                (name, status, duration, error_message)
            );

            check_futures.push(future);
        }

        // Collect results
        for future in check_futures {
            match future.await {
                Ok((name, status, duration, error_message)) => {
                    let check_result = HealthCheckResult {
                        timestamp: SystemTime::now(),
                        status: status.clone(),
                        duration,
                        error_message,
                    };
                    check_results.insert(name, (status, duration, check_result));
                }
                Err(e) => {
                    error!("Health check task failed: {}", e);
                }
            }
        }

        // Update health data
        {
            let mut health_data = self.health_data.write().await;

            // Update component health
            for (name, (status, duration, check_result)) in check_results {
                if let Some(component_health) = health_data.components.get_mut(&name) {
                    component_health.status = status;
                    component_health.last_check = SystemTime::now();
                    component_health.check_duration = duration;

                    // Add to history
                    component_health.history.push(check_result);

                    // Keep only recent history
                    if component_health.history.len() > self.config.history_size {
                        component_health.history.remove(0);
                    }
                }
            }

            // Update overall status and summary
            self.update_overall_status(&mut health_data).await;

            health_data.uptime = self.start_time.elapsed();
            health_data.last_updated = SystemTime::now();
        }

        let health_data = self.health_data.read().await;
        Ok(health_data.clone())
    }

    /// Update the overall system status based on component statuses
    async fn update_overall_status(&self, health_data: &mut SystemHealth) {
        let mut healthy = 0;
        let mut degraded = 0;
        let mut unhealthy = 0;
        let mut unknown = 0;

        for component in health_data.components.values() {
            match component.status {
                HealthStatus::Healthy => healthy += 1,
                HealthStatus::Degraded { .. } => degraded += 1,
                HealthStatus::Unhealthy { .. } => unhealthy += 1,
                HealthStatus::Unknown => unknown += 1,
            }
        }

        let total = healthy + degraded + unhealthy + unknown;

        health_data.summary = HealthSummary {
            total_components: total,
            healthy_components: healthy,
            degraded_components: degraded,
            unhealthy_components: unhealthy,
            unknown_components: unknown,
        };

        // Determine overall status
        health_data.overall_status = if unhealthy > 0 {
            let mut errors = Vec::new();
            for component in health_data.components.values() {
                if let HealthStatus::Unhealthy {
                    errors: component_errors,
                } = &component.status
                {
                    errors.extend(component_errors.iter().cloned());
                }
            }
            HealthStatus::Unhealthy { errors }
        } else if degraded > 0 {
            let mut warnings = Vec::new();
            for component in health_data.components.values() {
                if let HealthStatus::Degraded {
                    warnings: component_warnings,
                } = &component.status
                {
                    warnings.extend(component_warnings.iter().cloned());
                }
            }
            HealthStatus::Degraded { warnings }
        } else if unknown > 0 {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        };
    }

    /// Get current system health
    pub async fn get_system_health(&self) -> SystemHealth {
        self.health_data.read().await.clone()
    }

    /// Get health for a specific component
    pub async fn get_component_health(&self, component_name: &str) -> Option<ComponentHealth> {
        self.health_data
            .read()
            .await
            .components
            .get(component_name)
            .cloned()
    }

    /// Start background health checking
    pub fn start_health_checking(&self) -> tokio::task::JoinHandle<()> {
        let manager = Arc::new(self.clone());
        let interval = self.config.check_interval;

        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                interval_timer.tick().await;

                match manager.check_all_components().await {
                    Ok(health) => {
                        debug!(
                            "✅ Health check completed - Overall: {:?}",
                            health.overall_status
                        );

                        // Log warnings for degraded components
                        if let HealthStatus::Degraded { warnings } = &health.overall_status {
                            for warning in warnings {
                                warn!("⚠️ Health warning: {}", warning);
                            }
                        }

                        // Log errors for unhealthy components
                        if let HealthStatus::Unhealthy { errors } = &health.overall_status {
                            for error in errors {
                                error!("❌ Health error: {}", error);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Health check failed: {}", e);
                    }
                }
            }
        })
    }

    /// Generate health report
    pub async fn generate_health_report(&self) -> String {
        let health = self.get_system_health().await;

        let mut report = String::new();
        report.push_str(" NestGate Health Report\n\n");

        // Overall status
        report.push_str(&format!(
            "**Overall Status**: {:?}\n",
            health.overall_status
        ));
        report.push_str(&format!("**Uptime**: {health.uptime:?}\n"));
        report.push_str(&format!("**Last Updated**: {health.last_updated:?}\n\n"));

        // Summary
        report.push_str("# Summary\n\n");
        report.push_str(&format!(
            "- Total Components: {}\n",
            health.summary.total_components
        ));
        report.push_str(&format!(
            "- Healthy: {}\n",
            health.summary.healthy_components
        ));
        report.push_str(&format!(
            "- Degraded: {}\n",
            health.summary.degraded_components
        ));
        report.push_str(&format!(
            "- Unhealthy: {}\n",
            health.summary.unhealthy_components
        ));
        report.push_str(&format!(
            "- Unknown: {}\n\n",
            health.summary.unknown_components
        ));

        // Component details
        report.push_str("# Component Details\n\n");
        for (name, component) in &health.components {
            report.push_str(&format!("## {} ({})\n", name, component.component_type);
            report.push_str(&format!("- Status: {component.status:?}\n"));
            report.push_str(&format!("- Last Check: {component.last_check:?}\n"));
            report.push_str(&format!(
                "- Check Duration: {:?}\n",
                component.check_duration
            ));

            if !component.details.is_empty() {
                report.push_str("- Details:\n");
                for (key, value) in &component.details {
                    report.push_str(&format!("  - {key}: {value}\n"));
                }
            }

            report.push('\n');
        }

        report
    }
}

// Make HealthCheckManager cloneable for background tasks
impl Clone for HealthCheckManager {
    /// Clone
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            components: Arc::clone(&self.components),
            health_data: Arc::clone(&self.health_data),
            start_time: self.start_time,
        }
    }
}

/// Example implementation of HealthCheckable for a generic component
pub struct GenericHealthChecker {
    name: String,
    component_type: String,
    health_check_fn: Arc<dyn Fn() -> Result<HealthStatus> + Send + Sync>,
}
impl GenericHealthChecker {
    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub fn new<F>(name: String, component_type: String, health_check_fn: F) -> Self
    where
        F: Fn() -> Result<HealthStatus> + Send + Sync + 'static,
     {
        Self {
            name,
            component_type,
            health_check_fn: Arc::new(health_check_fn),
        }
    }
}

impl HealthCheckable for GenericHealthChecker {
    /// Health Check
    fn health_check(&self) -> impl std::future::Future<Output = Result<HealthStatus>> + Send {
        async move { (self.health_check_fn)() }
    }

    /// Component Name
    fn component_name(&self) -> &str {
        &self.name
    }

    /// Component Type
    fn component_type(&self) -> &str {
        &self.component_type
    }
}
