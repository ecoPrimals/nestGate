// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

// **ERROR VARIANTS - MODULAR ARCHITECTURE**
//! Domain-specific error variant modules for maintainable error handling.
//!
//! Module definitions and exports.
//! This module organizes error variants into focused domain modules, replacing
//! the monolithic variants.rs with a more maintainable modular structure.

// Core error types
pub mod api_errors;
pub mod automation_errors;
pub mod core_errors;
pub mod network_errors;
pub mod security_errors;
pub mod storage_errors;
pub mod system_errors;

#[cfg(test)]
mod core_errors_tests;

// Re-export the main unified error type
pub use core_errors::NestGateUnifiedError;

// Domain-specific error implementations are available through the unified type

/// The primary error type for all `NestGate` operations
///
/// This is the canonical error type used across all `NestGate` crates and modules.
/// It provides a unified interface for error handling with domain-specific variants.
pub type NestGateError = NestGateUnifiedError;
