// Development configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    pub enabled: bool,
}

impl Default for DevelopmentConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}
