// **CANONICAL CONSTANTS - MODULAR ARCHITECTURE**
//! Module definitions and exports.
// Domain-specific constants modules for maintainable architecture.
// Replaces the single 938-line constants_consolidation.rs with focused modules.

// Core constants management
pub mod manager;
pub mod types;

// Domain-specific constants
pub mod monitoring;
pub mod network;
pub mod performance;
pub mod security;
pub mod storage;

// Re-export main types
pub use manager::ConstantsConsolidationManager;
pub use types::*;

// Re-export domain constants
pub use monitoring::*;
pub use network::*;
pub use performance::*;
pub use security::*;
pub use storage::*;
