// **NETWORK VLAN CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkVlanConfig {
    pub enabled: bool,
    pub vlan_id: u16,
    pub trunk_ports: Vec<String>,
}

impl NetworkVlanConfig {
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            enabled: false,
            vlan_id: 100,
            trunk_ports: vec![],
        }
    }

    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            enabled: true,
            vlan_id: 200,
            trunk_ports: vec!["eth0".to_string()],
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
        self.enabled = other.enabled;
        self.vlan_id = other.vlan_id;
        self.trunk_ports = other.trunk_ports;
        self
    }
}
