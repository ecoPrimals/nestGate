//
// This module provides the enterprise filesystem backend implementation
// with full support for snapshots, replication, analytics, and tiering.
//
// The backend is split into logical modules for maintainability:
// - `core`: Core struct and basic operations
// - `storage_ops`: Standard storage operations (CanonicalStorageBackend)
// - `zero_copy_ops`: Zero-copy storage operations
// - `enterprise_ops`: Advanced enterprise features (snapshots, replication, etc.)

mod core;
// **MODULARIZED OPERATIONS** - Split from 934-line monolithic file
mod ops;
mod storage_ops;
mod zero_copy_ops;

// enterprise_ops has been removed - use modular ops/ structure instead

// Re-export the main backend struct and implementations
pub use core::EnterpriseStorageBackend;

// All trait implementations are defined in their respective modules
// and automatically available when the struct is imported
