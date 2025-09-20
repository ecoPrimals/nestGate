//! # Production Security Hardening Modules
//! Module definitions and exports.
// **ENTERPRISE-GRADE SECURITY HARDENING** split into logical modules
//! for better maintainability and adherence to the 1000-line limit.

pub mod config;
pub mod manager;
pub mod rate_limiter;
pub mod validator;
pub mod audit_logger;
pub mod intrusion_detection;
pub mod types;

// Re-export main types for backward compatibility
pub use config::*;
pub use manager::SecurityHardeningManager;
pub use rate_limiter::RateLimiter;
pub use validator::RequestValidator;
pub use audit_logger::SecurityAuditLogger;
pub use intrusion_detection::IntrusionDetectionSystem;
pub use types::*; 