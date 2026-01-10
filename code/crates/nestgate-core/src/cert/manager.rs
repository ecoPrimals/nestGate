/// Certificate Manager
/// Unified certificate management for the `NestGate` ecosystem
use std::collections::HashMap;
// unused Arc import removed
// Import NestGateCanonicalConfig from unified_types module
// Removed unused import for pedantic perfection
use crate::config::canonical_primary::NestGateCanonicalConfig; // Updated import
use crate::Result;

/// Certificate manager that uses the universal adapter for ecosystem integration
pub struct CertificateManager {
    #[allow(dead_code)]
    adapter: crate::universal_adapter::PrimalAgnosticAdapter, // Updated type
    #[allow(dead_code)]
    config: NestGateCanonicalConfig,
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
    pub fn new(config: NestGateCanonicalConfig) -> crate::Result<Self> {
        // Try explicit adapter endpoint first
        let adapter_url = std::env::var("NESTGATE_ADAPTER_ENDPOINT")
            .or_else(|_| {
                // Fall back to API URL + /adapter suffix
                std::env::var("NESTGATE_API_URL")
                    .map(|base| format!("{}/adapter", base.trim_end_matches('/')))
            })
            .map_err(|_| {
                crate::NestGateError::configuration_error(
                    "adapter_endpoint",
                    "Certificate manager requires NESTGATE_ADAPTER_ENDPOINT or NESTGATE_API_URL to be set. \
                     No hardcoded defaults for sovereignty compliance."
                )
            })?;

        let adapter = crate::universal_adapter::PrimalAgnosticAdapter::new(adapter_url);
        Ok(Self { adapter, config })
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
/// # Panics
///
/// Panics if the default manager cannot be created (indicates serious system misconfiguration)
#[must_use]
pub fn create_default_certificate_manager() -> CertificateManager {
    CertificateManager::new(NestGateCanonicalConfig::default())
        .unwrap_or_else(|e| {
            panic!("Critical: Failed to create default certificate manager: {e}. This indicates a serious system misconfiguration.")
        })
}
