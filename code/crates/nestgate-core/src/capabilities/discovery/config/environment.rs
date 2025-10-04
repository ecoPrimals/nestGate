use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentDiscoverySettings {
    pub enabled: bool,
}

impl EnvironmentDiscoverySettings {
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
