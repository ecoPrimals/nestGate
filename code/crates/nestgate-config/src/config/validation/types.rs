// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Core validation result types, errors, schema metadata, and the error builder.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use nestgate_types::error::NestGateError;

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
pub struct ValidationSchema {
    /// Fields
    pub fields: HashMap<String, FieldSchema>,
    /// Dependencies
    pub dependencies: Vec<FieldDependency>,
}

/// Schema for individual fields
#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct FieldDependency {
    /// Field
    pub field: String,
    /// Depends On
    pub depends_on: String,
    /// Condition
    pub condition: String,
}

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
