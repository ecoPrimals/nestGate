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
pub use config_builders::{build_access_grant, build_diagnostic, build_error_context};
pub use json_builders::{
    build_api_error, build_api_success, build_api_success_with_metadata, build_json_response,
};
pub use mock_builders::{
    build_mock_performance_metrics, build_mock_resource_allocation, build_mock_workload_result,
};
pub use response_builders::{
    build_error_response, build_service_error, build_service_response_with_headers,
    build_service_success, build_standardized_response, build_success_response,
};
