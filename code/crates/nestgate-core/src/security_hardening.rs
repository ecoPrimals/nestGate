//! # Security Hardening
//!
//! Comprehensive security hardening measures for production deployment,
//! including input validation, rate limiting, encryption, and security monitoring.
//!
//! ## Modules
//!
//! - `validation`: Input validation and sanitization
//! - `rate_limiting`: Rate limiting with token bucket algorithm
//! - `monitoring`: Security event monitoring and threat detection
//! - `encryption`: Data encryption and key management

pub mod validation;
pub mod rate_limiting;
pub mod monitoring;
pub mod encryption;

// Re-export all public types for backwards compatibility
pub use validation::{SecurityValidator, ValidationRule, ValidationType, ValidationResult};
pub use rate_limiting::{RateLimiter, RateLimit, RateLimitResult};
pub use monitoring::{
    SecurityMonitor, SecurityEvent, SecurityEventType, SecuritySeverity,
    ThreatPattern, ThreatAction, ThreatAnalysisResult,
};
pub use encryption::{EncryptionManager, EncryptedData};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_integration() {
        // Ensure all modules can be used together
        let _validator = SecurityValidator::new();
        let _limiter = RateLimiter::new();
        let _monitor = SecurityMonitor::new(1000);
        let _encryptor = EncryptionManager::new();
    }
}
