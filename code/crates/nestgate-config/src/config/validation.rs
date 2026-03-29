// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Modern Configuration Validation Module
//!
//! Provides comprehensive, type-safe configuration validation with detailed
//! error reporting and recovery suggestions using modern Rust patterns.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::time::Duration;

use nestgate_types::error::NestGateError;

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
/// Validationresult
pub struct ValidationResult {
    /// Whether valid
    pub is_valid: bool,
    /// Errors
    pub errors: Vec<ValidationError>,
    /// Warnings
    pub warnings: Vec<ValidationWarning>,
    /// Suggestions
    pub suggestions: Vec<ValidationSuggestion>,
}

impl ValidationResult {
    /// Create a successful validation result
    #[must_use]
    pub const fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Create a failed validation result with errors
    #[must_use]
    pub const fn with_errors(errors: Vec<ValidationError>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Add a warning to the result
    #[must_use]
    pub fn with_warning(mut self, warning: ValidationWarning) -> Self {
        self.warnings.push(warning);
        self
    }

    /// Add a suggestion to the result
    #[must_use]
    pub fn with_suggestion(mut self, suggestion: ValidationSuggestion) -> Self {
        self.suggestions.push(suggestion);
        self
    }

    /// Convert to a Result type
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when [`Self::is_valid`] is false, aggregating validation messages.
    pub fn into_result(self) -> nestgate_types::error::Result<()> {
        if self.is_valid {
            Ok(())
        } else {
            let error_messages: Vec<String> = self
                .errors
                .iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect();

            Err(NestGateError::validation_error(format!(
                "Configuration validation failed: {}",
                error_messages.join(", ")
            )))
        }
    }
}

/// Validation error with field context
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Error type for Validation operations
pub struct ValidationError {
    /// Field
    pub field: String,
    /// Message
    pub message: String,
    /// Error Type
    pub error_type: ValidationErrorType,
    /// Current Value
    pub current_value: Option<String>,
    /// Expected Format
    pub expected_format: Option<String>,
}

/// Types of validation errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Types of `ValidationError`
pub enum ValidationErrorType {
    /// Required
    Required,
    /// Invalidformat
    InvalidFormat,
    /// Outofrange
    OutOfRange,
    /// Invalidvalue
    InvalidValue,
    /// Conflict
    Conflict,
    /// Security
    Security,
}

/// Validation warning for non-critical issues
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationwarning
pub struct ValidationWarning {
    /// Field
    pub field: String,
    /// Message
    pub message: String,
    /// Severity
    pub severity: WarningSeverity,
}

/// Warning severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Warningseverity
pub enum WarningSeverity {
    /// Low
    Low,
    /// Medium
    Medium,
    /// High
    High,
}

/// Validation suggestion for improvements
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationsuggestion
pub struct ValidationSuggestion {
    /// Field
    pub field: String,
    /// Message
    pub message: String,
    /// Suggested Value
    pub suggested_value: Option<String>,
}

/// Validation schema for documentation
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Validationschema
pub struct ValidationSchema {
    /// Fields
    pub fields: HashMap<String, FieldSchema>,
    /// Dependencies
    pub dependencies: Vec<FieldDependency>,
}

/// Schema for individual fields
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Fieldschema
pub struct FieldSchema {
    /// Field Type
    pub field_type: String,
    /// Required
    pub required: bool,
    /// Default Value
    pub default_value: Option<String>,
    /// Constraints
    pub constraints: Vec<String>,
    /// Human-readable description
    pub description: String,
}

/// Field dependency specification
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Fielddependency
pub struct FieldDependency {
    /// Field
    pub field: String,
    /// Depends On
    pub depends_on: String,
    /// Condition
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
    #[must_use]
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
    #[must_use]
    pub fn current_value(mut self, value: &str) -> Self {
        self.current_value = Some(value.to_string());
        self
    }

    /// Set the expected format
    #[must_use]
    pub fn expected_format(mut self, format: &str) -> Self {
        self.expected_format = Some(format.to_string());
        self
    }

    /// Build the validation error
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn validate_timeout(field: &str, timeout: Duration) -> Option<ValidationError> {
        if timeout.is_zero() {
            Some(
                ValidationErrorBuilder::new(
                    field,
                    "Timeout cannot be zero",
                    ValidationErrorType::InvalidValue,
                )
                .current_value(&format!("{timeout:?}"))
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
                .current_value(&format!("{timeout:?}"))
                .expected_format("1ms - 1 hour")
                .build(),
            )
        } else {
            None
        }
    }

    /// Validate an IP address string
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
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
                    &format!("Value {value} is outside valid range"),
                    ValidationErrorType::OutOfRange,
                )
                .current_value(&value.to_string())
                .expected_format(&format!("{min} - {max}"))
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
// Network config is in domains/network module
pub use crate::config::canonical_primary::domains::network::CanonicalNetworkConfig as NetworkConfig;

// Note: Original validation impl has been moved to the CanonicalNetworkConfig's own validation
// This section is preserved for backward compatibility if any code directly calls
// validation on this module's NetworkConfig alias

impl ConfigValidation for NetworkConfig {
    /// Validates data
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

    /// Schema
    fn schema() -> ValidationSchema {
        let mut fields = HashMap::new();

        fields.insert(
            "bind_address".to_string(),
            FieldSchema {
                field_type: "string".to_string(),
                required: true,
                default_value: Some(
                    "127.0.0.1".to_string(), // Localhost IPv4 - safe development default
                ),
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
    ///
    /// # Errors
    ///
    /// Returns [`NestGateError`] when [`ValidationResult::into_result`] reports validation errors.
    pub fn validate_strict<T: ConfigValidation>(config: &T) -> nestgate_types::error::Result<()> {
        config.validate().into_result()
    }

    /// Generate validation report as formatted string
    pub fn generate_report<T: ConfigValidation>(config: &T) -> String {
        let result = config.validate();
        let mut report = String::new();

        report.push_str("Configuration Validation Report\n");
        let _ = writeln!(
            report,
            "Status: {}\n",
            if result.is_valid {
                "✅ VALID"
            } else {
                "❌ INVALID"
            }
        );

        if !result.errors.is_empty() {
            report.push_str("Errors:\n");
            for error in &result.errors {
                let _ = writeln!(report, "  ❌ {}: {}", error.field, error.message);
                if let Some(current) = &error.current_value {
                    let _ = writeln!(report, "     Current: {current}");
                }
                if let Some(expected) = &error.expected_format {
                    let _ = writeln!(report, "     Expected: {expected}");
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
                let _ = writeln!(report, "  {icon} {}: {}", warning.field, warning.message);
            }
            report.push('\n');
        }

        if !result.suggestions.is_empty() {
            report.push_str("Suggestions:\n");
            for suggestion in &result.suggestions {
                let _ = writeln!(report, "  💡 {}: {}", suggestion.field, suggestion.message);
                if let Some(suggested) = &suggestion.suggested_value {
                    let _ = writeln!(report, "     Suggested: {suggested}");
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
    use tempfile::NamedTempFile;
    use tempfile::tempdir;

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
    fn test_validation_utils_range_inclusive_boundaries() {
        assert!(ValidationUtils::validate_range("r", 1, 1, 10).is_none());
        assert!(ValidationUtils::validate_range("r", 10, 1, 10).is_none());
    }

    #[test]
    fn test_validation_utils_file_path_existing_file() {
        let file = NamedTempFile::new().expect("test: temp file");
        assert!(ValidationUtils::validate_file_path("path", file.path()).is_none());
    }

    #[test]
    fn test_validation_utils_file_path_missing() {
        let path = std::env::temp_dir().join("nestgate_validation_missing_file_xyz");
        assert!(ValidationUtils::validate_file_path("path", &path).is_some());
    }

    #[test]
    fn test_validation_utils_file_path_directory_not_file() {
        let dir = tempdir().expect("test: temp dir");
        assert!(ValidationUtils::validate_file_path("path", dir.path()).is_some());
    }

    #[test]
    fn test_validation_utils_directory_path_existing() {
        let dir = tempdir().expect("test: temp dir");
        assert!(ValidationUtils::validate_directory_path("dir", dir.path()).is_none());
    }

    #[test]
    fn test_validation_utils_directory_path_file_not_dir() {
        let file = NamedTempFile::new().expect("test: temp file");
        assert!(ValidationUtils::validate_directory_path("dir", file.path()).is_some());
    }

    #[test]
    fn test_validation_utils_directory_path_missing() {
        let path = std::env::temp_dir().join("nestgate_validation_missing_dir_xyz");
        assert!(ValidationUtils::validate_directory_path("dir", &path).is_some());
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
        assert!(result.into_result().is_ok());

        let result = ValidationResult::with_errors(vec![
            ValidationErrorBuilder::new("test", "test error", ValidationErrorType::Required)
                .build(),
        ]);
        assert!(result.into_result().is_err());
    }

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_utils_timeout_zero() {
        assert!(ValidationUtils::validate_timeout("timeout", Duration::ZERO).is_some());
    }

    #[test]
    fn test_validation_utils_timeout_too_long() {
        assert!(ValidationUtils::validate_timeout("timeout", Duration::from_secs(4000)).is_some());
    }

    #[test]
    fn test_validation_utils_timeout_valid() {
        assert!(ValidationUtils::validate_timeout("timeout", Duration::from_secs(30)).is_none());
    }

    #[test]
    fn test_validation_utils_socket_address() {
        assert!(ValidationUtils::validate_socket_address("addr", "invalid").is_some());
        assert!(ValidationUtils::validate_socket_address("addr", "127.0.0.1:8080").is_none());
    }

    #[test]
    fn test_validation_utils_non_empty_string() {
        assert!(ValidationUtils::validate_non_empty_string("field", "").is_some());
        assert!(ValidationUtils::validate_non_empty_string("field", "   ").is_some());
        assert!(ValidationUtils::validate_non_empty_string("field", "value").is_none());
    }

    #[test]
    fn test_validation_result_with_warning() {
        let result = ValidationResult::success().with_warning(ValidationWarning {
            field: "test".to_string(),
            message: "warning".to_string(),
            severity: WarningSeverity::Medium,
        });
        assert!(result.is_valid);
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_validation_result_with_suggestion() {
        let result = ValidationResult::success().with_suggestion(ValidationSuggestion {
            field: "test".to_string(),
            message: "suggestion".to_string(),
            suggested_value: Some("value".to_string()),
        });
        assert!(result.is_valid);
        assert_eq!(result.suggestions.len(), 1);
    }

    #[test]
    fn test_validation_error_builder_full() {
        let err = ValidationErrorBuilder::new("field", "msg", ValidationErrorType::InvalidFormat)
            .current_value("bad")
            .expected_format("good")
            .build();
        assert_eq!(err.field, "field");
        assert_eq!(err.error_type, ValidationErrorType::InvalidFormat);
        assert_eq!(err.current_value, Some("bad".to_string()));
        assert_eq!(err.expected_format, Some("good".to_string()));
    }

    #[test]
    fn test_config_validator_generate_report_valid() {
        let config = NetworkConfig::development_optimized();
        let report = ConfigValidator::generate_report(&config);
        assert!(report.contains("VALID"));
    }

    #[test]
    fn test_config_validator_generate_report_invalid() {
        let mut config = NetworkConfig::development_optimized();
        config.api.port = 0;
        let report = ConfigValidator::generate_report(&config);
        assert!(report.contains("INVALID"));
        assert!(report.contains("Errors:"));
    }

    #[test]
    fn test_validation_schema_network_config() {
        let schema = NetworkConfig::schema();
        assert!(!schema.fields.is_empty());
        assert!(schema.fields.contains_key("port"));
        assert!(!schema.dependencies.is_empty());
    }

    #[test]
    fn test_validation_error_types() {
        let _ = ValidationErrorType::Required;
        let _ = ValidationErrorType::InvalidFormat;
        let _ = ValidationErrorType::OutOfRange;
        let _ = ValidationErrorType::InvalidValue;
        let _ = ValidationErrorType::Conflict;
        let _ = ValidationErrorType::Security;
    }

    #[test]
    fn test_warning_severity_levels() {
        let _ = WarningSeverity::Low;
        let _ = WarningSeverity::Medium;
        let _ = WarningSeverity::High;
    }

    #[test]
    fn test_field_schema_construction() {
        let _schema = FieldSchema {
            field_type: "string".to_string(),
            required: true,
            default_value: Some("default".to_string()),
            constraints: vec!["pattern".to_string()],
            description: "desc".to_string(),
        };
    }

    #[test]
    fn test_config_validator_validate_matches_trait() {
        let config = NetworkConfig::development_optimized();
        let a = ConfigValidator::validate(&config);
        let b = ConfigValidation::validate(&config);
        assert_eq!(a.is_valid, b.is_valid);
        assert_eq!(a.errors.len(), b.errors.len());
    }

    #[test]
    fn test_config_validator_validate_strict_ok() {
        let config = NetworkConfig::development_optimized();
        ConfigValidator::validate_strict(&config)
            .expect("test: valid config should pass strict validation");
    }

    #[test]
    fn test_config_validator_validate_strict_err() {
        let mut config = NetworkConfig::development_optimized();
        config.api.port = 0;
        let err = ConfigValidator::validate_strict(&config)
            .expect_err("test: invalid port should fail strict validation");
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn test_network_config_invalid_buffer_trait() {
        let mut config = NetworkConfig::development_optimized();
        config.performance.buffer_size = 0;
        let result = ConfigValidation::validate(&config);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_network_config_firewall_warning_trait() {
        let mut config = NetworkConfig::development_optimized();
        config.security.firewall_enabled = true;
        config.security.allowed_ips.clear();
        let result = ConfigValidation::validate(&config);
        assert!(!result.warnings.is_empty());
    }

    // --- Round 5: extra validation branches ---
    #[test]
    fn validation_utils_port_max_boundary() {
        assert!(ValidationUtils::validate_port("p", u16::MAX).is_none());
    }

    #[test]
    fn validation_utils_range_exclusive_outside() {
        assert!(ValidationUtils::validate_range("r", 0, 1, 10).is_some());
        assert!(ValidationUtils::validate_range("r", 11, 1, 10).is_some());
    }

    #[test]
    fn validation_result_with_errors_not_valid() {
        let r = ValidationResult::with_errors(vec![
            ValidationErrorBuilder::new("f", "e", ValidationErrorType::InvalidValue).build(),
        ]);
        assert!(!r.is_valid);
    }

    #[test]
    fn validation_error_type_conflict_and_security() {
        let _ = ValidationErrorType::Conflict;
        let _ = ValidationErrorType::Security;
    }

    #[test]
    fn validation_utils_non_empty_trims_not_applied_to_whitespace_only_inner() {
        assert!(ValidationUtils::validate_non_empty_string("f", "  \t  ").is_some());
    }
}
