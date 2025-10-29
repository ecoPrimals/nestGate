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

// Production: Placeholder handlers
#[cfg(not(feature = "dev-stubs"))]
pub mod production_placeholders;
#[cfg(not(feature = "dev-stubs"))]
pub use production_placeholders as handlers;

pub mod types;

// Re-export the main types and functions
pub use handlers::*;
pub use types::*;

#[cfg(test)]
mod types_tests;
