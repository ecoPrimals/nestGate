//! **CANONICAL SECURITY CONFIGURATION**
//!
//! This module consolidates ALL security configuration variants across the NestGate ecosystem
//! into a single, authoritative configuration structure.
//!
//! **CONSOLIDATES**:
//! - nestgate-canonical/src/types.rs → SecurityConfig
//! - nestgate-core/src/universal_adapter/canonical.rs → SecurityConfig
//! - Multiple auth/security related configs across all crates
//! - 20+ other security-related configuration structures
//!
//! **MODULAR STRUCTURE**:
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
pub mod authentication;
pub mod authorization;
pub mod tls;
pub mod encryption;
pub mod policies;
pub mod monitoring;
pub mod threat_protection;
pub mod environment;

// Re-export all configuration types
pub use authentication::{
    AuthenticationConfig, AuthenticationMethod, MfaConfig, SessionConfig, 
    TokenConfig, PasswordPolicyConfig, AccountLockoutConfig, ExternalAuthProvider
};
pub use authorization::{
    AuthorizationConfig, AccessControlConfig, RoleConfig, PermissionConfig,
    ResourceConfig, PolicyConfig
};
pub use tls::{
    TlsSecurityConfig, CertificateManagementConfig, SslConfig, CertificateConfig,
    TlsVersionConfig, CipherSuiteConfig
};
pub use encryption::{
    EncryptionSecurityConfig, CryptographicConfig, KeyManagementConfig,
    EncryptionAlgorithmConfig, HashingConfig
};
pub use policies::{
    SecurityPoliciesConfig, ComplianceConfig, DataProtectionConfig,
    RetentionPolicyConfig, PrivacyConfig
};
pub use monitoring::{
    SecurityMonitoringConfig, AuditSecurityConfig, LoggingConfig,
    AlertingConfig, IncidentResponseConfig
};
pub use threat_protection::{
    ThreatProtectionConfig, IntrusionDetectionConfig, FirewallConfig,
    DdosProtectionConfig, MalwareProtectionConfig
};
pub use environment::{
    SecurityEnvironmentConfig, EnvironmentSecuritySettings,
    DeploymentSecurityConfig, RuntimeSecurityConfig
};

// ==================== CANONICAL SECURITY CONFIGURATION ====================

/// **THE** canonical security configuration for the entire NestGate ecosystem
/// This replaces ALL other SecurityConfig variants
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
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
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a configuration optimized for production environments
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
    pub fn validate(&self) -> crate::Result<()> {
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

/// Backward compatibility alias for existing SecurityConfig usage
pub type SecurityConfig = CanonicalSecurityConfig;

/// Backward compatibility alias for UnifiedSecurityConfig
pub type UnifiedSecurityConfig = CanonicalSecurityConfig; 