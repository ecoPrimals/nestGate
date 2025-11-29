//
// This module provides advanced configuration management capabilities including:
// - Hot-reload of configuration without service restart
// - Configuration versioning and history tracking
// - Rollback to previous configurations
// - Validation of configuration changes before application
// - Atomic configuration updates with rollback on failure

//! Dynamic Config module

pub mod manager;
pub mod types;
pub mod validators;

pub use manager::*;
pub use types::*;
pub use validators::*;
