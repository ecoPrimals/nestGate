//! **NETWORK ORCHESTRATION CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkOrchestrationConfig {
    pub enabled: bool,
    pub coordinator_address: String,
    pub heartbeat_interval_secs: u64,
}

impl NetworkOrchestrationConfig {
    pub fn development_optimized() -> Self {
        Self {
            enabled: false,
            coordinator_address: "127.0.0.1:9090".to_string(),
            heartbeat_interval_secs: 30,
        }
    }

    pub fn production_hardened() -> Self {
        Self {
            enabled: true,
            coordinator_address: "coordinator.nestgate.local:9090".to_string(),
            heartbeat_interval_secs: 10,
        }
    }

    pub fn validate(&self) -> Result<()> { Ok(()) }
    
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.coordinator_address = other.coordinator_address;
        self.heartbeat_interval_secs = other.heartbeat_interval_secs;
        self
    }
} 