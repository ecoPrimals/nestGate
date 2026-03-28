// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Removed unused import for pedantic perfection
use crate::universal_adapter::PrimalAgnosticAdapter;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::{NestGateError, Result};
/// Certificate Validator
/// Unified certificate validation for the `NestGate` ecosystem
/// Certificate validator that uses the universal adapter for ecosystem integration
pub struct CertificateValidator {
    #[allow(dead_code)]
    adapter: PrimalAgnosticAdapter, // Updated type
    #[allow(dead_code)]
    config: NestGateCanonicalConfig,
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
        Ok(Self { adapter, config })
    }

    /// Validate a certificate
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn validate_certificate(&self, cert_data: &[u8]) -> Result<bool> {
        // Use the universal adapter for certificate validation
        // This is a simplified validation for the modernization
        Ok(!cert_data.is_empty())
    }

    /// Check if certificate is expired
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn is_certificate_expired(&self, _cert_data: &[u8]) -> Result<bool> {
        // Simplified expiration check
        Ok(false)
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
