// SPDX-License-Identifier: AGPL-3.0-or-later
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

//! **CHAOS ENGINEERING AND FAULT TOLERANCE TEST SUITE**
//!
//! Comprehensive chaos testing to validate system resilience and fault tolerance
//! under adverse conditions. This significantly improves test coverage for edge cases.
//!
//! **MODERN CONCURRENCY**: Uses tokio::time::sleep for realistic async delays (network
//! latency, exponential backoff) and yield_now() for coordination where appropriate.
//!
//! Modules under `tests/chaos_engineering_suite/` group tests by concern: network chaos,
//! storage chaos, concurrency, recovery/config, fault-tolerance patterns, and light performance checks.

#[path = "chaos_engineering_suite/common.rs"]
mod common;
#[path = "chaos_engineering_suite/concurrency_tests.rs"]
mod concurrency_tests;
#[path = "chaos_engineering_suite/fault_tolerance_tests.rs"]
mod fault_tolerance_tests;
#[path = "chaos_engineering_suite/network_tests.rs"]
mod network_tests;
#[path = "chaos_engineering_suite/performance_tests.rs"]
mod performance_tests;
#[path = "chaos_engineering_suite/recovery_and_config_tests.rs"]
mod recovery_and_config_tests;
#[path = "chaos_engineering_suite/storage_chaos_tests.rs"]
mod storage_chaos_tests;
