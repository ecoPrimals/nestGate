use serde::{Deserialize, Serialize};

/// Storage discovery settings for capability detection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageDiscoverySettings {
    /// Whether storage discovery is enabled
    pub enabled: bool,
}

impl StorageDiscoverySettings {
    /// Validates the storage discovery settings
    ///
    /// # Errors
    ///
    /// Returns an error if the settings are invalid
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
