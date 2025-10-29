//! **NETWORK DISCOVERY CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkDiscoveryConfig {
    pub enabled: bool,
    pub multicast_address: String,
    pub discovery_interval_secs: u64,
}

impl NetworkDiscoveryConfig {
    pub fn development_optimized() -> Self {
        Self { enabled: false, multicast_address: "224.0.0.1".to_string(), discovery_interval_secs: 60 }
    }

    pub fn production_hardened() -> Self {
        Self { enabled: true, multicast_address: "224.0.0.100".to_string(), discovery_interval_secs: 30 }
    }

    pub fn validate(&self) -> Result<()> { Ok(()) }
    
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.multicast_address = other.multicast_address;
        self.discovery_interval_secs = other.discovery_interval_secs;
        self
    }
} 