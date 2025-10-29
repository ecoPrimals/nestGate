//! **COMPREHENSIVE WORKSPACE SECRETS TESTS**
//!
//! Test coverage for `workspace_management/secrets.rs` - Secrets management functionality.
//!
//! This test suite covers:
//! - Secret creation with security adapter
//! - Fallback behavior when security adapter fails
//! - Response structure validation
//! - Integration with `AuthTokenManager`
//! - Error handling scenarios

#[cfg(test)]
mod tests {
    use super::super::secrets::*;
    use axum::extract::Path;

    // ==================== CREATE WORKSPACE SECRET TESTS ====================

    #[tokio::test]
    async fn test_create_workspace_secret_success() {
        let workspace_id = "workspace-123".to_string();
        let result = create_workspace_secret(Path(workspace_id.clone())).await;

        assert!(result.is_ok(), "Should return Ok result");

        let response = result.unwrap();
        let json = response.0;

        // Should have status field
        assert!(json.get("status").is_some());

        // Should include workspace_id in response
        assert_eq!(
            json.get("workspace_id").and_then(|v| v.as_str()),
            Some(workspace_id.as_str())
        );
    }

    #[tokio::test]
    async fn test_create_workspace_secret_with_various_ids() {
        let workspace_ids = vec!["ws-001", "workspace-prod-123", "test_workspace", "dev-env"];

        for workspace_id in workspace_ids {
            let result = create_workspace_secret(Path(workspace_id.to_string())).await;
            assert!(
                result.is_ok(),
                "Should succeed for workspace_id: {workspace_id}"
            );

            let response = result.unwrap();
            let json = response.0;

            // Verify workspace_id is in response
            assert_eq!(
                json.get("workspace_id").and_then(|v| v.as_str()),
                Some(workspace_id)
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_response_structure() {
        let workspace_id = "workspace-test".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        // Verify required fields
        assert!(json.get("status").is_some(), "Should have status field");
        assert!(json.get("message").is_some(), "Should have message field");
        assert!(
            json.get("workspace_id").is_some(),
            "Should have workspace_id field"
        );
    }

    #[tokio::test]
    async fn test_create_workspace_secret_success_status() {
        let workspace_id = "workspace-success".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        let status = json.get("status").and_then(|v| v.as_str());
        // Status should be either "success" or "fallback" depending on security adapter
        assert!(
            status == Some("success") || status == Some("fallback"),
            "Status should be success or fallback, got: {status:?}"
        );
    }

    #[tokio::test]
    async fn test_create_workspace_secret_includes_timestamp() {
        let workspace_id = "workspace-timestamp".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        // If status is success, should have created_at timestamp
        if json.get("status").and_then(|v| v.as_str()) == Some("success") {
            assert!(
                json.get("created_at").is_some(),
                "Success response should include timestamp"
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_includes_secret_id() {
        let workspace_id = "workspace-secret-id".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        // If status is success, should have secret_id
        if json.get("status").and_then(|v| v.as_str()) == Some("success") {
            assert!(
                json.get("secret_id").is_some(),
                "Success response should include secret_id"
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_fallback_response() {
        let workspace_id = "workspace-fallback".to_string();
        let result = create_workspace_secret(Path(workspace_id.clone())).await;

        assert!(result.is_ok(), "Fallback should still return Ok");

        let response = result.unwrap();
        let json = response.0;

        // Fallback response should have appropriate fields
        assert!(json.get("workspace_id").is_some());
        assert!(json.get("message").is_some());

        // If fallback, should include note about delegation
        if json.get("status").and_then(|v| v.as_str()) == Some("fallback") {
            assert!(
                json.get("note").is_some(),
                "Fallback response should include note"
            );
            assert!(
                json.get("recommendation").is_some(),
                "Fallback response should include recommendation"
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_message_not_empty() {
        let workspace_id = "workspace-message".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        let message = json.get("message").and_then(|v| v.as_str());
        assert!(message.is_some(), "Should have message field");
        assert!(!message.unwrap().is_empty(), "Message should not be empty");
    }

    #[tokio::test]
    async fn test_create_workspace_secret_with_empty_id() {
        let workspace_id = String::new();
        let result = create_workspace_secret(Path(workspace_id)).await;

        // Should handle empty workspace_id gracefully
        assert!(result.is_ok(), "Should handle empty workspace_id");
    }

    #[tokio::test]
    async fn test_create_workspace_secret_with_special_characters() {
        let workspace_ids = vec![
            "workspace-with-dashes",
            "workspace_with_underscores",
            "workspace.with.dots",
            "workspace123",
        ];

        for workspace_id in workspace_ids {
            let result = create_workspace_secret(Path(workspace_id.to_string())).await;
            assert!(
                result.is_ok(),
                "Should handle special characters in workspace_id: {workspace_id}"
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_with_long_id() {
        let workspace_id = "workspace-".to_string() + &"a".repeat(200);
        let result = create_workspace_secret(Path(workspace_id.clone())).await;

        assert!(result.is_ok(), "Should handle long workspace_id");

        let response = result.unwrap();
        let json = response.0;

        // Should preserve the full workspace_id
        assert_eq!(
            json.get("workspace_id").and_then(|v| v.as_str()),
            Some(workspace_id.as_str())
        );
    }

    #[tokio::test]
    async fn test_create_workspace_secret_idempotent() {
        let workspace_id = "workspace-idempotent".to_string();

        // Create secret twice
        let result1 = create_workspace_secret(Path(workspace_id.clone())).await;
        let result2 = create_workspace_secret(Path(workspace_id.clone())).await;

        // Both should succeed
        assert!(result1.is_ok(), "First creation should succeed");
        assert!(result2.is_ok(), "Second creation should succeed");
    }

    #[tokio::test]
    async fn test_create_workspace_secret_concurrent() {
        let workspace_id = "workspace-concurrent".to_string();

        // Spawn multiple concurrent requests
        let handles = vec![
            tokio::spawn(create_workspace_secret(Path(workspace_id.clone()))),
            tokio::spawn(create_workspace_secret(Path(workspace_id.clone()))),
            tokio::spawn(create_workspace_secret(Path(workspace_id.clone()))),
        ];

        // All should complete successfully
        for handle in handles {
            let result = handle.await.expect("Task should complete");
            assert!(result.is_ok(), "Concurrent request should succeed");
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_response_serialization() {
        let workspace_id = "workspace-serialize".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        // Response should be serializable
        let serialized = serde_json::to_string(&json);
        assert!(serialized.is_ok(), "Response should serialize to JSON");

        let json_string = serialized.unwrap();
        assert!(
            !json_string.is_empty(),
            "Serialized JSON should not be empty"
        );
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_workspace_secrets_integration_flow() {
        // Simulate a complete workflow
        let workspace_id = "workspace-integration".to_string();

        // Step 1: Create secret
        let create_result = create_workspace_secret(Path(workspace_id.clone())).await;
        assert!(create_result.is_ok(), "Secret creation should succeed");

        let response = create_result.unwrap();
        let json = response.0;

        // Step 2: Verify response structure
        assert!(json.get("status").is_some());
        assert!(json.get("workspace_id").is_some());
        assert!(json.get("message").is_some());

        // Step 3: Verify workspace_id matches
        assert_eq!(
            json.get("workspace_id").and_then(|v| v.as_str()),
            Some(workspace_id.as_str())
        );
    }

    #[tokio::test]
    async fn test_workspace_secrets_with_multiple_workspaces() {
        let workspace_ids = vec![
            "workspace-001",
            "workspace-002",
            "workspace-003",
            "workspace-004",
            "workspace-005",
        ];

        for workspace_id in workspace_ids {
            let result = create_workspace_secret(Path(workspace_id.to_string())).await;
            assert!(
                result.is_ok(),
                "Should create secret for workspace: {workspace_id}"
            );
        }
    }

    // ==================== EDGE CASES ====================

    #[tokio::test]
    async fn test_create_workspace_secret_with_uuid() {
        let workspace_id = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let result = create_workspace_secret(Path(workspace_id.clone())).await;

        assert!(result.is_ok(), "Should handle UUID workspace_id");

        let response = result.unwrap();
        let json = response.0;

        assert_eq!(
            json.get("workspace_id").and_then(|v| v.as_str()),
            Some(workspace_id.as_str())
        );
    }

    #[tokio::test]
    async fn test_create_workspace_secret_numeric_id() {
        let workspace_id = "123456789".to_string();
        let result = create_workspace_secret(Path(workspace_id.clone())).await;

        assert!(result.is_ok(), "Should handle numeric workspace_id");
    }

    #[tokio::test]
    async fn test_create_workspace_secret_response_completeness() {
        let workspace_id = "workspace-complete".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        // Verify all expected fields are present
        let expected_fields = vec!["status", "message", "workspace_id"];

        for field in expected_fields {
            assert!(
                json.get(field).is_some(),
                "Response should contain field: {field}"
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_delegation_note() {
        let workspace_id = "workspace-delegation".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        assert!(result.is_ok());

        let response = result.unwrap();
        let json = response.0;

        // If fallback, verify delegation information
        if json.get("status").and_then(|v| v.as_str()) == Some("fallback") {
            let note = json.get("note").and_then(|v| v.as_str());
            assert!(note.is_some(), "Fallback should include note");

            let note_text = note.unwrap();
            assert!(
                note_text.contains("NestGate") || note_text.contains("storage"),
                "Note should mention NestGate's focus on storage"
            );
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_performance() {
        let workspace_id = "workspace-perf".to_string();

        let start = std::time::Instant::now();
        let result = create_workspace_secret(Path(workspace_id)).await;
        let duration = start.elapsed();

        assert!(result.is_ok(), "Secret creation should succeed");
        assert!(
            duration.as_millis() < 1000,
            "Secret creation should complete in under 1 second, took: {duration:?}"
        );
    }
}
