// **NETWORK SECURITY CONFIGURATION**

use crate::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSecurityConfig {
    pub firewall_enabled: bool,
    pub allowed_ips: Vec<String>,
    pub blocked_ips: Vec<String>,
}

impl NetworkSecurityConfig {
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            firewall_enabled: false,
            allowed_ips: vec![],
            blocked_ips: vec![],
        }
    }

    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            firewall_enabled: true,
            allowed_ips: vec!["10.0.0.0/8".to_string()],
            blocked_ips: vec![],
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
        self.firewall_enabled = other.firewall_enabled;
        self.allowed_ips = other.allowed_ips;
        self.blocked_ips = other.blocked_ips;
        self
    }
}
