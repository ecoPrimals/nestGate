// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::panic,
        clippy::unwrap_used,
        clippy::too_many_lines,
    )
)]
#![allow(
    deprecated,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::unused_async,
    clippy::cast_precision_loss,
    clippy::items_after_statements
)]

//! **NESTGATE BINARY LIBRARY**
//!
//! Command-line interface and utilities for `NestGate`

pub mod cli;
pub mod commands;
pub mod error;

pub use error::{NestGateBinError, Result};
