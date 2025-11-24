//! Modern Configuration Validation Module
//!
//! Provides comprehensive, type-safe configuration validation with detailed
//! error reporting and recovery suggestions using modern Rust patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::time::Duration;

use crate::error::NestGateError;

// ==================== VALIDATION FRAMEWORK ====================

/// Configuration validation trait for type-safe validation
pub trait ConfigValidation {
    /// Validate the configuration and return detailed results
    fn validate(&self) -> ValidationResult;

    /// Get validation schema for documentation
    fn schema() -> ValidationSchema;
}

/// Validation result with detailed error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub suggestions: Vec<ValidationSuggestion>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create a failed validation result with errors
    pub fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn with_warning(mut self, warning: ValidationWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add a suggestion to the result
    pub fn with_suggestion(mut self, suggestion: ValidationSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Convert to a Result type
    pub async fn into_result(self) -> crate::Result<()> {
        if self.is_valid {
            Ok(())
        } else {
            let error_messages: Vec<String> = self
                .errors
                .iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect();

            Err(NestGateError::validation_error(&format!(
                "Configuration validation failed: {}",
                error_messages.join(", ")
            )))
        }
    }
}

/// Validation error with field context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub error_type: ValidationErrorType,
    pub current_value: Option<String>,
    pub expected_format: Option<String>,
}

/// Types of validation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationErrorType {
    Required,
    InvalidFormat,
    OutOfRange,
    InvalidValue,
    Conflict,
    Security,
}

/// Validation warning for non-critical issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub field: String,
    pub message: String,
    pub severity: WarningSeverity,
}

/// Warning severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarningSeverity {
    Low,
    Medium,
    High,
}

/// Validation suggestion for improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSuggestion {
    pub field: String,
    pub message: String,
    pub suggested_value: Option<String>,
}

/// Validation schema for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSchema {
    pub fields: HashMap<String, FieldSchema>,
    pub dependencies: Vec<FieldDependency>,
}

/// Schema for individual fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    pub field_type: String,
    pub required: bool,
    pub default_value: Option<String>,
    pub constraints: Vec<String>,
    pub description: String,
}

/// Field dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDependency {
    pub field: String,
    pub depends_on: String,
    pub condition: String,
}

// ==================== VALIDATION BUILDERS ====================

/// Builder for creating validation errors
pub struct ValidationErrorBuilder {
    field: String,
    message: String,
    error_type: ValidationErrorType,
    current_value: Option<String>,
    expected_format: Option<String>,
}

impl ValidationErrorBuilder {
    /// Create a new validation error builder
    pub fn new(field: &str, message: &str, error_type: ValidationErrorType) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            error_type,
            current_value: None,
            expected_format: None,
        }
    }

    /// Set the current value
    pub fn current_value(mut self, value: &str) -> Self {
        self.current_value = Some(value.to_string());
        self
    }

    /// Set the expected format
    pub fn expected_format(mut self, format: &str) -> Self {
        self.expected_format = Some(format.to_string());
        self
    }

    /// Build the validation error
    pub fn build(self) -> ValidationError {
        ValidationError {
            field: self.field,
            message: self.message,
            error_type: self.error_type,
            current_value: self.current_value,
            expected_format: self.expected_format,
        }
    }
}

// ==================== VALIDATION UTILITIES ====================

/// Validation utilities for common patterns
pub struct ValidationUtils;

impl ValidationUtils {
    /// Validate a port number
    pub fn validate_port(field: &str, port: u16) -> Option<ValidationError> {
        if port == 0 {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Port cannot be 0",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&port.to_string())
                .expected_format("1-65535")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a timeout duration
    pub fn validate_timeout(field: &str, timeout: Duration) -> Option<ValidationError> {
        if timeout.is_zero() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Timeout cannot be zero",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&format!("{:?}", timeout))
                .expected_format("> 0ms")
                .build(),
            )
        } else if timeout > Duration::from_secs(3600) {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Timeout is unusually long (>1 hour)",
                    ValidationErrorType::OutOfRange,
                )
                .current_value(&format!("{:?}", timeout))
                .expected_format("1ms - 1 hour")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate an IP address string
    pub fn validate_ip_address(field: &str, ip: &str) -> Option<ValidationError> {
        if ip.parse::<IpAddr>().is_err() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Invalid IP address format",
                    ValidationErrorType::InvalidFormat,
                )
                .current_value(ip)
                .expected_format("IPv4 (e.g., 192.168.1.1) or IPv6 (e.g., ::1)")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a socket address string
    pub fn validate_socket_address(field: &str, addr: &str) -> Option<ValidationError> {
        if addr.parse::<SocketAddr>().is_err() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Invalid socket address format",
                    ValidationErrorType::InvalidFormat,
                )
                .current_value(addr)
                .expected_format("IP:PORT (e.g., 127.0.0.1:8080)")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a file path exists
    pub fn validate_file_path(field: &str, path: &Path) -> Option<ValidationError> {
        if !path.exists() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "File does not exist",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else if !path.is_file() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Path is not a file",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a directory path exists
    pub fn validate_directory_path(field: &str, path: &Path) -> Option<ValidationError> {
        if !path.exists() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Directory does not exist",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else if !path.is_dir() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Path is not a directory",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&path.display().to_string())
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a string is not empty
    pub fn validate_non_empty_string(field: &str, value: &str) -> Option<ValidationError> {
        if value.trim().is_empty() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Value cannot be empty",
                    ValidationErrorType::Required,
                )
                .current_value(value)
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate a numeric range
    pub fn validate_range<T>(field: &str, value: T, min: T, max: T) -> Option<ValidationError>
    where
        T: PartialOrd + std::fmt::Display + Copy,
    {
        if value < min || value > max {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    &format!("Value {} is outside valid range", value),
                    ValidationErrorType::OutOfRange,
                )
                .current_value(&value.to_string())
                .expected_format(&format!("{} - {}", min, max))
                .build(),
            )
        } else {
            None
        }
    }
}

/// Network configuration
///
/// **CONSOLIDATED**: Now uses `CanonicalNetworkConfig` from
/// `crate::config::canonical_primary::domains::network::CanonicalNetworkConfig`
pub use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig as NetworkConfig;

// Note: Original validation impl has been moved to the CanonicalNetworkConfig's own validation
// This section is preserved for backward compatibility if any code directly calls
// validation on this module's NetworkConfig alias

impl ConfigValidation for NetworkConfig {
    fn validate(&self) -> ValidationResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        let suggestions = Vec::new();

        // Validate API configuration
        if self.api.port == 0 {
            errors.push(
                ValidationErrorBuilder::new(
                    "api.port",
                    "API port cannot be 0",
                    ValidationErrorType::Required,
                )
                .build(),
            );
        }

        // Validate network performance configuration
        if self.performance.buffer_size == 0 {
            errors.push(
                ValidationErrorBuilder::new(
                    "performance.buffer_size",
                    "Buffer size cannot be 0",
                    ValidationErrorType::Required,
                )
                .build(),
            );
        }

        // Validate security configuration
        if self.security.firewall_enabled && self.security.allowed_ips.is_empty() {
            warnings.push(ValidationWarning {
                field: "security.firewall_enabled".to_string(),
                message:
                    "Firewall is enabled but no allowed IPs configured - this may block all traffic"
                        .to_string(),
                severity: WarningSeverity::High,
            });
        }

        ValidationResult {
            is_valid: errors.is_empty(),
            errors,
            warnings,
            suggestions,
        }
    }

    fn schema() -> ValidationSchema {
        let mut fields = HashMap::new();

        fields.insert(
            "bind_address".to_string(),
            FieldSchema {
                field_type: "string".to_string(),
                required: true,
                default_value: Some(crate::constants::hardcoding::addresses::LOCALHOST_IPV4.to_string()),
                constraints: vec!["Valid IPv4 or IPv6 address".to_string()],
                description: "IP address to bind the server to".to_string(),
            },
        );

        fields.insert(
            "port".to_string(),
            FieldSchema {
                field_type: "u16".to_string(),
                required: true,
                default_value: Some("8080".to_string()),
                constraints: vec!["1-65535".to_string()],
                description: "Port number to listen on".to_string(),
            },
        );

        fields.insert(
            "timeout_ms".to_string(),
            FieldSchema {
                field_type: "u64".to_string(),
                required: true,
                default_value: Some("30000".to_string()),
                constraints: vec!["> 0".to_string()],
                description: "Request timeout in milliseconds".to_string(),
            },
        );

        ValidationSchema {
            fields,
            dependencies: vec![
                FieldDependency {
                    field: "tls_cert_path".to_string(),
                    depends_on: "enable_tls".to_string(),
                    condition: "required when enable_tls is true".to_string(),
                },
                FieldDependency {
                    field: "tls_key_path".to_string(),
                    depends_on: "enable_tls".to_string(),
                    condition: "required when enable_tls is true".to_string(),
                },
            ],
        }
    }
}

// ==================== VALIDATION RUNNER ====================

/// Configuration validation runner
pub struct ConfigValidator;

impl ConfigValidator {
    /// Validate a configuration and return detailed results
    pub fn validate<T: ConfigValidation>(config: &T) -> ValidationResult {
        config.validate()
    }

    /// Validate and return a Result type
    pub async fn validate_strict<T: ConfigValidation>(config: &T) -> crate::Result<()> {
        config.validate().into_result().await
    }

    /// Generate validation report as formatted string
    pub fn generate_report<T: ConfigValidation>(config: &T) -> String {
        let result = config.validate();
        let mut report = String::new();

        report.push_str("Configuration Validation Report\n");
        report.push_str(&format!(
            "Status: {}\n\n",
            if result.is_valid {
                "✅ VALID"
            } else {
                "❌ INVALID"
            }
        ));

        if !result.errors.is_empty() {
            report.push_str("Errors:\n");
            for error in &result.errors {
                report.push_str(&format!("  ❌ {}: {}\n", error.field, error.message));
                if let Some(current) = &error.current_value {
                    report.push_str(&format!("     Current: {}\n", current));
                }
                if let Some(expected) = &error.expected_format {
                    report.push_str(&format!("     Expected: {}\n", expected));
                }
            }
            report.push('\n');
        }

        if !result.warnings.is_empty() {
            report.push_str("Warnings:\n");
            for warning in &result.warnings {
                let icon = match warning.severity {
                    WarningSeverity::High => "🔶",
                    WarningSeverity::Medium => "🔸",
                    WarningSeverity::Low => "🔹",
                };
                report.push_str(&format!(
                    "  {} {}: {}\n",
                    icon, warning.field, warning.message
                ));
            }
            report.push('\n');
        }

        if !result.suggestions.is_empty() {
            report.push_str("Suggestions:\n");
            for suggestion in &result.suggestions {
                report.push_str(&format!(
                    "  💡 {}: {}\n",
                    suggestion.field, suggestion.message
                ));
                if let Some(suggested) = &suggestion.suggested_value {
                    report.push_str(&format!("     Suggested: {}\n", suggested));
                }
            }
        }

        report
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_utils_port() {
        assert!(ValidationUtils::validate_port("test_port", 0).is_some());
        assert!(ValidationUtils::validate_port("test_port", 8080).is_none());
    }

    #[test]
    fn test_validation_utils_ip() {
        assert!(ValidationUtils::validate_ip_address("test_ip", "invalid").is_some());
        assert!(ValidationUtils::validate_ip_address("test_ip", "127.0.0.1").is_none());
        assert!(ValidationUtils::validate_ip_address("test_ip", "::1").is_none());
    }

    #[test]
    fn test_validation_utils_range() {
        assert!(ValidationUtils::validate_range("test_range", 5, 1, 10).is_none());
        assert!(ValidationUtils::validate_range("test_range", 15, 1, 10).is_some());
        assert!(ValidationUtils::validate_range("test_range", 0, 1, 10).is_some());
    }

    #[test]
    fn test_network_config_validation() {
        let config = NetworkConfig::development_optimized();
        // CanonicalNetworkConfig.validate() returns Result<()>
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_network_config_invalid_port() {
        let mut config = NetworkConfig::development_optimized();
        config.api.port = 0;

        // CanonicalNetworkConfig.validate() returns Result<()>
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_result_with_trait() {
        use crate::config::validation::ConfigValidation;

        let mut config = NetworkConfig::development_optimized();
        config.api.port = 0;

        // Using ConfigValidation trait returns ValidationResult
        let result = ConfigValidation::validate(&config);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[tokio::test]
    async fn test_validation_result_conversion() {
        let result = ValidationResult::success();
        assert!(result.into_result().await.is_ok());

        let result = ValidationResult::with_errors(vec![ValidationErrorBuilder::new(
            "test",
            "test error",
            ValidationErrorType::Required,
        )
        .build()]);
        assert!(result.into_result().await.is_err());
    }

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }
}
