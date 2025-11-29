//! Input Validation and Sanitization
//!
//! Comprehensive input validation with multiple validation types and sanitization

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::net::IpAddr;

/// **SECURITY VALIDATOR**
///
/// Comprehensive input validation and sanitization
pub struct SecurityValidator {
    rules: HashMap<String, ValidationRule>,
    stats: ValidationStats,
}

/// Validation rule configuration
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Validators to apply
    pub validators: Vec<ValidationType>,
    /// Maximum length constraint
    pub max_length: Option<usize>,
    /// Whether field is required
    pub required: bool,
    /// Whether to sanitize input
    pub sanitize: bool,
}

/// Types of validation
#[derive(Debug, Clone)]
pub enum ValidationType {
    /// Alphanumeric characters only
    Alphanumeric,
    /// Email address format
    Email,
    /// URL format
    Url,
    /// IP address format
    IpAddress,
    /// JSON path safe
    JsonPath,
    /// SQL safe (no injection)
    SqlSafe,
    /// No script tags
    NoScripts,
    /// Numeric only
    Numeric,
    /// UUID format
    Uuid,
    /// Base64 encoded
    Base64,
}

#[derive(Debug, Default)]
struct ValidationStats {
    validations_performed: AtomicU64,
    validations_failed: AtomicU64,
    sanitizations_performed: AtomicU64,
    blocked_attempts: AtomicU64,
}

impl SecurityValidator {
    /// Create a new security validator
    #[must_use]
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
            stats: ValidationStats::default(),
        }
    }
    
    /// Add a validation rule
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.insert(rule.name.clone(), rule);
    }
    
    /// Validate input against rules
    pub fn validate(&self, field_name: &str, value: &str) -> ValidationResult {
        self.stats.validations_performed.fetch_add(1, Ordering::Relaxed);
        
        if let Some(rule) = self.rules.get(field_name) {
            // Check required
            if rule.required && value.is_empty() {
                self.stats.validations_failed.fetch_add(1, Ordering::Relaxed);
                return ValidationResult::Invalid(format!("Field '{field_name}' is required"));
            }
            
            // Check length
            if let Some(max_len) = rule.max_length {
                if value.len() > max_len {
                    self.stats.validations_failed.fetch_add(1, Ordering::Relaxed);
                    return ValidationResult::Invalid(format!(
                        "Field '{}' exceeds maximum length of {}", 
                        field_name, max_len
                    ));
                }
            }
            
            // Apply validators
            for validator in &rule.validators {
                if !self.apply_validator(validator, value) {
                    self.stats.validations_failed.fetch_add(1, Ordering::Relaxed);
                    return ValidationResult::Invalid(format!(
                        "Field '{}' failed {:?} validation", 
                        field_name, validator
                    ));
                }
            }
            
            // Sanitize if needed
            let sanitized_value = if rule.sanitize {
                self.stats.sanitizations_performed.fetch_add(1, Ordering::Relaxed);
                self.sanitize_input(value)
            } else {
                value.to_string()
            };
            
            ValidationResult::Valid(sanitized_value)
        } else {
            // No rule defined, basic sanitization
            ValidationResult::Valid(self.sanitize_input(value))
        }
    }
    
    fn apply_validator(&self, validator: &ValidationType, value: &str) -> bool {
        match validator {
            ValidationType::Alphanumeric => value.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-'),
            ValidationType::Email => self.validate_email(value),
            ValidationType::Url => self.validate_url(value),
            ValidationType::IpAddress => value.parse::<IpAddr>().is_ok(),
            ValidationType::JsonPath => self.validate_json_path(value),
            ValidationType::SqlSafe => self.validate_sql_safe(value),
            ValidationType::NoScripts => !self.contains_script_tags(value),
            ValidationType::Numeric => value.chars().all(|c| c.is_numeric() || c == '.' || c == '-'),
            ValidationType::Uuid => self.validate_uuid(value),
            ValidationType::Base64 => self.validate_base64(value),
        }
    }
    
    fn validate_email(&self, value: &str) -> bool {
        value.contains('@') && value.contains('.') && value.len() > 5 && value.len() < 255
    }
    
    fn validate_url(&self, value: &str) -> bool {
        value.starts_with("http://") || value.starts_with("https://")
    }
    
    fn validate_json_path(&self, value: &str) -> bool {
        !value.contains("..") && !value.contains("//")
    }
    
    fn validate_sql_safe(&self, value: &str) -> bool {
        let dangerous_keywords = [
            "SELECT", "INSERT", "UPDATE", "DELETE", "DROP", "CREATE", 
            "ALTER", "EXEC", "UNION", "OR", "AND", "--", "/*", "*/"
        ];
        let upper_value = value.to_uppercase();
        !dangerous_keywords.iter().any(|&keyword| upper_value.contains(keyword))
    }
    
    fn contains_script_tags(&self, value: &str) -> bool {
        let lower_value = value.to_lowercase();
        lower_value.contains("<script") || 
        lower_value.contains("javascript:") || 
        lower_value.contains("onclick") || 
        lower_value.contains("onerror")
    }
    
    fn validate_uuid(&self, value: &str) -> bool {
        value.len() == 36 && value.chars().enumerate().all(|(i, c)| {
            match i {
                8 | 13 | 18 | 23 => c == '-',
                _ => c.is_ascii_hexdigit(),
            }
        })
    }
    
    fn validate_base64(&self, value: &str) -> bool {
        value.chars().all(|c| c.is_alphanumeric() || c == '+' || c == '/' || c == '=')
    }
    
    fn sanitize_input(&self, value: &str) -> String {
        value
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;")
            .replace('&', "&amp;")
            .trim()
            .to_string()
    }
    
    /// Get validation statistics
    pub fn stats(&self) -> (u64, u64, u64, f64) {
        let performed = self.stats.validations_performed.load(Ordering::Relaxed);
        let failed = self.stats.validations_failed.load(Ordering::Relaxed);
        let sanitized = self.stats.sanitizations_performed.load(Ordering::Relaxed);
        let success_rate = if performed > 0 { 
            (performed - failed) as f64 / performed as f64 
        } else { 
            1.0 
        };
        (performed, failed, sanitized, success_rate)
    }
}

impl Default for SecurityValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validation result
#[derive(Debug)]
pub enum ValidationResult {
    /// Input is valid (with sanitized value)
    Valid(String),
    /// Input is invalid (with error message)
    Invalid(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_validator() {
        let mut validator = SecurityValidator::new();
        
        let rule = ValidationRule {
            name: "username".to_string(),
            validators: vec![ValidationType::Alphanumeric],
            max_length: Some(50),
            required: true,
            sanitize: true,
        };
        
        validator.add_rule(rule);
        
        // Valid input
        match validator.validate("username", "john_doe") {
            ValidationResult::Valid(_) => {},
            ValidationResult::Invalid(msg) => panic!("Should be valid: {}", msg),
        }
        
        // Invalid input (too long)
        match validator.validate("username", &"a".repeat(51)) {
            ValidationResult::Invalid(_) => {},
            ValidationResult::Valid(_) => panic!("Should be invalid"),
        }
    }
    
    #[test]
    fn test_email_validation() {
        let validator = SecurityValidator::new();
        assert!(validator.validate_email("test@example.com"));
        assert!(!validator.validate_email("invalid"));
        assert!(!validator.validate_email("@example.com"));
    }
    
    #[test]
    fn test_sql_injection_detection() {
        let validator = SecurityValidator::new();
        assert!(!validator.validate_sql_safe("SELECT * FROM users"));
        assert!(!validator.validate_sql_safe("'; DROP TABLE users--"));
        assert!(validator.validate_sql_safe("normal input"));
    }
    
    #[test]
    fn test_xss_detection() {
        let validator = SecurityValidator::new();
        assert!(validator.contains_script_tags("<script>alert('xss')</script>"));
        assert!(validator.contains_script_tags("javascript:alert('xss')"));
        assert!(!validator.contains_script_tags("normal text"));
    }
}

