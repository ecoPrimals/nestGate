// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL SECURITY CONFIGURATION TYPES**
//!
//! Authentication, authorization, encryption, TLS, audit,
//! threat detection, and compliance configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityAuthenticationConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityAuthenticationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityAuthentication
pub struct SecurityAuthenticationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Methods
    pub methods: Vec<AuthenticationMethod>,
    /// Session Timeout
    pub session_timeout: Duration,
    /// Max Login Attempts
    pub max_login_attempts: usize,
    /// Lockout Duration
    pub lockout_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authenticationmethod
pub enum AuthenticationMethod {
    /// Password
    Password,
    /// Token
    Token,
    /// Certificate
    Certificate,
    /// Oauth
    OAuth,
    /// Saml
    Saml,
    /// Ldap
    Ldap,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityAuthorizationConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityAuthorizationConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityAuthorization
pub struct SecurityAuthorizationConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Rbac Enabled
    pub rbac_enabled: bool,
    /// Default Permissions
    pub default_permissions: Vec<String>,
    /// Permission Cache Ttl
    pub permission_cache_ttl: Duration,
    /// Audit Authorization
    pub audit_authorization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityEncryptionConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityEncryptionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityEncryption
pub struct SecurityEncryptionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Algorithm
    pub algorithm: String,
    /// Size of key
    pub key_size: usize,
    /// Key Rotation Interval
    pub key_rotation_interval: Duration,
    /// Key Derivation
    pub key_derivation: KeyDerivationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for KeyDerivation
pub struct KeyDerivationConfig {
    /// Algorithm
    pub algorithm: String,
    /// Iterations
    pub iterations: usize,
    /// Size of salt
    pub salt_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityTlsConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityTlsConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityTls
pub struct SecurityTlsConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Version
    pub version: String,
    /// Cipher Suites
    pub cipher_suites: Vec<String>,
    /// Client Auth Required
    pub client_auth_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityAuditConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityAuditConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityAudit
pub struct SecurityAuditConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Log Level
    pub log_level: String,
    /// Log Format
    pub log_format: String,
    /// Retention Days
    pub retention_days: usize,
    /// Events To Audit
    pub events_to_audit: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityThreatDetectionConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityThreatDetectionConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityThreatDetection
pub struct SecurityThreatDetectionConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Ml Enabled
    pub ml_enabled: bool,
    /// Rules Enabled
    pub rules_enabled: bool,
    /// Alert Threshold
    pub alert_threshold: f64,
    /// Response Actions
    pub response_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::config::SecurityComplianceConfig;
///
/// // NEW (canonical):
/// use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::config::SecurityComplianceConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for SecurityCompliance
pub struct SecurityComplianceConfig {
    /// Frameworks
    pub frameworks: Vec<String>,
    /// Reporting Enabled
    pub reporting_enabled: bool,
    /// Automated Checks
    pub automated_checks: bool,
    /// Compliance Level
    pub compliance_level: String,
}
