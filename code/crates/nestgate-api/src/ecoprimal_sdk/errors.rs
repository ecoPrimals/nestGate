//! Error types for EcoPrimal SDK
//!
//! This module defines all error types used throughout the EcoPrimal ecosystem.

use serde::{Deserialize, Serialize};
use std::fmt;
// Removed unused std import

/// EcoPrimal error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimalError {
    /// Configuration errors
    Configuration(String),
    /// Initialization errors
    Initialization(String),
    /// Request processing errors
    RequestProcessing(String),
    /// Resource errors (memory, disk, network)
    Resource(String),
    /// Network communication errors
    Network(String),
    /// Authentication errors
    Authentication(String),
    /// Authorization errors
    Authorization(String),
    /// Timeout errors
    Timeout(String),
    /// Internal server errors
    Internal(String),
    /// External dependency errors
    ExternalDependency(String),
    /// Validation errors
    Validation(String),
    /// Not found errors
    NotFound(String),
    /// Conflict errors
    Conflict(String),
    /// Rate limiting errors
    RateLimit(String),
    /// Service unavailable errors
    ServiceUnavailable(String),
    /// Unknown errors
    Unknown(String),
}

impl fmt::Display for PrimalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimalError::Configuration(msg) => write!(f, "Configuration error: {msg}"),
            PrimalError::Initialization(msg) => write!(f, "Initialization error: {msg}"),
            PrimalError::RequestProcessing(msg) => write!(f, "Request processing error: {msg}"),
            PrimalError::Resource(msg) => write!(f, "Resource error: {msg}"),
            PrimalError::Network(msg) => write!(f, "Network error: {msg}"),
            PrimalError::Authentication(msg) => write!(f, "Authentication error: {msg}"),
            PrimalError::Authorization(msg) => write!(f, "Authorization error: {msg}"),
            PrimalError::Timeout(msg) => write!(f, "Timeout error: {msg}"),
            PrimalError::Internal(msg) => write!(f, "Internal error: {msg}"),
            PrimalError::ExternalDependency(msg) => write!(f, "External dependency error: {msg}"),
            PrimalError::Validation(msg) => write!(f, "Validation error: {msg}"),
            PrimalError::NotFound(msg) => write!(f, "Not found error: {msg}"),
            PrimalError::Conflict(msg) => write!(f, "Conflict error: {msg}"),
            PrimalError::RateLimit(msg) => write!(f, "Rate limit error: {msg}"),
            PrimalError::ServiceUnavailable(msg) => write!(f, "Service unavailable error: {msg}"),
            PrimalError::Unknown(msg) => write!(f, "Unknown error: {msg}"),
        }
    }
}

impl std::error::Error for PrimalError {}
