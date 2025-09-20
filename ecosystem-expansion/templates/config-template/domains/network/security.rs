//! **NETWORK SECURITY CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkSecurityConfig {
    pub firewall_enabled: bool,
    pub allowed_ips: Vec<String>,
    pub blocked_ips: Vec<String>,
}

impl NetworkSecurityConfig {
    pub fn development_optimized() -> Self {
        Self { firewall_enabled: false, allowed_ips: vec![], blocked_ips: vec![] }
    }

    pub fn production_hardened() -> Self {
        Self { 
            firewall_enabled: true, 
            allowed_ips: vec!["10.0.0.0/8".to_string()], 
            blocked_ips: vec![] 
        }
    }

    pub fn validate(&self) -> Result<()> { Ok(()) }
    
    pub fn merge(mut self, other: Self) -> Self {
        self.firewall_enabled = other.firewall_enabled;
        self.allowed_ips = other.allowed_ips;
        self.blocked_ips = other.blocked_ips;
        self
    }
} 