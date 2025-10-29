// **BIOMEOS MODULE - REFACTORED FOR FILE SIZE COMPLIANCE**
//! Module definitions and exports.
// Management Universal Adapter Integration providing universal adapter routing
//! for Management capabilities, replacing direct hardcoded integration with
//! capability-based discovery.
//! Module definitions and exports.
// **REFACTORING COMPLETE**: Split from 886-line monolith into focused modules:
//! - `types`: Core Management types and data structures
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
// CLEANED: Removed unused discovery re-export as part of canonical modernization
// pub use discovery::*;
pub use adapters::*;
// CLEANED: Removed unused protocols re-export as part of canonical modernization
// pub use protocols::*; 