//! # Request Validation
//! Validation functionality and utilities.
// Comprehensive request validation and sanitization

use std::collections::HashMap;
use std::time::SystemTime;

use crate::error::Result;
use super::config::{SecurityAction, SecurityEventType, ThreatLevel, ValidationConfig};
use super::types::{RequestValidationResult, SecurityEvent};

/// **REQUEST VALIDATOR**
///
/// Comprehensive request validation and sanitization
pub struct RequestValidator {
    /// Validation configuration
    config: ValidationConfig,
    /// SQL injection patterns
    sql_injection_patterns: Vec<regex::Regex>,
    /// XSS patterns
    xss_patterns: Vec<regex::Regex>,
}
impl RequestValidator {
    /// Create new request validator
    pub fn new(config: ValidationConfig) -> Self {
        let sql_injection_patterns = vec![
            regex::Regex::new(r"(?i)(union\s+select|select\s+.*\s+from|drop\s+table|delete\s+from|insert\s+into)").unwrap(),
            regex::Regex::new(r"(?i)(\'\s*or\s+\'\s*1\s*=\s*1|--|\#|/\*|\*/)").unwrap(),
        ];

        let xss_patterns = vec![
            regex::Regex::new(r"(?i)(<script|javascript:|on\w+\s*=)").unwrap(),
            regex::Regex::new(r"(?i)(eval\s*\(|expression\s*\(|vbscript:)").unwrap(),
        ];

        Self {
            config,
            sql_injection_patterns,
            xss_patterns,
        }
    }

    /// Validate incoming request
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_request(
        &self,
        method: &str,
        headers: &HashMap<String, String>,
        body: &[u8],
    ) -> Result<RequestValidationResult>  {
        let mut events = Vec::new();
        let mut blocked = false;

        // 1. Check HTTP method
        if !self.config.allowed_methods.contains(&method.to_string()) {
            events.push(SecurityEvent {
                timestamp: SystemTime::now(),
                event_type: SecurityEventType::InvalidInput,
                source_ip: None,
                user_id: None,
                request_details: None,
                threat_level: ThreatLevel::Medium,
                description: format!("Disallowed HTTP method: {method}"),
                action_taken: SecurityAction::Rejected,
            });
            blocked = true;
        }

        // 2. Check request size
        if body.len() > self.config.max_request_size {
            events.push(SecurityEvent {
                timestamp: SystemTime::now(),
                event_type: SecurityEventType::InvalidInput,
                source_ip: None,
                user_id: None,
                request_details: None,
                threat_level: ThreatLevel::Medium,
                description: format!("Request size {} exceeds limit {}", body.len(), self.config.max_request_size),
                action_taken: SecurityAction::Rejected,
            });
            blocked = true;
        }

        // 3. Check header count and length
        if headers.len() > self.config.max_headers {
            events.push(SecurityEvent {
                timestamp: SystemTime::now(),
                event_type: SecurityEventType::InvalidInput,
                source_ip: None,
                user_id: None,
                request_details: None,
                threat_level: ThreatLevel::Medium,
                description: format!("Too many headers: {}", headers.len()),
                action_taken: SecurityAction::Rejected,
            });
            blocked = true;
        }

        for (name, value) in headers {
            if name.len() + value.len() > self.config.max_header_length {
                events.push(SecurityEvent {
                    timestamp: SystemTime::now(),
                    event_type: SecurityEventType::InvalidInput,
                    source_ip: None,
                    user_id: None,
                    request_details: None,
                    threat_level: ThreatLevel::Medium,
                    description: format!("Header too long: {name}"),
                    action_taken: SecurityAction::Rejected,
                });
                blocked = true;
            }
        }

        // 4. SQL injection detection
        if self.config.sql_injection_detection {
            let combined_input = format!("{} {}", path, String::from_utf8_lossy(body));
            for pattern in &self.sql_injection_patterns {
                if pattern.is_match(&combined_input) {
                    events.push(SecurityEvent {
                        timestamp: SystemTime::now(),
                        event_type: SecurityEventType::SqlInjectionAttempt,
                        source_ip: None,
                        user_id: None,
                        request_details: None,
                        threat_level: ThreatLevel::High,
                        description: "SQL injection attempt detected".to_string(),
                        action_taken: SecurityAction::Blocked,
                    });
                    blocked = true;
                    break;
                }
            }
        }

        // 5. XSS detection
        if self.config.xss_detection {
            let combined_input = format!("{} {}", path, String::from_utf8_lossy(body));
            for pattern in &self.xss_patterns {
                if pattern.is_match(&combined_input) {
                    events.push(SecurityEvent {
                        timestamp: SystemTime::now(),
                        event_type: SecurityEventType::XssAttempt,
                        source_ip: None,
                        user_id: None,
                        request_details: None,
                        threat_level: ThreatLevel::High,
                        description: "XSS attempt detected".to_string(),
                        action_taken: SecurityAction::Blocked,
                    });
                    blocked = true;
                    break;
                }
            }
        }

        Ok(RequestValidationResult {
            blocked,
            security_events: events,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sql_injection_detection() {
        let config = ValidationConfig {
            sql_injection_detection: true,
            ..Default::default()
        };
        let validator = RequestValidator::new(config);

        let headers = HashMap::new();
        let malicious_body = b"'; DROP TABLE users; --";

        let result = validator
            .validate_request("POST", "/api/data", &headers, malicious_body)
            .await
            .unwrap();

        assert!(result.blocked);
        assert!(!result.security_events.is_empty());
        assert!(matches!(
            result.security_events[0].event_type,
            SecurityEventType::SqlInjectionAttempt
        ));
    }

    #[tokio::test]
    async fn test_xss_detection() {
        let config = ValidationConfig {
            xss_detection: true,
            ..Default::default()
        };
        let validator = RequestValidator::new(config);

        let headers = HashMap::new();
        let malicious_body = b"<script>alert('xss')</script>";

        let result = validator
            .validate_request("POST", "/api/data", &headers, malicious_body)
            .await
            .unwrap();

        assert!(result.blocked);
        assert!(!result.security_events.is_empty());
        assert!(matches!(
            result.security_events[0].event_type,
            SecurityEventType::XssAttempt
        ));
    }
} 