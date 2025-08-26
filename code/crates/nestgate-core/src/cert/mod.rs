use serde::{Deserialize, Serialize};

/// Certificate management and validation for NestGate
///
/// ## Example
/// ```rust
/// use crate::cert::CertificateConfig;
/// let config = CertificateConfig::default();
/// ```
use std::path::PathBuf;
// REMOVED: unused imports

// Certificate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateConfig {
    /// Certificate file path
    pub cert_path: PathBuf,
    /// Private key file path  
    pub key_path: PathBuf,
    /// CA certificate path (optional)
    pub ca_path: Option<PathBuf>,
    /// Certificate validity period in days
    pub validity_days: u32,
}

impl Default for CertificateConfig {
    fn default() -> Self {
        Self {
            cert_path: PathBuf::from("cert.pem"),
            key_path: PathBuf::from("key.pem"),
            ca_path: None,
            validity_days: 365,
        }
    }
}

pub mod manager;
pub mod types;
pub mod utils;
pub mod validator;
