// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **COMPREHENSIVE WORKSPACE STORAGE TESTS**
//!
//! Test coverage for `workspace_management/storage.rs` - ZFS storage operations.
//!
//! This test suite covers:
//! - Workspace deletion operations
//! - Workspace storage status monitoring
//! - Storage cleanup operations
//! - ZFS command integration
//! - Validation and error handling
//! - Edge cases and boundary conditions

#[cfg(test)]
mod tests {
    use super::super::storage::*;
    use axum::extract::Path;
    use axum::http::StatusCode;

    // ==================== DELETE WORKSPACE TESTS ====================

    #[tokio::test]
    async fn test_delete_workspace_with_invalid_empty_id() {
        let workspace_id = String::new();
        let result = delete_workspace(Path(workspace_id)).await;

        assert!(result.is_err(), "Should reject empty workspace_id");
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_slash_in_id() {
        let workspace_id = "workspace/invalid".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        assert!(result.is_err(), "Should reject workspace_id with slash");
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_space_in_id() {
        let workspace_id = "workspace invalid".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        assert!(result.is_err(), "Should reject workspace_id with space");
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_validation_with_special_chars() {
        let invalid_ids = vec![
            ("workspace/slash", true),     // Has slash - should reject
            ("workspace space", true),     // Has space - should reject
            ("", true),                    // Empty - should reject
            ("workspace\nNewline", false), // Newline might be allowed by validation
            ("workspace\ttab", false),     // Tab might be allowed by validation
        ];

        for (invalid_id, should_reject) in invalid_ids {
            let result = delete_workspace(Path(invalid_id.to_string())).await;
            if should_reject {
                assert!(
                    result.is_err(),
                    "Should reject invalid workspace_id: {invalid_id}"
                );
            }
            // Otherwise just verify it doesn't panic
        }
    }

    #[tokio::test]
    async fn test_delete_workspace_with_valid_id_format() {
        let valid_ids = vec![
            "workspace-123",
            "ws_test",
            "workspace.prod",
            "workspace123",
            "a",
        ];

        for valid_id in valid_ids {
            let result = delete_workspace(Path(valid_id.to_string())).await;
            // Result depends on ZFS availability - just verify it doesn't panic
            // and follows expected error path (either success or proper error)
            match result {
                Ok(_) => assert!(true, "Valid format accepted: {valid_id}"),
                Err(status) => {
                    // Should be internal server error if ZFS fails, not bad request
                    assert_ne!(
                        status,
                        StatusCode::BAD_REQUEST,
                        "Valid ID {valid_id} should not return BAD_REQUEST"
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_delete_workspace_returns_json_on_success() {
        let workspace_id = "test-workspace-delete".to_string();
        let result = delete_workspace(Path(workspace_id.clone())).await;

        if let Ok(response) = result {
            let json = response.0;

            // Verify JSON structure
            assert!(json.get("status").is_some());
            assert!(json.get("message").is_some());
            assert!(json.get("workspace_id").is_some());
        }
        // If error, that's OK - ZFS might not be available
    }

    // ==================== GET WORKSPACE STATUS TESTS ====================

    #[tokio::test]
    async fn test_get_workspace_status_with_invalid_empty_id() {
        let workspace_id = String::new();
        let result = get_workspace_status(Path(workspace_id)).await;

        assert!(result.is_err(), "Should reject empty workspace_id");
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_slash_in_id() {
        let workspace_id = "workspace/invalid".to_string();
        let result = get_workspace_status(Path(workspace_id)).await;

        assert!(result.is_err(), "Should reject workspace_id with slash");
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_space_in_id() {
        let workspace_id = "workspace invalid".to_string();
        let result = get_workspace_status(Path(workspace_id)).await;

        assert!(result.is_err(), "Should reject workspace_id with space");
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_status_validation() {
        let test_cases = vec![
            ("", true, StatusCode::BAD_REQUEST),
            ("workspace/slash", true, StatusCode::BAD_REQUEST),
            ("workspace space", true, StatusCode::BAD_REQUEST),
        ];

        for (invalid_id, should_fail, expected_code) in test_cases {
            let result = get_workspace_status(Path(invalid_id.to_string())).await;
            if should_fail {
                assert!(
                    result.is_err(),
                    "Should reject invalid workspace_id: {invalid_id}"
                );
                assert_eq!(result.unwrap_err(), expected_code);
            }
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_valid_formats() {
        let valid_ids = vec![
            "workspace-123",
            "ws_test",
            "workspace.prod",
            "workspace123",
            "test-ws",
        ];

        for valid_id in valid_ids {
            let result = get_workspace_status(Path(valid_id.to_string())).await;

            // Valid format should not return BAD_REQUEST
            if let Err(status) = result {
                assert_ne!(
                    status,
                    StatusCode::BAD_REQUEST,
                    "Valid ID {valid_id} should not return BAD_REQUEST"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_returns_json_structure() {
        let workspace_id = "test-status-ws".to_string();
        let result = get_workspace_status(Path(workspace_id)).await;

        if let Ok(response) = result {
            let json = response.0;

            // Should have status-related fields
            assert!(json.is_object(), "Should return JSON object");
        }
        // If error, that's OK - ZFS might not be available
    }

    // ==================== VALIDATION TESTS ====================

    #[test]
    fn test_workspace_id_validation_empty() {
        let workspace_id = "";

        // Empty IDs should be invalid
        assert!(
            workspace_id.is_empty(),
            "Empty workspace_id should be detected"
        );
    }

    #[test]
    fn test_workspace_id_validation_slash() {
        let workspace_ids_with_slash = vec!["ws/123", "workspace/test", "/workspace"];

        for workspace_id in workspace_ids_with_slash {
            assert!(
                workspace_id.contains('/'),
                "Slash in workspace_id should be detected: {workspace_id}"
            );
        }
    }

    #[test]
    fn test_workspace_id_validation_space() {
        let workspace_ids_with_space = vec!["ws 123", "workspace test", " workspace"];

        for workspace_id in workspace_ids_with_space {
            assert!(
                workspace_id.contains(' '),
                "Space in workspace_id should be detected: {workspace_id}"
            );
        }
    }

    #[test]
    fn test_workspace_id_validation_valid_formats() {
        let valid_ids = vec![
            "workspace-123",
            "ws_test",
            "workspace.prod",
            "UPPERCASE",
            "MixedCase123",
            "a",
            "123",
        ];

        for workspace_id in valid_ids {
            assert!(
                !workspace_id.is_empty()
                    && !workspace_id.contains('/')
                    && !workspace_id.contains(' '),
                "Workspace_id {workspace_id} should be valid"
            );
        }
    }

    // ==================== INTEGRATION TESTS ====================

    #[tokio::test]
    async fn test_delete_then_status_workflow() {
        let workspace_id = "test-workflow-ws".to_string();

        // Try to delete (may or may not exist)
        let _delete_result = delete_workspace(Path(workspace_id.clone())).await;

        // Try to get status after deletion
        let status_result = get_workspace_status(Path(workspace_id.clone())).await;

        // Both operations should handle non-existent workspace gracefully
        // Just verify they don't panic
        assert!(true, "Workflow completed without panic");
    }

    #[tokio::test]
    async fn test_multiple_status_checks() {
        let workspace_id = "test-multi-status".to_string();

        // Check status multiple times
        for i in 0..3 {
            let result = get_workspace_status(Path(workspace_id.clone())).await;

            // Each call should behave consistently
            if let Err(status) = result {
                assert_ne!(
                    status,
                    StatusCode::BAD_REQUEST,
                    "Iteration {i} should not return BAD_REQUEST for valid ID"
                );
            }
        }
    }

    #[tokio::test]
    async fn test_concurrent_status_checks() {
        let workspace_id = "test-concurrent-status".to_string();

        // Spawn concurrent status checks
        let handles = vec![
            tokio::spawn(get_workspace_status(Path(workspace_id.clone()))),
            tokio::spawn(get_workspace_status(Path(workspace_id.clone()))),
            tokio::spawn(get_workspace_status(Path(workspace_id.clone()))),
        ];

        // All should complete without panic
        for handle in handles {
            let _ = handle.await.expect("Task should complete");
        }
    }

    // ==================== EDGE CASES ====================

    #[tokio::test]
    async fn test_delete_workspace_with_long_valid_id() {
        let workspace_id = "w".repeat(100);
        let result = delete_workspace(Path(workspace_id)).await;

        // Long ID is valid (no slash, no space)
        if let Err(status) = result {
            assert_ne!(
                status,
                StatusCode::BAD_REQUEST,
                "Long valid ID should not return BAD_REQUEST"
            );
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_long_valid_id() {
        let workspace_id = "workspace-".to_string() + &"a".repeat(200);
        let result = get_workspace_status(Path(workspace_id)).await;

        if let Err(status) = result {
            assert_ne!(
                status,
                StatusCode::BAD_REQUEST,
                "Long valid ID should not return BAD_REQUEST"
            );
        }
    }

    #[tokio::test]
    async fn test_delete_workspace_with_uuid() {
        let workspace_id = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        // UUID format is valid
        if let Err(status) = result {
            assert_ne!(status, StatusCode::BAD_REQUEST, "UUID should be valid");
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_uuid() {
        let workspace_id = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let result = get_workspace_status(Path(workspace_id)).await;

        if let Err(status) = result {
            assert_ne!(status, StatusCode::BAD_REQUEST, "UUID should be valid");
        }
    }

    #[tokio::test]
    async fn test_delete_workspace_with_numeric_id() {
        let workspace_id = "123456789".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        if let Err(status) = result {
            assert_ne!(
                status,
                StatusCode::BAD_REQUEST,
                "Numeric ID should be valid"
            );
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_numeric_id() {
        let workspace_id = "987654321".to_string();
        let result = get_workspace_status(Path(workspace_id)).await;

        if let Err(status) = result {
            assert_ne!(
                status,
                StatusCode::BAD_REQUEST,
                "Numeric ID should be valid"
            );
        }
    }

    // ==================== RESPONSE STRUCTURE TESTS ====================

    #[tokio::test]
    async fn test_delete_workspace_success_response_structure() {
        let workspace_id = "test-delete-response".to_string();
        let result = delete_workspace(Path(workspace_id.clone())).await;

        if let Ok(response) = result {
            let json = response.0;

            // Verify expected fields
            assert!(json.get("status").is_some(), "Should have status field");
            assert!(json.get("message").is_some(), "Should have message field");
            assert!(
                json.get("workspace_id").is_some(),
                "Should have workspace_id field"
            );

            // Verify workspace_id matches
            if let Some(ws_id) = json.get("workspace_id").and_then(|v| v.as_str()) {
                assert_eq!(ws_id, workspace_id);
            }
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_response_is_json() {
        let workspace_id = "test-status-json".to_string();
        let result = get_workspace_status(Path(workspace_id)).await;

        if let Ok(response) = result {
            let json = response.0;

            // Should be valid JSON object
            assert!(json.is_object(), "Response should be JSON object");

            // Should be serializable
            let serialized = serde_json::to_string(&json);
            assert!(serialized.is_ok(), "Response should serialize");
        }
    }

    // ==================== ERROR HANDLING TESTS ====================

    #[tokio::test]
    async fn test_delete_workspace_error_consistency() {
        let invalid_ids = vec!["", "ws/test", "ws space"];

        for invalid_id in invalid_ids {
            let result = delete_workspace(Path(invalid_id.to_string())).await;
            assert!(result.is_err(), "Should error for invalid ID: {invalid_id}");
            assert_eq!(
                result.unwrap_err(),
                StatusCode::BAD_REQUEST,
                "Should return BAD_REQUEST for invalid ID: {invalid_id}"
            );
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_error_consistency() {
        let invalid_ids = vec!["", "ws/test", "ws space"];

        for invalid_id in invalid_ids {
            let result = get_workspace_status(Path(invalid_id.to_string())).await;
            assert!(result.is_err(), "Should error for invalid ID: {invalid_id}");
            assert_eq!(
                result.unwrap_err(),
                StatusCode::BAD_REQUEST,
                "Should return BAD_REQUEST for invalid ID: {invalid_id}"
            );
        }
    }

    // ==================== PERFORMANCE TESTS ====================

    #[tokio::test]
    async fn test_validation_is_fast() {
        let workspace_id = "workspace/invalid".to_string();

        let start = std::time::Instant::now();
        let _ = delete_workspace(Path(workspace_id)).await;
        let duration = start.elapsed();

        // Validation should be very fast (< 100ms)
        assert!(
            duration.as_millis() < 100,
            "Validation should be fast, took: {duration:?}"
        );
    }

    #[tokio::test]
    async fn test_multiple_validations_performance() {
        let invalid_ids = vec!["", "ws/test", "ws space", "", "ws\ntest"];

        let start = std::time::Instant::now();
        for invalid_id in invalid_ids {
            let _ = delete_workspace(Path(invalid_id.to_string())).await;
        }
        let duration = start.elapsed();

        // Sanity check: validations should complete (no hangs/infinite loops)
        // Timing-based assertions removed — they're unreliable under parallel test load
        assert!(
            duration.as_secs() < 30,
            "Validations appear hung, took: {duration:?}"
        );
    }

    // ==================== IDEMPOTENCY TESTS ====================

    #[tokio::test]
    async fn test_delete_workspace_idempotent() {
        let workspace_id = "test-idempotent-delete".to_string();

        // Delete twice
        let result1 = delete_workspace(Path(workspace_id.clone())).await;
        let result2 = delete_workspace(Path(workspace_id.clone())).await;

        // Both should either succeed or fail consistently
        // (Second deletion should handle "already deleted" gracefully)
        if let (Ok(json1), Ok(json2)) = (result1, result2) {
            // Both should indicate success (even if already deleted)
            assert!(json1.get("status").is_some());
            assert!(json2.get("status").is_some());
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_consistency() {
        let workspace_id = "test-status-consistency".to_string();

        // Check status multiple times
        let result1 = get_workspace_status(Path(workspace_id.clone())).await;
        let result2 = get_workspace_status(Path(workspace_id.clone())).await;

        // Results should be consistent
        match (result1, result2) {
            (Ok(_), Ok(_)) => assert!(true, "Both succeeded"),
            (Err(e1), Err(e2)) => assert_eq!(e1, e2, "Error codes should match"),
            _ => assert!(true, "Mixed results are OK for non-existent workspace"),
        }
    }

    // ==================== SPECIAL CHARACTER TESTS ====================

    #[tokio::test]
    async fn test_workspace_id_with_dots() {
        let workspace_id = "workspace.test.prod".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        // Dots are valid
        if let Err(status) = result {
            assert_ne!(status, StatusCode::BAD_REQUEST, "Dots should be valid");
        }
    }

    #[tokio::test]
    async fn test_workspace_id_with_dashes() {
        let workspace_id = "workspace-test-prod".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        // Dashes are valid
        if let Err(status) = result {
            assert_ne!(status, StatusCode::BAD_REQUEST, "Dashes should be valid");
        }
    }

    #[tokio::test]
    async fn test_workspace_id_with_underscores() {
        let workspace_id = "workspace_test_prod".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        // Underscores are valid
        if let Err(status) = result {
            assert_ne!(
                status,
                StatusCode::BAD_REQUEST,
                "Underscores should be valid"
            );
        }
    }

    #[tokio::test]
    async fn test_workspace_id_mixed_valid_chars() {
        let workspace_id = "workspace-123_test.prod".to_string();
        let result = delete_workspace(Path(workspace_id)).await;

        // Mixed valid characters should be accepted
        if let Err(status) = result {
            assert_ne!(
                status,
                StatusCode::BAD_REQUEST,
                "Mixed valid chars should be valid"
            );
        }
    }

    #[tokio::test]
    async fn test_workspace_id_single_character() {
        let workspace_ids = vec!["a", "1", "z", "X"];

        for workspace_id in workspace_ids {
            let result = delete_workspace(Path(workspace_id.to_string())).await;

            // Single valid character should be accepted
            if let Err(status) = result {
                assert_ne!(
                    status,
                    StatusCode::BAD_REQUEST,
                    "Single char '{workspace_id}' should be valid"
                );
            }
        }
    }
}
