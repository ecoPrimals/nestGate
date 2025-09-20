use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheDiscoverySettings {
    pub enabled: bool,
}

impl CacheDiscoverySettings {
    pub const fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
