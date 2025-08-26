///
/// This module organizes storage-related type definitions into focused, maintainable modules.
/// This replaces the previous 1564-line monolithic file with a clean modular architecture.
///
/// **MODULES**:
/// - `resources` - Core storage resources, capacity, and health types
/// - `metrics` - Performance metrics, I/O statistics, and monitoring
/// - `config` - Configuration types for backup, encryption, and redundancy
/// - `access` - Access control, permissions, and metadata types
///
/// **PROVIDES**:
/// - Single source of truth for ALL storage types (maintained)
/// - Modular organization for better maintainability
/// - Clear separation of concerns
/// - <500 lines per module (2000-line compliance)
// Core storage resource types
pub mod resources;

// Storage metrics and performance monitoring
pub mod metrics;

// Storage configuration types
pub mod config;

// Access control and metadata types
pub mod access;

// Re-export all public types for backward compatibility
pub use access::*;
pub use config::*;
pub use metrics::*;
pub use resources::*;
