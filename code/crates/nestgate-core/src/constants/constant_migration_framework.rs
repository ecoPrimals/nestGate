// **CONSTANT MIGRATION FRAMEWORK - ENTRY POINT**
//! Constant Migration Framework functionality and utilities.
// This module provides the main entry point for the constant migration framework.
// The actual implementation has been split into focused modules for maintainability.

// Re-export the migration framework from the modular structure
pub use super::migration::*;

// Convenience functions for common migration operations
use crate::error::Result;

/// Create a new constant migrator for the most common use case
#[must_use]
pub const fn create_migrator() -> ConstantMigrator {
    ConstantMigrator::new("default".to_string())
}
/// Create a migrator with dry-run enabled for testing
#[must_use]
pub const fn create_dry_run_migrator() -> ConstantMigrator {
    let options = ConstantMigrationOptions {
        dry_run: true,
        verbose: true,
        ..ConstantMigrationOptions::default()
    };
    ConstantMigrator::with_options("dry-run".to_string(), options)
}
/// Run a quick migration with default settings
#[must_use]
pub fn quick_migrate() -> Result<ConstantMigrationReport> {
    let mut migrator = create_migrator();
    migrator.migrate()
}
/// Run a safe migration with all validation enabled
pub const fn safe_migrate() -> Result<ConstantMigrationReport> {
    let options = ConstantMigrationOptions {
        strict_mode: true,
        backup_original_files: true,
        validate_after_migration: true,
        ..Default::default()
    };

    let mut migrator = ConstantMigrator::with_options("safe-migration".to_string(), options);
    migrator.migrate()
}
