// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Modern Configuration Validation Module
//!
//! Provides comprehensive, type-safe configuration validation with detailed
//! error reporting and recovery suggestions using modern Rust patterns.

mod network_config;
mod runner;
mod types;
mod utils;

#[cfg(test)]
mod tests;

pub use network_config::NetworkConfig;
pub use runner::ConfigValidator;
pub use types::{
    ConfigValidation, FieldDependency, FieldSchema, ValidationError, ValidationErrorBuilder,
    ValidationErrorType, ValidationResult, ValidationSchema, ValidationSuggestion,
    ValidationWarning, WarningSeverity,
};
pub use utils::ValidationUtils;
