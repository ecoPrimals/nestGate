use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityDiscoverySettings {
    pub enabled: bool,
}

impl SecurityDiscoverySettings {
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
