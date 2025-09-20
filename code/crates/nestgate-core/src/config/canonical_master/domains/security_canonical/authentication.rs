// **AUTHENTICATION CONFIGURATION**
//! Authentication functionality and utilities.
// Comprehensive authentication configurations for the NestGate ecosystem,
//! including OAuth, SAML, MFA, session management, and external providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ==================== AUTHENTICATION CONFIGURATION ====================

/// Comprehensive authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Primary authentication method
    pub primary_method: AuthenticationMethod,

    /// Secondary authentication methods
    pub secondary_methods: Vec<AuthenticationMethod>,

    /// Multi-factor authentication
    pub mfa: MfaConfig,

    /// Session management
    pub session: SessionConfig,

    /// Token configuration
    pub tokens: TokenConfig,

    /// Password policies
    pub password_policy: PasswordPolicyConfig,

    /// Account lockout policies
    pub lockout: AccountLockoutConfig,

    /// External authentication providers
    pub external_providers: Vec<ExternalAuthProvider>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// Username/password authentication
    UsernamePassword,
    /// Certificate-based authentication
    Certificate,
    /// Token-based authentication (JWT, API keys)
    Token,
    /// OAuth 2.0 / `OpenID` Connect
    OAuth2,
    /// SAML authentication
    Saml,
    /// LDAP authentication
    Ldap,
    /// Biometric authentication
    Biometric,
    /// Hardware token authentication
    HardwareToken,
    /// Custom authentication method
    Custom(String),
}

// ==================== MULTI-FACTOR AUTHENTICATION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    /// Enable multi-factor authentication
    pub enabled: bool,

    /// Required MFA methods
    pub required_methods: Vec<MfaMethod>,

    /// Optional MFA methods
    pub optional_methods: Vec<MfaMethod>,

    /// MFA timeout settings
    pub timeout: Duration,

    /// Backup codes configuration
    pub backup_codes: BackupCodesConfig,

    /// Remember device settings
    pub remember_device: RememberDeviceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MfaMethod {
    /// SMS-based OTP
    Sms,
    /// Email-based OTP
    Email,
    /// TOTP (Time-based One-Time Password)
    Totp,
    /// HOTP (HMAC-based One-Time Password)
    Hotp,
    /// Push notification
    Push,
    /// Hardware token (`YubiKey`, etc.)
    HardwareToken,
    /// Biometric verification
    Biometric,
    /// Backup codes
    BackupCodes,
    /// Custom MFA method
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupCodesConfig {
    /// Enable backup codes
    pub enabled: bool,

    /// Number of backup codes to generate
    pub count: u32,

    /// Length of each backup code
    pub length: u32,

    /// Auto-regenerate after use
    pub auto_regenerate: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RememberDeviceConfig {
    /// Enable remember device functionality
    pub enabled: bool,

    /// Duration to remember device
    pub duration: Duration,

    /// Maximum remembered devices per user
    pub max_devices: u32,
}

// ==================== SESSION MANAGEMENT ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    /// Session timeout duration
    pub timeout: Duration,

    /// Idle timeout duration
    pub idle_timeout: Duration,

    /// Maximum concurrent sessions per user
    pub max_concurrent_sessions: u32,

    /// Session storage configuration
    pub storage: SessionStorageConfig,

    /// Session security settings
    pub security: SessionSecurityConfig,

    /// Session refresh settings
    pub refresh: SessionRefreshConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStorageConfig {
    /// Session storage type
    pub storage_type: SessionStorageType,

    /// Storage configuration
    pub config: HashMap<String, String>,

    /// Encryption settings
    pub encryption: SessionEncryptionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStorageType {
    /// In-memory storage
    Memory,
    /// Redis storage
    Redis,
    /// Database storage
    Database,
    /// File-based storage
    File,
    /// Custom storage
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionEncryptionConfig {
    /// Enable session encryption
    pub enabled: bool,

    /// Encryption algorithm
    pub algorithm: String,

    /// Key rotation settings
    pub key_rotation: KeyRotationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationConfig {
    /// Enable automatic key rotation
    pub enabled: bool,

    /// Key rotation interval
    pub interval: Duration,

    /// Number of old keys to keep
    pub keep_old_keys: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSecurityConfig {
    /// Secure cookie settings
    pub secure_cookies: bool,

    /// HTTP-only cookies
    pub http_only: bool,

    /// `SameSite` cookie policy
    pub same_site: SameSitePolicy,

    /// Session fixation protection
    pub fixation_protection: bool,

    /// IP address validation
    pub ip_validation: bool,

    /// User agent validation
    pub user_agent_validation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRefreshConfig {
    /// Enable automatic session refresh
    pub enabled: bool,

    /// Refresh threshold (percentage of session lifetime)
    pub threshold: f64,

    /// Refresh window duration
    pub window: Duration,
}

// ==================== TOKEN CONFIGURATION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenConfig {
    /// JWT configuration
    pub jwt: JwtConfig,

    /// API key configuration
    pub api_keys: ApiKeyConfig,

    /// Refresh token configuration
    pub refresh_tokens: RefreshTokenConfig,

    /// Access token configuration
    pub access_tokens: AccessTokenConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    /// JWT signing algorithm
    pub algorithm: JwtAlgorithm,

    /// JWT secret or key
    pub secret: String,

    /// JWT expiration time
    pub expiration: Duration,

    /// JWT issuer
    pub issuer: String,

    /// JWT audience
    pub audience: Vec<String>,

    /// Custom claims
    pub custom_claims: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JwtAlgorithm {
    HS256,
    HS384,
    HS512,
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    ES512,
    PS256,
    PS384,
    PS512,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Enable API key authentication
    pub enabled: bool,

    /// API key length
    pub key_length: u32,

    /// API key prefix
    pub prefix: String,

    /// API key expiration
    pub expiration: Option<Duration>,

    /// Rate limiting per API key
    pub rate_limit: Option<RateLimitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per window
    pub requests: u32,

    /// Time window
    pub window: Duration,

    /// Burst allowance
    pub burst: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshTokenConfig {
    /// Enable refresh tokens
    pub enabled: bool,

    /// Refresh token lifetime
    pub lifetime: Duration,

    /// Refresh token rotation
    pub rotation: bool,

    /// Maximum refresh token age
    pub max_age: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenConfig {
    /// Access token lifetime
    pub lifetime: Duration,

    /// Token type
    pub token_type: String,

    /// Scope configuration
    pub scopes: Vec<String>,
}

// ==================== PASSWORD POLICIES ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicyConfig {
    /// Minimum password length
    pub min_length: u32,

    /// Maximum password length
    pub max_length: u32,

    /// Require uppercase letters
    pub require_uppercase: bool,

    /// Require lowercase letters
    pub require_lowercase: bool,

    /// Require numbers
    pub require_numbers: bool,

    /// Require special characters
    pub require_special_chars: bool,

    /// Allowed special characters
    pub allowed_special_chars: String,

    /// Password history count
    pub history_count: u32,

    /// Password expiration
    pub expiration: Option<Duration>,

    /// Common password blacklist
    pub blacklist: Vec<String>,

    /// Dictionary check
    pub dictionary_check: bool,
}

// ==================== ACCOUNT LOCKOUT ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountLockoutConfig {
    /// Enable account lockout
    pub enabled: bool,

    /// Maximum failed attempts before lockout
    pub max_attempts: u32,

    /// Lockout duration
    pub lockout_duration: Duration,

    /// Reset attempt counter after duration
    pub reset_duration: Duration,

    /// Progressive lockout (increasing duration)
    pub progressive_lockout: bool,

    /// Lockout escalation multiplier
    pub escalation_multiplier: f64,
}

// ==================== EXTERNAL PROVIDERS ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalAuthProvider {
    /// Provider name
    pub name: String,

    /// Provider type
    pub provider_type: ExternalProviderType,

    /// Provider configuration
    pub config: HashMap<String, String>,

    /// Enabled status
    pub enabled: bool,

    /// Priority order
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExternalProviderType {
    OAuth2,
    Saml,
    Ldap,
    ActiveDirectory,
    Google,
    Microsoft,
    GitHub,
    Okta,
    Auth0,
    Custom(String),
}

// ==================== DEFAULT IMPLEMENTATIONS ====================

impl Default for AuthenticationConfig {
    fn default() -> Self {
        Self {
            primary_method: AuthenticationMethod::UsernamePassword,
            secondary_methods: vec![],
            mfa: MfaConfig::default(),
            session: SessionConfig::default(),
            tokens: TokenConfig::default(),
            password_policy: PasswordPolicyConfig::default(),
            lockout: AccountLockoutConfig::default(),
            external_providers: vec![],
        }
    }
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            required_methods: vec![],
            optional_methods: vec![MfaMethod::Totp, MfaMethod::Email],
            timeout: Duration::from_secs(300),
            backup_codes: BackupCodesConfig::default(),
            remember_device: RememberDeviceConfig::default(),
        }
    }
}

impl Default for BackupCodesConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            count: 10,
            length: 8,
            auto_regenerate: false,
        }
    }
}

impl Default for RememberDeviceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            duration: Duration::from_secs(30 * 24 * 60 * 60), // 30 days
            max_devices: 5,
        }
    }
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(24 * 60 * 60),     // 24 hours
            idle_timeout: Duration::from_secs(2 * 60 * 60), // 2 hours
            max_concurrent_sessions: 3,
            storage: SessionStorageConfig::default(),
            security: SessionSecurityConfig::default(),
            refresh: SessionRefreshConfig::default(),
        }
    }
}

impl Default for SessionStorageConfig {
    fn default() -> Self {
        Self {
            storage_type: SessionStorageType::Memory,
            config: HashMap::new(),
            encryption: SessionEncryptionConfig::default(),
        }
    }
}

impl Default for SessionEncryptionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "AES-256-GCM".to_string(),
            key_rotation: KeyRotationConfig::default(),
        }
    }
}

impl Default for KeyRotationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            keep_old_keys: 3,
        }
    }
}

impl Default for SessionSecurityConfig {
    fn default() -> Self {
        Self {
            secure_cookies: true,
            http_only: true,
            same_site: SameSitePolicy::Strict,
            fixation_protection: true,
            ip_validation: false,
            user_agent_validation: false,
        }
    }
}

impl Default for SessionRefreshConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            threshold: 0.8, // Refresh when 80% of session lifetime has passed
            window: Duration::from_secs(5 * 60), // 5 minutes
        }
    }
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            algorithm: JwtAlgorithm::HS256,
            secret: "change-me-in-production".to_string(),
            expiration: Duration::from_secs(15 * 60), // 15 minutes
            issuer: "nestgate".to_string(),
            audience: vec!["nestgate-api".to_string()],
            custom_claims: HashMap::new(),
        }
    }
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            key_length: 32,
            prefix: "nk_".to_string(),
            expiration: None,
            rate_limit: Some(RateLimitConfig::default()),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests: 1000,
            window: Duration::from_secs(60 * 60), // 1 hour
            burst: 100,
        }
    }
}

impl Default for RefreshTokenConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            lifetime: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
            rotation: true,
            max_age: Duration::from_secs(30 * 24 * 60 * 60), // 30 days
        }
    }
}

impl Default for AccessTokenConfig {
    fn default() -> Self {
        Self {
            lifetime: Duration::from_secs(15 * 60), // 15 minutes
            token_type: "Bearer".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
        }
    }
}

impl Default for PasswordPolicyConfig {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_special_chars: true,
            allowed_special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
            history_count: 5,
            expiration: Some(Duration::from_secs(90 * 24 * 60 * 60)), // 90 days
            blacklist: vec![],
            dictionary_check: true,
        }
    }
}

impl Default for AccountLockoutConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 5,
            lockout_duration: Duration::from_secs(15 * 60), // 15 minutes
            reset_duration: Duration::from_secs(60 * 60),   // 1 hour
            progressive_lockout: true,
            escalation_multiplier: 2.0,
        }
    }
}

// ==================== BUILDER METHODS ====================

impl AuthenticationConfig {
    /// Create a configuration optimized for production environments
    #[must_use]
    pub const fn production_hardened() -> Self {
        Self {
            primary_method: AuthenticationMethod::OAuth2,
            secondary_methods: vec![AuthenticationMethod::Certificate],
            mfa: MfaConfig {
                enabled: true,
                required_methods: vec![MfaMethod::Totp, MfaMethod::Push],
                optional_methods: vec![MfaMethod::BackupCodes],
                timeout: Duration::from_secs(180), // 3 minutes
                backup_codes: BackupCodesConfig {
                    enabled: true,
                    count: 10,
                    length: 12,
                    auto_regenerate: true,
                },
                remember_device: RememberDeviceConfig {
                    enabled: true,
                    duration: Duration::from_secs(7 * 24 * 60 * 60), // 7 days
                    max_devices: 3,
                },
            },
            password_policy: PasswordPolicyConfig {
                min_length: 12,
                max_length: 256,
                require_uppercase: true,
                require_lowercase: true,
                require_numbers: true,
                require_special_chars: true,
                allowed_special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
                history_count: 10,
                expiration: Some(Duration::from_secs(60 * 24 * 60 * 60)), // 60 days
                blacklist: vec![], // Would be populated from external source
                dictionary_check: true,
            },
            lockout: AccountLockoutConfig {
                enabled: true,
                max_attempts: 3,
                lockout_duration: Duration::from_secs(30 * 60), // 30 minutes
                reset_duration: Duration::from_secs(2 * 60 * 60), // 2 hours
                progressive_lockout: true,
                escalation_multiplier: 3.0,
            },
            ..Default::default()
        }
    }

    /// Create a configuration optimized for development environments
    #[must_use]
    pub const fn development_optimized() -> Self {
        Self {
            primary_method: AuthenticationMethod::UsernamePassword,
            secondary_methods: vec![],
            mfa: MfaConfig {
                enabled: false,
                ..Default::default()
            },
            password_policy: PasswordPolicyConfig {
                min_length: 6,
                max_length: 128,
                require_uppercase: false,
                require_lowercase: false,
                require_numbers: false,
                require_special_chars: false,
                allowed_special_chars: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
                history_count: 1,
                expiration: None,
                blacklist: vec![],
                dictionary_check: false,
            },
            lockout: AccountLockoutConfig {
                enabled: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create a configuration for compliance-focused environments
    #[must_use]
    pub const fn compliance_focused() -> Self {
        Self::production_hardened() // Start with production hardened
                                    // Add compliance-specific overrides here
    }

    /// Merge with another configuration
    #[must_use]
    pub const fn merge(self, _other: Self) -> Self {
        // Simple merge - in a real implementation, you'd want more sophisticated merging
        self
    }

    /// Validate the authentication configuration
    pub const fn validate(&self) -> crate::Result<()> {
        // Validate password policy
        if self.password_policy.min_length > self.password_policy.max_length {
            return Err(crate::NestGateError::validation_error(
                "Password policy: min_length cannot be greater than max_length",
            ));
        }

        // Validate MFA configuration
        if self.mfa.enabled && self.mfa.required_methods.is_empty() {
            return Err(crate::NestGateError::validation_error(
                "MFA is enabled but no required methods are configured",
            ));
        }

        // Validate session configuration
        if self.session.timeout < Duration::from_secs(60) {
            return Err(crate::NestGateError::validation_error(
                "Session timeout cannot be less than 1 minute",
            ));
        }

        Ok(())
    }
}
