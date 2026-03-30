// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]

//! **HARDWARE TUNING TYPES**
//!
//! Data structures and type definitions for hardware tuning operations.

mod allocation;
mod capabilities;
mod config;
mod monitors;
mod results;
mod sessions;
mod system_collector;

#[cfg(test)]
mod tests;

pub use allocation::*;
pub use capabilities::*;
pub use config::*;
pub use monitors::*;
pub use results::*;
pub use sessions::*;
pub use system_collector::*;
