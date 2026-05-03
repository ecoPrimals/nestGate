// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **COMPREHENSIVE INTEGRATION TEST SUITE - MODULARIZED**
//!
//! This module has been refactored from a large monolithic test file (874 lines)
//! into focused, maintainable test modules for better organization and compliance
//! with the <2000 lines per file standard.
//!
//! Currently only `environment` is active; the original adapter/service/workflow/
//! performance/error test modules were migrated to per-crate `#[cfg(test)]` tests.

/// Test environment setup and management
pub mod environment;

pub use environment::*;
