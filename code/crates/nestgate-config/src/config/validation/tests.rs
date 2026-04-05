// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use std::time::Duration;

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
        ValidationErrorBuilder::new("test", "test error", ValidationErrorType::Required).build(),
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
