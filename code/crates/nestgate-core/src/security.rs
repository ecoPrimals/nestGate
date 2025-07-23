// Removed unused error imports
/// # Security utilities for NestGate
///
/// This module provides security-related functionality including API key management,
/// authentication, and authorization for NestGate operations.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused tracing import
use crate::get_or_create_uuid;

/// Security manager for authentication and authorization
#[derive(Default)]
pub struct SecurityManager {
    /// API keys for different services
    api_keys: HashMap<String, String>,
    /// Active tokens
    tokens: HashMap<String, AuthToken>,
    /// User sessions - infrastructure for future session management
    #[allow(dead_code)]
    sessions: HashMap<String, String>,
    /// Security policies
    policies: Vec<SecurityPolicy>,
    /// Security events - infrastructure for future event logging
    #[allow(dead_code)]
    events: Vec<SecurityEvent>,
}

/// Authentication token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthToken {
    /// Token value
    pub token: String,
    /// Token expiration
    pub expires_at: DateTime<Utc>,
    /// Associated user ID
    pub user_id: String,
    /// Token permissions
    pub permissions: Vec<String>,
}

/// Security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Policy rules
    pub rules: Vec<SecurityRule>,
    /// Policy enabled
    pub enabled: bool,
}

/// Security rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    /// Rule ID
    pub id: String,
    /// Rule condition
    pub condition: String,
    /// Rule action
    pub action: SecurityAction,
}

/// Security action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAction {
    /// Allow the action
    Allow,
    /// Deny the action
    Deny,
    /// Require authentication
    RequireAuth,
    /// Log the action
    Log,
}

impl SecurityManager {
    /// Create new security manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add API key
    pub fn add_api_key(&mut self, service: String, key: String) {
        self.api_keys.insert(service, key);
    }

    /// Get API key for service
    pub fn get_api_key(&self, service: &str) -> Option<&String> {
        self.api_keys.get(service)
    }

    /// Extract API key from headers
    pub fn extract_api_key<'a>(&self, headers: &'a HashMap<String, String>) -> Option<&'a str> {
        headers
            .get("authorization")
            .and_then(|auth| auth.strip_prefix("Bearer "))
    }

    /// Validate authentication token
    pub fn validate_token(&self, token: &str) -> bool {
        if let Some(auth_token) = self.tokens.get(token) {
            auth_token.expires_at > Utc::now()
        } else {
            false
        }
    }

    /// Create authentication token
    pub fn create_token(&mut self, user_id: String, permissions: Vec<String>) -> String {
        let token = get_or_create_uuid("security_token").to_string();
        let auth_token = AuthToken {
            token: token.clone(),
            expires_at: Utc::now() + chrono::Duration::hours(24),
            user_id,
            permissions,
        };
        self.tokens.insert(token.clone(), auth_token);
        token
    }

    /// Add security policy
    pub fn add_policy(&mut self, policy: SecurityPolicy) {
        self.policies.push(policy);
    }

    /// Check if action is allowed
    pub fn is_allowed(&self, action: &str, context: &SecurityContext) -> bool {
        for policy in &self.policies {
            if !policy.enabled {
                continue;
            }

            for rule in &policy.rules {
                if self.matches_condition(&rule.condition, action, context) {
                    match rule.action {
                        SecurityAction::Allow => return true,
                        SecurityAction::Deny => return false,
                        SecurityAction::RequireAuth => {
                            return context.authenticated;
                        }
                        SecurityAction::Log => {
                            tracing::info!("Security action logged: {}", action);
                        }
                    }
                }
            }
        }

        // Default to allow if no policy matches
        true
    }

    fn matches_condition(&self, condition: &str, action: &str, _context: &SecurityContext) -> bool {
        // Simple condition matching - in production this would be more sophisticated
        condition == action || condition == "*"
    }
}

/// Security context for requests
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityContext {
    /// User ID
    pub user_id: Option<String>,
    /// User roles
    pub roles: Vec<String>,
    /// Permissions
    pub permissions: Vec<String>,
    /// Session ID
    pub session_id: Option<String>,
    /// API key
    pub api_key: Option<String>,
    /// Request IP address
    pub ip_address: Option<String>,
    /// Whether the request is authenticated
    pub authenticated: bool,
    /// Additional headers
    pub headers: HashMap<String, String>,
}

/// API key validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyValidation {
    /// Whether the key is valid
    pub valid: bool,
    /// Associated service name
    pub service: Option<String>,
    /// Key permissions
    pub permissions: Vec<String>,
    /// Key expiration
    pub expires_at: Option<DateTime<Utc>>,
}

/// Enhanced security manager with crypto locks
#[derive(Default)]
pub struct EnhancedSecurityManager {
    /// Base security manager
    base: SecurityManager,
    /// Crypto keys managed by BearDog
    crypto_keys: HashMap<String, String>,
    /// Active crypto locks - infrastructure for future lock management
    #[allow(dead_code)]
    active_locks: HashMap<String, String>,
    /// Security events
    events: Vec<SecurityEvent>,
    /// Default headers for HTTP requests
    pub default_headers: HashMap<String, String>,
}

/// Security event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    /// Event ID
    pub id: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: SecurityEventType,
    /// Event details
    pub details: HashMap<String, String>,
    /// Event severity
    pub severity: SecuritySeverity,
}

/// Security event type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    /// Authentication attempt
    AuthAttempt,
    /// Authorization failure
    AuthFailure,
    /// API key usage
    ApiKeyUsage,
    /// Crypto lock validation
    CryptoLockValidation,
    /// External access attempt
    ExternalAccess,
    /// Suspicious activity
    SuspiciousActivity,
}

/// Security severity level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecuritySeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

impl EnhancedSecurityManager {
    /// Create new enhanced security manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add crypto key
    pub fn add_crypto_key(&mut self, key_id: String, key_value: String) {
        self.crypto_keys.insert(key_id, key_value);
    }

    /// Validate crypto key
    pub fn validate_crypto_key(&self, key_id: &str, key_value: &str) -> bool {
        if let Some(stored_key) = self.crypto_keys.get(key_id) {
            stored_key == key_value
        } else {
            false
        }
    }

    /// Log security event
    pub fn log_event(
        &mut self,
        event_type: SecurityEventType,
        details: HashMap<String, String>,
        severity: SecuritySeverity,
    ) {
        let event = SecurityEvent {
            id: get_or_create_uuid("security_event").to_string(),
            timestamp: Utc::now(),
            event_type,
            details,
            severity,
        };
        self.events.push(event);
    }

    /// Get security events
    pub fn get_events(&self) -> &Vec<SecurityEvent> {
        &self.events
    }

    /// Get security manager
    pub fn get_base_manager(&self) -> &SecurityManager {
        &self.base
    }

    /// Get mutable security manager
    pub fn get_base_manager_mut(&mut self) -> &mut SecurityManager {
        &mut self.base
    }
}

/// Secure API wrapper
pub struct SecureApiWrapper {
    _auth_token: Option<String>,
    _encryption_key: Option<String>,
    _endpoint: String,
    /// Security manager for this wrapper
    security_manager: EnhancedSecurityManager,
}

impl SecureApiWrapper {
    /// Create new secure API wrapper
    pub fn new(endpoint: String) -> Self {
        Self {
            _auth_token: None,
            _encryption_key: None,
            _endpoint: endpoint,
            security_manager: EnhancedSecurityManager::new(),
        }
    }

    /// Set default header
    pub fn set_default_header(&mut self, key: String, value: String) {
        // Add the header to the internal headers map
        self.security_manager
            .default_headers
            .insert(key.clone(), value.clone());
        tracing::debug!("Set default header: {} = {}", key, value);
    }

    /// Get security manager
    pub fn get_security_manager(&self) -> &EnhancedSecurityManager {
        &self.security_manager
    }

    /// Get mutable security manager
    pub fn get_security_manager_mut(&mut self) -> &mut EnhancedSecurityManager {
        &mut self.security_manager
    }
}

/// Validate API key with headers
pub fn validate_api_key_with_headers(headers: &HashMap<String, String>) -> Result<String, String> {
    let security_manager = SecurityManager::new();
    let api_key = security_manager
        .extract_api_key(headers)
        .ok_or_else(|| "No API key found".to_string())?;

    Ok(api_key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_manager_creation() {
        let manager = SecurityManager::new();
        assert!(manager.api_keys.is_empty());
        assert!(manager.tokens.is_empty());
        assert!(manager.policies.is_empty());
    }

    #[test]
    fn test_api_key_management() {
        let mut manager = SecurityManager::new();
        manager.add_api_key("test_service".to_string(), "test_key".to_string());

        assert_eq!(
            manager.get_api_key("test_service"),
            Some(&"test_key".to_string())
        );
        assert_eq!(manager.get_api_key("unknown_service"), None);
    }

    #[test]
    fn test_token_validation() {
        let mut manager = SecurityManager::new();
        let token = manager.create_token("user123".to_string(), vec!["read".to_string()]);

        assert!(manager.validate_token(&token));
        assert!(!manager.validate_token("invalid_token"));
    }

    #[test]
    fn test_security_context() {
        let context = SecurityContext::default();
        assert!(!context.authenticated);
        assert!(context.permissions.is_empty());
        assert!(context.user_id.is_none());
    }

    #[test]
    fn test_enhanced_security_manager() {
        let mut manager = EnhancedSecurityManager::new();
        manager.add_crypto_key("key1".to_string(), "value1".to_string());

        assert!(manager.validate_crypto_key("key1", "value1"));
        assert!(!manager.validate_crypto_key("key1", "wrong_value"));
        assert!(!manager.validate_crypto_key("unknown_key", "value1"));
    }
}
