// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **STORAGE TRAIT MIGRATION ADAPTERS**
//!
//! Adapters that wrap old storage provider traits and implement `CanonicalStorage`.
//! These enable gradual migration from old traits to the canonical hierarchy.

mod native_async;
mod primal;
mod traits;
mod zero_cost;

#[cfg(test)]
mod tests;

pub use native_async::NativeAsyncStorageAdapter;
pub use primal::StoragePrimalAdapter;
pub use traits::{NativeAsyncStorageProvider, StoragePrimalProvider, ZeroCostStorageProvider};
pub use zero_cost::ZeroCostStorageAdapter;
