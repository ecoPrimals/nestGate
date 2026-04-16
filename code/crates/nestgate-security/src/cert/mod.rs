// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cert module

use serde::{Deserialize, Serialize};

// Certificate management and validation for NestGate
//
// ## Example
// ```rust
// ```
// Certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::CertificateConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::CertificateConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for Certificate
pub struct CertificateConfig {
    /// Certificate file path
    /// Private key file path\
    /// CA certificate path (optional)
    /// Certificate validity period in days
    pub validity_days: u32,
}

impl Default for CertificateConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self { validity_days: 365 }
    }
}

/// Certificate lifecycle management including issuance, renewal, and revocation.
pub mod manager;
/// Core certificate types and data structures for X.509 certificates.
pub mod types;
/// Certificate utility functions for encoding, decoding, and validation.
pub mod utils;
/// Certificate validation logic for trust chains and expiration checking.
pub mod validator;

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Certificateconfigcanonical
pub type CertificateConfigCanonical =
    nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CertificateConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

#[cfg(test)]
mod certificate_config_tests {
    use super::CertificateConfig;
    use serde_json::json;

    #[test]
    #[expect(deprecated)]
    fn certificate_config_default_and_serde_roundtrip() {
        let c = CertificateConfig::default();
        assert_eq!(c.validity_days, 365);
        let v = json!({ "validity_days": c.validity_days });
        let back: CertificateConfig =
            serde_json::from_value(v).expect("serde_json from_value CertificateConfig");
        assert_eq!(back.validity_days, 365);
    }
}

#[cfg(test)]
mod cert_module_tests {
    use std::time::{Duration, SystemTime};

    use super::utils::{CertUtils, format_system_time, parse_system_time};

    #[test]
    fn format_parse_system_time_roundtrip() {
        let t = SystemTime::UNIX_EPOCH + Duration::from_secs(12_345);
        let s = format_system_time(t);
        let back = parse_system_time(&s).expect("parse_system_time");
        assert_eq!(back, t);
    }

    #[test]
    fn parse_system_time_rejects_non_numeric() {
        assert!(parse_system_time("not-a-number").is_err());
    }

    #[test]
    fn cert_utils_fingerprint_is_hex_sha256() {
        let fp = CertUtils::calculate_fingerprint(b"hello");
        assert_eq!(fp.len(), 64);
        assert_eq!(fp, CertUtils::calculate_fingerprint(b"hello"));
    }

    #[test]
    fn cert_utils_generate_self_signed_returns_not_implemented() {
        let err = CertUtils::generate_self_signed().expect_err("not implemented");
        assert!(err.to_string().to_lowercase().contains("implement"));
    }
}
