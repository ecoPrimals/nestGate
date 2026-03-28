// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

// Deprecated `ZeroCostSecurityProvider` and related surface until v0.12 removal

//! **ZERO-COST UNIVERSAL PROVIDERS - CANONICAL MODERNIZATION COMPLETE**
//!
//! This module provides zero-cost universal provider implementations that eliminate
//! the runtime overhead of `async_trait` and `Arc<dyn>` patterns.

mod aliases;
mod compute;
mod migration;
mod orchestration;
mod security;
mod status;
mod types;

pub use aliases::{
    HighPerformanceZeroCostComputeWrapper, HighPerformanceZeroCostOrchestrationWrapper,
    HighPerformanceZeroCostSecurityWrapper, StandardZeroCostComputeWrapper,
    StandardZeroCostOrchestrationWrapper, StandardZeroCostSecurityWrapper,
};
pub use compute::{ZeroCostComputeProvider, ZeroCostUniversalComputeWrapper};
pub use migration::ZERO_COST_MIGRATION_GUIDE;
pub use orchestration::{ZeroCostOrchestrationProvider, ZeroCostUniversalOrchestrationWrapper};
pub use security::{ZeroCostSecurityProvider, ZeroCostUniversalSecurityWrapper};
pub use status::{ComputeResources, ServiceStatus};
pub use types::{AuthToken, Credentials, SecurityDecision, Signature};

#[cfg(test)]
mod tests;
