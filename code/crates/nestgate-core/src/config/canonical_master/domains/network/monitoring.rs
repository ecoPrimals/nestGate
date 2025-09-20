// **NETWORK MONITORING CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkMonitoringConfig {
    pub metrics_enabled: bool,
    pub health_check_interval_secs: u64,
    pub log_network_events: bool,
}

impl NetworkMonitoringConfig {
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            metrics_enabled: true,
            health_check_interval_secs: 60,
            log_network_events: false,
        }
    }

    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            metrics_enabled: true,
            health_check_interval_secs: 30,
            log_network_events: true,
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
        pub const fn validate(&self) -> Result<()>  {
        Ok(())
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.metrics_enabled = other.metrics_enabled;
        self.health_check_interval_secs = other.health_check_interval_secs;
        self.log_network_events = other.log_network_events;
        self
    }
}
