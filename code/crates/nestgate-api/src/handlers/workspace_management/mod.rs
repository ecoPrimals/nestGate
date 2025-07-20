//! Workspace Management API Handlers
//!
//! Comprehensive workspace lifecycle management including CRUD operations,
//! team management, storage operations, backup/recovery, optimization,
//! migration, and collaborative features.
//!
//! This module has been refactored into focused sub-modules for better organization:
//! - crud: Basic workspace CRUD operations  
//! - teams: Team management functionality
//! - storage: Storage operations and status monitoring
//! - backup: Backup and recovery operations
//! - optimization: Workspace optimization functionality
//! - migration: Migration operations and helpers
//! - collaboration: Sharing functionality
//! - templates: Template management
//! - secrets: Secrets management (delegated to security module)

mod backup;
mod collaboration;
mod crud;
mod migration;
mod optimization;
mod secrets;
mod storage;
mod teams;
mod templates;

// Re-export public API to maintain backward compatibility
pub use backup::{create_workspace_backup, restore_workspace};
pub use collaboration::{share_workspace, unshare_workspace};
pub use crud::{create_workspace, get_workspace, get_workspaces, update_workspace_config};
pub use migration::migrate_workspace;
pub use optimization::optimize_workspace;
pub use secrets::create_workspace_secret;
pub use storage::{cleanup_workspace, delete_workspace, get_workspace_status, scale_workspace};
pub use teams::{create_team, get_teams};
pub use templates::{apply_workspace_template, create_workspace_template};
