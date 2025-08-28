use std::collections::HashMap;
///
/// This module contains all security-related configuration types including authentication,
/// encryption, access control, certificates, and compliance settings.
/// Split from unified_types/mod.rs for better maintainability and 2000-line compliance.
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Import timeout config from the existing module
use super::timeout_config::UnifiedTimeoutConfig;

// ==================== SECTION ====================

/// Unified Security Configuration - consolidates all security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedSecurityConfig {
    /// Enable security features
    pub enabled: bool,
    /// Authentication configuration
    pub auth_config: AuthConfig,
    /// Encryption settings
    pub encryption: EncryptionConfig,
    /// Access control settings
    pub access_control: AccessControlConfig,
    /// Security audit settings
    pub audit_config: AuditConfig,
    /// Certificate management
    pub cert_config: CertificateConfig,
    /// Security timeout overrides
    pub security_timeouts: UnifiedTimeoutConfig,
    /// Rate limiting for security
    pub rate_limiting: SecurityRateLimitConfig,
    /// Intrusion detection settings
    pub intrusion_detection: IntrusionDetectionConfig,
    /// Security logging
    pub security_logging: SecurityLoggingConfig,
    /// Compliance settings
    pub compliance: ComplianceConfig,
}

impl Default for UnifiedSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auth_config: AuthConfig::default(),
            encryption: EncryptionConfig::default(),
            access_control: AccessControlConfig::default(),
            audit_config: AuditConfig::default(),
            cert_config: CertificateConfig::default(),
            security_timeouts: UnifiedTimeoutConfig::default(),
            rate_limiting: SecurityRateLimitConfig::default(),
            intrusion_detection: IntrusionDetectionConfig::default(),
            security_logging: SecurityLoggingConfig::default(),
            compliance: ComplianceConfig::default(),
        }
    }
}

// ==================== SECTION ====================

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub require_auth: bool,
    pub auth_methods: Vec<AuthMethod>,
    pub session_timeout: Duration,
    pub max_login_attempts: u32,
    pub lockout_duration: Duration,
    pub multi_factor_auth: MfaConfig,
    pub single_sign_on: SsoConfig,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            require_auth: true,
            auth_methods: vec![AuthMethod::Password],
            session_timeout: Duration::from_secs(3600), // 1 hour
            max_login_attempts: 5,
            lockout_duration: Duration::from_secs(300), // 5 minutes
            multi_factor_auth: MfaConfig::default(),
            single_sign_on: SsoConfig::default(),
        }
    }
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthMethod {
    Password,
    Certificate,
    Token,
    OAuth,
    Ldap,
    Kerberos,
    Custom(String),
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: u32,
    pub require_uppercase: bool,
    pub require_lowercase: bool,
    pub require_numbers: bool,
    pub require_symbols: bool,
    pub max_age_days: Option<u32>,
    pub history_count: u32,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            require_uppercase: true,
            require_lowercase: true,
            require_numbers: true,
            require_symbols: false,
            max_age_days: Some(90),
            history_count: 5,
        }
    }
}

/// Multi-factor authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfig {
    pub enabled: bool,
    pub required_for_admin: bool,
    pub methods: Vec<MfaMethod>,
    pub backup_codes: bool,
}

impl Default for MfaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            required_for_admin: true,
            methods: vec![MfaMethod::Totp],
            backup_codes: true,
        }
    }
}

/// Multi-factor authentication methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MfaMethod {
    Totp, // Time-based One-Time Password
    Sms,
    Email,
    Hardware,
    Biometric,
}

/// Single sign-on configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SsoConfig {
    pub enabled: bool,
    pub provider: SsoProvider,
    pub auto_provision: bool,
    pub attribute_mapping: std::collections::HashMap<String, String>,
}

impl Default for SsoConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: SsoProvider::Saml,
            auto_provision: false,
            attribute_mapping: std::collections::HashMap::new(),
        }
    }
}

/// Single sign-on providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SsoProvider {
    Saml,
    Oidc,
    Oauth2,
    Ldap,
    ActiveDirectory,
    Custom(String),
}

// ==================== SECTION ====================

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub enable_tls: bool,
    pub tls_version: TlsVersion,
    pub cipher_suites: Vec<String>,
    pub certificate_path: Option<String>,
    pub private_key_path: Option<String>,
    pub ca_certificate_path: Option<String>,
    pub verify_certificates: bool,
    pub encryption_at_rest: bool,
    pub key_rotation_interval: Duration,
}

impl Default for EncryptionConfig {
    fn default() -> Self {
        Self {
            enable_tls: true,
            tls_version: TlsVersion::V1_3,
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
            ],
            certificate_path: None,
            private_key_path: None,
            ca_certificate_path: None,
            verify_certificates: true,
            encryption_at_rest: false,
            key_rotation_interval: Duration::from_secs(86400 * 30), // 30 days
        }
    }
}

/// TLS versions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TlsVersion {
    V1_2,
    V1_3,
}

// ==================== SECTION ====================

/// Access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub enabled: bool,
    pub role_based_access: bool,
    pub attribute_based_access: bool,
    pub resource_permissions: std::collections::HashMap<String, Vec<Permission>>,
    pub role_definitions: std::collections::HashMap<String, Role>,
}

impl Default for AccessControlConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            role_based_access: true,
            attribute_based_access: false,
            resource_permissions: std::collections::HashMap::new(),
            role_definitions: std::collections::HashMap::new(),
        }
    }
}

/// Access policies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AccessPolicy {
    Allow,
    Deny,
    Conditional,
}

/// Permissions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Delete,
    Admin,
    Custom(String),
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub inherits_from: Vec<String>,
}

// ==================== SECTION ====================

/// Security audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    pub enabled: bool,
    pub audit_level: AuditLevel,
    pub log_successful_auth: bool,
    pub log_failed_auth: bool,
    pub log_privilege_escalation: bool,
    pub log_data_access: bool,
    pub retention_days: u32,
    pub audit_storage_path: String,
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            audit_level: AuditLevel::Standard,
            log_successful_auth: false,
            log_failed_auth: true,
            log_privilege_escalation: true,
            log_data_access: false,
            retention_days: 90,
            audit_storage_path: "/var/log/nestgate/audit".to_string(),
        }
    }
}

/// Audit levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditLevel {
    Minimal,
    Standard,
    Verbose,
    Debug,
}

// ==================== SECTION ====================

/// Certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateConfig {
    pub auto_renewal: bool,
    pub renewal_threshold_days: u32,
    pub certificate_authority: CertificateAuthority,
    pub key_algorithm: KeyAlgorithm,
    pub key_size: u32,
    pub certificate_lifetime_days: u32,
}

impl Default for CertificateConfig {
    fn default() -> Self {
        Self {
            auto_renewal: true,
            renewal_threshold_days: 30,
            certificate_authority: CertificateAuthority::LetsEncrypt,
            key_algorithm: KeyAlgorithm::Rsa,
            key_size: 2048,
            certificate_lifetime_days: 365,
        }
    }
}

/// Certificate authorities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificateAuthority {
    LetsEncrypt,
    SelfSigned,
    Internal,
    External(String),
}

/// Key algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyAlgorithm {
    Rsa,
    Ecdsa,
    Ed25519,
}

// ==================== SECTION ====================

/// Security rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRateLimitConfig {
    pub enabled: bool,
    pub login_attempts_per_minute: u32,
    pub api_requests_per_minute: u32,
    pub password_reset_per_hour: u32,
    pub account_creation_per_hour: u32,
}

impl Default for SecurityRateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            login_attempts_per_minute: 5,
            api_requests_per_minute: 100,
            password_reset_per_hour: 3,
            account_creation_per_hour: 10,
        }
    }
}

// ==================== SECTION ====================

/// Intrusion detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrusionDetectionConfig {
    pub enabled: bool,
    pub suspicious_activity_threshold: u32,
    pub block_suspicious_ips: bool,
    pub alert_on_multiple_failures: bool,
    pub geo_blocking: GeoBlockingConfig,
}

impl Default for IntrusionDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            suspicious_activity_threshold: 10,
            block_suspicious_ips: true,
            alert_on_multiple_failures: true,
            geo_blocking: GeoBlockingConfig::default(),
        }
    }
}

/// Geographic blocking configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeoBlockingConfig {
    pub enabled: bool,
    pub blocked_countries: Vec<String>,
    pub allowed_countries: Vec<String>,
    pub block_unknown_locations: bool,
}

// ==================== SECTION ====================

/// Security logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityLoggingConfig {
    pub enabled: bool,
    pub log_level: SecurityLogLevel,
    pub log_format: SecurityLogFormat,
    pub log_destination: SecurityLogDestination,
    pub include_request_details: bool,
    pub include_response_details: bool,
}

impl Default for SecurityLoggingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: SecurityLogLevel::Info,
            log_format: SecurityLogFormat::Json,
            log_destination: SecurityLogDestination::File(
                "/var/log/nestgate/security.log".to_string(),
            ),
            include_request_details: true,
            include_response_details: false,
        }
    }
}

/// Security log levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Security log formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SecurityLogFormat {
    Json,
    Text,
    Structured,
}

/// Security log destinations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLogDestination {
    File(String),
    Syslog,
    Remote(String),
    Database,
}

// ==================== SECTION ====================

/// Compliance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceConfig {
    pub gdpr_compliance: bool,
    pub hipaa_compliance: bool,
    pub sox_compliance: bool,
    pub pci_compliance: bool,
    pub data_retention_days: u32,
    pub data_anonymization: bool,
    pub privacy_controls: PrivacyControls,
}

impl Default for ComplianceConfig {
    fn default() -> Self {
        Self {
            gdpr_compliance: false,
            hipaa_compliance: false,
            sox_compliance: false,
            pci_compliance: false,
            data_retention_days: 365,
            data_anonymization: false,
            privacy_controls: PrivacyControls::default(),
        }
    }
}

/// Privacy controls
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyControls {
    pub data_minimization: bool,
    pub purpose_limitation: bool,
    pub consent_management: bool,
    pub right_to_erasure: bool,
    pub data_portability: bool,
}

impl Default for PrivacyControls {
    fn default() -> Self {
        Self {
            data_minimization: true,
            purpose_limitation: true,
            consent_management: false,
            right_to_erasure: false,
            data_portability: false,
        }
    }
}
