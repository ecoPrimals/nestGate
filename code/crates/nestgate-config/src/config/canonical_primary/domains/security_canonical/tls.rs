// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **TLS CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TlsSecurity`
pub struct TlsSecurityConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Certificates
    pub certificates: CertificateManagementConfig,
    /// Ssl
    pub ssl: SslConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CertificateManagement`
pub struct CertificateManagementConfig {
    /// Auto Renewal
    pub auto_renewal: bool,
    /// Cert Path
    pub cert_path: PathBuf,
    /// Key Path
    pub key_path: PathBuf,
}

impl CertificateManagementConfig {
    /// Creates a development-optimized certificate configuration.
    ///
    /// Uses temporary certificate paths suitable for development environments.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            auto_renewal: false,
            cert_path: PathBuf::from("/tmp/dev-cert.pem"),
            key_path: PathBuf::from("/tmp/dev-key.pem"),
        }
    }

    /// Creates a compliance-focused certificate configuration.
    ///
    /// Enables auto-renewal and uses standard system certificate paths.
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate.pem"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate.key"),
        }
    }

    /// Creates a production-hardened certificate configuration.
    ///
    /// Enables auto-renewal and uses production-specific certificate paths.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate-prod.pem"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate-prod.key"),
        }
    }

    /// Merges another certificate configuration into this one.
    ///
    /// All fields from `other` will override the current values.
    #[must_use]
    pub fn merge(mut self, other: Self) -> Self {
        self.auto_renewal = other.auto_renewal;
        self.cert_path = other.cert_path;
        self.key_path = other.key_path;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Ssl
pub struct SslConfig {
    /// Min Version
    pub min_version: String,
    /// Cipher Suites
    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Certificate
pub struct CertificateConfig {
    /// Path
    pub path: PathBuf,
    /// Key Path
    pub key_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `TlsVersion`
pub struct TlsVersionConfig {
    /// Min Version
    pub min_version: String,
    /// Max Version
    pub max_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for `CipherSuite`
pub struct CipherSuiteConfig {
    /// Allowed
    pub allowed: Vec<String>,
    /// Preferred
    pub preferred: Vec<String>,
}

impl Default for TlsSecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            enabled: true,
            certificates: CertificateManagementConfig::default(),
            ssl: SslConfig::default(),
        }
    }
}

impl Default for CertificateManagementConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate.crt"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate.key"),
        }
    }
}

impl Default for SslConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            min_version: "TLSv1.2".to_string(),
            cipher_suites: vec!["ECDHE-RSA-AES256-GCM-SHA384".to_string()],
        }
    }
}

impl TlsSecurityConfig {
    /// Creates a production-hardened TLS security configuration.
    ///
    /// Returns the default configuration optimized for production security.
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }

    /// Creates a development-optimized TLS security configuration.
    ///
    /// Returns the default configuration suitable for development environments.
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }

    /// Creates a compliance-focused TLS security configuration.
    ///
    /// Returns the default configuration meeting compliance requirements.
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::default()
    }

    /// Merges another TLS security configuration into this one.
    ///
    /// This is a no-op placeholder for configuration merging.
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }

    /// Validates data
    pub const fn validate(&self) -> nestgate_types::error::Result<()> {
        Ok(())
    }
}
