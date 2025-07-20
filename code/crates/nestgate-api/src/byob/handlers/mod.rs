//! BYOB API Handler Modules
//!
//! This module contains all the HTTP handlers for BYOB operations,
//! organized by functionality.

pub mod datasets;
pub mod projects;
pub mod snapshots;
pub mod storage;
pub mod teams;
pub mod workspaces;

// Re-export all handlers for convenience
pub use datasets::*;
pub use projects::*;
pub use snapshots::*;
pub use storage::*;
pub use teams::*;
pub use workspaces::*;
