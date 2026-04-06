// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! JSON-RPC + tarpc IPC layer for primal-to-primal communication.
//!
//! Extracted from nestgate-core to enable parallel compilation.

#![warn(missing_docs)]
// Many `Result` helpers differ only by error path; per-function `# Errors` would duplicate noise.
// Unit tests use unwrap/expect/panic and exercise large API surfaces; keep lib code strict elsewhere.
#![cfg_attr(
    test,
    allow(
        dead_code,
        deprecated,
        missing_docs,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::pedantic,
        clippy::nursery,
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
    )
)]

pub mod rpc;

pub use rpc::*;
