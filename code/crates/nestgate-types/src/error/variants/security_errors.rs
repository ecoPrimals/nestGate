// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Security error variants and utilities
//! Security Errors functionality and utilities.
//! This module provides security-specific error types and helper functions.

use super::core_errors::{NestGateUnifiedError, SecurityErrorDetails};

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

// ==================== CONVENIENCE CONSTRUCTORS ON MAIN ERROR TYPE ====================

impl NestGateUnifiedError {
    /// Create an authentication error (convenience constructor)
    ///
    /// # Example
    /// ```
    /// use nestgate_types::error::NestGateError;
    /// let error = NestGateError::auth("Invalid credentials");
    /// ```
    pub fn auth(message: impl Into<String>) -> Self {
        Self::Security(Box::new(SecurityErrorDetails::authentication_error(
            message,
        )))
    }

    /// Create a security error (full form)
    pub fn security(message: impl Into<String>) -> Self {
        Self::Security(Box::new(SecurityErrorDetails::new(message)))
    }

    /// Create an authorization error with principal
    pub fn authorization(message: impl Into<String>, principal: impl Into<String>) -> Self {
        Self::Security(Box::new(SecurityErrorDetails::authorization_error(
            message, principal,
        )))
    }
}
