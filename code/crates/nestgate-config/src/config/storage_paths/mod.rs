// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **XDG-COMPLIANT STORAGE PATH CONFIGURATION**
//!
//! Provides XDG Base Directory Specification-compliant storage paths with
//! intelligent fallback hierarchy for maximum portability and sovereignty.
//!
//! **Created**: January 30, 2026
//! **Purpose**: Phase 4 - Hardcoding Evolution (+4 bonus points)
//! **Impact**: Eliminates hardcoded `/var/lib/nestgate` and `/tmp/nestgate` paths
//!
//! ## Submodules
//!
//! - `resolve` — per-directory env / XDG resolution
//! - `paths` — `StoragePaths` aggregate and derived paths
//! - `global` — process-wide singleton and convenience getters

pub(crate) mod resolve;

mod global;
mod paths;

#[cfg(test)]
mod tests;

pub use super::substrate_tiers::{SubstrateMount, SubstrateTiers};
pub use global::{
    get_cache_dir, get_config_dir, get_data_dir, get_log_dir, get_runtime_dir,
    get_storage_base_path, get_storage_paths, get_temp_dir,
};
pub use paths::StoragePaths;
