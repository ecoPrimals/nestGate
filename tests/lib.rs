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

//! Test library: exposes `common` test infrastructure and `chaos` test suite.

pub mod common;

#[cfg(test)]
pub mod chaos;
