// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
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
// Test library — exposes common test infrastructure.
// Test utility modules contain building blocks that are selectively used.
#![expect(dead_code)]

pub mod common;

/// Comprehensive chaos testing module
#[cfg(test)]
pub mod chaos;
