// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Validation, timeout, I/O, and resource exhaustion error details.

use std::borrow::Cow;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::context::ErrorContext;

/// Validation error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Validation error: {message}")]
/// Validationerrordetails
pub struct ValidationErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Field that failed validation
    pub field: Option<Cow<'static, str>>,
    /// Expected value or format
    pub expected: Option<Cow<'static, str>>,
    /// Actual value that failed
    pub actual: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Timeout error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Timeout error: {message}")]
/// Timeouterrordetails
pub struct TimeoutErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Operation that timed out
    pub operation: Option<Cow<'static, str>>,
    /// Timeout duration
    pub timeout: Duration,
    /// Whether the operation is retryable
    pub retryable: bool,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// I/O error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("I/O error: {message}")]
/// Ioerrordetails
pub struct IoErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Path or resource involved
    pub path: Option<Cow<'static, str>>,
    /// I/O operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Resource exhausted error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Resource exhausted: {message}")]
/// Resourceexhaustederrordetails
pub struct ResourceExhaustedErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Resource that was exhausted
    pub resource: Cow<'static, str>,
    /// Current usage
    pub current: Option<u64>,
    /// Maximum limit
    pub limit: Option<u64>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}
