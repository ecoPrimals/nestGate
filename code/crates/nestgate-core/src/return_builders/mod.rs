/// **RETURN BUILDERS MODULE SYSTEM**
/// Breaking down the large return_builders.rs (898 lines) into focused modules
/// to achieve 2000-line file size compliance while maintaining functionality.
///
/// Pure builder functions for complex return value constructions.
/// Extracted from inline struct creation to enable precise testing
/// and catch field assignment mutations.
// Response builder functions
pub mod response_builders;
// Mock data builders for testing
pub mod mock_builders;
// Configuration and utility builders
pub mod config_builders;
// Specialized JSON and API builders
pub mod json_builders;
// Test modules
pub mod tests;

// Re-export all public builder functions for backward compatibility
pub use response_builders::{
    build_error_response, build_success_response,
};
