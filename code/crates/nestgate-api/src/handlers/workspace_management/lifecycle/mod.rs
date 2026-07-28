// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Workspace Lifecycle Management
//!
//! This module provides advanced lifecycle operations for workspaces including
//! backup, restore, migration, and lifecycle policy management using ZFS snapshots
//! and send/receive operations.

mod backup;
mod list;
mod migration;
mod restore;
mod types;

#[cfg(test)]
pub use backup::backup_workspace;
#[cfg(test)]
pub use list::list_workspace_backups;
#[cfg(test)]
pub use migration::migrate_workspace;
#[cfg(test)]
pub use restore::restore_workspace;
#[cfg(test)]
pub use types::{BackupConfig, MigrationConfig, MigrationStrategy, RestoreConfig};

#[cfg(test)]
mod tests;
