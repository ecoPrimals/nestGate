//
// Automated data migration system for moving files between storage tiers
// based on access patterns, performance requirements, and system policies.
//
// This module provides comprehensive tier migration capabilities with:
// - Intelligent migration job scheduling and queue management
// - Automated discovery of migration candidates based on access patterns
// - High-performance file operations with progress tracking
// - Integrity verification and metadata preservation
// - Configurable policies and performance limits

// Module declarations
pub mod discovery;
pub mod engine;
pub mod file_operations;
pub mod queue;
pub mod tests;
pub mod types;
pub mod utilities;

// Re-export public API
pub use engine::MigrationEngine;
pub use types::*;

// Re-export key functions for use by other modules
pub use discovery::discover_migration_candidates;
pub use file_operations::{copy_file_with_progress, update_file_metadata, verify_file_integrity};
pub use queue::process_migration_queue;
pub use utilities::{
    construct_target_path, ensure_target_dataset_exists, get_target_dataset_for_tier,
    get_tier_from_path,
};

// Re-export for backward compatibility
pub use engine::MigrationEngine as ZfsMigrationEngine;
pub use types::MigrationConfig;
