//! # Security Hardening Configuration
//! Configuration types and utilities.
// Configuration types for enterprise-grade security hardening

use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
use serde::{Deserialize, Serialize};

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
    pub suspicious_pattern_detection: bool,
    /// Geographic restriction enabled
    pub geographic_restrictions: bool,
    /// Allowed countries (ISO country codes)
    pub allowed_countries: Vec<String>,
}
/// **SECURITY HEADERS CONFIGURATION**
#[derive(Debug, Clone)]
pub struct SecurityHeadersConfig {
    /// Enable security headers
    pub enabled: bool,
    /// Content Security Policy
    pub csp: Option<String>,
    /// X-Frame-Options
    pub x_frame_options: Option<String>,
    /// X-Content-Type-Options
    pub x_content_type_options: bool,
    /// X-XSS-Protection
    pub x_xss_protection: bool,
    /// Strict-Transport-Security
    pub hsts: Option<String>,
}
/// **SECURITY LOG LEVELS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityLogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}
impl Default for SecurityHardeningConfig {
    fn default() -> Self {
        Self {
            rate_limiting: RateLimitConfig::default(),
            validation: ValidationConfig::default(),
            audit_logging: AuditConfig::default(),
            intrusion_detection: IntrusionDetectionConfig::default(),
            security_headers: SecurityHeadersConfig::default(),
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_minute: 100,
            burst_size: 20,
            block_duration: Duration::from_secs(300), // 5 minutes
            allowlist: Vec::new(),
        }
    }
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_request_size: 1024 * 1024, // 1MB
            max_headers: 50,
            max_header_length: 8192,
            allowed_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
            ],
            sql_injection_detection: true,
            xss_detection: true,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_level: SecurityLogLevel::Info,
            include_request_details: true,
            include_response_details: false,
        }
    }
}

impl Default for IntrusionDetectionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failed_auth_threshold: 5,
            failed_auth_window: Duration::from_secs(300), // 5 minutes
            suspicious_pattern_detection: true,
            geographic_restrictions: false,
            allowed_countries: Vec::new(),
        }
    }
}

impl Default for SecurityHeadersConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            csp: Some("default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';".to_string()),
            x_frame_options: Some("DENY".to_string()),
            x_content_type_options: true,
            x_xss_protection: true,
            hsts: Some("max-age=31536000; includeSubDomains".to_string()),
        }
    }
} 