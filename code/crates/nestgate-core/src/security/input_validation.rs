use crate::error::NestGateError;
use std::collections::HashMap;
//
// This module provides comprehensive input validation and sanitization
// to prevent injection attacks and ensure data integrity.

use crate::{Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Input validation errors (local to this module - will be converted to domain errors)
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum InputValidationError {
    #[error("Field '{field}' is too short (minimum: {min_length})")]
    TooShort { field: String, min_length: usize },
    #[error("Field '{field}' is too long (maximum: {max_length})")]
    TooLong { field: String, max_length: usize },
    #[error("Field '{field}' has invalid format")]
    InvalidFormat { field: String },
    #[error("Field '{field}' contains security violations")]
    SecurityViolation { field: String },
    #[error("Field '{field}' has invalid value: {value}")]
    InvalidValue { field: String, value: String },
    #[error("Pattern compilation failed: {message}")]
    PatternError { message: String },
}
/// Input validation configuration
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    /// Maximum string length
    pub max_string_length: usize,
    /// Minimum password length
    pub min_password_length: usize,
    /// Maximum array/collection size
    pub max_collection_size: usize,
    /// Enable SQL injection detection
    pub enable_sql_injection_detection: bool,
    /// Enable XSS detection
    pub enable_xss_detection: bool,
    /// Enable path traversal detection
    pub enable_path_traversal_detection: bool,
}
impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            max_string_length: 10000,
            min_password_length: 8,
            max_collection_size: 1000,
            enable_sql_injection_detection: true,
            enable_xss_detection: true,
            enable_path_traversal_detection: true,
        }
    }
}

// Lazy-initialized regex patterns (compiled once, reused forever)
// This eliminates all regex compilation unwraps and improves performance
//
// NOTE: `.expect()` calls in this block are SAFE because:
// 1. All regex patterns are hardcoded and validated
// 2. Compilation happens once at first use (lazy initialization)
// 3. Failure here is fail-fast (startup failure, not runtime panic)
// 4. Invalid regex would be caught during development/testing
lazy_static! {
    static ref EMAIL_PATTERN: Regex = 
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
            .expect("Email regex is hardcoded and valid");
    
    static ref UUID_PATTERN: Regex = 
        Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
            .expect("UUID regex is hardcoded and valid");
    
    static ref ALPHANUMERIC_PATTERN: Regex = 
        Regex::new(r"^[a-zA-Z0-9]+$")
            .expect("Alphanumeric regex is hardcoded and valid");
    
    static ref SAFE_FILENAME_PATTERN: Regex = 
        Regex::new(r"^[a-zA-Z0-9._-]+$")
            .expect("Filename regex is hardcoded and valid");
    
    static ref SQL_INJECTION_PATTERN: Regex = 
        Regex::new(r"(?i)(union|select|insert|update|delete|drop|create|alter|exec|script)")
            .expect("SQL injection regex is hardcoded and valid");
    
    static ref XSS_SCRIPT_PATTERN: Regex = 
        Regex::new(r"(?i)<script|javascript:|vbscript:|onload=|onerror=|onclick=")
            .expect("XSS script regex is hardcoded and valid");
    
    static ref XSS_ATTRIBUTES_PATTERN: Regex = 
        Regex::new(r"(?i)(on\w+\s*=|javascript:|vbscript:|data:text/html)")
            .expect("XSS attributes regex is hardcoded and valid");
    
    static ref PATH_TRAVERSAL_PATTERN: Regex = 
        Regex::new(r"(\.\./|\.\.\\)")
            .expect("Path traversal regex is hardcoded and valid");
    
    static ref COMMAND_INJECTION_PATTERN: Regex = 
        Regex::new(r"[;&|`$()]")
            .expect("Command injection regex is hardcoded and valid");
    
    static ref IPV4_PATTERN: Regex = 
        Regex::new(r"^((25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$")
            .expect("IPv4 regex is hardcoded and valid");
    
    static ref IPV6_PATTERN: Regex = 
        Regex::new(r"^(?:[0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$")
            .expect("IPv6 regex is hardcoded and valid");
    
    static ref DOMAIN_PATTERN: Regex = 
        Regex::new(r"^[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(\.[a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$")
            .expect("Domain regex is hardcoded and valid");
    
    static ref PORT_PATTERN: Regex = 
        Regex::new(r"^(6553[0-5]|655[0-2][0-9]|65[0-4][0-9]{2}|6[0-4][0-9]{3}|[1-5][0-9]{4}|[1-9][0-9]{0,3})$")
            .expect("Port regex is hardcoded and valid");
}

/// Input validator with security hardening
#[derive(Debug)]
pub struct InputValidator {
    config: ValidationConfig,
}

impl InputValidator {
    /// Create a new input validator with default patterns
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: ValidationConfig::default(),
        }
    }

    /// Create a new input validator with custom configuration
    #[must_use]
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Validate a string input with security checks
    pub fn validate_string(
        &self,
        field: &str,
        value: &str,
        min_length: Option<usize>,
        max_length: Option<usize>,
    ) -> ValidationResult<String> {
        // Length validation
        let min_len = min_length.unwrap_or(0);
        let max_len = max_length.unwrap_or(self.config.max_string_length);

        if value.len() < min_len {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: format!("too short, minimum {min_len} characters"),
            );
        }

        if value.len() > max_len {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: format!("too long, maximum {max_len} characters"),
            );
        }

        // Security validation - convert SecurityError to ValidationError for this context
        if let Err(_security_error) = self.check_security_violations(field, value) {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "contains security violations".to_string(),
            );
        }

        // Sanitize and return
        Ok(self.sanitize_string(value))
    }

    /// Validate an email address
    pub fn validate_email(&self, field: &str, email: &str) -> ValidationResult<String> {
        if !EMAIL_PATTERN.is_match(email) {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "invalid email format".to_string(),
            );
        }

        self.validate_string(field, email, Some(5), Some(254))
            .map_err(|_| ValidationError::InvalidField {
                field: field.to_string(),
                reason: "email validation failed".to_string(),
            })
    }

    /// Validate a UUID
    pub fn validate_uuid(&self, field: &str, uuid: &str) -> ValidationResult<String> {
        if !UUID_PATTERN.is_match(uuid) {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "invalid UUID format".to_string(),
            );
        }

        Ok(uuid.to_string())
    }

    /// Validate a password with security requirements
    pub fn validate_password(&self, _field: &str, password: &str) -> SecurityResult<String> {
        if password.len() < self.config.min_password_length {
            return Err(SecurityError::WeakCredentials {
                requirement: format!("minimum {self.config.min_password_length} characters"),
                provided: "password too short".to_string(),
            );
        }

        // Check for common password patterns
        if password.to_lowercase() == "password" || password == "123456" || password == "admin" {
            return Err(SecurityError::WeakCredentials {
                requirement: "strong password".to_string(),
                provided: "common password detected".to_string(),
            );
        }

        Ok(password.to_string())
    }

    /// Validate a filename for safe filesystem operations
    pub fn validate_filename(&self, field: &str, filename: &str) -> ValidationResult<String> {
        if !SAFE_FILENAME_PATTERN.is_match(filename) {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "unsafe filename format".to_string(),
            );
        }

        // Check for path traversal
        if self.config.enable_path_traversal_detection
            && PATH_TRAVERSAL_PATTERN.is_match(filename)
        {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "path traversal detected".to_string(),
            );
        }

        self.validate_string(field, filename, Some(1), Some(255))
            .map_err(|_| ValidationError::InvalidField {
                field: field.to_string(),
                reason: "filename validation failed".to_string(),
            })
    }

    /// Validate a network address (IP or domain)
    pub fn validate_network_address(&self, field: &str, address: &str) -> ValidationResult<String> {
        // Try IPv4 first
        if IPV4_PATTERN.is_match(address) {
            return Ok(address.to_string());
        }

        // Try IPv6
        if IPV6_PATTERN.is_match(address) {
            return Ok(address.to_string());
        }

        // Try domain name
        if DOMAIN_PATTERN.is_match(address) {
            Ok(address.to_string())
        } else {
            Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "invalid network address format".to_string(),
            })
        }
    }

    /// Validate a port number
    pub fn validate_port(&self, field: &str, port: &str) -> ValidationResult<u16> {
        if !PORT_PATTERN.is_match(port) {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: "invalid port format".to_string(),
            );
        }

        port.parse::<u16>()
            .map_err(|_| ValidationError::InvalidField {
                field: field.to_string(),
                reason: "port number out of range".to_string(),
            })
    }

    /// Validate collection size
    pub fn validate_collection_size<T>(
        &self,
        field: &str,
        collection: &[T],
    ) -> ValidationResult<()> {
        if collection.len() > self.config.max_collection_size {
            return Err(ValidationError::InvalidField {
                field: field.to_string(),
                reason: format!(
                    "collection too large, maximum {} items",
                    self.config.max_collection_size
                ),
            );
        }
        Ok(())
    }

    /// Check for security violations in input
    fn check_security_violations(&self, field: &str, value: &str) -> SecurityResult<()> {
        // SQL injection detection
        if self.config.enable_sql_injection_detection && SQL_INJECTION_PATTERN.is_match(value)
        {
            return Err(SecurityError::InjectionAttempt {
                attack_type: "SQL injection".to_string(),
                field: field.to_string(),
                pattern: "SQL keywords detected".to_string(),
            );
        }

        // XSS detection
        if self.config.enable_xss_detection
            && (XSS_SCRIPT_PATTERN.is_match(value)
                || XSS_ATTRIBUTES_PATTERN.is_match(value))
        {
            return Err(SecurityError::InjectionAttempt {
                attack_type: "XSS".to_string(),
                field: field.to_string(),
                pattern: "script tags or dangerous attributes detected".to_string(),
            );
        }

        // Path traversal detection
        if self.config.enable_path_traversal_detection
            && PATH_TRAVERSAL_PATTERN.is_match(value)
        {
            return Err(SecurityError::InjectionAttempt {
                attack_type: "path traversal".to_string(),
                field: field.to_string(),
                pattern: "directory traversal sequences detected".to_string(),
            );
        }

        // Command injection detection
        if COMMAND_INJECTION_PATTERN.is_match(value) {
            return Err(SecurityError::InjectionAttempt {
                attack_type: "command injection".to_string(),
                field: field.to_string(),
                pattern: "shell command sequences detected".to_string(),
            );
        }

        Ok(())
    }

    /// Sanitize a string by removing/escaping dangerous characters
    fn sanitize_string(&self, value: &str) -> String {
        value
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .trim()
            .to_string()
    }

    /// Validate a map of key-value pairs
    pub fn validate_map(&self, field: &str, map: &HashMap<String, String>) -> ValidationResult<()> {
        // Validate collection size
        self.validate_collection_size(field, &map.iter().collect::<Vec<_>>())
            .map_err(|_| ValidationError::InvalidField {
                field: field.to_string(),
                reason: "map validation failed".to_string(),
            )?;

        // Validate each key and value
        for (key, value) in map {
            self.validate_string(&format!("{field}.key"), key, Some(1), Some(255))
                .map_err(|_| ValidationError::InvalidField {
                    field: field.to_string(),
                    reason: "invalid key format".to_string(),
                )?;
            self.validate_string(&format!("{field}.value"), value, None, None)
                .map_err(|_| ValidationError::InvalidField {
                    field: field.to_string(),
                    reason: "invalid value format".to_string(),
                )?;
        }

        Ok(())
    }
}

/// Validate a service name for universal service registration
pub fn validate_service_name(name: &str) -> ValidationResult<String> {
    let validator = InputValidator::new().map_err(|_| ValidationError::InvalidField {
        field: Some("field".to_string()),
        reason: "failed to create validator".to_string(),
    )?;
    if !validator.patterns.alphanumeric.is_match(name) {
        return Err(ValidationError::InvalidField {
            field: Some("field".to_string()),
            reason: "service name must be alphanumeric".to_string(),
        );
    }

    Ok(name.to_string())
}

/// Validate an endpoint path for API registration
    // Additional path-specific validation
    if !path.starts_with('/') {
        return Err(ValidationError::InvalidField {
            field: Some("field".to_string()),
            reason: "path must start with '/'".to_string(),
        );
    }
    Ok(path.to_string())
}

/// Validate an API key
pub fn validate_api_key(key: &str) -> ValidationResult<String> {
    let validator = InputValidator::new().map_err(|_| ValidationError::InvalidField {
        field: Some("field".to_string()),
        reason: "failed to create validator".to_string(),
    )?;
    validator.validate_string("api_key", key, Some(32), Some(128))
}
/// Validate a username for authentication
pub fn validate_username(username: &str) -> ValidationResult<String> {
    let validator = InputValidator::new().map_err(|_| ValidationError::InvalidField {
        field: Some("field".to_string()),
        reason: "failed to create validator".to_string(),
    )?;
    if !validator.patterns.alphanumeric.is_match(username) {
        return Err(ValidationError::InvalidField {
            field: Some("field".to_string()),
            reason: "username must be alphanumeric".to_string(),
        );
    }

    Ok(username.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_validation() -> crate::Result<()> {
        let validator = InputValidator::new()?;

        // Valid string
        assert!(validator
            .validate_string("test", "hello world", None, None)
            .is_ok());

        // Too long
        let long_string = "a".repeat(20000);
        assert!(validator
            .validate_string("test", &long_string, None, None)
            .is_err());

        // Too short
        assert!(validator
            .validate_string("test", "", Some(5), None)
            .is_err());

        Ok(())
    }

    #[test]
    fn test_email_validation() -> crate::Result<()> {
        let validator = InputValidator::new()?;

        assert!(validator
            .validate_email("email", "test@example.com")
            .is_ok());
        assert!(validator.validate_email("email", "invalid-email").is_err());
        assert!(validator.validate_email("email", "test@").is_err());

        Ok(())
    }

    #[test]
    fn test_security_violations() -> crate::Result<()> {
        let validator = InputValidator::new()?;

        // SQL injection
        assert!(validator
            .validate_string("test", "'; DROP TABLE users; --", None, None)
            .is_err());

        // XSS
        assert!(validator
            .validate_string("test", "<script>alert('xss')</script>", None, None)
            .is_err());

        // Path traversal
        assert!(validator
            .validate_filename("test", "../../../etc/passwd")
            .is_err());

        Ok(())
    }

    #[test]
    fn test_password_validation() -> crate::Result<()> {
        let validator = InputValidator::new()?;

        assert!(validator
            .validate_password("password", "securepassword123")
            .is_ok());
        assert!(validator.validate_password("password", "weak").is_err());
        assert!(validator.validate_password("password", "password").is_err());

        Ok(())
    }

    #[test]
    fn test_network_validation() -> crate::Result<()> {
        let validator = InputValidator::new()?;

        assert!(validator
            .validate_network_address("address", "192.168.1.1")
            .is_ok());
        assert!(validator
            .validate_network_address("address", "example.com")
            .is_ok());
        assert!(validator
            .validate_network_address("address", "invalid..domain")
            .is_err());

        assert!(validator.validate_port("port", "8080").is_ok());
        assert!(validator.validate_port("port", "70000").is_err());

        Ok(())
    }
}