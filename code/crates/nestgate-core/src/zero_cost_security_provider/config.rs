///
/// This module contains configuration structures and utilities for the
/// zero-cost security provider system.
///
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **Zero-cost security provider configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostSecurityConfig {
    /// Provider identifier
    pub provider_id: String,
    /// Token expiration time in seconds
    pub token_expiry_seconds: u64,
    /// Maximum concurrent authentication attempts
    pub max_concurrent_auth: usize,
    /// Enable detailed security logging
    pub enable_detailed_logging: bool,
    /// Encryption algorithm preference
    pub preferred_encryption: String,
    /// Signing algorithm preference
    pub preferred_signing: String,
    /// Rate limiting configuration
    pub rate_limit_per_minute: usize,
    /// Authentication timeout
    pub auth_timeout_seconds: u64,
    /// Key rotation interval in days
    pub key_rotation_days: u32,
    /// Enable security audit logging
    pub enable_audit_logging: bool,
    /// Maximum failed attempts before lockout
    pub max_failed_attempts: u32,
    /// Account lockout duration in seconds
    pub lockout_duration_seconds: u64,
}

impl Default for ZeroCostSecurityConfig {
    fn default() -> Self {
        Self {
            provider_id: "zero-cost-security-provider".to_string(),
            token_expiry_seconds: 3600,
            max_concurrent_auth: 1000,
            enable_detailed_logging: false,
            preferred_encryption: "AES-256-GCM".to_string(),
            preferred_signing: "ECDSA-P256".to_string(),
            rate_limit_per_minute: 100,
            auth_timeout_seconds: 30,
            key_rotation_days: 90,
            enable_audit_logging: true,
            max_failed_attempts: 5,
            lockout_duration_seconds: 900, // 15 minutes
        }
    }
}

impl ZeroCostSecurityConfig {
    /// Create a new security configuration
    pub fn new(provider_id: String) -> Self {
        Self {
            provider_id,
            ..Default::default()
        }
    }

    /// Get token expiry duration
    pub fn token_expiry_duration(&self) -> Duration {
        Duration::from_secs(self.token_expiry_seconds)
    }

    /// Get authentication timeout duration
    pub fn auth_timeout_duration(&self) -> Duration {
        Duration::from_secs(self.auth_timeout_seconds)
    }

    /// Get lockout duration
    pub fn lockout_duration(&self) -> Duration {
        Duration::from_secs(self.lockout_duration_seconds)
    }

    /// Get key rotation interval
    pub fn key_rotation_interval(&self) -> Duration {
        Duration::from_secs(self.key_rotation_days as u64 * 24 * 3600)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.provider_id.is_empty() {
            return Err("Provider ID cannot be empty".to_string());
        }

        if self.token_expiry_seconds == 0 {
            return Err("Token expiry must be greater than 0".to_string());
        }

        if self.max_concurrent_auth == 0 {
            return Err("Max concurrent auth must be greater than 0".to_string());
        }

        if self.rate_limit_per_minute == 0 {
            return Err("Rate limit must be greater than 0".to_string());
        }

        if self.auth_timeout_seconds == 0 {
            return Err("Auth timeout must be greater than 0".to_string());
        }

        if self.max_failed_attempts == 0 {
            return Err("Max failed attempts must be greater than 0".to_string());
        }

        if self.lockout_duration_seconds == 0 {
            return Err("Lockout duration must be greater than 0".to_string());
        }

        Ok(())
    }

    /// Create a high-security configuration
    pub fn high_security() -> Self {
        Self {
            token_expiry_seconds: 1800, // 30 minutes
            max_concurrent_auth: 500,
            enable_detailed_logging: true,
            rate_limit_per_minute: 50,
            auth_timeout_seconds: 15,
            key_rotation_days: 30,
            enable_audit_logging: true,
            max_failed_attempts: 3,
            lockout_duration_seconds: 1800, // 30 minutes
            ..Default::default()
        }
    }

    /// Create a development configuration
    pub fn development() -> Self {
        Self {
            token_expiry_seconds: 7200, // 2 hours
            max_concurrent_auth: 10000,
            enable_detailed_logging: true,
            rate_limit_per_minute: 1000,
            auth_timeout_seconds: 60,
            key_rotation_days: 365,
            enable_audit_logging: false,
            max_failed_attempts: 10,
            lockout_duration_seconds: 60, // 1 minute
            ..Default::default()
        }
    }
}

/// **Authentication configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Enable password authentication
    pub enable_password_auth: bool,
    /// Enable token authentication
    pub enable_token_auth: bool,
    /// Enable certificate authentication
    pub enable_certificate_auth: bool,
    /// Enable biometric authentication
    pub enable_biometric_auth: bool,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
    /// Minimum password length
    pub min_password_length: usize,
    /// Require password complexity
    pub require_password_complexity: bool,
    /// Password history size
    pub password_history_size: usize,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            enable_password_auth: true,
            enable_token_auth: true,
            enable_certificate_auth: true,
            enable_biometric_auth: false,
            enable_mfa: false,
            min_password_length: 8,
            require_password_complexity: true,
            password_history_size: 5,
        }
    }
}

/// **Encryption configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Supported encryption algorithms
    pub supported_algorithms: Vec<String>,
    /// Key size for symmetric encryption
    pub symmetric_key_size: usize,
    /// Key size for asymmetric encryption
    pub asymmetric_key_size: usize,
    /// Enable key rotation
    pub enable_key_rotation: bool,
    /// Key rotation interval in days
    pub key_rotation_interval_days: u32,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            supported_algorithms: vec![
                "AES-256-GCM".to_string(),
                "AES-128-GCM".to_string(),
                "ChaCha20-Poly1305".to_string(),
            ],
            symmetric_key_size: 256,
            asymmetric_key_size: 2048,
            enable_key_rotation: true,
            key_rotation_interval_days: 90,
        }
    }
}

/// **Signing configuration**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningConfig {
    /// Default signing algorithm
    pub default_algorithm: String,
    /// Supported signing algorithms
    pub supported_algorithms: Vec<String>,
    /// Key size for signing
    pub key_size: usize,
    /// Enable signature verification
    pub enable_verification: bool,
    /// Signature validity duration in seconds
    pub signature_validity_seconds: u64,
}

impl Default for SigningConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "ECDSA-P256".to_string(),
            supported_algorithms: vec![
                "ECDSA-P256".to_string(),
                "ECDSA-P384".to_string(),
                "RSA-PSS-2048".to_string(),
                "Ed25519".to_string(),
            ],
            key_size: 256,
            enable_verification: true,
            signature_validity_seconds: 3600,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation() {
        let config = ZeroCostSecurityConfig::default();
        assert!(config.validate().is_ok());

        let invalid_config = ZeroCostSecurityConfig {
            provider_id: "".to_string(),
            ..Default::default()
        };
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_high_security_config() {
        let config = ZeroCostSecurityConfig::high_security();
        assert_eq!(config.token_expiry_seconds, 1800);
        assert_eq!(config.max_failed_attempts, 3);
        assert!(config.enable_detailed_logging);
    }

    #[test]
    fn test_duration_conversions() {
        let config = ZeroCostSecurityConfig::default();
        assert_eq!(config.token_expiry_duration(), Duration::from_secs(3600));
        assert_eq!(config.auth_timeout_duration(), Duration::from_secs(30));
    }
}
