// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// Removed unused import for pedantic perfection
use crate::universal_adapter::PrimalAgnosticAdapter;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::{NestGateError, Result};
use x509_parser::pem::parse_x509_pem;
use x509_parser::prelude::*;
use x509_parser::time::ASN1Time;

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
                     No hardcoded defaults for sovereignty compliance.",
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
    /// Performs **local structural checks** only: PEM/DER decoding and validity window
    /// (`notBefore` / `notAfter` vs current time). Returns `Ok(true)` when the object parses
    /// as X.509 and the current time falls within the validity interval.
    ///
    /// **Not performed here:** signature verification, chain building, revocation, or trust store
    /// evaluation — those remain the responsibility of the security capability provider
    /// (`crypto.validate_cert` IPC).
    pub fn validate_certificate(&self, cert_data: &[u8]) -> Result<bool> {
        let Some(valid_now) = x509_valid_now(cert_data) else {
            return Ok(false);
        };
        Ok(valid_now)
    }

    /// Check if certificate is expired
    ///
    /// Uses the same PEM/DER parsing as [`Self::validate_certificate`] and compares
    /// `notAfter` to the current time.
    ///
    /// # Errors
    ///
    /// Returns a validation error if the data is not a parseable X.509 certificate.
    pub fn is_certificate_expired(&self, cert_data: &[u8]) -> Result<bool> {
        let expired = x509_expired(cert_data).ok_or_else(|| {
            NestGateError::validation_error(
                "certificate: could not parse PEM or DER as X.509 for expiration check",
            )
        })?;
        Ok(expired)
    }
}

/// First PEM `CERTIFICATE` block or raw DER: `true` if validity window contains "now".
fn x509_valid_now(cert_data: &[u8]) -> Option<bool> {
    let mut rest = cert_data;
    while !rest.is_empty() {
        if let Ok((rem, pem)) = parse_x509_pem(rest) {
            if pem.label == "CERTIFICATE"
                && let Ok((_, cert)) = parse_x509_certificate(&pem.contents)
            {
                return Some(cert.validity().is_valid());
            }
            rest = rem;
        } else {
            break;
        }
    }
    parse_x509_certificate(cert_data)
        .ok()
        .map(|(_, cert)| cert.validity().is_valid())
}

/// `Ok(true)` if expired, `Ok(false)` if still valid, `None` if unparseable.
fn x509_expired(cert_data: &[u8]) -> Option<bool> {
    let mut rest = cert_data;
    while !rest.is_empty() {
        if let Ok((rem, pem)) = parse_x509_pem(rest) {
            if pem.label == "CERTIFICATE"
                && let Ok((_, cert)) = parse_x509_certificate(&pem.contents)
            {
                let now = ASN1Time::now();
                return Some(now > cert.validity().not_after);
            }
            rest = rem;
        } else {
            break;
        }
    }
    parse_x509_certificate(cert_data).ok().map(|(_, cert)| {
        let now = ASN1Time::now();
        now > cert.validity().not_after
    })
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
    fn validate_certificate_rejects_garbage() {
        temp_env::with_var(
            "NESTGATE_ADAPTER_ENDPOINT",
            Some("http://localhost/adapter"),
            || {
                let v = CertificateValidator::new(NestGateCanonicalConfig::default())
                    .expect("validator new");
                assert!(!v.validate_certificate(&[]).expect("structural result"));
                assert!(!v.validate_certificate(b"x").expect("structural result"));
            },
        );
    }

    #[test]
    fn is_certificate_expired_errors_on_garbage() {
        temp_env::with_var(
            "NESTGATE_ADAPTER_ENDPOINT",
            Some("http://localhost/adapter"),
            || {
                let v = CertificateValidator::new(NestGateCanonicalConfig::default())
                    .expect("validator new");
                let e = v
                    .is_certificate_expired(b"not-a-cert")
                    .expect_err("parse should fail");
                assert!(matches!(e, NestGateError::Validation(_)));
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
                let v = create_default_certificate_validator().expect("default validator");
                assert!(!v.validate_certificate(b"x").expect("structural"));
            },
        );
    }
}
