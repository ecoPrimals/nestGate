// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **RETURN BUILDERS MODULE SYSTEM**
// Breaking down the large return_builders.rs (898 lines) into focused modules
//! to achieve 2000-line file size compliance while maintaining functionality.

/// Return builders module for constructing complex return values.
///
/// This module provides builder functions for complex return value constructions,
/// extracted from inline struct creation to enable precise testing and catch
/// field assignment mutations.
// Response builder functions
pub mod response_builders;
// Mock data builders for testing / dev-stubs only
#[cfg(any(test, feature = "dev-stubs"))]
pub mod mock_builders;
// Configuration and utility builders
/// Configuration builder utilities for type-safe config construction
pub mod config_builders;
// Specialized JSON and API builders
/// JSON response builders for API handlers and test assertions
pub mod json_builders;
// Test modules
/// Test utilities and helper functions for builder pattern testing
pub mod tests;
// Re-export all public builder functions for backward compatibility
pub use json_builders::build_json_response;
#[cfg(any(test, feature = "dev-stubs"))]
pub use mock_builders::{build_access_grant, build_mock_resource_allocation};
pub use response_builders::{build_error_response, build_success_response};
