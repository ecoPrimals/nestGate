pub mod ecosystem;
pub mod optimization;
/// **AUTOMATION TYPES MODULE**
/// Provides type definitions for the automation system
pub mod prediction;

// Re-export commonly used types
pub use config::*;
pub use ecosystem::*;
pub use optimization::*;
pub use prediction::*;
