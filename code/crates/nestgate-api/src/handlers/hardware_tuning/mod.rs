// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **HARDWARE TUNING MODULE**
//!
//! Hardware tuning functionality split into logical modules for better maintainability.
//!
//! With `dev-stubs`: full handlers via `handlers` and `handlers_production`.
//! Without `dev-stubs` (production): read-only endpoints backed by `/proc` and
//! ZFS kstat; service registration returns `501 Not Implemented`.

// Development: Real stub handlers
#[cfg(feature = "dev-stubs")]
pub mod handlers;
/// Axum JSON handlers backed by the same `/proc` logic as [`handlers::RealHardwareTuningHandler`].
#[cfg(feature = "dev-stubs")]
pub mod handlers_production;
/// `/hardware/config` and `/hardware/tune` HTTP entry points (always available).
pub mod http_routes;
/// `/proc`-based resource helpers (production hardware tuning and dev-stubs).
pub(crate) mod linux_proc;
/// Best-effort hardware snapshots from procfs/sysfs (shared by production and tooling).
pub(crate) mod procfs_helpers;

/// Production handlers: read-only endpoints backed by `/proc` and ZFS kstat.
#[cfg(not(feature = "dev-stubs"))]
pub mod native_handlers;
#[cfg(not(feature = "dev-stubs"))]
pub use native_handlers as handlers;

pub mod types;

#[cfg(test)]
mod strategic_coverage_tests;

// Re-export the main types and functions
pub use handlers::*;
#[cfg(feature = "dev-stubs")]
pub use handlers_production::*;
pub use http_routes::{get_hardware_config, post_hardware_tune};
pub use types::*;
