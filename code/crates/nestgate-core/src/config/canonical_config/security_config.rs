//
// Security-related configuration including authentication, authorization,
// encryption, and security policies.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Security configuration (consolidates 20+ security configs)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityConfig {
    /// Authentication configuration
    pub authentication: AuthenticationConfig,
    /// Authorization configuration  
    pub authorization: AuthorizationConfig,
    /// Encryption configuration
    pub encryption: EncryptionConfig,
    /// Security policies
    pub policies: SecurityPoliciesConfig,
}
/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Authentication method
    pub method: String,
    /// Token expiry
    pub token_expiry: Duration,
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
}
/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// Authorization method
    pub method: String,
    /// Default permissions
    pub default_permissions: Vec<String>,
}
/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Encryption algorithm
    pub algorithm: String,
    /// Key size
    pub key_size: u32,
}
/// Security policies configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct SecurityPoliciesConfig {
    /// Password policy
    pub password_policy: PasswordPolicyConfig,
    /// Session policy
    pub session_policy: SessionPolicyConfig,
}
/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicyConfig {
    /// Minimum length
    pub min_length: u32,
    /// Require uppercase
    pub require_uppercase: bool,
    /// Require numbers
    pub require_numbers: bool,
}
/// Session policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPolicyConfig {
    /// Session timeout
    pub timeout: Duration,
    /// Maximum concurrent sessions
    pub max_concurrent: u32,
}

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            method: "jwt".to_string(),
            token_expiry: Duration::from_secs(3600),
            enable_mfa: false,
        }
    }
}

impl Default for AuthorizationConfig {
    fn default() -> Self {
        Self {
            method: "rbac".to_string(),
            default_permissions: vec!["read".to_string()],
        }
    }
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            algorithm: "aes-256-gcm".to_string(),
            key_size: 256,
        }
    }
}


impl Default for PasswordPolicyConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_numbers: true,
        }
    }
}

impl Default for SessionPolicyConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(1800),
            max_concurrent: 5,
        }
    }
} 