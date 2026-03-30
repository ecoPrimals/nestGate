// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// **CORE ERROR TYPES**
//! Core system error types and handling for the `NestGate` system.
//! The main [`NestGateUnifiedError`] enum and core error handling.

mod constructors;
mod constructors_migration;
pub mod details;
mod severity;
mod unified_enum;

pub use details::{
    ApiErrorDetails, AutomationErrorDetails, ConfigurationErrorDetails, ExternalErrorDetails,
    HandlerErrorDetails, InternalErrorDetails, IoErrorDetails, LoadBalancerErrorDetails,
    NetworkErrorDetails, NotImplementedErrorDetails, PerformanceErrorDetails,
    ResourceExhaustedErrorDetails, SecurityErrorDetails, StorageErrorDetails, SystemErrorDetails,
    TestType, TestingErrorDetails, TimeoutErrorDetails, ValidationErrorDetails,
};
pub use severity::ErrorSeverity;
pub use unified_enum::NestGateUnifiedError;
