//! Security configuration module
//!
//! Provides configuration for authentication, encryption, and access control.

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::env;

/// Security configuration for authentication and encryption.
///
/// # Environment Variables
///
/// - `NESTGATE_AUTH_ENABLED` - Enable authentication (default: true)
/// - `NESTGATE_JWT_SECRET` - JWT secret key (required if auth enabled)
/// - `NESTGATE_ENCRYPTION_ENABLED` - Enable encryption (default: true)
/// - `NESTGATE_TLS_CERT_PATH` - TLS certificate path (optional)
/// - `NESTGATE_TLS_KEY_PATH` - TLS key path (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    pub auth_enabled: bool,

    /// JWT secret (sensitive)
    #[serde(skip_serializing)]
    pub jwt_secret: String,

    /// Enable encryption at rest
    pub encryption_enabled: bool,

    /// TLS certificate path
    pub tls_cert_path: Option<String>,

    /// TLS key path
    pub tls_key_path: Option<String>,
}

impl SecurityConfig {
    /// Load security configuration from environment variables.
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            auth_enabled: env::var("NESTGATE_AUTH_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            jwt_secret: env::var("NESTGATE_JWT_SECRET").unwrap_or_else(|_| {
                // In production, this should be set explicitly
                "change-me-in-production".to_string()
            }),
            encryption_enabled: env::var("NESTGATE_ENCRYPTION_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            tls_cert_path: env::var("NESTGATE_TLS_CERT_PATH").ok(),
            tls_key_path: env::var("NESTGATE_TLS_KEY_PATH").ok(),
        })
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_enabled: true,
            jwt_secret: "change-me-in-production".to_string(),
            encryption_enabled: true,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_config_default() {
        let config = SecurityConfig::default();
        assert!(config.auth_enabled);
        assert_eq!(config.jwt_secret, "change-me-in-production");
        assert!(config.encryption_enabled);
        assert!(config.tls_cert_path.is_none());
        assert!(config.tls_key_path.is_none());
    }

    #[test]
    fn test_security_config_from_environment_defaults() {
        temp_env::with_vars(
            [
                ("NESTGATE_AUTH_ENABLED", None::<&str>),
                ("NESTGATE_JWT_SECRET", None::<&str>),
                ("NESTGATE_ENCRYPTION_ENABLED", None::<&str>),
                ("NESTGATE_TLS_CERT_PATH", None::<&str>),
                ("NESTGATE_TLS_KEY_PATH", None::<&str>),
            ],
            || {
                let config = SecurityConfig::from_environment().unwrap();
                assert!(config.auth_enabled);
                assert_eq!(config.jwt_secret, "change-me-in-production");
                assert!(config.encryption_enabled);
                assert!(config.tls_cert_path.is_none());
                assert!(config.tls_key_path.is_none());
            },
        );
    }

    #[test]
    fn test_security_config_from_environment_overrides() {
        temp_env::with_vars(
            [
                ("NESTGATE_AUTH_ENABLED", Some("false")),
                ("NESTGATE_JWT_SECRET", Some("my-secret-key")),
                ("NESTGATE_ENCRYPTION_ENABLED", Some("false")),
                ("NESTGATE_TLS_CERT_PATH", Some("/path/to/cert.pem")),
                ("NESTGATE_TLS_KEY_PATH", Some("/path/to/key.pem")),
            ],
            || {
                let config = SecurityConfig::from_environment().unwrap();
                assert!(!config.auth_enabled);
                assert_eq!(config.jwt_secret, "my-secret-key");
                assert!(!config.encryption_enabled);
                assert_eq!(config.tls_cert_path.as_deref(), Some("/path/to/cert.pem"));
                assert_eq!(config.tls_key_path.as_deref(), Some("/path/to/key.pem"));
            },
        );
    }

    #[test]
    fn test_security_config_jwt_secret_skipped_in_serialization() {
        let config = SecurityConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.contains("change-me-in-production"));
    }
}
