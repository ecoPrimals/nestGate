//! **DASHBOARDS MODULE - REFACTORED FOR FILE SIZE COMPLIANCE**
//!
//! Comprehensive dashboard system for generating monitoring dashboards for
//! Grafana, Prometheus, and custom web interfaces.
//!
//! **REFACTORING COMPLETE**: Split from 882-line monolith into focused modules:
//! - `types`: Dashboard configuration types and enums
//! - `manager`: Dashboard manager and template creation
//! - `html`: HTML dashboard generation for development

// Module declarations
pub mod types;
pub mod manager;
pub mod html;

// Re-export all public types for backward compatibility
pub use types::*;
pub use manager::*;
pub use html::*;
