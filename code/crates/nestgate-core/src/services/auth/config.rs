// **AUTHENTICATION CONFIGURATION**
//! Configuration types and utilities.
// Configuration management for the authentication service.

use crate::config::dynamic_config::DynamicConfigManager;
use std::time::Duration;

/// Authentication configuration
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// Token expiration duration
    pub token_expiration: Duration,
    /// Session timeout
    pub session_timeout: Duration,
    /// Maximum login attempts
    pub max_login_attempts: u32,
    /// Account lockout duration
    pub lockout_duration: Duration,
    /// Password policy
    pub password_policy: PasswordPolicy,
    /// MFA settings
    pub mfa_config: MfaConfig,
    /// OAuth settings
    pub oauth_config: OAuthConfig,
}
impl Default for AuthConfig {
    fn default() -> Self {
        let config_manager = DynamicConfigManager::new("NESTGATE_AUTH");

        Self {
            jwt_secret: config_manager.get_or_default(
                "JWT_SECRET",
                "default-secret-change-in-production".to_string(),
            ),
            token_expiration: config_manager
                .get_or_default("TOKEN_EXPIRATION", Duration::from_secs(3600)), // 1 hour
            session_timeout: config_manager
                .get_or_default("SESSION_TIMEOUT", Duration::from_secs(86400)), // 24 hours
            max_login_attempts: config_manager.get_or_default("MAX_LOGIN_ATTEMPTS", 5),
            lockout_duration: config_manager
                .get_or_default("LOCKOUT_DURATION", Duration::from_secs(900)), // 15 minutes
            password_policy: PasswordPolicy::default(),
            mfa_config: MfaConfig::default(),
            oauth_config: OAuthConfig::default(),
        }
    }
}

/// Password policy configuration
#[derive(Debug, Clone)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_special_chars: bool,
    pub max_age_days: Option<u32>,
    pub history_count: usize,
}
impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            max_age_days: Some(90),
            history_count: 5,
        }
    }
}

/// Multi-Factor Authentication configuration
#[derive(Debug, Clone)]
pub struct MfaConfig {
    pub enabled: bool,
    pub totp_issuer: String,
    pub backup_codes_count: usize,
    pub sms_enabled: bool,
    pub email_enabled: bool,
}
impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            totp_issuer: "NestGate".to_string(),
            backup_codes_count: 10,
            sms_enabled: false, // Requires external SMS service
            email_enabled: true,
        }
    }
}

/// OAuth configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub enabled: bool,
    pub providers: Vec<String>,
    pub callback_url: String,
}
impl Default for OAuthConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Disabled by default for security
            providers: vec!["google".to_string(), "github".to_string()],
            callback_url: crate::constants::canonical_defaults::network::DEFAULT_API_BASE_URL.to_string() + "/auth/callback",
        }
    }
} 