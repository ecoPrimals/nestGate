// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

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

//! Unit Tests Runner
//!
//! Integration test runner that executes unit tests from the unit/ subdirectory.
//! This is necessary because Rust doesn't automatically discover tests in subdirectories.

// Include unit test modules
#[path = "unit/network_edge_cases_tests.rs"]
mod network_edge_cases_tests;

#[path = "unit/zfs_operations_tests.rs"]
mod zfs_operations_tests;

#[path = "unit/error_recovery_tests.rs"]
mod error_recovery_tests;

// Re-export the tests so they're discovered by cargo test
