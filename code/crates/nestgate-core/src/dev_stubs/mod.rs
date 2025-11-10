//! **DEVELOPMENT STUBS MODULE**
//!
//! This module provides stub implementations for development and testing environments
//! where full functionality may not be available (e.g., non-Linux platforms, testing without real services).
//!
//! ## Purpose
//!
//! Development stubs enable:
//! - **Cross-platform development** - Work on macOS/Windows without Linux-specific features
//! - **Testing** - Mock implementations for unit and integration tests
//! - **CI/CD** - Run tests in environments without real services
//! - **Local development** - Quick iterations without full infrastructure
//!
//! ## Feature Gate
//!
//! Most stubs are gated behind the `dev-stubs` feature flag:
//!
//! ```toml
//! [dependencies]
//! nestgate-core = { version = "0.1", features = ["dev-stubs"] }
//! ```
//!
//! ## Modules
//!
//! - `primal_discovery` - Universal Primal Discovery fallback stubs
//!
//! ## When to Use Stubs
//!
//! **Development** ✅
//! - Local development on non-Linux platforms
//! - Quick prototyping without infrastructure
//! - Testing without external dependencies
//!
//! **Production** ❌
//! - Never enable `dev-stubs` in production
//! - Always use real implementations
//! - Stubs provide sensible defaults but limited functionality
//!
//! ## Migration from Old Locations
//!
//! **November 10, 2025**: Stubs organized into this module from scattered locations:
//! - `universal_primal_discovery/stubs.rs` → `dev_stubs/primal_discovery.rs`
//!
//! Old imports are deprecated and will be removed in v0.12.0 (May 2026).

#[cfg(feature = "dev-stubs")]
pub mod primal_discovery;

// Re-export for convenience when dev-stubs feature is enabled
#[cfg(feature = "dev-stubs")]
pub use primal_discovery::*;
