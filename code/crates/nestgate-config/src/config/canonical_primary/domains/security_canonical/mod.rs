// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **CANONICAL SECURITY CONFIGURATION**
//! Module definitions and exports.
// This module consolidates ALL security configuration variants across the NestGate ecosystem
//! into a single, authoritative configuration structure.
//! Module definitions and exports.
// **CONSOLIDATES**:
//! - nestgate-core/src/config/security.rs → `SecurityConfig`
//! - nestgate-api/src/handlers/auth.rs → `AuthConfig`  
//! - 15+ other security-related configuration structures
//!
//! Module definitions and exports.
// **MODULAR STRUCTURE**:
//! - `authentication`: Authentication configurations (OAuth, SAML, MFA)
//! - `authorization`: Authorization and access control configurations
//! - `tls`: TLS/SSL and certificate management configurations
//! - `encryption`: Encryption and cryptographic configurations
//! - `policies`: Security policies and compliance configurations
//! - `monitoring`: Security monitoring and audit configurations
//! - `threat_protection`: Threat detection and protection configurations
//! - `environment`: Environment-specific security settings

use serde::{Deserialize, Serialize};

// Import all security configuration modules
/// Authentication configuration
pub mod authentication;
/// Authorization configuration
pub mod authorization;
/// Encryption configuration
pub mod encryption;
/// Environment security configuration
pub mod environment;
/// Security monitoring configuration
pub mod monitoring;
/// Security policies configuration
pub mod policies;
/// Threat protection configuration
pub mod threat_protection;
/// TLS configuration
pub mod tls;

// Re-export all configuration types
pub use authentication::{
    AccountLockoutConfig, AuthenticationConfig, AuthenticationMethod, ExternalAuthProvider,
    MfaConfig, PasswordPolicyConfig, SessionConfig, TokenConfig,
};
pub use authorization::{
    AccessControlConfig, AuthorizationConfig, PermissionConfig, PolicyConfig, ResourceConfig,
    RoleConfig,
};
pub use encryption::{
    CryptographicConfig, EncryptionAlgorithmConfig, EncryptionSecurityConfig, HashingConfig,
    KeyManagementConfig,
};
pub use environment::{
    DeploymentSecurityConfig, EnvironmentSecuritySettings, RuntimeSecurityConfig,
    SecurityEnvironmentConfig,
};
pub use monitoring::{
    AlertingConfig, AuditSecurityConfig, IncidentResponseConfig, LoggingConfig,
    SecurityMonitoringConfig,
};
pub use policies::{
    ComplianceConfig, DataProtectionConfig, PrivacyConfig, RetentionPolicyConfig,
    SecurityPoliciesConfig,
};
pub use threat_protection::{
    DdosProtectionConfig, FirewallConfig, IntrusionDetectionConfig, MalwareProtectionConfig,
    ThreatProtectionConfig,
};
pub use tls::{
    CertificateConfig, CertificateManagementConfig, CipherSuiteConfig, SslConfig,
    TlsSecurityConfig, TlsVersionConfig,
};

// ==================== CANONICAL SECURITY CONFIGURATION ====================

// **THE** canonical security configuration for the entire NestGate ecosystem
// This replaces ALL other SecurityConfig variants
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for `CanonicalSecurity`
pub struct CanonicalSecurityConfig {
    /// Authentication configuration
    pub authentication: AuthenticationConfig,

    /// Authorization configuration
    pub authorization: AuthorizationConfig,

    /// TLS/SSL configuration
    pub tls: TlsSecurityConfig,

    /// Certificate management
    pub certificates: CertificateManagementConfig,

    /// Access control configuration
    pub access_control: AccessControlConfig,

    /// Security policies
    pub policies: SecurityPoliciesConfig,

    /// Audit and compliance configuration
    pub audit: AuditSecurityConfig,

    /// Threat protection configuration
    pub threat_protection: ThreatProtectionConfig,

    /// Encryption configuration
    pub encryption: EncryptionSecurityConfig,

    /// Security monitoring
    pub monitoring: SecurityMonitoringConfig,

    /// Environment-specific security settings
    pub environment: SecurityEnvironmentConfig,
}

impl CanonicalSecurityConfig {
    /// Create a new canonical security configuration
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for production environments
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            authentication: AuthenticationConfig::production_hardened(),
            authorization: AuthorizationConfig::production_hardened(),
            tls: TlsSecurityConfig::production_hardened(),
            certificates: CertificateManagementConfig::production_hardened(),
            access_control: AccessControlConfig::production_hardened(),
            policies: SecurityPoliciesConfig::production_hardened(),
            audit: AuditSecurityConfig::production_hardened(),
            threat_protection: ThreatProtectionConfig::production_hardened(),
            encryption: EncryptionSecurityConfig::production_hardened(),
            monitoring: SecurityMonitoringConfig::production_hardened(),
            environment: SecurityEnvironmentConfig::production_hardened(),
        }
    }

    /// Create a configuration optimized for development environments
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            authentication: AuthenticationConfig::development_optimized(),
            authorization: AuthorizationConfig::development_optimized(),
            tls: TlsSecurityConfig::development_optimized(),
            certificates: CertificateManagementConfig::development_optimized(),
            access_control: AccessControlConfig::development_optimized(),
            policies: SecurityPoliciesConfig::development_optimized(),
            audit: AuditSecurityConfig::development_optimized(),
            threat_protection: ThreatProtectionConfig::development_optimized(),
            encryption: EncryptionSecurityConfig::development_optimized(),
            monitoring: SecurityMonitoringConfig::development_optimized(),
            environment: SecurityEnvironmentConfig::development_optimized(),
        }
    }

    /// Create a configuration for compliance-focused environments (SOC2, GDPR, etc.)
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self {
            authentication: AuthenticationConfig::compliance_focused(),
            authorization: AuthorizationConfig::compliance_focused(),
            tls: TlsSecurityConfig::compliance_focused(),
            certificates: CertificateManagementConfig::compliance_focused(),
            access_control: AccessControlConfig::compliance_focused(),
            policies: SecurityPoliciesConfig::compliance_focused(),
            audit: AuditSecurityConfig::compliance_focused(),
            threat_protection: ThreatProtectionConfig::compliance_focused(),
            encryption: EncryptionSecurityConfig::compliance_focused(),
            monitoring: SecurityMonitoringConfig::compliance_focused(),
            environment: SecurityEnvironmentConfig::compliance_focused(),
        }
    }

    /// Merge with another configuration (other takes precedence)
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.authentication = self.authentication.merge(other.authentication);
        self.authorization = self.authorization.merge(other.authorization);
        self.tls = self.tls.merge(other.tls);
        self.certificates = self.certificates.merge(other.certificates);
        self.access_control = self.access_control.merge(other.access_control);
        self.policies = self.policies.merge(other.policies);
        self.audit = self.audit.merge(other.audit);
        self.threat_protection = self.threat_protection.merge(other.threat_protection);
        self.encryption = self.encryption.merge(other.encryption);
        self.monitoring = self.monitoring.merge(other.monitoring);
        self.environment = self.environment.merge(other.environment);
        self
    }

    /// Validate the security configuration for completeness and security best practices
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        // Validate authentication configuration
        self.authentication.validate()?;

        // Validate authorization configuration
        self.authorization.validate()?;

        // Validate TLS configuration
        self.tls.validate()?;

        // Validate encryption configuration
        self.encryption.validate()?;

        // Validate policies configuration
        self.policies.validate()?;

        // Validate monitoring configuration
        self.monitoring.validate()?;

        Ok(())
    }
}

// ==================== BACKWARD COMPATIBILITY ALIASES ====================

/// Backward compatibility alias for existing `SecurityConfig` usage
pub type SecurityConfig = CanonicalSecurityConfig;

/// Backward compatibility alias for `UnifiedSecurityConfig`
pub type UnifiedSecurityConfig = CanonicalSecurityConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_security_config_new() {
        let config = CanonicalSecurityConfig::new();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_canonical_security_config_merge() {
        let a = CanonicalSecurityConfig::default();
        let b = CanonicalSecurityConfig::default();
        let _merged = a.merge(b);
    }
}
