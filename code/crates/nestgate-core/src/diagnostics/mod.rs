/// Diagnostics module for NestGate
/// This module provides system diagnostics and monitoring functionality
/// for the NestGate system. It has been refactored into focused sub-modules:
/// - `types`: Core diagnostic types and enums
/// - `metrics`: System metrics collection and structures
/// - `diagnostic`: Individual diagnostic entries
/// - `manager`: Main diagnostics management logic
pub mod diagnostic;
pub mod manager;
pub mod metrics;
pub mod types;

pub use diagnostic::*;
pub use manager::*;
#[allow(ambiguous_glob_reexports)]
pub use metrics::*;
#[allow(ambiguous_glob_reexports)]
pub use types::*;
