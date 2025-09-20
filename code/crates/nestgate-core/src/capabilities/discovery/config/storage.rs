use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageDiscoverySettings {
    pub enabled: bool,
}

impl StorageDiscoverySettings {
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
