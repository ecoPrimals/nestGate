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
/// ```rust
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
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
pub struct CertificateConfig {
    /// Certificate file path
    /// Private key file path  
    /// CA certificate path (optional)
    /// Certificate validity period in days
    pub validity_days: u32,
}

impl Default for CertificateConfig {
    fn default() -> Self {
        Self { validity_days: 365 }
    }
}

pub mod manager;
pub mod types;
pub mod utils;
pub mod validator;

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
pub type CertificateConfigCanonical = crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using CertificateConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

