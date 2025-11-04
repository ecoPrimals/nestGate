//! # Security Hardening Manager
//! Manager functionality and utilities.
// Central manager for all production security hardening measures

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::error::Result;
use super::audit::SecurityAuditLogger;
use super::config::{SecurityAction, SecurityEventType, SecurityHardeningConfig, ThreatLevel};
use super::intrusion_detection::IntrusionDetectionSystem;
use super::rate_limiting::RateLimiter;
use super::types::{RequestDetails, SecurityEvent, SecurityMetrics, SecurityValidationResult};
use super::validation::RequestValidator;

/// **SECURITY HARDENING MANAGER**
///
/// Central manager for all production security hardening measures
pub struct SecurityHardeningManager {
    /// Configuration for security hardening
    config: SecurityHardeningConfig,
    /// Rate limiting state
    rate_limiter: Arc<RwLock<RateLimiter>>,
    /// Request validation engine
    validator: Arc<RequestValidator>,
    /// Security audit logger
    audit_logger: Arc<SecurityAuditLogger>,
    /// Intrusion detection system
    ids: Arc<IntrusionDetectionSystem>,
}
impl SecurityHardeningManager {
    /// Create new security hardening manager
    pub fn new(config: SecurityHardeningConfig) -> Self {
        let rate_limiter = Arc::new(RwLock::new(RateLimiter::new(config.rate_limiting.clone())));
        let validator = Arc::new(RequestValidator::new(config.validation.clone()));
        let audit_logger = Arc::new(SecurityAuditLogger::new(config.audit_logging.clone()));
        let ids = Arc::new(IntrusionDetectionSystem::new(config.intrusion_detection.clone()));

        Self {
            config,
            rate_limiter,
            validator,
            audit_logger,
            ids,
        }
    }

    /// **VALIDATE REQUEST SECURITY**
    ///
    /// Comprehensive security validation for incoming requests
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn validate_request_security(
        &self,
        source_ip: IpAddr,
        method: &str,
        headers: &HashMap<String, String>,
        body: &[u8],
    ) -> Result<SecurityValidationResult>  {
        let mut events = Vec::new();
        let mut blocked = false;

        // 1. Rate limiting check
        if !self.rate_limiter.read().await.is_whitelisted(source_ip) {
            match self.rate_limiter.write().await.check_rate_limit(source_ip).await {
                Ok(allowed) => {
                    if !allowed {
                        events.push(SecurityEvent {
                            timestamp: std::time::SystemTime::now(),
                            event_type: SecurityEventType::RateLimitExceeded,
                            source_ip: Some(source_ip),
                            user_id: None,
                            request_details: Some(RequestDetails {
                                method: method.to_string(),
                                user_agent: headers.get("User-Agent").cloned(),
                                size: body.len(),
                                header_count: headers.len(),
                            }),
                            threat_level: ThreatLevel::Medium,
                            description: "Rate limit exceeded".to_string(),
                            action_taken: SecurityAction::RateLimited,
                        });
                        blocked = true;
                    }
                }
                Err(e) => {
                    tracing::error!("Rate limiting error: {}", e);
                }
            }
        }

        // 2. Request validation
        match self.validator.validate_request(method, path, headers, body).await {
            Ok(validation_result) => {
                events.extend(validation_result.security_events);
                if validation_result.blocked {
                    blocked = true;
                }
            }
            Err(e) => {
                tracing::error!("Request validation error: {}", e);
                blocked = true;
            }
        }

        // 3. Intrusion detection
        if let Err(e) = self.ids.analyze_request(source_ip, method, path, headers, body).await {
            tracing::error!("Intrusion detection error: {}", e);
        }

        // 4. Log security events
        for event in &events {
            if let Err(e) = self.audit_logger.log_security_event(event).await {
                tracing::error!("Security audit logging error: {}", e);
            }
        }

        Ok(SecurityValidationResult {
            allowed: !blocked,
            security_events: events,
            security_headers: self.generate_security_headers(),
        })
    }

    /// **HANDLE AUTHENTICATION FAILURE**
    ///
    /// Handle failed authentication attempts with intrusion detection
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn handle_auth_failure(&self, source_ip: IpAddr, user_id: Option<String>) -> Result<()>  {
        let event = SecurityEvent {
            timestamp: std::time::SystemTime::now(),
            event_type: SecurityEventType::AuthenticationFailure,
            source_ip: Some(source_ip),
            user_id,
            request_details: None,
            threat_level: ThreatLevel::High,
            description: "Authentication failure".to_string(),
            action_taken: SecurityAction::Logged,
        };

        // Log the event
        self.audit_logger.log_security_event(&event).await?;

        // Update intrusion detection
        self.ids.record_auth_failure(source_ip).await?;

        Ok(())
    }

    /// **CHECK IP BLOCKED**
    ///
    /// Check if an IP address is currently blocked
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn is_ip_blocked(&self, ip: IpAddr) -> Result<bool>  {
        self.ids.is_ip_blocked(ip).await
    }

    /// **GENERATE SECURITY HEADERS**
    ///
    /// Generate security headers for responses
    pub fn generate_security_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();

        // Content Security Policy
        if let Some(csp) = &self.config.security_headers.csp {
            headers.insert("Content-Security-Policy".to_string(), csp.clone());
        }

        // HTTP Strict Transport Security
        if self.config.security_headers.hsts {
            headers.insert(
                "Strict-Transport-Security".to_string(),
                "max-age=31536000; includeSubDomains; preload".to_string(),
            );
        }

        // X-Frame-Options
        headers.insert(
            "X-Frame-Options".to_string(),
            self.config.security_headers.x_frame_options.clone(),
        );

        // X-Content-Type-Options
        if self.config.security_headers.x_content_type_options {
            headers.insert("X-Content-Type-Options".to_string(), "nosniff".to_string());
        }

        // X-XSS-Protection
        if self.config.security_headers.x_xss_protection {
            headers.insert("X-XSS-Protection".to_string(), "1; mode=block".to_string());
        }

        // Additional security headers
        headers.insert("X-Robots-Tag".to_string(), "noindex, nofollow".to_string());
        headers.insert("Referrer-Policy".to_string(), "strict-origin-when-cross-origin".to_string());
        headers.insert("Permissions-Policy".to_string(), "geolocation=(), microphone=(), camera=()".to_string());

        headers
    }

    /// **GET SECURITY METRICS**
    ///
    /// Get comprehensive security metrics
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn get_security_metrics(&self) -> Result<SecurityMetrics>  {
        let rate_limit_stats = self.rate_limiter.read().await.get_statistics().await;
        let ids_stats = self.ids.get_statistics().await?;

        Ok(SecurityMetrics {
            rate_limit_stats,
            ids_stats,
            total_security_events: 0, // Would track in production
            blocked_requests: 0,      // Would track in production
            blocked_ips: ids_stats.blocked_ips_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_security_hardening_manager_creation() {
        let config = SecurityHardeningConfig::default();
        let manager = SecurityHardeningManager::new(config);
        
        let test_ip = IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100));
        let blocked = manager.is_ip_blocked(test_ip).await.expect("Security operation failed");
        assert!(!blocked);
    }

    #[tokio::test]
    async fn test_security_headers() {
        let config = SecurityHardeningConfig::default();
        let manager = SecurityHardeningManager::new(config);

        let headers = manager.generate_security_headers();

        assert!(headers.contains_key("Content-Security-Policy"));
        assert!(headers.contains_key("Strict-Transport-Security"));
        assert!(headers.contains_key("X-Frame-Options"));
        assert!(headers.contains_key("X-Content-Type-Options"));
        assert!(headers.contains_key("X-XSS-Protection"));
    }
} 