//! # Security Hardening Types
//! Type definitions and data structures.
// Core types and data structures for security hardening

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::SystemTime;

use super::config::{SecurityAction, SecurityEventType, ThreatLevel};

/// **SECURITY EVENT**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event timestamp
    pub timestamp: SystemTime,
    /// Event type
    pub event_type: SecurityEventType,
    /// Source IP address
    pub source_ip: Option<IpAddr>,
    /// User ID (if authenticated)
    pub user_id: Option<String>,
    /// Request details
    pub request_details: Option<RequestDetails>,
    /// Threat level
    pub threat_level: ThreatLevel,
    /// Description
    pub description: String,
    /// Action taken
    pub action_taken: SecurityAction,
}
/// **REQUEST DETAILS**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestDetails {
    /// HTTP method
    pub method: String,
    /// Request path
    /// User agent
    pub user_agent: Option<String>,
    /// Request size
    pub size: usize,
    /// Header count
    pub header_count: usize,
}
/// **SECURITY VALIDATION RESULT**
#[derive(Debug)]
pub struct SecurityValidationResult {
    /// Whether the request is allowed
    pub allowed: bool,
    /// Security events generated
    pub security_events: Vec<SecurityEvent>,
    /// Security headers to add to response
    pub security_headers: HashMap<String, String>,
}
/// **REQUEST VALIDATION RESULT**
#[derive(Debug)]
pub struct RequestValidationResult {
    /// Whether the request should be blocked
    pub blocked: bool,
    /// Security events generated
    pub security_events: Vec<SecurityEvent>,
}
/// **SECURITY METRICS**
#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityMetrics {
    /// Rate limiting statistics
    pub rate_limit_stats: RateLimitStatistics,
    /// Intrusion detection statistics
    pub ids_stats: IntrusionDetectionStatistics,
    /// Total security events
    pub total_security_events: u64,
    /// Blocked requests
    pub blocked_requests: u64,
    /// Blocked IPs count
    pub blocked_ips: usize,
}
/// **RATE LIMIT STATISTICS**
#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitStatistics {
    /// Active buckets count
    pub active_buckets: usize,
    /// Total requests processed
    pub total_requests: u64,
    /// Rate limited requests
    pub rate_limited_requests: u64,
    /// Blocked IPs count
    pub blocked_ips_count: usize,
}
/// **INTRUSION DETECTION STATISTICS**
#[derive(Debug, Serialize, Deserialize)]
pub struct IntrusionDetectionStatistics {
    /// Failed authentication attempts
    pub failed_auth_attempts: u64,
    /// Blocked IPs count
    pub blocked_ips_count: usize,
    /// Suspicious patterns detected
    pub suspicious_patterns_detected: u64,
    /// Auto-blocked IPs
    pub auto_blocked_ips: u64,
}
/// **RATE LIMIT BUCKET**
#[derive(Debug, Clone)]
pub struct RateLimitBucket {
    /// Request timestamps in sliding window
    pub requests: Vec<SystemTime>,
    /// Current token count
    pub tokens: u32,
    /// Last refill time
    pub last_refill: SystemTime,
    /// Block until time (if blocked)
    pub blocked_until: Option<SystemTime>,
}
/// **SUSPICIOUS PATTERNS**
#[derive(Debug, Clone)]
pub struct SuspiciousPattern {
    /// Pattern name
    pub name: String,
    /// Pattern regex
    pub pattern: regex::Regex,
    /// Threat level
    pub threat_level: ThreatLevel,
} 