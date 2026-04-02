// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
#![expect(
    clippy::too_long_first_doc_paragraph,
    reason = "Module overview is intentionally detailed for capability-discovery architecture."
)]

//! **UNIVERSAL CAPABILITY SYSTEM**
//! Capability System functionality and utilities.
//! This module implements a capability-based discovery and routing system that eliminates
//! all primal hardcoding. Each primal only knows itself and discovers others through
//! capability advertisement and discovery.

mod matching;
mod registry;
mod router;
mod self_knowledge;
mod types;

#[cfg(test)]
mod tests;

pub use matching::select_best_by_recency;
pub use registry::CapabilityRegistry;
pub use router::CapabilityRouter;
pub use self_knowledge::NestGateSelfKnowledge;
pub use types::{
    CapabilityCategory, CapabilityRequest, CapabilityResponse, DiscoveredService, ServiceCapability,
};
