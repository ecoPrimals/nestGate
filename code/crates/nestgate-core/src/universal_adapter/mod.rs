pub mod adapter;
pub mod config;
pub mod discovery;
pub mod stats;

// Re-export main types - updated to match new structure
pub use adapter::UniversalAdapter; // Changed from UniversalAdapter
pub use config::*;
pub use discovery::*;

// Update function exports to match new implementation
pub use adapter::*;

// Remove missing imports - these functions don't exist in the new implementation
// pub use adapter::{increment_discovery_attempts, increment_successful_discoveries};
