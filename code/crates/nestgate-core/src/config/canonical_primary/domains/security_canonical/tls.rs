// **TLS CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for TlsSecurity
pub struct TlsSecurityConfig {
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Certificates
    pub certificates: CertificateManagementConfig,
    /// Ssl
    pub ssl: SslConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for CertificateManagement
pub struct CertificateManagementConfig {
    /// Auto Renewal
    pub auto_renewal: bool,
    /// Cert Path
    pub cert_path: PathBuf,
    /// Key Path
    pub key_path: PathBuf,
}

impl CertificateManagementConfig {
    #[must_use]
    pub fn development_optimized() -> Self {
        Self {
            auto_renewal: false,
            cert_path: PathBuf::from("/tmp/dev-cert.pem"),
            key_path: PathBuf::from("/tmp/dev-key.pem"),
        }
    }

    #[must_use]
    pub fn compliance_focused() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate.pem"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate.key"),
        }
    }

    #[must_use]
    pub fn production_hardened() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate-prod.pem"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate-prod.key"),
        }
    }

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
/// Configuration for TlsVersion
pub struct TlsVersionConfig {
    /// Min Version
    pub min_version: String,
    /// Max Version
    pub max_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for CipherSuite
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
    #[must_use]
    pub fn production_hardened() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn development_optimized() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn compliance_focused() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn merge(self, _other: Self) -> Self {
        self
    }
    /// Validates data
    pub fn validate(&self) -> crate::Result<()> {
        Ok(())
    }
}
