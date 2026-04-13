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
