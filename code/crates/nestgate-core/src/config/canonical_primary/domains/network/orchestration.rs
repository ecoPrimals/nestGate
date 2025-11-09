// **NETWORK ORCHESTRATION CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkOrchestrationConfig {
    pub enabled: bool,
    pub coordinator_address: String,
    pub heartbeat_interval_secs: u64,
}

impl NetworkOrchestrationConfig {
    #[must_use]
    pub fn development_optimized() -> Self {
        use crate::constants::hardcoding::{addresses, ports};
        Self {
            enabled: false,
            coordinator_address: format!(
                "{}:{}",
                addresses::LOCALHOST_IPV4,
                ports::METRICS_DEFAULT
            ),
            heartbeat_interval_secs: 30,
        }
    }

    #[must_use]
    pub fn production_hardened() -> Self {
        use crate::constants::hardcoding::ports;
        Self {
            enabled: true,
            coordinator_address: format!("coordinator.nestgate.local:{}", ports::METRICS_DEFAULT),
            heartbeat_interval_secs: 10,
        }
    }

    /// Function description
    ///
    /// # Errors
    ///
    /// This function will return an error if the operation fails.
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }

    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.coordinator_address = other.coordinator_address;
        self.heartbeat_interval_secs = other.heartbeat_interval_secs;
        self
    }
}
