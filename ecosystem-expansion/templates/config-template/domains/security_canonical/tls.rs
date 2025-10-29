//! **TLS CONFIGURATION**

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsSecurityConfig {
    pub enabled: bool,
    pub certificates: CertificateManagementConfig,
    pub ssl: SslConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateManagementConfig {
    pub auto_renewal: bool,
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl CertificateManagementConfig {
    pub fn development_optimized() -> Self {
        Self {
            auto_renewal: false,
            cert_path: PathBuf::from("/tmp/dev-cert.pem"),
            key_path: PathBuf::from("/tmp/dev-key.pem"),
        }
    }

    pub fn compliance_focused() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate.pem"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate.key"),
        }
    }

    pub fn production_hardened() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate-prod.pem"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate-prod.key"),
        }
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.auto_renewal = other.auto_renewal;
        self.cert_path = other.cert_path;
        self.key_path = other.key_path;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SslConfig {
    pub min_version: String,
    pub cipher_suites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateConfig {
    pub path: PathBuf,
    pub key_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsVersionConfig {
    pub min_version: String,
    pub max_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CipherSuiteConfig {
    pub allowed: Vec<String>,
    pub preferred: Vec<String>,
}

impl Default for TlsSecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            certificates: CertificateManagementConfig::default(),
            ssl: SslConfig::default(),
        }
    }
}

impl Default for CertificateManagementConfig {
    fn default() -> Self {
        Self {
            auto_renewal: true,
            cert_path: PathBuf::from("/etc/ssl/certs/nestgate.crt"),
            key_path: PathBuf::from("/etc/ssl/private/nestgate.key"),
        }
    }
}

impl Default for SslConfig {
    fn default() -> Self {
        Self {
            min_version: "TLSv1.2".to_string(),
            cipher_suites: vec!["ECDHE-RSA-AES256-GCM-SHA384".to_string()],
        }
    }
}

impl TlsSecurityConfig {
    pub fn production_hardened() -> Self { Self::default() }
    pub fn development_optimized() -> Self { Self::default() }
    pub fn compliance_focused() -> Self { Self::default() }
    pub fn merge(self, _other: Self) -> Self { self }
    pub fn validate(&self) -> crate::Result<()> { Ok(()) }
} 