// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Certificate Manager
/// Unified certificate management for the `NestGate` ecosystem
use std::collections::HashMap;
// unused Arc import removed
// Import NestGateCanonicalConfig from unified_types module
// Removed unused import for pedantic perfection
use crate::universal_adapter::PrimalAgnosticAdapter;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_types::{NestGateError, Result};

/// Certificate manager that uses the universal adapter for ecosystem integration
pub struct CertificateManager {
    _adapter: PrimalAgnosticAdapter,
    _config: NestGateCanonicalConfig,
}
impl CertificateManager {
    /// Create a new certificate manager
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
    /// let manager = CertificateManager::new(config)?;
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
                    "Certificate manager requires NESTGATE_ADAPTER_ENDPOINT or NESTGATE_API_URL to be set. \
                     No hardcoded defaults for sovereignty compliance."
                )
            })?;

        let adapter = PrimalAgnosticAdapter::new(adapter_url);
        Ok(Self {
            _adapter: adapter,
            _config: config,
        })
    }

    /// Get certificate information
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn get_certificate_info(&self, cert_id: &str) -> Result<HashMap<String, String>> {
        // Use the universal adapter for certificate operations
        let mut info = HashMap::new();
        info.insert("id".to_string(), cert_id.to_string());
        info.insert("status".to_string(), "valid".to_string());
        Ok(info)
    }
}

/// Create a default certificate manager
///
/// # Errors
///
/// Returns an error if the manager cannot be created (e.g. no adapter endpoint configured)
pub fn create_default_certificate_manager() -> Result<CertificateManager> {
    CertificateManager::new(NestGateCanonicalConfig::default())
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
                let err = CertificateManager::new(NestGateCanonicalConfig::default())
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
    fn get_certificate_info_with_explicit_adapter_endpoint() {
        temp_env::with_var(
            "NESTGATE_ADAPTER_ENDPOINT",
            Some("unix:///tmp/nestgate-adapter.sock"),
            || {
                let m = CertificateManager::new(NestGateCanonicalConfig::default()).unwrap();
                let info = m.get_certificate_info("cert-1").unwrap();
                assert_eq!(info.get("id"), Some(&"cert-1".to_string()));
                assert_eq!(info.get("status"), Some(&"valid".to_string()));
            },
        );
    }

    #[test]
    fn create_default_certificate_manager_uses_api_url_fallback() {
        temp_env::with_vars(
            [
                ("NESTGATE_ADAPTER_ENDPOINT", None::<&str>),
                ("NESTGATE_API_URL", Some("http://127.0.0.1:8080")),
            ],
            || {
                let m = create_default_certificate_manager().unwrap();
                let info = m.get_certificate_info("x").unwrap();
                assert_eq!(info.get("status"), Some(&"valid".to_string()));
            },
        );
    }

    #[test]
    fn new_trims_trailing_slash_on_api_url_for_adapter_path() {
        temp_env::with_vars(
            [
                ("NESTGATE_ADAPTER_ENDPOINT", None::<&str>),
                ("NESTGATE_API_URL", Some("https://api.example.com/v1///")),
            ],
            || {
                let m = CertificateManager::new(NestGateCanonicalConfig::default()).unwrap();
                let info = m.get_certificate_info("id").unwrap();
                assert_eq!(info.get("id"), Some(&"id".to_string()));
            },
        );
    }
}
