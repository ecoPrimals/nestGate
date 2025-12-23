/// Simple certificate types and validation for API compatibility
use serde::{Deserialize, Serialize};
/// Certificate validation mode
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
/// Certmode
pub enum CertMode {
    /// Strict certificate validation
    Strict,
    /// Lenient certificate validation
    Lenient,
    /// Development mode (minimal validation)
    #[default]
    /// Development
    Development,
    /// Custom validation mode
    Custom(String),
}
/// Certificate types module
pub mod types {
    pub use super::CertMode;
}
/// Certificate validator module
pub mod validator {
    use super::*;
    /// Simple certificate validator
    #[derive(Debug, Clone)]
    /// Certificatevalidator
    pub struct CertificateValidator {
        mode: CertMode,
    }

    impl CertificateValidator {
        /// Creates a new instance
        pub fn new(mode: CertMode) -> Self {
            Self { mode }
        }

        /// Mode
        pub fn mode(&self) -> &CertMode {
            &self.mode
        }
    }

    /// Create default certificate validator
    pub fn create_default_certificate_validator() -> CertificateValidator {
        CertificateValidator::new(CertMode::Development)
    }
}
