// **STORAGE MONITORING CONFIGURATION**

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageMonitoringConfig {
    pub metrics: MetricsStorageConfig,
    pub alerting: AlertingStorageConfig,
    pub logging: LoggingStorageConfig,
    pub health_check: HealthCheckStorageConfig,
    pub diagnostics: DiagnosticsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsStorageConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingStorageConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingStorageConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckStorageConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticsConfig {
    pub enabled: bool,
}

impl Default for StorageMonitoringConfig {
    fn default() -> Self {
        Self {
            metrics: MetricsStorageConfig { enabled: true },
            alerting: AlertingStorageConfig { enabled: false },
            logging: LoggingStorageConfig { enabled: true },
            health_check: HealthCheckStorageConfig { enabled: true },
            diagnostics: DiagnosticsConfig { enabled: false },
        }
    }
}

impl StorageMonitoringConfig {
    #[must_use]
    pub fn production_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn high_performance() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn cloud_native() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
