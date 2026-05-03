// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    unused,
    dead_code,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction
)]

//! Integration test suite harness.
//!
//! Original adapter/service/workflow/performance/error test modules were migrated
//! to per-crate `#[cfg(test)]` tests. Only `environment` remains here.

pub mod integration_test_suite;
pub use self::integration_test_suite::*;
