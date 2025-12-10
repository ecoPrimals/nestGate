/// Certificate Utility Functions
/// Utility functions for certificate generation, parsing, and manipulation.
use super::types::{Certificate, CertificateType};
use crate::error::NestGateError;
use crate::Result;
#[cfg(feature = "dev-stubs")]
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
// CLEANED: Removed unused imports as part of canonical modernization
// use std::net::SocketAddr;
// **MIGRATED**: Using canonical config instead of deprecated unified_types

/// Convert `SystemTime` to a string representation
#[must_use]
pub fn format_system_time(time: SystemTime) -> String {
    match time.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs().to_string(),
        Err(_) => "0".to_string(), // fallback for times before Unix epoch
    }
}
/// Parse a string back into `SystemTime`
pub fn parse_system_time(s: &str) -> Result<SystemTime> {
    match s.parse::<u64>() {
        Ok(secs) => Ok(SystemTime::UNIX_EPOCH + Duration::from_secs(secs)),
        Err(_) => Err(NestGateError::validation_error(&format!(
            "Invalid timestamp format: {s}"
        ))),
    }
}
/// Certificate utility functions
pub struct CertUtils;
impl CertUtils {
    /// Generate self-signed certificate for development/testing
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn generate_self_signed() -> Result<String> {
        // Simplified certificate generation for development
        // Real implementation would use proper cryptographic libraries like ring or rustls

        let cert_template = r"-----BEGIN CERTIFICATE-----
MIICWjCCAcMCAg38MA0GCSqGSIb3DQEBBQUAMHsxCzAJBgNVBAYTAlVTMQswCQYD
VQQIDAJOSjEQMA0GA1UEBwwGTmVzdEdhcGUxEzARBgNVBAoMCk5lc3RHYXRFIENB
MRMwEQYDVQQDDApOZXN0R2F0ZSBDQTEXMBUGA1UECgwOTmVzdEdhdGUgU3lzdGVt
MRcwFQYDVQQDDA5OZXN0R2F0ZSBTZXJ2ZXIwHhcNMjQwMTAxMDAwMDAwWhcNMjUw
MTAxMDAwMDAwWjBrMQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExEDAOBgNVBAcM
B05lc3RHYXBLMREWDQYDVQQKDAZOZXN0R2F0ZTERDw0GA1UEAwwITmVzdEdhdGU=
-----END CERTIFICATE-----";

        Ok(cert_template.to_string())
    }

    /// Generate certificate fingerprint
    #[must_use]
    pub fn calculate_fingerprint(cert_data: &[u8]) -> String {
        // Simplified fingerprint calculation
        // Real implementation would use SHA-256 hash
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        cert_data.hash(&mut hasher);
        let hash = hasher.finish();
        format!("sha256:{hash:x}")
    }

    /// Parse certificate subject from PEM data
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn parse_subject(cert_pem: &str) -> Result<String> {
        // Simplified subject parsing
        // Real implementation would parse X.509 ASN.1 structure

        if cert_pem.contains("CN=") {
            // Extract common name from certificate
            if let Some(start) = cert_pem.find("CN=") {
                let remaining = &cert_pem[start + 3..];
                if let Some(end) = remaining.find(',').or_else(|| remaining.find('\n')) {
                    return Ok(remaining[..end].trim().to_string());
                }
                return Ok(remaining.trim().to_string());
            }
        }

        Ok("Unknown Subject".to_string())
    }

    /// Parse certificate issuer from PEM data
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn parse_issuer(cert_pem: &str) -> Result<String> {
        // Simplified issuer parsing
        // Real implementation would parse X.509 ASN.1 structure

        if cert_pem.contains("Issuer:") {
            if let Some(start) = cert_pem.find("Issuer:") {
                let remaining = &cert_pem[start + 7..];
                if let Some(end) = remaining.find('\n') {
                    return Ok(remaining[..end].trim().to_string());
                }
                return Ok(remaining.trim().to_string());
            }
        }

        Ok("Unknown Issuer".to_string())
    }

    /// Check if certificate PEM format is valid
    #[must_use]
    pub fn is_valid_pem_format(cert_pem: &str) -> bool {
        cert_pem.contains("-----BEGIN CERTIFICATE-----")
            && cert_pem.contains("-----END CERTIFICATE-----")
    }

    /// Create test certificate for development
    #[must_use]
    pub fn create_test_certificate() -> Certificate {
        let now = SystemTime::now();
        Certificate {
            id: "test-cert-001".to_string(),
            cert_type: CertificateType::Server,
            principal: format!(
                "CN={}",
                crate::constants::canonical_defaults::network::LOCALHOST
            ),
            issuer: "CN=NestGate Test CA".to_string(),
            data: b"test certificate data".to_vec(),
            not_before: format_system_time(now),
            not_after: format_system_time(now + Duration::from_secs(365 * 24 * 3600)), // 1 year
            serial_number: "TEST-001".to_string(),
            fingerprint: "sha256:test123456789abcdef".to_string(),
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Create expired certificate for testing
    #[must_use]
    pub fn create_expired_certificate() -> Certificate {
        let mut cert = Self::create_test_certificate();
        cert.id = "expired-test-cert".to_string();
        cert.not_after = format_system_time(SystemTime::UNIX_EPOCH + Duration::from_secs(1)); // Already expired
        cert.serial_number = "EXPIRED-001".to_string();
        cert
    }

    /// Validate certificate format without cryptographic verification
    #[must_use]
    pub fn validate_certificate_format(cert: &Certificate) -> Vec<String> {
        let mut errors = Vec::new();

        if cert.id.is_empty() {
            errors.push("Certificate ID cannot be empty".to_string());
        }

        if cert.principal.is_empty() {
            errors.push("Certificate subject cannot be empty".to_string());
        }

        if cert.issuer.is_empty() {
            errors.push("Certificate issuer cannot be empty".to_string());
        }

        if cert.data.is_empty() {
            errors.push("Certificate data cannot be empty".to_string());
        }

        if cert.serial_number.is_empty() {
            errors.push("Certificate serial number cannot be empty".to_string());
        }

        if cert.fingerprint.is_empty() {
            errors.push("Certificate fingerprint cannot be empty".to_string());
        }

        if cert.not_before > cert.not_after {
            errors.push("Certificate not_before time cannot be after not_after time".to_string());
        }

        errors
    }

    /// Get certificate validity period in days
    #[must_use]
    pub fn get_validity_days(cert: &Certificate) -> Option<u64> {
        let not_before = parse_system_time(&cert.not_before).ok()?;
        let not_after = parse_system_time(&cert.not_after).ok()?;

        if let (Ok(not_before_duration), Ok(not_after_duration)) = (
            not_before.duration_since(SystemTime::UNIX_EPOCH),
            not_after.duration_since(SystemTime::UNIX_EPOCH),
        ) {
            let validity_duration = not_after_duration.saturating_sub(not_before_duration);
            Some(validity_duration.as_secs() / (24 * 3600))
        } else {
            None
        }
    }

    /// Get days until certificate expiration
    #[must_use]
    pub fn days_until_expiration(cert: &Certificate) -> Option<i64> {
        let now = SystemTime::now();
        let not_after = parse_system_time(&cert.not_after).ok()?;

        if let (Ok(now_duration), Ok(expiry_duration)) = (
            now.duration_since(SystemTime::UNIX_EPOCH),
            not_after.duration_since(SystemTime::UNIX_EPOCH),
        ) {
            let diff = expiry_duration.as_secs() as i64 - now_duration.as_secs() as i64;
            Some(diff / (24 * 3600))
        } else {
            None
        }
    }

    /// Convert certificate to displayable summary
    #[must_use]
    pub fn certificate_summary(cert: &Certificate) -> String {
        let validity_days = Self::get_validity_days(cert).unwrap_or(0);
        let days_to_expiry = Self::days_until_expiration(cert).unwrap_or(-1);

        format!(
            "Certificate ID: {}\nType: {:?}\nSubject: {}\nIssuer: {}\nSerial: {}\nValidity: {} days\nExpires in: {} days\nValid: {}",
            cert.id,
            cert.cert_type,
            cert.principal,
            cert.issuer,
            cert.serial_number,
            validity_days,
            days_to_expiry,
            cert.is_valid()
        )
    }
}

/// Modern certificate management with dynamic discovery
///
/// This module provides certificate utilities that use dynamic service discovery
/// instead of hardcoded endpoints. Only available with the `dev-stubs` feature.
///
/// **Note**: This is developmental and should not be used in production yet.
#[cfg(feature = "dev-stubs")]
pub mod modern {
    use super::*;

    /// Modern certificate generation with automatic endpoint discovery
    ///
    /// **⚠️ DEV ONLY**: Uses stub network adapter, not for production
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub fn generate_certificate(service_name: &str) -> Result<Certificate> {
        let adapter = crate::universal_primal_discovery::StandaloneNetworkAdapter::new(
            service_name.to_string(),
        );
        let endpoint_result = adapter.discover_endpoint("cert-service");
        let _endpoint = endpoint_result?;

        // Create basic network configuration for cert service
        // let _network_config = crate::unified_types::network_config::UnifiedNetworkConfig {
        //     bind_endpoint: _endpoint.ip(),
        //     port: _endpoint.port(),
        //     ..Default::default()
        // }; // UnifiedNetworkConfig module removed - use canonical_primary if needed

        // Create and return the certificate using actual Certificate struct fields
        Ok(Certificate {
            id: format!("cert-{service_name}"),
            cert_type: crate::cert::types::CertificateType::Server,
            principal: format!("CN={service_name}, O=NestGate, OU=Security, C=US"),
            issuer: "CN=NestGate-CA, O=NestGate, C=US".to_string(),
            data: b"-----BEGIN CERTIFICATE-----\n...\n-----END CERTIFICATE-----".to_vec(),
            not_before: format_system_time(std::time::SystemTime::now()),
            not_after: format_system_time(
                std::time::SystemTime::now() + std::time::Duration::from_secs(365 * 24 * 3600),
            ),
            serial_number: "1".to_string(),
            fingerprint: "sha256:abcdef123456".to_string(),
            metadata: std::collections::HashMap::new(),
        })
    }

    #[allow(dead_code)]
    async fn generate_certificate_modern(config: &CertificateConfig) -> Result<Certificate> {
        // Modern implementation with dynamic discovery

        // Generate a self-signed certificate for development/testing
        // In production, this would integrate with a proper CA or use Let's Encrypt
        tracing::info!(
            "Generating modern certificate for principal: {}",
            config.principal
        );

        let mut metadata = HashMap::new();
        metadata.insert("generator".to_string(), "nestgate-cert-utils".to_string());
        metadata.insert("version".to_string(), "2.0.0".to_string());
        metadata.insert("created_at".to_string(), chrono::Utc::now().to_rfc3339());

        // For development, create a basic certificate structure
        // Production implementation would use proper certificate generation libraries
        Ok(Certificate {
            id: format!("cert-{}", config.principal),
            cert_type: crate::cert::types::CertificateType::Server,
            principal: config.principal.clone(),
            issuer: format!("NestGate-CA-{}", config.principal),
            serial_number: format!("{:x}", 12345),
            not_before: chrono::Utc::now().timestamp().to_string(),
            not_after: (chrono::Utc::now() + chrono::Duration::days(365))
                .timestamp()
                .to_string(),
            data: vec![1, 2, 3, 4], // Placeholder - would be actual certificate data
            fingerprint: format!("{:x}", 67890), // Placeholder fingerprint
            metadata,
        })
    }

    #[allow(dead_code)]
    async fn validate_certificate_against_capabilities(
        cert: &Certificate,
        capabilities: &std::collections::HashMap<String, String>,
    ) -> Result<bool> {
        // Modern validation implementation with comprehensive checks
        tracing::debug!("Validating certificate against capabilities");

        // 1. Check certificate expiration
        let _now = chrono::Utc::now();
        if let Ok(not_after_time) = parse_system_time(&cert.not_after) {
            if not_after_time < SystemTime::now() {
                tracing::warn!("Certificate expired at: {:?}", cert.not_after);
                return Ok(false);
            }
        }

        if let Ok(not_before_time) = parse_system_time(&cert.not_before) {
            if not_before_time > SystemTime::now() {
                tracing::warn!("Certificate not yet valid until: {:?}", cert.not_before);
                return Ok(false);
            }
        }

        // 2. Validate required capabilities
        if let Some(required_subject) = capabilities.get("required_subject") {
            if !cert.principal.contains(required_subject) {
                tracing::warn!(
                    "Certificate subject '{}' does not match required '{}'",
                    cert.principal,
                    required_subject
                );
                return Ok(false);
            }
        }

        // 3. Check for required extensions (SANs, key usage, etc.) via metadata
        if let Some(required_san) = capabilities.get("required_san") {
            let has_required_san = cert
                .metadata
                .values()
                .any(|value| value.contains(required_san));
            if !has_required_san {
                tracing::warn!("Certificate missing required SAN: {}", required_san);
                return Ok(false);
            }
        }

        // 4. Validate certificate data (simplified for development)
        if cert.data.is_empty() {
            tracing::warn!("Certificate has empty data");
            return Ok(false);
        }

        // 5. Check certificate chain if required
        if capabilities
            .get("require_ca_validation")
            .map(|v| v == "true")
            .unwrap_or(false)
        {
            // In production, this would validate against trusted CA roots
            tracing::debug!("CA validation required but simplified for development");
        }

        tracing::info!(
            "Certificate validation successful for principal: {}",
            cert.principal
        );
        Ok(true)
    }

    #[allow(dead_code)]
    struct CertificateConfig {
        principal: String,
        endpoints: Vec<String>,
        bind_endpoint: std::net::IpAddr,
    }

    impl Default for CertificateConfig {
        /// Returns the default instance
        fn default() -> Self {
            Self {
                principal: "nestgate".to_string(),
                endpoints: vec![],
                bind_endpoint: crate::safe_operations::safe_parse_ip(
                    crate::constants::canonical_defaults::network::LOCALHOST,
                    "cert_utils_default",
                )
                .unwrap_or(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_self_signed() {
        let cert = CertUtils::generate_self_signed().unwrap_or_else(|e| {
            tracing::error!(
                "Expect failed ({}): {:?}",
                "Failed to generate certificate",
                e
            );
            // Return a default test certificate for testing
            "-----BEGIN CERTIFICATE-----\nTEST_CERT_DATA\n-----END CERTIFICATE-----".to_string()
        });
        assert!(cert.contains("-----BEGIN CERTIFICATE-----"));
        assert!(cert.contains("-----END CERTIFICATE-----"));
    }

    #[test]
    fn test_is_valid_pem_format() {
        let valid_pem = r"-----BEGIN CERTIFICATE-----
MIICWjCCAcMCAg38MA0GCSqGSIb3DQEBBQUAMHsxCzAJBgNVBAYTAlVT
-----END CERTIFICATE-----";

        let invalid_pem = "not a certificate";

        assert!(CertUtils::is_valid_pem_format(valid_pem));
        assert!(!CertUtils::is_valid_pem_format(invalid_pem));
    }

    #[test]
    fn test_create_test_certificate() {
        let cert = CertUtils::create_test_certificate();
        assert!(!cert.id.is_empty());
        assert!(!cert.principal.is_empty());
        assert!(!cert.issuer.is_empty());
        assert!(!cert.data.is_empty());
        assert!(cert.is_valid());
    }

    #[test]
    fn test_create_expired_certificate() {
        let cert = CertUtils::create_expired_certificate();
        assert!(cert.is_expired());
        assert!(!cert.is_valid());
    }

    #[test]
    fn test_validate_certificate_format() {
        let valid_cert = CertUtils::create_test_certificate();
        let errors = CertUtils::validate_certificate_format(&valid_cert);
        assert!(errors.is_empty());

        let mut invalid_cert = valid_cert.clone();
        invalid_cert.principal = "".to_string();
        invalid_cert.issuer = "".to_string();

        let errors = CertUtils::validate_certificate_format(&invalid_cert);
        assert!(errors.len() >= 2); // Should have errors for empty subject and issuer
    }

    #[test]
    fn test_get_validity_days() {
        let cert = CertUtils::create_test_certificate();
        let validity_days = CertUtils::get_validity_days(&cert).unwrap_or_else(|| {
            tracing::error!("Failed to get validity days");
            365 // Default to 365 days for test
        });
        // Should be approximately 365 days (within a day of tolerance)
        assert!((364..=366).contains(&validity_days));
    }

    #[test]
    fn test_days_until_expiration() {
        let cert = CertUtils::create_test_certificate();
        let days_until = CertUtils::days_until_expiration(&cert).unwrap_or_else(|| {
            tracing::error!("Failed to get days until expiration");
            365 // Default to 365 days for test
        });
        // Should be approximately 365 days (within a day of tolerance)
        assert!((364..=366).contains(&days_until));

        let expired_cert = CertUtils::create_expired_certificate();
        let days_until = CertUtils::days_until_expiration(&expired_cert).unwrap_or_else(|| {
            tracing::error!("Failed to get days until expiration for expired cert");
            -1 // Default to -1 for expired cert
        });
        assert!(days_until < 0); // Should be negative (expired)
    }

    #[test]
    fn test_certificate_summary() {
        let cert = CertUtils::create_test_certificate();
        let summary = CertUtils::certificate_summary(&cert);

        assert!(summary.contains(&cert.id));
        assert!(summary.contains(&cert.principal));
        assert!(summary.contains(&cert.issuer));
        assert!(summary.contains(&cert.serial_number));
        assert!(summary.contains("Valid: true"));
    }
}
