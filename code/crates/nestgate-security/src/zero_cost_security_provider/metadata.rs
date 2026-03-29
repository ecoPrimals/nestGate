// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

///
/// This module contains metadata and capability information for the
/// zero-cost security provider system.
///
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
/// **Zero-cost security provider metadata**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Zerocostsecuritymetadata
pub struct ZeroCostSecurityMetadata {
    /// Provider type
    pub provider_type: String,
    /// Supported authentication methods
    pub supported_auth_methods: Vec<String>,
    /// Supported encryption algorithms
    pub supported_encryption: Vec<String>,
    /// Supported signing algorithms
    pub supported_signing: Vec<String>,
    /// Security compliance level
    pub compliance_level: String,
    /// Version information
    pub version: String,
    /// Provider capabilities
    pub capabilities: SecurityCapabilities,
    /// Performance characteristics
    pub performance: PerformanceCharacteristics,
    /// Security certifications
    pub certifications: Vec<String>,
}
impl Default for ZeroCostSecurityMetadata {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            provider_type: "zero-cost-security-provider".to_string(),
            supported_auth_methods: vec![
                "password".to_string(),
                "token".to_string(),
                "certificate".to_string(),
                "multi_factor".to_string(),
            ],
            supported_encryption: vec![
                "AES-256-GCM".to_string(),
                "ChaCha20-Poly1305".to_string(),
                "AES-128-GCM".to_string(),
            ],
            supported_signing: vec![
                "ECDSA-P256".to_string(),
                "ECDSA-P384".to_string(),
                "RSA-PSS-2048".to_string(),
                "Ed25519".to_string(),
            ],
            compliance_level: "enterprise".to_string(),
            version: "2.0.0-zero-cost".to_string(),
            capabilities: SecurityCapabilities::default(),
            performance: PerformanceCharacteristics::default(),
            certifications: vec![
                "FIPS-140-2".to_string(),
                "Common Criteria".to_string(),
                "SOC 2".to_string(),
            ],
        }
    }
}

/// **Security capabilities**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securitycapabilities
#[allow(clippy::struct_excessive_bools)]
pub struct SecurityCapabilities {
    /// Supports hardware security modules
    pub hardware_security_module: bool,
    /// Supports key rotation
    pub key_rotation: bool,
    /// Supports multi-factor authentication
    pub multi_factor_auth: bool,
    /// Supports role-based access control
    pub rbac: bool,
    /// Supports audit logging
    pub audit_logging: bool,
    /// Supports secure key storage
    pub secure_key_storage: bool,
    /// Supports cryptographic agility
    pub crypto_agility: bool,
    /// Supports zero-knowledge proofs
    pub zero_knowledge_proofs: bool,
}
impl Default for SecurityCapabilities {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            hardware_security_module: false,
            key_rotation: true,
            multi_factor_auth: true,
            rbac: true,
            audit_logging: true,
            secure_key_storage: true,
            crypto_agility: true,
            zero_knowledge_proofs: false,
        }
    }
}

/// **Performance characteristics**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancecharacteristics
pub struct PerformanceCharacteristics {
    /// Authentication operations per second
    pub auth_ops_per_second: u64,
    /// Encryption operations per second
    pub encryption_ops_per_second: u64,
    /// Signing operations per second
    pub signing_ops_per_second: u64,
    /// Average authentication latency (microseconds)
    pub avg_auth_latency_us: u64,
    /// Average encryption latency (microseconds)
    pub avg_encryption_latency_us: u64,
    /// Average signing latency (microseconds)
    pub avg_signing_latency_us: u64,
    /// Memory usage (bytes)
    pub memory_usage_bytes: u64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
}
impl Default for PerformanceCharacteristics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auth_ops_per_second: 10000,
            encryption_ops_per_second: 50000,
            signing_ops_per_second: 5000,
            avg_auth_latency_us: 100,
            avg_encryption_latency_us: 20,
            avg_signing_latency_us: 200,
            memory_usage_bytes: 1024 * 1024, // 1MB
            cpu_usage_percent: 5.0,
        }
    }
}

/// **Security provider health information**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securityproviderhealth
pub struct SecurityProviderHealth {
    /// Overall health status
    pub status: String,
    /// Health check timestamp
    pub timestamp: SystemTime,
    /// Active sessions count
    pub active_sessions: usize,
    /// Total authentications processed
    pub total_authentications: u64,
    /// Failed authentication attempts
    pub failed_attempts: u64,
    /// Success rate percentage
    pub success_rate: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Last authentication timestamp
    pub last_authentication: Option<SystemTime>,
    /// Security events count
    pub security_events: u64,
    /// Key rotation status
    pub key_rotation_count: u64,
}
impl Default for SecurityProviderHealth {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            active_sessions: 0,
            total_authentications: 0,
            failed_attempts: 0,
            success_rate: 100.0,
            avg_response_time_ms: 10.0,
            last_authentication: None,
            security_events: 0,
            key_rotation_count: 0,
            status: "healthy".to_string(),
        }
    }
}

/// **Health status enumeration**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for Health
pub enum HealthStatus {
    /// Provider is healthy and operational
    Healthy,
    /// Provider is operational but with warnings
    Warning,
    /// Provider is experiencing issues
    Degraded,
    /// Provider is not operational
    Unhealthy,
    /// Provider status is unknown
    Unknown,
}
impl HealthStatus {
    /// Check if the status indicates a healthy provider
    #[must_use]
    pub const fn is_healthy(&self) -> bool {
        matches!(self, Self::Healthy)
    }

    /// Check if the status indicates an operational provider
    #[must_use]
    pub const fn is_operational(&self) -> bool {
        matches!(self, Self::Healthy | Self::Warning | Self::Degraded)
    }

    /// Get status as string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Warning => "warning",
            Self::Degraded => "degraded",
            Self::Unhealthy => "unhealthy",
            Self::Unknown => "unknown",
        }
    }
}

/// **Key rotation status**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
/// Status values for `KeyRotation`
pub enum KeyRotationStatus {
    /// Keys are current and valid
    Current,
    /// Keys are approaching expiration
    Expiring,
    /// Key rotation is in progress
    Rotating,
    /// Keys have expired and need rotation
    Expired,
    /// Key rotation failed
    Failed,
}
impl KeyRotationStatus {
    /// Check if keys are valid for use
    #[must_use]
    pub const fn is_valid(&self) -> bool {
        matches!(self, Self::Current | Self::Expiring | Self::Rotating)
    }

    /// Check if immediate action is required
    #[must_use]
    pub const fn requires_action(&self) -> bool {
        matches!(self, Self::Expired | Self::Failed)
    }
}

/// **Security provider metrics**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Securityprovidermetrics
pub struct SecurityProviderMetrics {
    /// Metrics collection timestamp
    pub timestamp: SystemTime,
    /// Authentication metrics
    pub authentication: AuthenticationMetrics,
    /// Encryption metrics
    pub encryption: EncryptionMetrics,
    /// Signing metrics
    pub signing: SigningMetrics,
    /// Performance metrics
    pub performance: PerformanceMetrics,
    /// Security metrics
    pub security: SecurityMetrics,
}
impl Default for SecurityProviderMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            timestamp: SystemTime::now(),
            authentication: AuthenticationMetrics::default(),
            encryption: EncryptionMetrics::default(),
            signing: SigningMetrics::default(),
            performance: PerformanceMetrics::default(),
            security: SecurityMetrics::default(),
        }
    }
}

/// **Authentication metrics**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authenticationmetrics
pub struct AuthenticationMetrics {
    /// Total authentication attempts
    pub total_attempts: u64,
    /// Successful authentications
    pub successful_auths: u64,
    /// Failed authentication attempts
    pub failed_auths: u64,
    /// Active sessions
    pub active_sessions: u64,
    /// Average authentication time (ms)
    pub avg_auth_time_ms: f64,
    /// Peak authentication rate (per second)
    pub peak_auth_rate: f64,
}
impl Default for AuthenticationMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            total_attempts: 0,
            successful_auths: 0,
            failed_auths: 0,
            active_sessions: 0,
            avg_auth_time_ms: 0.0,
            peak_auth_rate: 0.0,
        }
    }
}

/// **Encryption metrics**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Encryptionmetrics
pub struct EncryptionMetrics {
    /// Total encryption operations
    pub total_encryptions: u64,
    /// Total decryption operations
    pub total_decryptions: u64,
    /// Failed encryption operations
    pub failed_encryptions: u64,
    /// Failed decryption operations
    pub failed_decryptions: u64,
    /// Average encryption time (ms)
    pub avg_encryption_time_ms: f64,
    /// Average decryption time (ms)
    pub avg_decryption_time_ms: f64,
    /// Total bytes encrypted
    pub total_bytes_encrypted: u64,
    /// Total bytes decrypted
    pub total_bytes_decrypted: u64,
}
impl Default for EncryptionMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            total_encryptions: 0,
            total_decryptions: 0,
            failed_encryptions: 0,
            failed_decryptions: 0,
            avg_encryption_time_ms: 0.0,
            avg_decryption_time_ms: 0.0,
            total_bytes_encrypted: 0,
            total_bytes_decrypted: 0,
        }
    }
}

/// **Signing metrics**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Signingmetrics
pub struct SigningMetrics {
    /// Total signing operations
    pub total_signatures: u64,
    /// Total verification operations
    pub total_verifications: u64,
    /// Failed signing operations
    pub failed_signatures: u64,
    /// Failed verification operations
    pub failed_verifications: u64,
    /// Average signing time (ms)
    pub avg_signing_time_ms: f64,
    /// Average verification time (ms)
    pub avg_verification_time_ms: f64,
}
impl Default for SigningMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            total_signatures: 0,
            total_verifications: 0,
            failed_signatures: 0,
            failed_verifications: 0,
            avg_signing_time_ms: 0.0,
            avg_verification_time_ms: 0.0,
        }
    }
}

/// **Performance metrics**
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Performancemetrics
pub struct PerformanceMetrics {
    /// CPU usage percentage
    pub cpu_usage: f64,
    /// Memory usage in bytes
    pub memory_usage: u64,
    /// Network I/O bytes per second
    pub network_io_bps: u64,
    /// Disk I/O bytes per second
    pub disk_io_bps: u64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Average response time (ms)
    pub avg_response_time_ms: f64,
}
impl Default for PerformanceMetrics {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0,
            network_io_bps: 0,
            disk_io_bps: 0,
            requests_per_second: 0.0,
            avg_response_time_ms: 0.0,
        }
    }
}

/// **Security metrics**
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Securitymetrics
pub struct SecurityMetrics {
    /// Security events detected
    pub security_events: u64,
    /// Blocked attacks
    pub blocked_attacks: u64,
    /// Suspicious activities
    pub suspicious_activities: u64,
    /// Key rotations performed
    pub key_rotations: u64,
    /// Audit log entries
    pub audit_entries: u64,
    /// Compliance violations
    pub compliance_violations: u64,
}
impl ZeroCostSecurityMetadata {
    /// Check if authentication method is supported
    #[must_use]
    pub fn supports_auth_method(&self, method: &str) -> bool {
        self.supported_auth_methods.contains(&method.to_string())
    }

    /// Check if encryption algorithm is supported
    #[must_use]
    pub fn supports_encryption(&self, algorithm: &str) -> bool {
        self.supported_encryption.contains(&algorithm.to_string())
    }

    /// Check if signing algorithm is supported
    #[must_use]
    pub fn supports_signing(&self, algorithm: &str) -> bool {
        self.supported_signing.contains(&algorithm.to_string())
    }

    /// Get compliance level
    #[must_use]
    pub fn compliance_level(&self) -> &str {
        &self.compliance_level
    }

    /// Check if provider meets compliance requirements
    #[must_use]
    pub fn meets_compliance(&self, required_level: &str) -> bool {
        match (self.compliance_level.as_str(), required_level) {
            // Government compliance requires explicit government level
            (level, "government") => level == "government",
            ("enterprise", "enterprise" | "professional" | "basic")
            | ("government", _)
            | ("professional", "professional" | "basic")
            | ("basic", "basic") => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_metadata() {
        let metadata = ZeroCostSecurityMetadata::default();

        assert_eq!(metadata.provider_type, "zero-cost-security-provider");
        assert!(metadata.supports_auth_method("password"));
        assert!(metadata.supports_encryption("AES-256-GCM"));
        assert!(metadata.supports_signing("ECDSA-P256"));
        assert!(!metadata.supports_auth_method("unknown"));
    }

    #[test]
    fn test_health_status() {
        let status = HealthStatus::Healthy;
        assert!(status.is_healthy());
        assert!(status.is_operational());
        assert_eq!(status.as_str(), "healthy");

        let status = HealthStatus::Unhealthy;
        assert!(!status.is_healthy());
        assert!(!status.is_operational());
    }

    #[test]
    fn test_key_rotation_status() {
        let status = KeyRotationStatus::Current;
        assert!(status.is_valid());
        assert!(!status.requires_action());

        let status = KeyRotationStatus::Expired;
        assert!(!status.is_valid());
        assert!(status.requires_action());
    }

    #[test]
    fn test_compliance_check() {
        let metadata = ZeroCostSecurityMetadata::default();

        assert!(metadata.meets_compliance("basic"));
        assert!(metadata.meets_compliance("professional"));
        assert!(metadata.meets_compliance("enterprise"));
        assert!(!metadata.meets_compliance("government"));
    }
}
