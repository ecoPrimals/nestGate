// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Automation, system, internal, and external error detail structs.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::context::ErrorContext;
use crate::error::data::AutomationErrorData;

/// Automation error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Automation error: {message}")]
/// Automationerrordetails
pub struct AutomationErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Automation operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Target resource
    pub target: Option<Cow<'static, str>>,
    /// Automation-specific error data
    pub automation_data: Option<Box<AutomationErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// System error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("System error: {message}")]
/// Systemerrordetails
pub struct SystemErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// System component that failed
    pub component: Cow<'static, str>,
    /// System operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Internal error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Internal error: {message}")]
/// Internalerrordetails
pub struct InternalErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Component where error occurred
    pub component: Cow<'static, str>,
    /// Location in code (<file:line>)
    pub location: Option<Cow<'static, str>>,
    /// Whether this indicates a bug
    pub is_bug: bool,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// External error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("External error: {message}")]
/// Externalerrordetails
pub struct ExternalErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// External service or dependency
    pub service: Cow<'static, str>,
    /// Whether the operation is retryable
    pub retryable: bool,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}
