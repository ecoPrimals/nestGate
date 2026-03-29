// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **WORKSPACE MANAGEMENT MODULE**
//!
//! Comprehensive workspace management system providing CRUD operations,
//! optimization, storage management, collaboration, teams, secrets, and templates.

// Core workspace operations
/// **CRUD OPERATIONS**
///
/// Create, Read, Update, Delete operations for workspace entities.
pub mod crud;

/// **WORKSPACE LIFECYCLE** (backup, restore, migration)
pub mod lifecycle;

/// **WORKSPACE OPTIMIZATION**
///
/// Performance optimization and resource management for workspaces.
pub mod optimization;
#[cfg(test)]
mod optimization_tests;

#[cfg(test)]
mod lifecycle_tests;

/// **WORKSPACE STORAGE**
///
/// Storage allocation and management for workspace data.
pub mod storage;

// Team and collaboration management modules

/// **COLLABORATION FEATURES**
///
/// Real-time collaboration and sharing features for workspaces.
pub mod collaboration;

/// **TEAM MANAGEMENT**
///
/// Team creation, management, and permission handling.
pub mod teams;

// Security and configuration modules

/// **SECRETS MANAGEMENT**
///
/// Secure storage and management of workspace secrets and credentials.
pub mod secrets;

/// **WORKSPACE TEMPLATES**
///
/// Pre-configured workspace templates for common use cases.
pub mod templates;

// Re-export key functions for public API
pub use crud::{
    create_workspace, delete_workspace, get_workspace, get_workspaces, update_workspace_config,
};
// pub use lifecycle::{
//     backup_workspace, migrate_workspace, restore_workspace, schedule_cleanup,
//     update_workspace_lifecycle,
// };
pub use teams::{create_team, get_teams};
// pub use lifecycle::{BackupConfig, MigrationConfig, MigrationStrategy, RestoreConfig};

// Team management
pub use secrets::create_workspace_secret;

// Collaboration features (stub implementations) - commented out until implemented
// pub use collaboration::{add_collaborator, get_collaborators, remove_collaborator};

// Template management - commented out until implemented
// pub use templates::create_from_template;

/// **WORKSPACE MANAGER**
///
/// Central manager for all workspace operations and coordination.
#[derive(Debug, Clone)]
/// Manager for Workspace operations
pub struct WorkspaceManager {
    // Implementation details hidden for brevity
}

impl Default for WorkspaceManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl WorkspaceManager {
    /// Create a new workspace manager instance
    #[must_use]
    pub const fn new() -> Self {
        Self {
            // Initialize with default configuration
        }
    }
}

// TEMP_DISABLED: mod lifecycle_tests - file does not exist (use lifecycle_comprehensive_tests or lifecycle_new_tests)

#[cfg(test)]
mod collaboration_tests;

#[cfg(test)]
mod storage_workspace_tests;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod crud_final_tests;
#[cfg(test)]
mod crud_tests;

#[cfg(test)]
mod storage_tests;

#[cfg(test)]
mod teams_tests;

#[cfg(test)]
mod secrets_tests;

#[cfg(test)]
mod templates_tests;

#[cfg(test)]
mod error_path_tests_comprehensive;
