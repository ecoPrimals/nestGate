// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![forbid(unsafe_code)]
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

//! **NESTGATE BINARY LIBRARY**
//!
//! Command-line interface and utilities for `NestGate`

pub mod cli;
pub mod commands;
pub mod error;

pub use error::{NestGateBinError, Result};
