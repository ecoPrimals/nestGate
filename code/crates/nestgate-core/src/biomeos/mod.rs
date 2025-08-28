//! **BIOMEOS MODULE - REFACTORED FOR FILE SIZE COMPLIANCE**
//!
//! BiomeOS Universal Adapter Integration providing universal adapter routing
//! for BiomeOS capabilities, replacing direct hardcoded integration with
//! capability-based discovery.
//!
//! **REFACTORING COMPLETE**: Split from 886-line monolith into focused modules:
//! - `types`: Core BiomeOS types and data structures
//! - `discovery`: Capability discovery and routing functionality  
//! - `adapters`: Universal adapter integration and routing
//! - `protocols`: Protocol definitions and parsing utilities

// Module declarations
pub mod types;
pub mod discovery;
pub mod adapters;
pub mod protocols;

// Re-export all public types for backward compatibility
pub use types::*;
pub use discovery::*;
pub use adapters::*;
pub use protocols::*; 