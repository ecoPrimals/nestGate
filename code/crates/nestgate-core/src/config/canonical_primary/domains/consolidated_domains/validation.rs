//! **CONFIGURATION VALIDATION FRAMEWORK**
//!
//! Provides traits and utilities for validating domain configurations.
//!
//! ## Usage
//!
//! ```rust
//! use nestgate_core::config::canonical_primary::domains::consolidated_domains::validation::*;
//!
//! struct MyConfig;
//!
//! impl DomainConfigValidation for MyConfig {
//!     fn validate(&self) -> crate::error::Result<Vec<String>> {
//!         Ok(vec![])
//!     }
//!
//!     fn validate_for_environment(&self, env: &str) -> crate::error::Result<()> {
//!         Ok(())
//!     }
//!
//!     fn required_fields() -> Vec<&'static str> {
//!         vec!["field1", "field2"]
//!     }
//!
//!     fn optional_fields() -> Vec<&'static str> {
//!         vec!["field3"]
//!     }
//! }
//! ```

use serde::{Deserialize, Serialize};

/// **DOMAIN CONFIGURATION VALIDATION**
///
/// Trait for validating domain-specific configurations.
/// All domain configs should implement this trait.
pub trait DomainConfigValidation {
    /// Validate configuration and return warnings (non-fatal issues)
    ///
    /// # Errors
    ///
    /// Returns error if configuration is fundamentally invalid
    fn validate(&self) -> crate::error::Result<Vec<String>>;

    /// Validate configuration for specific environment
    ///
    /// # Errors
    ///
    /// Returns error if configuration is invalid for the given environment
    fn validate_for_environment(&self, env: &str) -> crate::error::Result<()>;

    /// Get list of required field names
    fn required_fields() -> Vec<&'static str> {
        Vec::new()
    }

    /// Get list of optional field names
    fn optional_fields() -> Vec<&'static str> {
        Vec::new()
    }
}

/// **VALIDATION ERROR**
///
/// Represents a configuration validation error with context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// Field that failed validation
    pub field: String,

    /// Error message
    pub message: String,

    /// Suggested fix
    pub suggestion: Option<String>,
}

impl ValidationError {
    /// Create new validation error
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
            suggestion: None,
        }
    }

    /// Add suggestion to error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.message)?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, " (Suggestion: {})", suggestion)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error() {
        let error = ValidationError::new("port", "Invalid port number")
            .with_suggestion("Use port between 1024-65535");

        assert_eq!(error.field, "port");
        assert_eq!(error.message, "Invalid port number");
        assert!(error.suggestion.is_some());
    }

    #[test]
    fn test_validation_error_display() {
        let error = ValidationError::new("port", "Invalid port number");
        let display = format!("{}", error);
        assert!(display.contains("port"));
        assert!(display.contains("Invalid port number"));
    }
}
