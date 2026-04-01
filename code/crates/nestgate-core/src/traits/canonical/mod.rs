// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **CANONICAL UNIFIED TRAIT SYSTEM**
//!
//! This module provides the unified trait system that consolidates all `NestGate` functionality.
//!
//! # Architecture
//!
//! **Refactored**: November 19, 2025 - Split from single 1,100-line file into focused modules
//!
//! ## Module Structure:
//!
//! - `service` - Core service trait (~150 lines)
//! - `provider` - Provider trait (~40 lines)
//! - `storage` - Storage operations trait (~220 lines)
//! - `network` - Network operations trait (~200 lines)
//! - `security` - Security operations trait (~280 lines)
//! - `factories` - Factory traits (~25 lines)
//! - `types` - Supporting types (~160 lines)
//!
//! ## Modern Rust Patterns:
//!
//! 1. **Native Async** - Uses `impl Future` (RPITIT) instead of `async_trait`
//! 2. **Zero-Cost** - No heap allocations, no vtable dispatch overhead
//! 3. **Type Safety** - Strong typing with associated types
//! 4. **Modularity** - Logical separation by concern (each file <400 lines)
//!
//! ## Benefits of Refactoring:
//!
//! - ✅ **File Compliance** - All files under 1,000 lines
//! - ✅ **Maintainability** - Easier to navigate and modify
//! - ✅ **Clarity** - Each concern in its own module
//! - ✅ **API Compatibility** - Re-exports maintain existing imports
//!
//! These traits use `impl Future` returns which may trigger `clippy::type_complexity` warnings
//! but represent modern async Rust patterns and are more efficient than boxed futures.

// Module declarations
mod factories;
mod network;
mod provider;
mod security;
mod service;
mod storage;
pub(crate) mod types;

// Re-export all public items to maintain API compatibility
pub use factories::{
    CanonicalAutomation, CanonicalMcp, CanonicalProviderFactory, CanonicalServiceFactory,
    ZeroCostService,
};
pub use network::CanonicalNetwork;
pub use provider::CanonicalProvider;
pub use security::CanonicalSecurity;
pub use service::CanonicalService;
pub use storage::CanonicalStorage;
pub use types::{
    ConnectionHandle, ConnectionStatus, CronSchedule, DataStream, HealthStatus, MessageMetadata,
    NetworkConnection, ProviderCapabilities, ProviderHealth, ScheduleId, ScheduleInfo,
    SecurityCredentials, SecurityPolicy, ServiceCapabilities, SnapshotInfo, StorageBackendType,
    StorageCapability, StorageMetadata, StorageUsageStats, StreamHandle, WriteStream,
};
