//! Security configuration structures

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub enabled: bool,
    pub session_timeout: std::time::Duration,
    pub encryption_at_rest: bool,
    pub basic_encryption: BasicEncryptionConfig,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicEncryptionConfig {
    pub algorithm: String,
    pub key_size: u32,
    pub enabled: bool,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            session_timeout: std::time::Duration::from_secs(
                std::env::var("NESTGATE_SESSION_TIMEOUT")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(3600), // 1 hour default
            ),
            encryption_at_rest: std::env::var("NESTGATE_ENCRYPTION_AT_REST")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true), // Enable by default for security
            basic_encryption: BasicEncryptionConfig::default(),
        }
    }
}

impl Default for BasicEncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: std::env::var("NESTGATE_ENCRYPTION_ALGORITHM")
                .unwrap_or_else(|_| "aes-256-gcm".to_string()), // Most secure default
            key_size: std::env::var("NESTGATE_ENCRYPTION_KEY_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(256), // 256-bit keys by default
            enabled: std::env::var("NESTGATE_BASIC_ENCRYPTION_ENABLED")
                .map(|v| v.parse().unwrap_or(true))
                .unwrap_or(true), // Enable by default for security
        }
    }
}
