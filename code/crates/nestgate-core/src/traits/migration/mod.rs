// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **TRAIT MIGRATION ADAPTERS**
//!
//! This module provides adapter types that bridge old storage provider traits
//! to the new `CanonicalStorage` trait. These adapters enable gradual migration
//! without breaking existing code.
//!
//! **Purpose**: Allow implementations of old traits to work with code expecting
//! the new canonical traits during the migration period.
//!
//! **Usage Example**:
//! ```rust,ignore
//! use nestgate_core::traits::migration::NativeAsyncStorageAdapter;
//! use nestgate_core::traits::CanonicalStorage;
//!
//! let old_storage = MyOldNativeAsyncStorage::new();
//! let adapted: Box<dyn CanonicalStorage> = Box::new(
//!     NativeAsyncStorageAdapter::new(old_storage)
//! );
//! ```
//!
//! **Removal**: These adapters are temporary and will be removed in Week 10-12
//! after all migrations are complete.

pub mod storage_adapters;

// Re-export commonly used adapters
pub use storage_adapters::{
    NativeAsyncStorageAdapter, StoragePrimalAdapter, ZeroCostStorageAdapter,
};
