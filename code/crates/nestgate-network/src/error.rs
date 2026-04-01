// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

/// **NETWORK ERROR TYPES**
///
/// Unified error handling for network operations and configurations.
use nestgate_core::error::NestGateError;
use thiserror::Error;

// ==================== SECTION ====================

/// Network-specific error types
#[derive(Debug, Error)]
/// Errors that can occur during Network operations
pub enum NetworkError {
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    #[error("Timeout occurred: {operation:?}")]
    Timeout { operation: Option<String> },
    #[error("Configuration error: {field} - {message}")]
    Configuration { field: String, message: String },
    #[error("Protocol error: {protocol} - {message}")]
    Protocol { protocol: String, message: String },
    #[error("Service unavailable: {service}")]
    ServiceUnavailable { service: String },
    #[error("Core error: {0}")]
    Core(#[from] NestGateError),
}

/// **CANONICAL**: Network-specific Result type using canonical Result
/// This follows the canonical Result<T,E> pattern with domain-specific error type
pub type NetworkResult<T> = std::result::Result<T, NetworkError>;
