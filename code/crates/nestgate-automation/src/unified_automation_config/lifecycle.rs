/// **LIFECYCLE MODULE**
/// Lifecycle management configuration - extracted from monolithic config
/// Handles service lifecycle, startup, shutdown, health checks, and recovery
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
/// Lifecycle management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Lifecyclesettings
pub struct LifecycleSettings {
    /// Enable lifecycle management
    pub enabled: bool,
    /// Startup configuration
    pub startup: StartupSettings,
    /// Shutdown configuration
    pub shutdown: ShutdownSettings,
    /// Health check configuration
    pub health_checks: HealthCheckSettings,
    /// Recovery configuration
    pub recovery: RecoverySettings,
    /// Service dependencies
    pub dependencies: DependencySettings,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Startupsettings
pub struct StartupSettings {
    /// Enable automatic startup
    pub auto_start: bool,
    /// Startup timeout
    pub timeout: Duration,
    /// Startup retry attempts
    pub retry_attempts: u32,
    /// Startup delay
    pub delay: Duration,
    /// Parallel startup
    pub parallel: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Shutdownsettings
pub struct ShutdownSettings {
    /// Enable graceful shutdown
    pub graceful: bool,
    /// Shutdown timeout
    pub timeout: Duration,
    /// Force shutdown after timeout
    pub force_after_timeout: bool,
    /// Cleanup on shutdown
    pub cleanup: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Healthchecksettings
pub struct HealthCheckSettings {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Failure threshold
    pub failure_threshold: u32,
    /// Recovery threshold
    pub recovery_threshold: u32,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Recoverysettings
pub struct RecoverySettings {
    /// Enable automatic recovery
    pub enabled: bool,
    /// Recovery strategy
    pub strategy: String,
    /// Recovery timeout
    pub timeout: Duration,
    /// Maximum recovery attempts
    pub max_attempts: u32,
    /// Recovery delay
    pub delay: Duration,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Dependencysettings
pub struct DependencySettings {
    /// Service dependencies
    pub services: HashMap<String, ServiceDependency>,
    /// Dependency timeout
    pub timeout: Duration,
    /// Wait for dependencies on startup
    pub wait_on_startup: bool,
}
    #[derive(Debug, Clone, Serialize, Deserialize)]
/// Servicedependency
pub struct ServiceDependency {
    /// Dependency name
    pub name: String,
    /// Dependency required
    pub required: bool,
    /// Dependency timeout
    pub timeout: Duration,
    /// Health check endpoint
    pub health_endpoint: Option<String>,
}
impl SmartDefault for LifecycleSettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            enabled: true,
            startup: StartupSettings::smart_default(),
            shutdown: ShutdownSettings::smart_default(),
            health_checks: HealthCheckSettings::smart_default(),
            recovery: RecoverySettings::smart_default(),
            dependencies: DependencySettings::smart_default(),
        }
    }
}

impl Default for LifecycleSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}

impl SmartDefault for StartupSettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            auto_start: true,
            timeout: Duration::from_secs(60),
            retry_attempts: 3,
            delay: Duration::from_secs(1),
            parallel: true,
        }
    }
}

impl Default for StartupSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}

impl SmartDefault for ShutdownSettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            graceful: true,
            timeout: Duration::from_secs(30),
            force_after_timeout: true,
            cleanup: true,
        }
    }
}

impl Default for ShutdownSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}

impl SmartDefault for HealthCheckSettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            failure_threshold: 3,
            recovery_threshold: 2,
        }
    }
}

impl Default for HealthCheckSettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}

impl SmartDefault for RecoverySettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            enabled: true,
            strategy: "restart".to_string(),
            timeout: Duration::from_secs(60),
            max_attempts: 3,
            delay: Duration::from_secs(5),
        }
    }
}

impl Default for RecoverySettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}

impl SmartDefault for DependencySettings {
    /// Smart Default
    fn smart_default() -> Self {
        Self {
            services: HashMap::default(),
            timeout: Duration::from_secs(30),
            wait_on_startup: true,
        }
    }
}

impl Default for DependencySettings {
    /// Returns the default instance
    fn default() -> Self {
        Self::smart_default()
    }
}
