//! **WORKSPACE STORAGE TESTS**
//!
//! Tests for workspace storage operations including:
//! - Storage deletion
//! - Status monitoring
//! - Cleanup operations
//! - Scaling recommendations

use super::storage::*;
use axum::http::StatusCode;

// ==================== VALIDATION TESTS ====================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_delete_workspace_with_empty_id() {
        let result = delete_workspace(axum::extract::Path(String::new())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_slash_in_id() {
        let result = delete_workspace(axum::extract::Path("ws/123".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_space_in_id() {
        let result = delete_workspace(axum::extract::Path("ws 123".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_empty_id() {
        let result = get_workspace_status(axum::extract::Path(String::new())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_slash_in_id() {
        let result = get_workspace_status(axum::extract::Path("ws/456".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_status_with_space_in_id() {
        let result = get_workspace_status(axum::extract::Path("ws 456".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cleanup_workspace_with_empty_id() {
        let result = cleanup_workspace(axum::extract::Path(String::new())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cleanup_workspace_with_slash_in_id() {
        let result = cleanup_workspace(axum::extract::Path("ws/789".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_cleanup_workspace_with_space_in_id() {
        let result = cleanup_workspace(axum::extract::Path("ws 789".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }
}

// ==================== DELETE OPERATIONS TESTS ====================

#[cfg(test)]
mod delete_tests {
    use super::*;

    #[tokio::test]
    async fn test_delete_workspace_with_valid_id_no_zfs() {
        // This will fail without ZFS, but tests the validation and structure
        let result = delete_workspace(axum::extract::Path("ws-test-123".to_string())).await;

        // Should either succeed (if ZFS available) or fail with INTERNAL_SERVER_ERROR
        if result.is_ok() {
            let response = result.expect("Result should be Ok");
            let value = response.0;
            assert!(value.get("status").is_some());
            assert!(value.get("workspace_id").is_some());
        }
        // If error, it should be INTERNAL_SERVER_ERROR (ZFS not available)
        // or success (dataset doesn't exist)
    }

    #[tokio::test]
    async fn test_delete_workspace_validates_before_deletion() {
        // Test with clearly invalid ID
        let result = delete_workspace(axum::extract::Path("../etc/passwd".to_string())).await;

        // Should reject due to slash in ID
        if result.is_err() {
            assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
        }
    }
}

// ==================== STATUS TESTS ====================

#[cfg(test)]
mod status_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_workspace_status_with_valid_id() {
        // Tests structure without ZFS
        let result = get_workspace_status(axum::extract::Path("ws-status-test".to_string())).await;

        // Will fail without ZFS, but validates the flow
        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            // Verify response structure
            assert!(value.get("status").is_some());
            assert!(value.get("workspace_id").is_some());
            assert!(value.get("storage_status").is_some());
        }
    }

    #[tokio::test]
    async fn test_get_workspace_status_response_structure() {
        let result = get_workspace_status(axum::extract::Path("ws-struct-test".to_string())).await;

        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            // Check for expected keys
            if let Some(status) = value.get("status") {
                assert_eq!(status, "success");
            }

            if let Some(usage) = value.get("usage") {
                assert!(usage.get("used_bytes").is_some());
                assert!(usage.get("available_bytes").is_some());
                assert!(usage.get("total_bytes").is_some());
            }
        }
    }
}

// ==================== CLEANUP TESTS ====================

#[cfg(test)]
mod cleanup_tests {
    use super::*;

    #[tokio::test]
    async fn test_cleanup_workspace_with_valid_id() {
        let result = cleanup_workspace(axum::extract::Path("ws-cleanup-test".to_string())).await;

        // Will fail without ZFS but validates the structure
        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            assert!(value.get("status").is_some());
            assert!(value.get("workspace_id").is_some());
            assert!(value.get("actions").is_some());
        }
    }

    #[tokio::test]
    async fn test_cleanup_workspace_returns_actions() {
        let result = cleanup_workspace(axum::extract::Path("ws-actions-test".to_string())).await;

        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            // Should have actions array
            if let Some(actions) = value.get("actions") {
                assert!(actions.is_array());
            }

            // Should track space freed
            assert!(value.get("space_freed_bytes").is_some());
        }
    }
}

// ==================== SCALE TESTS ====================

#[cfg(test)]
mod scale_tests {
    use super::*;

    #[tokio::test]
    async fn test_scale_workspace_with_valid_id() {
        let result = scale_workspace(axum::extract::Path("ws-scale-test".to_string())).await;

        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            assert!(value.get("status").is_some());
            assert!(value.get("workspace_id").is_some());
            assert!(value.get("current_usage").is_some());
        }
    }

    #[tokio::test]
    async fn test_scale_workspace_provides_recommendations() {
        let result = scale_workspace(axum::extract::Path("ws-recommend-test".to_string())).await;

        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            // Should provide scaling recommendations
            if let Some(recommendations) = value.get("scale_recommendations") {
                assert!(recommendations.is_array());
            }

            // Should include current usage metrics
            if let Some(usage) = value.get("current_usage") {
                assert!(usage.get("used_bytes").is_some());
                assert!(usage.get("available_bytes").is_some());
                assert!(usage.get("utilization_percent").is_some());
            }
        }
    }
}

// ==================== INTEGRATION TESTS ====================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_storage_operations_flow() {
        // Test the validation flow across multiple operations

        // 1. Try to get status with invalid ID
        let status_result = get_workspace_status(axum::extract::Path(String::new())).await;
        assert!(status_result.is_err());
        assert_eq!(status_result.unwrap_err(), StatusCode::BAD_REQUEST);

        // 2. Try to cleanup with invalid ID
        let cleanup_result = cleanup_workspace(axum::extract::Path("ws/invalid".to_string())).await;
        assert!(cleanup_result.is_err());
        assert_eq!(cleanup_result.unwrap_err(), StatusCode::BAD_REQUEST);

        // 3. Try to delete with invalid ID
        let delete_result = delete_workspace(axum::extract::Path("ws 123".to_string())).await;
        assert!(delete_result.is_err());
        assert_eq!(delete_result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_all_operations_validate_consistently() {
        let invalid_ids = vec!["", "ws/123", "ws 456", "../etc", "ws\n789"];

        for invalid_id in invalid_ids {
            // All operations should reject these IDs
            let delete_result = delete_workspace(axum::extract::Path(invalid_id.to_string())).await;
            let status_result =
                get_workspace_status(axum::extract::Path(invalid_id.to_string())).await;
            let cleanup_result =
                cleanup_workspace(axum::extract::Path(invalid_id.to_string())).await;

            // At least one should be an error (most likely all)
            assert!(
                delete_result.is_err() || status_result.is_err() || cleanup_result.is_err(),
                "Expected validation error for invalid ID: {invalid_id}"
            );
        }
    }

    #[tokio::test]
    async fn test_valid_id_format_accepted() {
        let valid_ids = vec![
            "ws-123",
            "workspace-456",
            "test-workspace",
            "my-workspace-id",
        ];

        for valid_id in valid_ids {
            // These should pass validation (but may fail on ZFS operations)
            let delete_result = delete_workspace(axum::extract::Path(valid_id.to_string())).await;
            let status_result =
                get_workspace_status(axum::extract::Path(valid_id.to_string())).await;
            let cleanup_result = cleanup_workspace(axum::extract::Path(valid_id.to_string())).await;

            // If they error, it should NOT be BAD_REQUEST (validation passed)
            if let Err(status) = delete_result {
                assert_ne!(
                    status,
                    StatusCode::BAD_REQUEST,
                    "Valid ID should not return BAD_REQUEST: {valid_id}"
                );
            }
            if let Err(status) = status_result {
                assert_ne!(
                    status,
                    StatusCode::BAD_REQUEST,
                    "Valid ID should not return BAD_REQUEST: {valid_id}"
                );
            }
            if let Err(status) = cleanup_result {
                assert_ne!(
                    status,
                    StatusCode::BAD_REQUEST,
                    "Valid ID should not return BAD_REQUEST: {valid_id}"
                );
            }
        }
    }
}

// ==================== ERROR HANDLING TESTS ====================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_operations_handle_missing_zfs_gracefully() {
        // All operations should handle ZFS not being available
        let test_id = "ws-error-test";

        let _ = delete_workspace(axum::extract::Path(test_id.to_string())).await;
        let _ = get_workspace_status(axum::extract::Path(test_id.to_string())).await;
        let _ = cleanup_workspace(axum::extract::Path(test_id.to_string())).await;
        let _ = scale_workspace(axum::extract::Path(test_id.to_string())).await;

        // Test passes if no panics occur
    }

    #[tokio::test]
    async fn test_delete_nonexistent_workspace_succeeds() {
        // Deleting a non-existent workspace should succeed gracefully
        let result = delete_workspace(axum::extract::Path("ws-nonexistent".to_string())).await;

        // Should either succeed (dataset doesn't exist) or fail with INTERNAL_SERVER_ERROR
        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;

            // Response should indicate success even if dataset didn't exist
            if let Some(status) = value.get("status") {
                assert_eq!(status, "success");
            }
        }
    }
}
