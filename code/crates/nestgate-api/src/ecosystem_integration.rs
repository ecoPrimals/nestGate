//! **UNIVERSAL ECOSYSTEM INTEGRATION - MODULARIZED**
//!
//! This file has been refactored from a large monolithic implementation (873 lines)
//! into a clean modular structure for better maintainability and compliance with
//! the <2000 lines per file standard.
//!
//! **MIGRATION**: All functionality has been moved to focused modules:
//! - `ecosystem_integration/types` - Core types and data structures
//! - `ecosystem_integration/registration` - Service registration logic
//! - `ecosystem_integration/discovery` - Service discovery implementation
//! - `ecosystem_integration/capabilities` - Capability management
//! - `ecosystem_integration/resources` - Resource management
//! - `ecosystem_integration/endpoints` - Endpoint management
//! - `ecosystem_integration/integration` - Integration preferences
//! - `ecosystem_integration/adapter` - Universal adapter

// Re-export the modular ecosystem integration
pub use self::ecosystem_integration::*;

/// Modular ecosystem integration implementation
pub mod ecosystem_integration;
