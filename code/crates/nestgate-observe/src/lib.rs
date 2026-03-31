// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Observability, diagnostics, and event system for `NestGate`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::module_inception,
        clippy::unwrap_used,
        clippy::panic,
        clippy::field_reassign_with_default,
        clippy::missing_const_for_fn,
        clippy::unnecessary_literal_bound,
    )
)]

mod compat;

pub use compat::canonical_types;
pub use compat::traits;

pub mod diagnostics;
pub mod observability;
