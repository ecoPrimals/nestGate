// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

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
mod tests;

pub use backup::backup_workspace;
pub use list::list_workspace_backups;
pub use migration::migrate_workspace;
pub use restore::restore_workspace;
pub use types::{
    BackupConfig, BackupConfigCanonical, MigrationConfig, MigrationConfigCanonical,
    MigrationStrategy, RestoreConfig, RestoreConfigCanonical,
};
