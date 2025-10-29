//! **UNIVERSAL ECOSYSTEM INTEGRATION - MODULARIZED**
//!
//! This module has been refactored from a large monolithic file (873 lines) into
//! focused, maintainable modules for better code organization and compliance
//! with the <2000 lines per file standard.
//!
//! **MODULAR STRUCTURE**:
//! - `types`: Core types and data structures
//! - `registration`: Service registration logic
//! - `discovery`: Service discovery implementation
//! - `capabilities`: Capability management
//! - `resources`: Resource specification and management
//! - `endpoints`: Endpoint management and routing
//! - `integration`: Integration preferences and configuration

// ==================== MODULAR ORGANIZATION ====================

//! Core types and data structures
pub mod types;
//! Service registration logic
pub mod registration;
//! Service discovery implementation
pub mod discovery;
//! Capability management
pub mod capabilities;
//! Resource specification and management
pub mod resources;
//! Endpoint management and routing
pub mod endpoints;
//! Integration preferences and configuration
pub mod integration;
//! Universal ecosystem adapter
pub mod adapter;
// ==================== RE-EXPORTS FOR COMPATIBILITY ====================

pub use types::*;
pub use registration::*;
pub use discovery::*;
pub use capabilities::*;
pub use resources::*;
pub use endpoints::*;
pub use integration::*;
pub use adapter::*; 