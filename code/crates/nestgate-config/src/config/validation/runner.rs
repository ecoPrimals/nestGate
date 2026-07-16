// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! [`ConfigValidator`] report and strict validation entry points.

use std::fmt::Write;

use super::types::{ConfigValidation, ValidationResult, WarningSeverity};

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
    /// Returns [`NestGateError`](nestgate_types::error::NestGateError) when [`ValidationResult::into_result`] reports validation errors.
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
            if result.is_valid { "VALID" } else { "INVALID" }
        );

        if !result.errors.is_empty() {
            report.push_str("Errors:\n");
            for error in &result.errors {
                let _ = writeln!(report, "  error {}: {}", error.field, error.message);
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
                    WarningSeverity::High => "[high]",
                    WarningSeverity::Medium => "[medium]",
                    WarningSeverity::Low => "[low]",
                };
                let _ = writeln!(report, "  {icon} {}: {}", warning.field, warning.message);
            }
            report.push('\n');
        }

        if !result.suggestions.is_empty() {
            report.push_str("Suggestions:\n");
            for suggestion in &result.suggestions {
                let _ = writeln!(
                    report,
                    "  suggestion {}: {}",
                    suggestion.field, suggestion.message
                );
                if let Some(suggested) = &suggestion.suggested_value {
                    let _ = writeln!(report, "     Suggested: {suggested}");
                }
            }
        }

        report
    }
}

#[cfg(test)]
mod tests {
    use super::super::types::{
        ValidationError, ValidationErrorType, ValidationResult, ValidationSchema,
        ValidationSuggestion, ValidationWarning, WarningSeverity,
    };
    use super::*;

    fn empty_schema() -> ValidationSchema {
        ValidationSchema {
            fields: std::collections::HashMap::new(),
            dependencies: Vec::new(),
        }
    }

    struct ValidConfig;
    impl ConfigValidation for ValidConfig {
        fn validate(&self) -> ValidationResult {
            ValidationResult::success()
        }
        fn schema() -> ValidationSchema {
            empty_schema()
        }
    }

    struct InvalidConfig;
    impl ConfigValidation for InvalidConfig {
        fn validate(&self) -> ValidationResult {
            ValidationResult::with_errors(vec![ValidationError {
                field: "port".into(),
                message: "port must be > 0".into(),
                error_type: ValidationErrorType::OutOfRange,
                current_value: Some("0".into()),
                expected_format: Some("1..65535".into()),
            }])
        }
        fn schema() -> ValidationSchema {
            empty_schema()
        }
    }

    struct WarningConfig;
    impl ConfigValidation for WarningConfig {
        fn validate(&self) -> ValidationResult {
            ValidationResult::success()
                .with_warning(ValidationWarning {
                    field: "timeout".into(),
                    message: "timeout is very high".into(),
                    severity: WarningSeverity::Medium,
                })
                .with_suggestion(ValidationSuggestion {
                    field: "timeout".into(),
                    message: "consider lowering timeout".into(),
                    suggested_value: Some("30".into()),
                })
        }
        fn schema() -> ValidationSchema {
            empty_schema()
        }
    }

    #[test]
    fn validate_valid_config() {
        let result = ConfigValidator::validate(&ValidConfig);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn validate_strict_valid_config() {
        assert!(ConfigValidator::validate_strict(&ValidConfig).is_ok());
    }

    #[test]
    fn validate_strict_invalid_config() {
        assert!(ConfigValidator::validate_strict(&InvalidConfig).is_err());
    }

    #[test]
    fn generate_report_valid() {
        let report = ConfigValidator::generate_report(&ValidConfig);
        assert!(report.contains("VALID"));
        assert!(!report.contains("Errors:"));
    }

    #[test]
    fn generate_report_invalid_contains_errors() {
        let report = ConfigValidator::generate_report(&InvalidConfig);
        assert!(report.contains("INVALID"));
        assert!(report.contains("Errors:"));
        assert!(report.contains("port must be > 0"));
        assert!(report.contains("Current: 0"));
        assert!(report.contains("Expected: 1..65535"));
    }

    #[test]
    fn generate_report_warnings_and_suggestions() {
        let report = ConfigValidator::generate_report(&WarningConfig);
        assert!(report.contains("VALID"));
        assert!(report.contains("Warnings:"));
        assert!(report.contains("[medium]"));
        assert!(report.contains("timeout is very high"));
        assert!(report.contains("Suggestions:"));
        assert!(report.contains("consider lowering timeout"));
        assert!(report.contains("Suggested: 30"));
    }
}
