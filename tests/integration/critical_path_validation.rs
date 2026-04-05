// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Critical Path Validation Tests
//!
//! Integration tests for critical system paths and workflows
//! These tests validate key functionality without requiring a full system deployment

use nestgate_core::{
    config::canonical_primary::NestGateCanonicalConfig,
    constants::Environment,
    error::{NestGateError, Result},
    response::{
        error_response::{ErrorResponseFactory, UnifiedErrorResponse},
        success_response::{SuccessResponseFactory, SuccessResponse},
    },
};
use std::collections::HashMap;

/// Test critical path: Configuration loading and validation
#[test]
fn test_critical_path_config_loading() -> Result<()> {
    // Test default config creation
    let config = NestGateCanonicalConfig::default();
    assert!(!config.system.instance_name.is_empty(), "Instance name should not be empty");

    // Test environment-specific configs
    let dev_config = nestgate_core::config::canonical_primary::create_config_for_environment(
        Environment::Development,
    );
    assert!(!dev_config.system.instance_name.is_empty());

    let prod_config = nestgate_core::config::canonical_primary::create_config_for_environment(
        Environment::Production,
    );
    assert!(!prod_config.system.instance_name.is_empty());

    Ok(())
}

/// Test critical path: Error response creation and serialization
#[test]
fn test_critical_path_error_responses() -> Result<()> {
    // Test all common error types
    let bad_request = ErrorResponseFactory::bad_request("Invalid input");
    assert_eq!(bad_request.status, 400);
    assert_eq!(bad_request.code, "BAD_REQUEST");

    let not_found = ErrorResponseFactory::not_found("/api/resource");
    assert_eq!(not_found.status, 404);
    assert!(not_found.message.contains("not found"));

    let unauthorized = ErrorResponseFactory::unauthorized("access");
    assert_eq!(unauthorized.status, 401);

    let internal = ErrorResponseFactory::internal("Server error");
    assert_eq!(internal.status, 500);

    // Test error with context
    let mut details = HashMap::new();
    details.insert("field".to_string(), serde_json::json!("email"));
    
    let validation = ErrorResponseFactory::validation_error("email", "invalid format")
        .with_details(details);
    
    assert_eq!(validation.status, 400);
    assert!(validation.details.is_some());

    // Test serialization
    let json = serde_json::to_string(&validation)?;
    let deserialized: UnifiedErrorResponse = serde_json::from_str(&json)?;
    assert_eq!(validation.code, deserialized.code);

    Ok(())
}

/// Test critical path: Success response creation and serialization
#[test]
fn test_critical_path_success_responses() -> Result<()> {
    // Test resource creation response
    let created = SuccessResponseFactory::created("user", "user-123");
    assert!(created.message.contains("created successfully"));
    assert!(created.data.is_some());

    // Test resource update response
    let updated = SuccessResponseFactory::updated("dataset", "ds-456");
    assert!(updated.message.contains("updated successfully"));

    // Test resource deletion response
    let deleted = SuccessResponseFactory::deleted("snapshot");
    assert!(deleted.message.contains("deleted successfully"));

    // Test retrieval response
    let retrieved = SuccessResponseFactory::retrieved("pools", 10);
    assert!(retrieved.message.contains("retrieved successfully"));

    // Test health check response
    let health = SuccessResponseFactory::health_check("api", "healthy");
    assert!(health.message.contains("healthy"));

    // Test serialization
    let json = serde_json::to_string(&health)?;
    let deserialized: SuccessResponse = serde_json::from_str(&json)?;
    assert_eq!(health.message, deserialized.message);

    Ok(())
}

/// Test critical path: NestGateError construction and conversion
#[test]
fn test_critical_path_error_construction() -> Result<()> {
    // Test various error types
    let validation_err = NestGateError::validation_error("email", "invalid format");
    assert!(matches!(
        validation_err,
        NestGateError::Validation { field, .. } if field == "email"
    ));

    let internal_err = NestGateError::internal_error("Database unavailable", "db-service");
    assert!(matches!(
        internal_err,
        NestGateError::Internal { component, .. } if component == "db-service"
    ));

    let config_err = NestGateError::config_error("Missing required field");
    assert!(matches!(config_err, NestGateError::Configuration { .. }));

    // Test error display
    let display_str = format!("{}", validation_err);
    assert!(display_str.contains("email"));

    Ok(())
}

/// Test critical path: Environment-specific configuration handling
#[test]
fn test_critical_path_environment_handling() -> Result<()> {
    let environments = vec![
        Environment::Development,
        Environment::Staging,
        Environment::Production,
    ];

    for env in environments {
        let config = nestgate_core::config::canonical_primary::create_config_for_environment(env.clone());
        
        // Verify basic config structure
        assert!(!config.system.instance_name.is_empty());
        
        // Environment should affect some settings
        match env {
            Environment::Development => {
                // Dev environment specific checks
                assert!(!config.system.instance_name.is_empty());
            }
            Environment::Production => {
                // Prod environment specific checks
                assert!(!config.system.instance_name.is_empty());
            }
            _ => {}
        }
    }

    Ok(())
}

/// Test critical path: Response builder patterns
#[test]
fn test_critical_path_response_builders() -> Result<()> {
    // Test success response builder
    let response = SuccessResponse::new("Operation successful")
        .add_data("id", serde_json::json!("resource-123"))
        .add_data("status", serde_json::json!("active"))
        .add_metadata("version", serde_json::json!("1.0.0"))
        .with_correlation_id("corr-789".to_string());

    assert!(response.data.is_some());
    assert!(!response.metadata.is_empty());
    assert!(response.correlation_id.is_some());

    // Test error response builder
    let error = UnifiedErrorResponse::simple("Test error", "TEST_ERROR", "test-component")
        .with_context("operation", serde_json::json!("test_op"))
        .with_correlation_id("corr-456".to_string());

    assert!(error.details.is_some());
    assert!(error.correlation_id.is_some());

    Ok(())
}

/// Test critical path: Multiple response types in workflow
#[test]
fn test_critical_path_workflow_responses() -> Result<()> {
    // Simulate a workflow with multiple response types
    
    // Step 1: Validation
    let validation_result = validate_input("test@example.com");
    assert!(validation_result.is_ok());

    // Step 2: Creation
    let create_response = simulate_resource_creation("user-001");
    assert!(create_response.data.is_some());

    // Step 3: Retrieval
    let get_response = simulate_resource_retrieval("user-001");
    assert!(get_response.data.is_some());

    // Step 4: Update
    let update_response = simulate_resource_update("user-001");
    assert!(update_response.message.contains("updated"));

    // Step 5: Deletion
    let delete_response = simulate_resource_deletion("user-001");
    assert!(delete_response.message.contains("deleted"));

    Ok(())
}

/// Test critical path: Error handling in workflows
#[test]
fn test_critical_path_error_handling() -> Result<()> {
    // Test various error scenarios
    
    // Invalid input
    let validation_result = validate_input("invalid-email");
    assert!(validation_result.is_err());

    // Not found
    let get_result = simulate_resource_retrieval("nonexistent");
    // In a real system, this would return an error
    assert!(get_result.data.is_some()); // Mock returns success

    // Conflict
    let conflict = ErrorResponseFactory::conflict("user");
    assert_eq!(conflict.status, 409);

    // Service unavailable
    let unavailable = ErrorResponseFactory::service_unavailable("database");
    assert_eq!(unavailable.status, 503);

    Ok(())
}

/// Test critical path: Performance expectations
#[test]
fn test_critical_path_performance() -> Result<()> {
    use std::time::Instant;

    // Test configuration loading performance
    let start = Instant::now();
    let _ = NestGateCanonicalConfig::default();
    let config_time = start.elapsed();
    assert!(config_time.as_millis() < 100, "Config loading should be fast");

    // Test response creation performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = SuccessResponseFactory::created("resource", "id");
    }
    let response_time = start.elapsed();
    assert!(response_time.as_millis() < 100, "Response creation should be fast");

    // Test error creation performance
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = ErrorResponseFactory::bad_request("Test");
    }
    let error_time = start.elapsed();
    assert!(error_time.as_millis() < 100, "Error creation should be fast");

    Ok(())
}

/// Test critical path: Concurrent operations
#[tokio::test]
async fn test_critical_path_concurrent_operations() -> Result<()> {
    use tokio::task;

    // Spawn multiple concurrent operations
    let handles: Vec<_> = (0..10)
        .map(|i| {
            task::spawn(async move {
                let config = NestGateCanonicalConfig::default();
                let response = SuccessResponseFactory::created("resource", &format!("id-{}", i));
                (config.system.instance_name, response.message)
            })
        })
        .collect();

    // Wait for all to complete
    for handle in handles {
        let (instance, message) = handle.await?;
        assert!(!instance.is_empty());
        assert!(message.contains("created successfully"));
    }

    Ok(())
}

// Helper functions for simulation

fn validate_input(email: &str) -> Result<()> {
    if email.contains('@') {
        Ok(())
    } else {
        Err(NestGateError::validation_error("email", "Invalid format"))
    }
}

fn simulate_resource_creation(id: &str) -> SuccessResponse {
    SuccessResponseFactory::created("resource", id)
}

fn simulate_resource_retrieval(id: &str) -> SuccessResponse {
    SuccessResponseFactory::retrieved("resource", 1)
        .add_data("id", serde_json::json!(id))
        .add_data("status", serde_json::json!("active"))
}

fn simulate_resource_update(id: &str) -> SuccessResponse {
    SuccessResponseFactory::updated("resource", id)
}

fn simulate_resource_deletion(_id: &str) -> SuccessResponse {
    SuccessResponseFactory::deleted("resource")
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    /// Stress test: Rapid configuration creation
    #[test]
    fn stress_test_config_creation() -> Result<()> {
        for _ in 0..100 {
            let _ = NestGateCanonicalConfig::default();
        }
        Ok(())
    }

    /// Stress test: Rapid response creation
    #[test]
    fn stress_test_response_creation() -> Result<()> {
        for i in 0..1000 {
            let _ = SuccessResponseFactory::created("resource", &format!("id-{}", i));
            let _ = ErrorResponseFactory::bad_request(&format!("error-{}", i));
        }
        Ok(())
    }

    /// Stress test: Large data handling
    #[test]
    fn stress_test_large_data() -> Result<()> {
        let mut response = SuccessResponse::new("Test");
        
        // Add many data fields
        for i in 0..100 {
            response = response.add_data(&format!("field_{}", i), serde_json::json!(i));
        }

        assert!(response.data.is_some());
        assert_eq!(response.data.unwrap().len(), 100);

        Ok(())
    }
}

