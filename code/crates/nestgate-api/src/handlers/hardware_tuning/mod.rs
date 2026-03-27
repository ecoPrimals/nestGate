//! **HARDWARE TUNING MODULE**
//!
//! Hardware tuning functionality split into logical modules for better maintainability.
//!
//! **⚠️ DEVELOPMENT STUBS ⚠️**
//!
//! Handlers are only available with `dev-stubs` feature.
//! Production builds use placeholders that return "not implemented".

// Development: Real stub handlers
#[cfg(feature = "dev-stubs")]
pub mod handlers;
/// Axum JSON handlers backed by the same `/proc` logic as [`handlers::RealHardwareTuningHandler`].
#[cfg(feature = "dev-stubs")]
pub mod handlers_production;
/// `/proc`-based resource helpers (shared with production route helpers).
#[cfg(feature = "dev-stubs")]
pub mod linux_proc;

// Production: Placeholder handlers
#[cfg(not(feature = "dev-stubs"))]
pub mod production_placeholders;
#[cfg(not(feature = "dev-stubs"))]
pub use production_placeholders as handlers;

pub mod types;

#[cfg(test)]
mod strategic_coverage_tests_dec11;

// Re-export the main types and functions
pub use handlers::*;
#[cfg(feature = "dev-stubs")]
pub use handlers_production::*;
pub use types::*;
