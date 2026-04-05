// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Configuration and HTTP API error detail structs.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::context::ErrorContext;

/// Configuration error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Configuration error in {field}: {message}")]
/// Configurationerrordetails
pub struct ConfigurationErrorDetails {
    /// The configuration field that caused the error
    pub field: Cow<'static, str>,
    /// Error message
    pub message: Cow<'static, str>,
    /// Current invalid value
    pub currentvalue: Option<Cow<'static, str>>,
    /// Expected value or format
    pub expected: Option<Cow<'static, str>>,
    /// Whether this is a user configuration error
    pub user_error: bool,
}

/// API error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("API error: {message}")]
/// Apierrordetails
pub struct ApiErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// HTTP status code
    pub status_code: Option<u16>,
    /// Request ID for tracing
    pub request_id: Option<Cow<'static, str>>,
    /// API endpoint that failed
    pub endpoint: Option<Cow<'static, str>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}
