//! **TRACING MODULE - REFACTORED FOR FILE SIZE COMPLIANCE**
//!
//! Comprehensive tracing and logging system for NestGate including structured
//! logging, log aggregation, distributed tracing, and integration with external
//! logging systems like ELK stack, Loki, and Jaeger.
//!
//! **REFACTORING COMPLETE**: Split from 891-line monolith into focused modules:
//! - `config`: Configuration types and structures
//! - `setup`: Core tracing initialization and setup
//! - `collectors`: Log aggregation and collection
//! - `retention`: Log retention and cleanup management
//! - `macros`: Logging macros and utilities

// Module declarations
pub mod config;
pub mod setup;
pub mod collectors;
pub mod retention;
pub mod macros;

// Re-export all public types for backward compatibility
pub use config::*;
pub use setup::*;
pub use collectors::*;
pub use retention::*;
pub use macros::*;

// Re-export the main initialization function
pub use setup::init_tracing; 