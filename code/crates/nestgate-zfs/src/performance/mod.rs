// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Real-time performance monitoring, metrics collection, and alerting
// for ZFS storage tiers with integration to orchestrator and AI systems.

//! Performance module

pub mod defaults;
pub mod monitor;
pub mod types;
#[cfg(test)]
mod types_tests;

// Re-export all public types and functions

pub use types::*;
