// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! The unified [`NestGateUnifiedError`] enum (boxed variants for size efficiency).

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::details::{
    ApiErrorDetails, AutomationErrorDetails, ConfigurationErrorDetails, ExternalErrorDetails,
    HandlerErrorDetails, InternalErrorDetails, IoErrorDetails, LoadBalancerErrorDetails,
    NetworkErrorDetails, NotImplementedErrorDetails, PerformanceErrorDetails,
    ResourceExhaustedErrorDetails, SecurityErrorDetails, StorageErrorDetails, SystemErrorDetails,
    TestingErrorDetails, TimeoutErrorDetails, ValidationErrorDetails,
};

/// **THE** definitive `NestGate` error type - single source of truth for all errors
///
/// This enum is designed to be small in memory by boxing all the large variants.
/// This eliminates the `clippy::result_large_err` warnings while maintaining full functionality.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
/// Errors that can occur during `NestGateUnified` operations
pub enum NestGateUnifiedError {
    /// Configuration-related errors (boxed for size efficiency)
    #[error("Configuration error: {0}")]
    Configuration(Box<ConfigurationErrorDetails>),

    /// API-related errors (boxed for size efficiency)
    #[error("API error: {0}")]
    Api(Box<ApiErrorDetails>),

    /// Storage and ZFS errors (boxed for size efficiency)
    #[error("Storage error: {0}")]
    Storage(Box<StorageErrorDetails>),

    /// Network and communication errors (boxed for size efficiency)
    #[error("Network error: {0}")]
    Network(Box<NetworkErrorDetails>),

    /// Security and authentication errors (boxed for size efficiency)
    #[error("Security error: {0}")]
    Security(Box<SecurityErrorDetails>),

    /// Automation system errors (boxed for size efficiency)
    #[error("Automation error: {0}")]
    Automation(Box<AutomationErrorDetails>),

    /// System resource and internal errors (boxed for size efficiency)
    #[error("System error: {0}")]
    System(Box<SystemErrorDetails>),

    /// Internal processing errors (boxed for size efficiency)
    #[error("Internal error: {0}")]
    Internal(Box<InternalErrorDetails>),

    /// External dependency errors (boxed for size efficiency)
    #[error("External error: {0}")]
    External(Box<ExternalErrorDetails>),

    /// Validation errors (boxed for size efficiency)
    #[error("Validation error: {0}")]
    Validation(Box<ValidationErrorDetails>),

    /// Timeout errors (boxed for size efficiency)
    #[error("Timeout error: {0}")]
    Timeout(Box<TimeoutErrorDetails>),

    /// I/O operation errors (boxed for size efficiency)
    #[error("I/O error: {0}")]
    Io(Box<IoErrorDetails>),

    /// Resource exhaustion errors (boxed for size efficiency)
    #[error("Resource exhausted: {0}")]
    ResourceExhausted(Box<ResourceExhaustedErrorDetails>),

    /// Testing framework errors (boxed for size efficiency)
    #[error("Testing error: {0}")]
    Testing(Box<TestingErrorDetails>),

    /// Performance and benchmarking errors (boxed for size efficiency)
    #[error("Performance error: {0}")]
    Performance(Box<PerformanceErrorDetails>),

    /// Handler execution errors (boxed for size efficiency)
    #[error("Handler error: {0}")]
    Handler(Box<HandlerErrorDetails>),

    /// Load balancer errors (boxed for size efficiency)
    #[error("Load balancer error: {0}")]
    LoadBalancer(Box<LoadBalancerErrorDetails>),

    /// Not implemented functionality (boxed for size efficiency)
    #[error("Not implemented: {0}")]
    NotImplemented(Box<NotImplementedErrorDetails>),
}
