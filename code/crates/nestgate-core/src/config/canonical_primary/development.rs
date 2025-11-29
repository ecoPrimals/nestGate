// Development configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Development
pub struct DevelopmentConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
}

impl Default for DevelopmentConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { enabled: false }
    }
}
