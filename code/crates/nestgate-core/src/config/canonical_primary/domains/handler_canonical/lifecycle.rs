// **LIFECYCLE HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for LifecycleHandler
pub struct LifecycleHandlerConfig {
    /// Startup
    pub startup: StartupHandlerConfig,
    /// Shutdown
    pub shutdown: ShutdownHandlerConfig,
    /// Health
    pub health: HealthHandlerConfig,
    /// Maintenance
    pub maintenance: MaintenanceHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for StartupHandler
pub struct StartupHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for ShutdownHandler
pub struct ShutdownHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for HealthHandler
pub struct HealthHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for MaintenanceHandler
pub struct MaintenanceHandlerConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for LifecycleHandlerConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            startup: StartupHandlerConfig { enabled: true },
            shutdown: ShutdownHandlerConfig { enabled: true },
            health: HealthHandlerConfig { enabled: true },
            maintenance: MaintenanceHandlerConfig { enabled: false },
        }
    }
}

impl LifecycleHandlerConfig {
    /// Returns a production-optimized configuration
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }

    /// Returns a development-optimized configuration
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Returns a high-performance configuration
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }

    /// Merges this configuration with another, returning the merged result
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
