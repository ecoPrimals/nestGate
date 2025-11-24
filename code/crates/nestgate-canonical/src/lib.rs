//! **NESTGATE CANONICAL MODULE**
//!
//! This crate provides the canonical, standardized interfaces and types for the entire
//! `NestGate` ecosystem. It serves as the single source of truth for all cross-crate
//! communication and ensures consistent patterns throughout the system.

// ==================== SECTION: CANONICAL EXPORTS ====================

pub mod config;
pub mod error;
pub mod traits;
// Note: types is now a directory with sub-modules (mod.rs)
// All types are re-exported from types/mod.rs for backward compatibility
pub mod types;

// Re-export main types
pub use config::NestGateConfig;
pub use error::{NestGateError, Result};
pub use types::CanonicalConfig;

// Local result type
pub type NestGateResult<T> = Result<T>;

// ==================== SECTION ====================

/// Modernization complete
/// All `NestGate` components now use these canonical interfaces
pub const CANONICAL_VERSION: &str = "2.0.0";
