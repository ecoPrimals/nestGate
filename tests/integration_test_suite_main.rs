#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! **COMPREHENSIVE INTEGRATION TEST SUITE - MODULARIZED**
//!
//! This file has been refactored from a large monolithic test implementation (874 lines)
//! into a clean modular structure for better maintainability and compliance with
//! the <2000 lines per file standard.
//!
//! **MIGRATION**: All test functionality has been moved to focused modules:
//! - `integration_test_suite/environment` - Test environment setup
//! - `integration_test_suite/adapter_tests` - Universal adapter tests
//! - `integration_test_suite/service_tests` - Service discovery tests
//! - `integration_test_suite/workflow_tests` - End-to-end workflow tests
//! - `integration_test_suite/performance_tests` - Performance tests
//! - `integration_test_suite/error_tests` - Error handling tests

// Re-export the modular integration test suite
pub mod integration_test_suite;
pub use self::integration_test_suite::*;
