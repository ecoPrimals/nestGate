// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **ZERO-COST UNIVERSAL PROVIDERS - CANONICAL MODERNIZATION COMPLETE**
//!
//! This module provides zero-cost universal provider implementations that eliminate
//! the runtime overhead of `async_trait` and `Arc<dyn>` patterns.

mod aliases;
mod compute;
mod migration;
mod orchestration;
mod status;
mod types;

pub use aliases::{
    HighPerformanceZeroCostComputeWrapper, HighPerformanceZeroCostOrchestrationWrapper,
    StandardZeroCostComputeWrapper, StandardZeroCostOrchestrationWrapper,
};
pub use compute::{ZeroCostComputeProvider, ZeroCostUniversalComputeWrapper};
pub use migration::ZERO_COST_MIGRATION_GUIDE;
pub use orchestration::{ZeroCostOrchestrationProvider, ZeroCostUniversalOrchestrationWrapper};
pub use status::{ComputeResources, ServiceStatus};
pub use types::{AuthToken, Credentials, SecurityDecision, Signature};

#[cfg(test)]
mod tests;
