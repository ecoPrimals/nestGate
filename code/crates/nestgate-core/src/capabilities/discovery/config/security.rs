use serde::{Deserialize, Serialize};

/// Security discovery settings for capability detection
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityDiscoverySettings {
    /// Whether security discovery is enabled
    pub enabled: bool,
}

impl SecurityDiscoverySettings {
    /// Validates the security discovery settings
    ///
    /// # Errors
    ///
    /// Returns an error if the settings are invalid
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
