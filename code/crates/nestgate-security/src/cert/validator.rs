// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Removed unused import for pedantic perfection
use crate::universal_adapter::PrimalAgnosticAdapter;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::{NestGateError, Result};
/// Certificate Validator
/// Unified certificate validation for the `NestGate` ecosystem
/// Certificate validator that uses the universal adapter for ecosystem integration
pub struct CertificateValidator {
    _adapter: PrimalAgnosticAdapter,
    _config: NestGateCanonicalConfig,
}
impl CertificateValidator {
    /// Create a new certificate validator
    ///
    /// # Configuration
    ///
    /// The adapter endpoint is determined from environment variables:
    /// - `NESTGATE_ADAPTER_ENDPOINT` - Full adapter URL (preferred)
    /// - Falls back to `NESTGATE_API_URL` with `/adapter` suffix
    ///
    /// # Errors
    ///
    /// Returns an error if no adapter endpoint is configured. This ensures
    /// explicit configuration rather than hidden hardcoded values.
    ///
    /// # Migration from hardcoded localhost
    ///
    /// **Before** (hardcoded):
    /// ```ignore
    /// // Hardcoded http://localhost:8080/adapter
    /// let validator = CertificateValidator::new(config)?;
    /// ```
    ///
    /// **After** (environment-driven):
    /// ```bash
    /// export NESTGATE_ADAPTER_ENDPOINT="http://your-server:8080/adapter"
    /// # OR
    /// export NESTGATE_API_URL="http://your-server:8080"
    /// ```
    pub fn new(config: NestGateCanonicalConfig) -> Result<Self> {
        // Try explicit adapter endpoint first
        let adapter_url = std::env::var("NESTGATE_ADAPTER_ENDPOINT")
            .or_else(|_| {
                // Fall back to API URL + /adapter suffix
                std::env::var("NESTGATE_API_URL")
                    .map(|base| format!("{}/adapter", base.trim_end_matches('/')))
            })
            .map_err(|_| {
                NestGateError::configuration_error(
                    "adapter_endpoint",
                    "Certificate validator requires NESTGATE_ADAPTER_ENDPOINT or NESTGATE_API_URL to be set. \
                     No hardcoded defaults for sovereignty compliance."
                )
            })?;

        let adapter = PrimalAgnosticAdapter::new(adapter_url);
        Ok(Self {
            _adapter: adapter,
            _config: config,
        })
    }

    /// Validate a certificate.
    ///
    /// Cryptographic verification is delegated to the security provider (`crypto.validate_cert` IPC);
    /// this type does not perform local X.509 parsing or trust decisions.
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError::not_implemented`] until that IPC path is wired.
    pub fn validate_certificate(&self, _cert_data: &[u8]) -> Result<bool> {
        Err(NestGateError::not_implemented(
            "Certificate validation delegated to security provider via crypto.validate_cert IPC",
        ))
    }

    /// Check if certificate is expired
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError::not_implemented`] until expiration is read via the security provider.
    pub fn is_certificate_expired(&self, _cert_data: &[u8]) -> Result<bool> {
        Err(NestGateError::not_implemented(
            "Certificate validation delegated to security provider via crypto.validate_cert IPC",
        ))
    }
}

/// Create a default certificate validator
///
/// # Errors
///
/// Returns an error if the validator cannot be created (e.g. no adapter endpoint configured)
pub fn create_default_certificate_validator() -> Result<CertificateValidator> {
    CertificateValidator::new(NestGateCanonicalConfig::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_types::NestGateError;

    #[test]
    fn new_errors_when_no_adapter_env() {
        temp_env::with_vars(
            [
                ("NESTGATE_ADAPTER_ENDPOINT", None::<&str>),
                ("NESTGATE_API_URL", None::<&str>),
            ],
            || {
                let err = CertificateValidator::new(NestGateCanonicalConfig::default())
                    .err()
                    .expect("expected configuration error");
                match err {
                    NestGateError::Configuration(d) => {
                        assert_eq!(d.field, "adapter_endpoint");
                    }
                    ref other => panic!("unexpected {other:?}"),
                }
            },
        );
    }

    #[test]
    fn validate_certificate_returns_not_implemented() {
        temp_env::with_var(
            "NESTGATE_ADAPTER_ENDPOINT",
            Some("http://localhost/adapter"),
            || {
                let v = CertificateValidator::new(NestGateCanonicalConfig::default()).unwrap();
                let e = v
                    .validate_certificate(&[])
                    .expect_err("delegated validation");
                match e {
                    NestGateError::NotImplemented { .. } => {}
                    ref other => panic!("unexpected {other:?}"),
                }
                let e = v
                    .validate_certificate(b"x")
                    .expect_err("delegated validation");
                assert!(matches!(e, NestGateError::NotImplemented { .. }));
            },
        );
    }

    #[test]
    fn is_certificate_expired_returns_not_implemented() {
        temp_env::with_var(
            "NESTGATE_ADAPTER_ENDPOINT",
            Some("http://localhost/adapter"),
            || {
                let v = CertificateValidator::new(NestGateCanonicalConfig::default()).unwrap();
                let e = v
                    .is_certificate_expired(b"any")
                    .expect_err("delegated expiration check");
                assert!(matches!(e, NestGateError::NotImplemented { .. }));
            },
        );
    }

    #[test]
    fn create_default_certificate_validator_uses_api_url_fallback() {
        temp_env::with_vars(
            [
                ("NESTGATE_ADAPTER_ENDPOINT", None::<&str>),
                ("NESTGATE_API_URL", Some("https://example.com/")),
            ],
            || {
                let v = create_default_certificate_validator().unwrap();
                assert!(v.validate_certificate(b"x").is_err());
            },
        );
    }
}
