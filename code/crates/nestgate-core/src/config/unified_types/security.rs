//! Security configuration types.

use serde::{Deserialize, Serialize};

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub encryption_enabled: bool,
    pub auth_required: bool,
    pub token_expiry_hours: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            encryption_enabled: true,
            auth_required: true,
            token_expiry_hours: 24,
        }
    }
} 