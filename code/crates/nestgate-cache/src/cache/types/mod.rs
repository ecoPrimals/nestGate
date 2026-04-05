// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Cache Types and Configuration
//! Core types, enums, and configuration structures for the caching system.

mod entry;
mod policy;
mod stats;
mod tier;

#[cfg(test)]
mod tests;

pub use entry::CacheEntry;
pub use policy::CachePolicy;
pub use stats::{CacheStats, EfficiencyMetrics};
pub use tier::{CacheStorageTierExt, StorageTier};
