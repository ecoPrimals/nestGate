// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Cert module

use serde::{Deserialize, Serialize};

// Certificate management and validation for NestGate
//
// ## Example
// ```rust
// use crate::cert::CertificateConfig;
// let config = CertificateConfig::default();
// ```
// Certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
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
    /// Private key file path  
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
#[allow(deprecated)]
/// Type alias for Certificateconfigcanonical
pub type CertificateConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CertificateConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
