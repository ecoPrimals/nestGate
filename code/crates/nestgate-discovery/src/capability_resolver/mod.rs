// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Unified Capability Resolver - Bridge for All Registry Systems
//!
//! **ARCHITECTURAL UNIFICATION**: This module creates a common interface for all
//! registry and discovery systems in the codebase, enabling them to work together.
//!
//! **SYSTEMS UNIFIED**:
//! 1. `InMemoryServiceRegistry` (`service_discovery`)
//! 2. `ServiceRegistry` (`universal_primal_discovery`)
//! 3. `CapabilityDiscoveryManager`
//! 4. Application layer capabilities
//!
//! **PHILOSOPHY**: One interface to rule them all - capability-based discovery
//! regardless of underlying implementation.
//!
//! ## Submodules
//!
//! - [`types`] — [`ResolvedService`] and [`CapabilityResolver`]
//! - [`primal_discovery`] — Primal discovery registry adapter
//! - [`in_memory_registry`] — In-memory registry adapter and capability mapping
//! - [`environment`] — Environment-variable fallback resolver
//! - [`composite`] — Ordered multi-resolver chain (priority / fallback)

pub mod composite;
pub mod environment;
pub mod in_memory_registry;
pub mod primal_discovery;
pub mod types;

#[cfg(test)]
mod tests;

pub use composite::CompositeResolver;
pub use environment::EnvironmentResolver;
pub use in_memory_registry::InMemoryRegistryAdapter;
pub use primal_discovery::PrimalDiscoveryAdapter;
pub use types::{CapabilityResolver, ResolvedService};
