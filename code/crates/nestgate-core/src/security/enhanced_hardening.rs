use crate::NestGateError;
use std::collections::HashMap;
//
// This module provides additional security hardening measures beyond the basic
// safe operations, including rate limiting, input sanitization, and attack prevention.

use crate::{Result, NestGateError};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Type alias for sliding window storage
type SlidingWindowMap = Arc<RwLock<HashMap<String, SlidingWindow>>>;

/// Enhanced rate limiter with sliding window
pub struct EnhancedRateLimiter {
    window_size: Duration,
    max_requests: usize,
    windows: SlidingWindowMap,
}

struct SlidingWindow {
    requests: Vec<Instant>,
    last_cleanup: Instant,
}

impl EnhancedRateLimiter {
    /// Create new enhanced rate limiter
    pub fn new(window_size: Duration, max_requests: usize) -> Self {
        Self {
            windows: Arc::new(RwLock::new(HashMap::new())),
            window_size,
            max_requests,
        }
    }

    /// Check if request is allowed for given identifier
    pub async fn is_allowed(&self, identifier: &str) -> Result<bool> {
        let mut windows = self.windows.write().await;
        let now = Instant::now();

        let window = windows
            .entry(identifier.to_string())
            .or_insert_with(|| SlidingWindow {
                requests: Vec::new(),
                last_cleanup: now,
            });

        // Clean old requests
        let cutoff = now - self.window_size;
        window.requests.retain(|&time| time > cutoff);

        // Check if under limit
        if window.requests.len() < self.max_requests {
            window.requests.push(now);
            Ok(true)
        } else {
            tracing::warn!(
                "Rate limit exceeded for identifier: {} ({} requests in {:?})",
                identifier,
                window.requests.len(),
                self.window_size
            );
            Ok(false)
        }
    }

    /// Clean up old windows periodically
    pub async fn cleanup_old_windows(&self) {
        let mut windows = self.windows.write().await;
        let now = Instant::now();
        let cleanup_threshold = self.window_size * 2;

        windows.retain(|_, window| now.duration_since(window.last_cleanup) < cleanup_threshold);
    }
}

/// Input sanitization and validation utilities
pub struct InputSanitizer;

impl InputSanitizer {
    /// Sanitize string input to prevent injection attacks
    pub fn sanitize_string(input: &str, max_length: usize) -> Result<String> {
        if input.len() > max_length {
            return Err(NestGateError::Validation {
                field: "input".to_string(),
                message: format!("Input too long: {} > {max_length}", input.len()),
                current_value: Some(input[..50.min(input.len())].to_string()),
                expected: Some(format!("Length <= {max_length}")),
                user_error: true,
            });
        }

        // Remove potentially dangerous characters
        let sanitized = input
            .chars()
            .filter(|&c| c.is_alphanumeric() || c.is_whitespace() || "-_./".contains(c))
            .collect::<String>();

        // Check for common injection patterns
        let dangerous_patterns = [
            "script",
            "javascript",
            "vbscript",
            "onload",
            "onerror",
            "SELECT",
            "INSERT",
            "UPDATE",
            "DELETE",
            "DROP",
            "UNION",
            "../",
            "..\\",
            "<",
            ">",
            "&lt;",
            "&gt;",
        ];

        let lower_input = sanitized.to_lowercase();
        for pattern in &dangerous_patterns {
            if lower_input.contains(&pattern.to_lowercase()) {
                tracing::warn!("Potentially dangerous pattern detected: {}", pattern);
                return Err(NestGateError::simple(
                    "Input contains potentially dangerous content",
                ));
            }
        }

        Ok(sanitized)
    }

    /// Validate file path to prevent directory traversal
    pub fn validate_file_path(path: &str) -> Result<String> {
        let path = path.trim();

        // Check for directory traversal attempts
        if path.contains("..") || path.contains("~") {
            return Err(NestGateError::simple(
                "Directory traversal attempt detected",
            ));
        }

        // Ensure path is within allowed boundaries
        let normalized =
            std::path::Path::new(path)
                .canonicalize()
                .map_err(|e| NestGateError::Validation {
                    field: "file_path".to_string(),
                    message: format!("Invalid file path: {e}"),
                    current_value: Some(path.to_string()),
                    expected: Some("Valid file path".to_string()),
                    user_error: true,
                })?;

        Ok(normalized.to_string_lossy().to_string())
    }

    /// Validate network address to prevent SSRF attacks
    pub fn validate_network_address(address: &str) -> Result<String> {
        // Parse as IP address
        if let Ok(ip) = address.parse::<IpAddr>() {
            // Block private and loopback addresses in production
            match ip {
                IpAddr::V4(ipv4) => {
                    if ipv4.is_private() || ipv4.is_loopback() || ipv4.is_multicast() {
                        return Err(NestGateError::simple(
                            "Private/loopback IP addresses not allowed",
                        ));
                    }
                }
                IpAddr::V6(ipv6) => {
                    if ipv6.is_loopback() || ipv6.is_multicast() {
                        return Err(NestGateError::simple(
                            "Loopback/multicast IPv6 addresses not allowed",
                        ));
                    }
                }
            }
        }

        Ok(address.to_string())
    }
}

/// Type alias for attempt record storage
type AttemptRecordMap = Arc<RwLock<HashMap<String, AttemptRecord>>>;

/// Brute force protection
pub struct BruteForceProtection {
    max_attempts: u32,
    lockout_duration: Duration,
    attempts: AttemptRecordMap,
}

struct AttemptRecord {
    count: usize,
    last_attempt: Instant,
    lockout_until: Option<Instant>,
}

impl BruteForceProtection {
    /// Create new brute force protection
    pub fn new(max_attempts: usize, lockout_duration: Duration) -> Self {
        Self {
            attempts: Arc::new(RwLock::new(HashMap::new())),
            max_attempts: max_attempts as u32,
            lockout_duration,
        }
    }

    /// Record failed authentication attempt
    pub async fn record_failed_attempt(&self, identifier: &str) -> Result<()> {
        let mut attempts = self.attempts.write().await;
        let now = Instant::now();

        let record = attempts
            .entry(identifier.to_string())
            .or_insert_with(|| AttemptRecord {
                count: 0,
                last_attempt: now,
                lockout_until: None,
            });

        record.count += 1;
        record.last_attempt = now;

        if record.count >= self.max_attempts as usize {
            record.lockout_until = Some(now + self.lockout_duration);
            tracing::warn!(
                "Brute force protection activated for {}: {} attempts",
                identifier,
                record.count
            );
        }

        Ok(())
    }

    /// Check if identifier is currently locked out
    pub async fn is_locked_out(&self, identifier: &str) -> Result<bool> {
        let attempts = self.attempts.read().await;
        let now = Instant::now();

        if let Some(record) = attempts.get(identifier) {
            if let Some(lockout_until) = record.lockout_until {
                if now < lockout_until {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Record successful authentication (reset counter)
    pub async fn record_successful_attempt(&self, identifier: &str) -> Result<()> {
        let mut attempts = self.attempts.write().await;
        attempts.remove(identifier);
        Ok(())
    }
}

/// Security event logger for audit trails
pub struct SecurityEventLogger {
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    max_events: usize,
}

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub timestamp: Instant,
    pub event_type: String,
    pub severity: String,
    pub source_ip: Option<IpAddr>,
    pub details: HashMap<String, String>,
}

impl SecurityEventLogger {
    /// Create new security event logger
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
            max_events,
        }
    }

    /// Log security event
    pub async fn log_event(
        &self,
        event_type: &str,
        severity: &str,
        source_ip: Option<IpAddr>,
        details: HashMap<String, String>,
    ) -> Result<()> {
        let mut events = self.events.write().await;

        let event = SecurityEvent {
            timestamp: Instant::now(),
            event_type: event_type.to_string(),
            severity: severity.to_string(),
            source_ip,
            details,
        };

        events.push(event);

        // Keep only the most recent events
        if events.len() > self.max_events {
            let excess = events.len() - self.max_events;
            events.drain(0..excess);
        }

        tracing::info!(
            "Security event logged: {} (severity: {})",
            event_type,
            severity
        );

        Ok(())
    }

    /// Get recent security events
    pub async fn get_recent_events(&self, limit: usize) -> Result<Vec<SecurityEvent>> {
        let events = self.events.read().await;
        let start = events.len().saturating_sub(limit);
        Ok(events[start..].to_vec())
    }
}

/// Comprehensive security middleware
pub struct SecurityMiddleware {
    rate_limiter: EnhancedRateLimiter,
    brute_force_protection: BruteForceProtection,
    event_logger: SecurityEventLogger,
}

impl SecurityMiddleware {
    /// Create new security middleware with production settings
    pub fn new_production() -> Self {
        Self {
            rate_limiter: EnhancedRateLimiter::new(Duration::from_secs(60), 100),
            brute_force_protection: BruteForceProtection::new(5, Duration::from_secs(300)),
            event_logger: SecurityEventLogger::new(10000),
        }
    }

    /// Create new security middleware with development settings
    pub fn new_development() -> Self {
        Self {
            rate_limiter: EnhancedRateLimiter::new(Duration::from_secs(60), 1000),
            brute_force_protection: BruteForceProtection::new(10, Duration::from_secs(60)),
            event_logger: SecurityEventLogger::new(1000),
        }
    }

    /// Validate incoming request
    pub async fn validate_request(
        &self,
        client_ip: IpAddr,
        path: &str,
        user_agent: Option<&str>,
    ) -> Result<bool> {
        let client_id = client_ip.to_string();

        // Check rate limiting
        if !self.rate_limiter.is_allowed(&client_id).await? {
            self.event_logger
                .log_event(
                    "rate_limit_exceeded",
                    "medium",
                    Some(client_ip),
                    [("path".to_string(), path.to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                )
                .await?;
            return Ok(false);
        }

        // Check brute force protection
        if self
            .brute_force_protection
            .is_locked_out(&client_id)
            .await?
        {
            self.event_logger
                .log_event(
                    "brute_force_lockout",
                    "high",
                    Some(client_ip),
                    [("path".to_string(), path.to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                )
                .await?;
            return Ok(false);
        }

        // Validate path
        if InputSanitizer::validate_file_path(path).is_err() {
            self.event_logger
                .log_event(
                    "directory_traversal_attempt",
                    "high",
                    Some(client_ip),
                    [("path".to_string(), path.to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                )
                .await?;
            return Ok(false);
        }

        // Check suspicious user agents
        if let Some(ua) = user_agent {
            let suspicious_patterns = ["sqlmap", "nmap", "nikto", "burp"];
            let ua_lower = ua.to_lowercase();

            for pattern in &suspicious_patterns {
                if ua_lower.contains(pattern) {
                    self.event_logger
                        .log_event(
                            "suspicious_user_agent",
                            "high",
                            Some(client_ip),
                            [
                                ("user_agent".to_string(), ua.to_string()),
                                ("pattern".to_string(), pattern.to_string()),
                            ]
                            .iter()
                            .cloned()
                            .collect(),
                        )
                        .await?;
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Get security statistics
    pub async fn get_security_stats(&self) -> Result<HashMap<String, usize>> {
        let mut stats = HashMap::new();

        let recent_events = self.event_logger.get_recent_events(1000).await?;

        // Count events by type
        for event in recent_events {
            *stats.entry(event.event_type).or_insert(0) += 1;
        }

        Ok(stats)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = EnhancedRateLimiter::new(Duration::from_secs(1), 2);

        assert!(limiter.is_allowed("test").await.unwrap());
        assert!(limiter.is_allowed("test").await.unwrap());
        assert!(!limiter.is_allowed("test").await.unwrap());

        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(limiter.is_allowed("test").await.unwrap());
    }

    #[tokio::test]
    async fn test_input_sanitization() {
        // Valid input
        let result = InputSanitizer::sanitize_string("hello_world-123", 100);
        assert!(result.is_ok());

        // Too long
        let result = InputSanitizer::sanitize_string("a".repeat(200).as_str(), 100);
        assert!(result.is_err());

        // Dangerous pattern
        let result = InputSanitizer::sanitize_string("SELECT * FROM users", 100);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_brute_force_protection() {
        let protection = BruteForceProtection::new(2, Duration::from_secs(1));

        assert!(!protection.is_locked_out("test").await.unwrap());

        protection.record_failed_attempt("test").await.unwrap();
        protection.record_failed_attempt("test").await.unwrap();

        assert!(protection.is_locked_out("test").await.unwrap());

        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(!protection.is_locked_out("test").await.unwrap());
    }

    #[tokio::test]
    async fn test_security_middleware() {
        let middleware = SecurityMiddleware::new_development();
        let client_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 1));

        // Valid request
        let result = middleware
            .validate_request(client_ip, "/api/status", Some("Mozilla/5.0"))
            .await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Suspicious user agent
        let result = middleware
            .validate_request(client_ip, "/api/status", Some("sqlmap/1.0"))
            .await;
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }
}
