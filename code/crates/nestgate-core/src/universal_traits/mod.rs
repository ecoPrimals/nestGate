// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

// **UNIVERSAL TRAITS - CANONICAL MODERNIZED**
//! Module definitions and exports.
// This module provides the universal trait system for NestGate ecosystem integration,
// organized into focused modules for better maintainability and clarity.
//! Module definitions and exports.
// Universal trait system for NestGate ecosystem integration with modular organization.

// ==================== CORE TRAIT MODULES ====================

// Security-related traits and types
pub mod security;
// Orchestration and workflow traits
pub mod orchestration;
// Compute and resource management traits
pub mod compute;
// Service discovery and ecosystem integration
pub mod ecosystem;
// Common types and structures used across traits
pub mod types;
// ==================== RE-EXPORTS FOR COMPATIBILITY ====================

pub use compute::ComputePrimalProvider;
pub use ecosystem::{EcosystemIntegration, UniversalPrimalProvider};
pub use orchestration::OrchestrationPrimalProvider;
pub use security::SecurityDecision;
#[expect(deprecated, reason = "migration in progress")]
// Re-export for backwards compatibility
pub use security::SecurityPrimalProvider;
pub use types::*;

// ==================== UNIVERSAL TRAITS SYSTEM ====================
