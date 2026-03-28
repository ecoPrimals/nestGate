// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **CANONICAL TRAIT HIERARCHY**
//!
//! This module defines the canonical trait hierarchy for the NestGate ecosystem.
//! It consolidates 35+ scattered provider trait variants into 5 core traits.
//!
//! **Design Principles**:
//! - Single Responsibility - Each trait has one clear purpose
//! - Native Async - Zero-cost abstractions (no `async_trait`)
//! - Composability - Traits build on each other
//! - Type Safety - Strong typing with clear contracts
//! - Performance - Zero-cost where possible
//!
//! **Trait Hierarchy**:
//! ```text
//! CanonicalService (base)
//!   ├─ CanonicalProvider (generic type `T`)
//!   ├─ CanonicalStorage (storage operations)
//!   ├─ CanonicalSecurity (security operations)
//!   └─ ZeroCostService (generic type `T`, performance marker)
//! ```
//!
//! **Date**: October 1, 2025
//! **Status**: Initial implementation - Week 3
//! **Replaces**: 35+ provider trait variants

mod canonical_provider;
mod canonical_security;
mod canonical_service;
mod canonical_storage;
mod zero_cost;

pub use canonical_provider::CanonicalProvider;
pub use canonical_security::CanonicalSecurity;
pub use canonical_service::CanonicalService;
pub use canonical_storage::CanonicalStorage;
pub use zero_cost::ZeroCostService;

// NOTE: These traits are NOT re-exported in the parent mod.rs yet
// to avoid conflicts with existing traits during the migration period.
// They will be exported after migration is complete (Week 8).
//
// For now, use them explicitly:
// use nestgate_core::traits::canonical_hierarchy::{CanonicalService, ...};

#[cfg(test)]
mod tests;
