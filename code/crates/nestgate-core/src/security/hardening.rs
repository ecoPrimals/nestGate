use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module implements additional security measures to protect against
// common vulnerabilities and ensure robust security posture.

use crate::{Result};
use std::time::{Duration, SystemTime};
use tracing::{debug, warn};

/// Rate limiting configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests per window
    pub max_requests: u32,
    /// Time window duration
    pub window_duration: Duration,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
}
impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_duration: Duration::from_secs(60),
            cleanup_interval: Duration::from_secs(300),
        }
    }
}

/// Rate limiter for API endpoints
#[derive(Debug)]
pub struct RateLimiter {
    config: RateLimitConfig,
    requests: HashMap<String, Vec<SystemTime>>,
    last_cleanup: SystemTime,
}
impl RateLimiter {
    /// Create a new rate limiter
    #[must_use]
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            config,
            requests: HashMap::new(),
            last_cleanup: SystemTime::now(),
        }
    }

    /// Check if a request should be allowed
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn check_rate_limit(&mut self, client_id: &str) -> Result<bool>  {
        let now = SystemTime::now();

        // Periodic cleanup of expired entries
        if now.duration_since(self.last_cleanup).unwrap_or_default() > self.config.cleanup_interval
        {
            self.cleanup_expired_entries(now);
            self.last_cleanup = now;
        }

        let client_requests = self.requests.entry(client_id.to_string()).or_default();

        // Remove requests outside the current window
        client_requests.retain(|&request_time| {
            now.duration_since(request_time).unwrap_or(Duration::MAX) <= self.config.window_duration
        );

        // Check if limit is exceeded
        if client_requests.len() >= self.config.max_requests as usize {
            warn!("Rate limit exceeded for client: {}", client_id);
            return Ok(false);
        }

        // Record the current request
        client_requests.push(now);
        debug!(
            "Rate limit check passed for client: {} ({}/{})",
            client_id,
            client_requests.len(),
            self.config.max_requests
        );

        Ok(true)
    }

    /// Clean up expired entries
    fn cleanup_expired_entries(&mut self, now: SystemTime) {
        self.requests.retain(|_, requests| {
            requests.retain(|&request_time| {
                now.duration_since(request_time).unwrap_or(Duration::MAX)
                    <= self.config.window_duration
            );
            !requests.is_empty()
        );
    }
}

/// Input validation utilities
pub struct InputValidator;
impl InputValidator {
    /// Validate that input contains only safe characters
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_safe_string(input: &str, max_length: usize) -> Result<()>  {
        if input.len() > max_length {
            return Err(NestGateError::security(
                "Input exceeds maximum length",
                "validate_input",
                Some("input_validation"),
                None,
            ));
        }

        // Check for potentially dangerous characters
        if input.contains('\0') || input.contains('\x1b') {
            return Err(NestGateError::security(
                "Input contains potentially dangerous characters",
                "validate_input",
                Some("input_validation"),
                None,
            ));
        }

        // Check for SQL injection patterns (basic)
        let dangerous_patterns = ["'", "\"", ";", "--", "/*", "*/", "xp_", "sp_"];
        for pattern in &dangerous_patterns {
            if input.to_lowercase().contains(pattern) {
                warn!(
                    "Potentially dangerous pattern detected in input: {}",
                    pattern
                );
                return Err(NestGateError::security(
                    "Input contains potentially dangerous SQL patterns",
                    "validate_input",
                    Some("sql_injection_prevention"),
                    None,
                ));
            }
        }

        Ok(())
    }

    /// Validate file paths to prevent directory traversal
        // Check for directory traversal attempts
        if path.contains("..") || path.contains("./") || path.starts_with('/') {
            return Err(NestGateError::security(
                "Path contains directory traversal patterns",
                "validate_path",
                Some("directory_traversal_prevention"),
                None,
            ));
        }

        // Check for null bytes
        if path.contains('\0') {
            return Err(NestGateError::security(
                "Path contains null bytes",
                "validate_path",
                Some("path_validation"),
                None,
            ));
        }

        Ok(())
    }

    /// Validate command arguments to prevent injection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub fn validate_command_args(args: &[String]) -> Result<()>  {
        for arg in args {
            // Check for command injection patterns
            let dangerous_chars = ['|', '&', ';', '>', '<', '`', '$', '(', ')'];
            if arg.chars().any(|c| dangerous_chars.contains(&c)) {
                return Err(NestGateError::security(
                    "Command argument contains potentially dangerous characters",
                    "validate_command",
                    Some("command_injection_prevention"),
                    None,
                ));
            }
        }
        Ok(())
    }
}

/// Security headers for HTTP responses
pub struct SecurityHeaders;
impl SecurityHeaders {
    /// Get recommended security headers
    pub fn get_security_headers() -> HashMap<String, String> {
        let mut headers = HashMap::new();

        // Prevent XSS attacks
        headers.insert("X-XSS-Protection".to_string(), "1; mode=block".to_string());

        // Prevent MIME type sniffing
        headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());

        // Prevent clickjacking
        headers.insert("X-Frame-Options".to_string(), "DENY".to_string());

        // Enforce HTTPS
        headers.insert(
            "Strict-Transport-Security".to_string(),
            "max-age=31536000; includeSubDomains".to_string(),
        );

        // Content Security Policy
        headers.insert(
            "Content-Security-Policy".to_string(),
            "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'".to_string(),
        );

        // Referrer policy
        headers.insert(
            "Referrer-Policy".to_string(),
            "strict-origin-when-cross-origin".to_string(),
        );

        headers
    }
}

/// Audit logging for security events
pub struct SecurityAuditor;
impl SecurityAuditor {
    /// Log authentication events
    pub fn log_auth_event(event_type: &str, user: &str, success: bool, details: Option<&str>) {
        if success {
            tracing::info!(
                event_type = event_type,
                user = user,
                success = success,
                details = details.unwrap_or("N/A"),
                "Authentication event"
            );
        } else {
            tracing::warn!(
                event_type = event_type,
                user = user,
                success = success,
                details = details.unwrap_or("N/A"),
                "Authentication failure"
            );
        }
    }

    /// Log authorization events
        if allowed {
            tracing::info!(
                user = user,
                resource = resource,
                action = action,
                allowed = allowed,
                "Authorization granted"
            );
        } else {
            tracing::warn!(
                user = user,
                resource = resource,
                action = action,
                allowed = allowed,
                "Authorization denied"
            );
        }
    }

    /// Log security violations
    pub fn log_security_violation(violation_type: &str, source: &str, details: &str) {
        tracing::error!(
            violation_type = violation_type,
            source = source,
            details = details,
            "Security violation detected"
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validation() {
        // Valid input should pass
        assert!(InputValidator::validate_safe_string("valid_input", 100).is_ok());

        // Too long input should fail
        assert!(InputValidator::validate_safe_string("x".repeat(1000).as_str(), 100).is_err());

        // Dangerous characters should fail
        assert!(InputValidator::validate_safe_string("test'; DROP TABLE users;--", 100).is_err());
    }

    #[test]
    fn test_path_validation() {
        // Valid path should pass
        assert!(InputValidator::validate_file_path("valid/path.txt").is_ok());

        // Directory traversal should fail
        assert!(InputValidator::validate_file_path("../../../etc/passwd").is_err());
        assert!(InputValidator::validate_file_path("./secret").is_err());
        assert!(InputValidator::validate_file_path("/absolute/path").is_err());
    }

    #[test]
    fn test_command_validation() {
        // Valid args should pass
        let valid_args = vec!["arg1".to_string(), "arg2".to_string()];
        assert!(InputValidator::validate_command_args(&valid_args).is_ok());

        // Dangerous args should fail
        let dangerous_args = vec!["arg1; rm -rf /".to_string()];
        assert!(InputValidator::validate_command_args(&dangerous_args).is_err());
    }

    #[tokio::test]
    async fn test_rate_limiter() -> crate::Result<()> {
        let config = RateLimitConfig {
            max_requests: 2,
            window_duration: Duration::from_secs(1),
            cleanup_interval: Duration::from_secs(10),
        };

        let mut limiter = RateLimiter::new(config);

        // First two requests should pass
        assert!(limiter.check_rate_limit("client1")?);
        assert!(limiter.check_rate_limit("client1")?);

        // Third request should fail
        assert!(!limiter.check_rate_limit("client1")?);

        // Different client should still work
        assert!(limiter.check_rate_limit("client2")?);

        Ok(())
    }
}
