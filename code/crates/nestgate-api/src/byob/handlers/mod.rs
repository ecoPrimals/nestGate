//
// Canonical modernized handlers for BYOB operations integrated with
// the unified storage system for workspace functionality.

//! Handlers module

pub mod datasets;
pub mod projects;
pub mod snapshots;
pub mod teams;

// Re-export for convenience
pub use datasets::*;
pub use projects::*;
pub use snapshots::*;
pub use teams::*;
