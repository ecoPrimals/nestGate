//! Security error variants and utilities
//! Security Errors functionality and utilities.
//! This module provides security-specific error types and helper functions.

use super::core_errors::SecurityErrorDetails;

impl SecurityErrorDetails {
    /// Create a security error with just a message
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: None,
            principal: None,
            security_data: None,
            context: None,
        }
    }

    /// Create an authentication error
    pub fn authentication_error(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: Some("authentication".to_string()),
            principal: None,
            security_data: None,
            context: None,
        }
    }

    /// Create an authorization error
    pub fn authorization_error(message: impl Into<String>, principal: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            operation: Some("authorization".to_string()),
            principal: Some(principal.into()),
            security_data: None,
            context: None,
        }
    }
}
