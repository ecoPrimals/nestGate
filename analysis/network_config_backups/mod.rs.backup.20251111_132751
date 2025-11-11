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
