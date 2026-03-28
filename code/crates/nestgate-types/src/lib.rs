// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Foundational error, result, and unified enum types shared across NestGate crates.

#![warn(missing_docs)]

pub mod error;
pub mod result_types;
pub mod unified_enums;

pub use error::{
    CanonicalResult, ErrorContext, NestGateError, NestGateUnifiedError, Result, ResultExt,
    RetryInfo, TestResult,
};
pub use result_types::{
    ConnectionFactory, HealthCheckFn, ValidatorFn, VoidResult,
};
pub use unified_enums::UnifiedEnum;
