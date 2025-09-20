// **LIFECYCLE HANDLER CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHandlerConfig {
    pub startup: StartupHandlerConfig,
    pub shutdown: ShutdownHandlerConfig,
    pub health: HealthHandlerConfig,
    pub maintenance: MaintenanceHandlerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShutdownHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthHandlerConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceHandlerConfig {
    pub enabled: bool,
}

impl Default for LifecycleHandlerConfig {
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
    #[must_use]
    pub const fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        self
    }
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
