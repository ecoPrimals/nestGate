use serde::{Deserialize, Serialize};

/// Cache discovery settings for capability detection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheDiscoverySettings {
    /// Whether cache discovery is enabled
    pub enabled: bool,
}

impl CacheDiscoverySettings {
    /// Validates the cache discovery settings
    ///
    /// # Errors
    ///
    /// Returns an error if the settings are invalid
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
