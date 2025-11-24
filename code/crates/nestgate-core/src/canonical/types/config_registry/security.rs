//! **CANONICAL SECURITY CONFIGURATION TYPES**
//!
//! Authentication, authorization, encryption, TLS, audit,
//! threat detection, and compliance configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityAuthenticationConfig {
    pub enabled: bool,
    pub methods: Vec<AuthenticationMethod>,
    pub session_timeout: Duration,
    pub max_login_attempts: usize,
    pub lockout_duration: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    Password,
    Token,
    Certificate,
    OAuth,
    Saml,
    Ldap,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityAuthorizationConfig {
    pub enabled: bool,
    pub rbac_enabled: bool,
    pub default_permissions: Vec<String>,
    pub permission_cache_ttl: Duration,
    pub audit_authorization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityEncryptionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub key_size: usize,
    pub key_rotation_interval: Duration,
    pub key_derivation: KeyDerivationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KeyDerivationConfig {
    pub algorithm: String,
    pub iterations: usize,
    pub salt_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityTlsConfig {
    pub enabled: bool,
    pub version: String,
    pub cipher_suites: Vec<String>,
    pub client_auth_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityAuditConfig {
    pub enabled: bool,
    pub log_level: String,
    pub log_format: String,
    pub retention_days: usize,
    pub events_to_audit: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityThreatDetectionConfig {
    pub enabled: bool,
    pub ml_enabled: bool,
    pub rules_enabled: bool,
    pub alert_threshold: f64,
    pub response_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
#[deprecated(
    since = "0.11.0",
    note = "Use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
pub struct SecurityComplianceConfig {
    pub frameworks: Vec<String>,
    pub reporting_enabled: bool,
    pub automated_checks: bool,
    pub compliance_level: String,
}
