// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Storage, network, and security error detail structs.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::error::context::ErrorContext;
use crate::error::data::{NetworkErrorData, SecurityErrorData, StorageErrorData};

/// Storage error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Storage error: {message}")]
/// Storageerrordetails
pub struct StorageErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Storage operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Path or resource involved
    pub resource: Option<Cow<'static, str>>,
    /// Storage-specific error data
    pub storage_data: Option<Box<StorageErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Network error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Network error: {message}")]
/// Networkerrordetails
pub struct NetworkErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Network operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// Remote endpoint
    pub endpoint: Option<Cow<'static, str>>,
    /// Network-specific error data
    pub network_data: Option<Box<NetworkErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}

/// Security error details
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[error("Security error: {message}")]
/// Securityerrordetails
pub struct SecurityErrorDetails {
    /// Error message
    pub message: Cow<'static, str>,
    /// Security operation that failed
    pub operation: Option<Cow<'static, str>>,
    /// User or principal involved
    pub principal: Option<Cow<'static, str>>,
    /// Security-specific error data
    pub security_data: Option<Box<SecurityErrorData>>,
    /// Error context
    pub context: Option<Box<ErrorContext>>,
}
