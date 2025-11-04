//! # Security Hardening Configuration
//! Configuration types and utilities.
// Configuration structures for production security hardening

use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::time::Duration;

/// **SECURITY HARDENING CONFIGURATION**
#[derive(Debug, Clone)]
pub struct SecurityHardeningConfig {
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// Input validation configuration
    pub validation: ValidationConfig,
    /// Audit logging configuration
    pub audit_logging: AuditConfig,
    /// Intrusion detection configuration
    pub intrusion_detection: IntrusionDetectionConfig,
    /// Security headers configuration
    pub security_headers: SecurityHeadersConfig,
}
/// **RATE LIMITING CONFIGURATION**
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Requests per minute per IP
    pub requests_per_minute: u32,
    /// Burst allowance
    pub burst_size: u32,
    /// Block duration for rate limit violations
    pub block_duration: Duration,
    /// Allowlist of IPs exempt from rate limiting
    pub allowlist: Vec<IpAddr>,
}
/// **REQUEST VALIDATION CONFIGURATION**
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum request size in bytes
    pub max_request_size: usize,
    /// Maximum number of headers
    pub max_headers: usize,
    /// Maximum header value length
    pub max_header_length: usize,
    /// Allowed HTTP methods
    pub allowed_methods: Vec<String>,
    /// SQL injection detection enabled
    pub sql_injection_detection: bool,
    /// XSS detection enabled
    pub xss_detection: bool,
}
/// **AUDIT LOGGING CONFIGURATION**
#[derive(Debug, Clone)]
pub struct AuditConfig {
    /// Enable security audit logging
    pub enabled: bool,
    /// Log level for security events
    pub log_level: SecurityLogLevel,
    /// Include request details in audit logs
    pub include_request_details: bool,
    /// Include response details in audit logs
    pub include_response_details: bool,
}
/// **INTRUSION DETECTION CONFIGURATION**
#[derive(Debug, Clone)]
pub struct IntrusionDetectionConfig {
    /// Enable intrusion detection
    pub enabled: bool,
    /// Failed authentication threshold
    pub failed_auth_threshold: u32,
    /// Time window for failed authentication attempts
    pub failed_auth_window: Duration,
    /// Suspicious pattern detection enabled
    pub pattern_detection: bool,
    /// Automatic blocking enabled
    pub auto_blocking: bool,
}
/// **SECURITY HEADERS CONFIGURATION**
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    /// Content Security Policy
    pub csp: Option<String>,
    /// HTTP Strict Transport Security
    pub hsts: bool,
    /// X-Frame-Options
    pub x_frame_options: String,
    /// X-Content-Type-Options
    pub x_content_type_options: bool,
    /// X-XSS-Protection
    pub x_xss_protection: bool,
}
/// **SECURITY LOG LEVELS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLogLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}
/// **THREAT LEVELS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Critical,
    High,
    Medium,
    Low,
}
/// **SECURITY EVENT TYPES**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    RateLimitExceeded,
    InvalidInput,
    SqlInjectionAttempt,
    XssAttempt,
    AuthenticationFailure,
    SuspiciousActivity,
    IntrusionAttempt,
    AccessDenied,
    SecurityHeaderViolation,
}
/// **SECURITY ACTIONS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAction {
    Logged,
    Blocked,
    RateLimited,
    Sanitized,
    Rejected,
    Monitored,
}
impl Default for SecurityHardeningConfig {
    fn default() -> Self {
        Self {
            rate_limiting: RateLimitConfig {
                requests_per_minute: 1000,
                burst_size: 100,
                block_duration: Duration::from_secs(300),
                allowlist: Vec::new(),
            },
            validation: ValidationConfig {
                max_request_size: 10 * 1024 * 1024, // 10MB
                max_headers: 50,
                max_header_length: 8192,
                allowed_methods: vec![
                    "GET".to_string(),
                    "POST".to_string(),
                    "PUT".to_string(),
                    "DELETE".to_string(),
                    "PATCH".to_string(),
                    "OPTIONS".to_string(),
                ],
                sql_injection_detection: true,
                xss_detection: true,
            },
            audit_logging: AuditConfig {
                enabled: true,
                log_level: SecurityLogLevel::Medium,
                include_request_details: true,
                include_response_details: false,
            },
            intrusion_detection: IntrusionDetectionConfig {
                enabled: true,
                failed_auth_threshold: 5,
                failed_auth_window: Duration::from_secs(300),
                pattern_detection: true,
                auto_blocking: true,
            },
            security_headers: SecurityHeadersConfig {
                csp: Some("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';".to_string()),
                hsts: true,
                x_frame_options: "DENY".to_string(),
                x_content_type_options: true,
                x_xss_protection: true,
            },
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_request_size: 10 * 1024 * 1024, // 10MB
            max_headers: 50,
            max_header_length: 8192,
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
                "OPTIONS".to_string(),
            ],
            sql_injection_detection: true,
            xss_detection: true,
        }
    }
} 