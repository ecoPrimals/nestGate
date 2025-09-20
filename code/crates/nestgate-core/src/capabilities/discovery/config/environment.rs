use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentDiscoverySettings {
    pub enabled: bool,
}

impl EnvironmentDiscoverySettings {
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
