// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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
        let tmp = std::env::temp_dir();
        Self {
            auto_renewal: false,
            cert_path: tmp.join("dev-cert.pem"),
            key_path: tmp.join("dev-key.pem"),
        }
    }

    /// Creates a compliance-focused certificate configuration.
    ///
    /// Enables auto-renewal and uses standard system certificate paths.
    #[must_use]
    pub fn compliance_focused() -> Self {
        let config = crate::config::storage_paths::get_config_dir();
        Self {
            auto_renewal: true,
            cert_path: config.join("ssl").join("nestgate.pem"),
            key_path: config.join("ssl").join("nestgate.key"),
        }
    }

    /// Creates a production-hardened certificate configuration.
    ///
    /// Enables auto-renewal and uses production-specific certificate paths.
    #[must_use]
    pub fn production_hardened() -> Self {
        let config = crate::config::storage_paths::get_config_dir();
        Self {
            auto_renewal: true,
            cert_path: config.join("ssl").join("nestgate-prod.pem"),
            key_path: config.join("ssl").join("nestgate-prod.key"),
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
            min_version: "TLSv1.2".into(),
            cipher_suites: vec!["ECDHE-RSA-AES256-GCM-SHA384".into()],
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

    /// Validate TLS configuration consistency.
    ///
    /// When TLS is enabled, cert and key paths must be non-empty.
    pub fn validate(&self) -> nestgate_types::error::Result<()> {
        if self.enabled {
            if self.certificates.cert_path.as_os_str().is_empty() {
                return Err(nestgate_types::error::NestGateError::validation_error(
                    "TLS enabled but certificates.cert_path is empty",
                ));
            }
            if self.certificates.key_path.as_os_str().is_empty() {
                return Err(nestgate_types::error::NestGateError::validation_error(
                    "TLS enabled but certificates.key_path is empty",
                ));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_tls_config_validates() {
        let config = TlsSecurityConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn disabled_tls_skips_path_checks() {
        let config = TlsSecurityConfig {
            enabled: false,
            certificates: CertificateManagementConfig {
                auto_renewal: false,
                cert_path: PathBuf::new(),
                key_path: PathBuf::new(),
            },
            ssl: SslConfig::default(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn enabled_tls_rejects_empty_cert_path() {
        let config = TlsSecurityConfig {
            enabled: true,
            certificates: CertificateManagementConfig {
                auto_renewal: false,
                cert_path: PathBuf::new(),
                key_path: PathBuf::from("/etc/ssl/private/key.pem"),
            },
            ssl: SslConfig::default(),
        };
        let err = config.validate();
        assert!(err.is_err());
        assert!(
            err.unwrap_err().to_string().contains("cert_path"),
            "error should mention cert_path"
        );
    }

    #[test]
    fn enabled_tls_rejects_empty_key_path() {
        let config = TlsSecurityConfig {
            enabled: true,
            certificates: CertificateManagementConfig {
                auto_renewal: true,
                cert_path: PathBuf::from("/etc/ssl/certs/cert.pem"),
                key_path: PathBuf::new(),
            },
            ssl: SslConfig::default(),
        };
        let err = config.validate();
        assert!(err.is_err());
        assert!(
            err.unwrap_err().to_string().contains("key_path"),
            "error should mention key_path"
        );
    }

    #[test]
    fn enabled_tls_accepts_valid_paths() {
        let config = TlsSecurityConfig {
            enabled: true,
            certificates: CertificateManagementConfig {
                auto_renewal: true,
                cert_path: PathBuf::from("/etc/ssl/certs/nestgate.pem"),
                key_path: PathBuf::from("/etc/ssl/private/nestgate.key"),
            },
            ssl: SslConfig::default(),
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn production_hardened_equals_default() {
        let prod = TlsSecurityConfig::production_hardened();
        let def = TlsSecurityConfig::default();
        assert_eq!(prod.enabled, def.enabled);
        assert_eq!(prod.ssl.min_version, def.ssl.min_version);
    }

    #[test]
    fn development_optimized_cert_uses_temp_dir() {
        let cert = CertificateManagementConfig::development_optimized();
        assert!(!cert.auto_renewal);
        let path_str = cert.cert_path.to_string_lossy();
        assert!(
            path_str.contains("dev-cert"),
            "dev cert should use dev-cert name"
        );
    }

    #[test]
    fn compliance_focused_cert_enables_renewal() {
        let cert = CertificateManagementConfig::compliance_focused();
        assert!(cert.auto_renewal);
    }

    #[test]
    fn production_hardened_cert_enables_renewal() {
        let cert = CertificateManagementConfig::production_hardened();
        assert!(cert.auto_renewal);
    }

    #[test]
    fn cert_merge_overrides_all_fields() {
        let base = CertificateManagementConfig {
            auto_renewal: false,
            cert_path: PathBuf::from("/old/cert"),
            key_path: PathBuf::from("/old/key"),
        };
        let overlay = CertificateManagementConfig {
            auto_renewal: true,
            cert_path: PathBuf::from("/new/cert"),
            key_path: PathBuf::from("/new/key"),
        };
        let merged = base.merge(overlay);
        assert!(merged.auto_renewal);
        assert_eq!(merged.cert_path, PathBuf::from("/new/cert"));
        assert_eq!(merged.key_path, PathBuf::from("/new/key"));
    }

    #[test]
    fn ssl_config_default_uses_tls12() {
        let ssl = SslConfig::default();
        assert_eq!(ssl.min_version, "TLSv1.2");
        assert!(!ssl.cipher_suites.is_empty());
    }
}
