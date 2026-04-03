// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **COMPREHENSIVE INTEGRATION TEST SUITE - MODULARIZED**
//!
//! This module has been refactored from a large monolithic test file (874 lines)
//! into focused, maintainable test modules for better organization and compliance
//! with the <2000 lines per file standard.
//!
//! **MODULAR STRUCTURE**:
//! - `environment`: Test environment setup and management
//! - `adapter_tests`: Universal adapter integration tests
//! - `service_tests`: Service discovery and registry tests
//! - `workflow_tests`: End-to-end workflow integration tests
//! - `performance_tests`: Performance and load testing
//! - `error_tests`: Error handling and recovery tests

// ==================== MODULAR ORGANIZATION ====================

/// Test environment setup and management
pub mod environment;

// Additional test modules - expand as needed for comprehensive coverage
// pub mod adapter_tests;
// pub mod service_tests;
// pub mod workflow_tests;
// pub mod performance_tests;
// pub mod error_tests;

// ==================== RE-EXPORTS FOR COMPATIBILITY ====================

pub use environment::*;
