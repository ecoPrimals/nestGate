/// Security Configuration Domain
///
/// Replaces: SecurityConfig, AuthConfig, TlsConfig, KeyManagementConfig,
/// AccessControlConfig, and 6+ other security config structures
use super::CanonicalDomainConfig;
use crate::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL SECURITY CONFIGURATION**
/// Replaces: SecurityConfig, AuthConfig, TlsConfig, KeyManagementConfig,
/// AccessControlConfig, and 6+ other security config structures
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalSecurityConfig {
    /// Authentication settings
    pub authentication: SecurityAuthentication,
    /// Authorization settings
    pub authorization: SecurityAuthorization,
    /// Encryption settings
    pub encryption: SecurityEncryption,
    /// Key management settings
    pub key_management: SecurityKeyManagement,
    /// Access control settings
    pub access_control: SecurityAccessControl,
    /// Audit settings
    pub audit: SecurityAudit,
    /// Environment-specific overrides
    pub environment_overrides: HashMap<String, serde_json::Value>,
}

impl CanonicalDomainConfig for CanonicalSecurityConfig {
    fn domain() -> &'static str {
        "security"
    }

    fn validate(&self) -> Result<()> {
        if self.key_management.rotation_interval.as_secs() < 3600 {
            return Err(NestGateError::config_error(
                "key_management.rotation_interval",
                "must be at least 1 hour",
            ));
        }
        Ok(())
    }

    fn merge(mut self, other: Self) -> Self {
        self.environment_overrides
            .extend(other.environment_overrides);
        self
    }

    fn from_environment() -> Result<Self> {
        Ok(Self::default())
    }

    fn schema() -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "authentication": {"type": "object", "description": "Authentication settings"},
                "encryption": {"type": "object", "description": "Encryption settings"}
            }
        })
    }
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuthentication {
    pub method: AuthMethod,
    pub token_lifetime: Duration,
    pub multi_factor_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuthorization {
    pub rbac_enabled: bool,
    pub default_permissions: Vec<String>,
    pub admin_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEncryption {
    pub algorithm: EncryptionAlgorithm,
    pub cipher_mode: CipherMode,
    pub key_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityKeyManagement {
    pub rotation_interval: Duration,
    pub backup_enabled: bool,
    pub hsm_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAccessControl {
    pub rate_limiting: bool,
    pub max_requests_per_minute: u32,
    pub ip_whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAudit {
    pub logging_enabled: bool,
    pub log_level: String,
    pub retention_days: u32,
}

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    Basic,
    Bearer,
    OAuth2,
    Jwt,
    ApiKey,
    Mutual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    Aes256,
    ChaCha20,
    Rsa2048,
    Rsa4096,
    Ed25519,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CipherMode {
    Gcm,
    Cbc,
    Ctr,
    Ecb,
}

// Default implementations
impl Default for SecurityAuthentication {
    fn default() -> Self {
        Self {
            method: AuthMethod::Jwt,
            token_lifetime: Duration::from_secs(3600), // 1 hour
            multi_factor_enabled: false,
        }
    }
}

impl Default for SecurityAuthorization {
    fn default() -> Self {
        Self {
            rbac_enabled: true,
            default_permissions: vec!["read".to_string()],
            admin_roles: vec!["admin".to_string(), "root".to_string()],
        }
    }
}

impl Default for SecurityEncryption {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256,
            cipher_mode: CipherMode::Gcm,
            key_size: 256,
        }
    }
}

impl Default for SecurityKeyManagement {
    fn default() -> Self {
        Self {
            rotation_interval: Duration::from_secs(86400), // 24 hours
            backup_enabled: true,
            hsm_enabled: false,
        }
    }
}

impl Default for SecurityAccessControl {
    fn default() -> Self {
        Self {
            rate_limiting: true,
            max_requests_per_minute: 1000,
            ip_whitelist: Vec::new(),
        }
    }
}

impl Default for SecurityAudit {
    fn default() -> Self {
        Self {
            logging_enabled: true,
            log_level: "INFO".to_string(),
            retention_days: 30,
        }
    }
}
