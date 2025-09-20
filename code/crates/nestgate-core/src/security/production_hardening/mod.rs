//! # NestGate Production Security Hardening
//! Module definitions and exports.
// **ENTERPRISE-GRADE SECURITY HARDENING** for production deployments
//! Module definitions and exports.
// This module implements comprehensive security hardening measures that protect
// NestGate in production environments, following canonical modernization principles
//! and zero-trust security architecture.

// Module declarations
pub mod audit;
pub mod config;
pub mod intrusion_detection;
pub mod manager;
pub mod rate_limiting;
pub mod types;
pub mod validation;

// Re-export all public items for backward compatibility
pub use audit::SecurityAuditLogger;
pub use config::{
    AuditConfig, IntrusionDetectionConfig, RateLimitConfig, SecurityAction, SecurityEventType,
    SecurityHardeningConfig, SecurityHeadersConfig, SecurityLogLevel, ThreatLevel, ValidationConfig,
};
pub use intrusion_detection::IntrusionDetectionSystem;
pub use manager::SecurityHardeningManager;
pub use rate_limiting::RateLimiter;
pub use types::{
    IntrusionDetectionStatistics, RateLimitBucket, RateLimitStatistics, RequestDetails,
    RequestValidationResult, SecurityEvent, SecurityMetrics, SecurityValidationResult,
    SuspiciousPattern,
};
pub use validation::RequestValidator; 