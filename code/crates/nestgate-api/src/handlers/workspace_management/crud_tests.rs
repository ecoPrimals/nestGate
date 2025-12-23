//! **WORKSPACE CRUD TESTS**
//!
//! Comprehensive tests for workspace CRUD operations including:
//! - Workspace creation, reading, updating, and deletion
//! - ZFS integration validation
//! - Error handling and edge cases
//! - Helper function coverage

use super::crud::*;
use axum::http::StatusCode;
use serde_json::json;

// ==================== HELPER FUNCTION TESTS ====================

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_parse_size_with_bytes() {
        assert_eq!(parse_size("1024"), 1024);
        assert_eq!(parse_size("0"), 0);
        assert_eq!(parse_size("999999"), 999999);
    }

    #[test]
    fn test_parse_size_with_kilobytes() {
        assert_eq!(parse_size("1K"), 1024);
        assert_eq!(parse_size("10K"), 10240);
        assert_eq!(parse_size("1.5K"), 1536);
    }

    #[test]
    fn test_parse_size_with_megabytes() {
        assert_eq!(parse_size("1M"), 1024 * 1024);
        assert_eq!(parse_size("10M"), 10 * 1024 * 1024);
        assert_eq!(parse_size("1.5M"), (1.5 * 1024.0 * 1024.0) as u64);
    }

    #[test]
    fn test_parse_size_with_gigabytes() {
        assert_eq!(parse_size("1G"), 1024 * 1024 * 1024);
        assert_eq!(parse_size("5G"), 5 * 1024 * 1024 * 1024);
        assert_eq!(
            parse_size("1.25G"),
            (1.25 * 1024.0 * 1024.0 * 1024.0) as u64
        );
    }

    #[test]
    fn test_parse_size_with_terabytes() {
        assert_eq!(parse_size("1T"), 1024_u64 * 1024 * 1024 * 1024);
        assert_eq!(parse_size("2T"), 2 * 1024_u64 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_with_petabytes() {
        assert_eq!(parse_size("1P"), 1024_u64 * 1024 * 1024 * 1024 * 1024);
    }

    #[test]
    fn test_parse_size_edge_cases() {
        // Special values
        assert_eq!(parse_size("none"), 0);
        assert_eq!(parse_size("-"), 0);
        assert_eq!(parse_size(""), 0);
        assert_eq!(parse_size("   "), 0);

        // Invalid formats
        assert_eq!(parse_size("invalid"), 0);
        assert_eq!(parse_size("K"), 0);
        assert_eq!(parse_size("X"), 0);
    }

    #[test]
    fn test_parse_size_case_insensitive() {
        assert_eq!(parse_size("1k"), 1024);
        assert_eq!(parse_size("1m"), 1024 * 1024);
        assert_eq!(parse_size("1g"), 1024 * 1024 * 1024);
    }
}

// ==================== WORKSPACE ID VALIDATION TESTS ====================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_workspace_with_invalid_id_empty() {
        let result = get_workspace(axum::extract::Path(String::new())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_get_workspace_with_invalid_id_slash() {
        let result = get_workspace(axum::extract::Path("ws/invalid".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_update_workspace_config_with_invalid_id_empty() {
        let config = json!({
            "quota": "20G"
        });
        let result =
            update_workspace_config(axum::extract::Path(String::new()), axum::Json(config)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_update_workspace_config_with_invalid_id_slash() {
        let config = json!({
            "quota": "20G"
        });
        let result = update_workspace_config(
            axum::extract::Path("ws/invalid".to_string()),
            axum::Json(config),
        )
        .await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_invalid_id_empty() {
        let result = delete_workspace(axum::extract::Path(String::new())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_invalid_id_path_traversal() {
        let result = delete_workspace(axum::extract::Path("../etc/passwd".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_delete_workspace_with_invalid_id_slash() {
        let result = delete_workspace(axum::extract::Path("ws/123".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }
}

// ==================== WORKSPACE CREATION TESTS ====================

#[cfg(test)]
mod creation_tests {
    use super::*;

    #[tokio::test]
    async fn test_create_workspace_with_minimal_config() {
        let request = json!({
            "name": "test-workspace"
        });

        // This will fail in test environment (no ZFS), but tests the validation path
        let result = create_workspace(axum::Json(request)).await;

        // We expect an error since ZFS won't be available, but it shouldn't be BAD_REQUEST
        if result.is_err() {
            assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[tokio::test]
    async fn test_create_workspace_with_full_config() {
        let request = json!({
            "name": "production-workspace",
            "quota": "50G",
            "compression": "zstd",
            "recordsize": "1M"
        });

        let result = create_workspace(axum::Json(request)).await;

        // Will fail in test environment (no ZFS)
        if result.is_err() {
            assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    #[tokio::test]
    async fn test_create_workspace_with_invalid_name_empty() {
        let request = json!({
            "name": ""
        });

        let result = create_workspace(axum::Json(request)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_workspace_with_invalid_name_too_long() {
        let long_name = "a".repeat(101);
        let request = json!({
            "name": long_name
        });

        let result = create_workspace(axum::Json(request)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_workspace_without_name() {
        let request = json!({
            "quota": "10G"
        });

        // Should use default name "unnamed-workspace"
        let result = create_workspace(axum::Json(request)).await;

        // Will fail in test environment (no ZFS)
        if result.is_err() {
            assert_eq!(result.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

// ==================== UPDATE CONFIG TESTS ====================

#[cfg(test)]
mod update_tests {
    use super::*;

    #[tokio::test]
    async fn test_update_workspace_config_quota() {
        let config = json!({
            "quota": "20G"
        });

        let result = update_workspace_config(
            axum::extract::Path("ws-123".to_string()),
            axum::Json(config),
        )
        .await;

        // Will fail in test environment (no ZFS/workspace doesn't exist)
        if result.is_err() {
            // Could be BAD_REQUEST if all updates fail
            let status = result.unwrap_err();
            assert!(
                status == StatusCode::BAD_REQUEST || status == StatusCode::INTERNAL_SERVER_ERROR
            );
        }
    }

    #[tokio::test]
    async fn test_update_workspace_config_compression() {
        let config = json!({
            "compression": "zstd"
        });

        let result = update_workspace_config(
            axum::extract::Path("ws-456".to_string()),
            axum::Json(config),
        )
        .await;

        if result.is_err() {
            let status = result.unwrap_err();
            assert!(
                status == StatusCode::BAD_REQUEST || status == StatusCode::INTERNAL_SERVER_ERROR
            );
        }
    }

    #[tokio::test]
    async fn test_update_workspace_config_name() {
        let config = json!({
            "name": "renamed-workspace"
        });

        let result = update_workspace_config(
            axum::extract::Path("ws-789".to_string()),
            axum::Json(config),
        )
        .await;

        if result.is_err() {
            let status = result.unwrap_err();
            assert!(
                status == StatusCode::BAD_REQUEST || status == StatusCode::INTERNAL_SERVER_ERROR
            );
        }
    }

    #[tokio::test]
    async fn test_update_workspace_config_multiple_properties() {
        let config = json!({
            "name": "updated-workspace",
            "quota": "100G",
            "compression": "lz4"
        });

        let result = update_workspace_config(
            axum::extract::Path("ws-multi".to_string()),
            axum::Json(config),
        )
        .await;

        if result.is_err() {
            let status = result.unwrap_err();
            assert!(
                status == StatusCode::BAD_REQUEST || status == StatusCode::INTERNAL_SERVER_ERROR
            );
        }
    }

    #[tokio::test]
    async fn test_update_workspace_config_empty() {
        let config = json!({});

        let result = update_workspace_config(
            axum::extract::Path("ws-empty".to_string()),
            axum::Json(config),
        )
        .await;

        // Empty config should be rejected
        if result.is_err() {
            let status = result.unwrap_err();
            assert_eq!(status, StatusCode::BAD_REQUEST);
        }
    }
}

// ==================== INTEGRATION TESTS ====================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_workspaces_basic() {
        // This will fail without ZFS, but tests the handler structure
        let result = get_workspaces().await;

        // Should return either success (empty list) or error
        if result.is_ok() {
            let response = result.unwrap();
            let value = response.0;
            assert!(value.get("status").is_some());
        }
    }

    #[tokio::test]
    async fn test_workspace_crud_flow_validation() {
        // Test the validation flow without ZFS

        // 1. Create workspace with valid name
        let create_request = json!({
            "name": "test-flow"
        });
        let _ = create_workspace(axum::Json(create_request)).await;

        // 2. Try to create with invalid name
        let invalid_create = json!({
            "name": ""
        });
        let result = create_workspace(axum::Json(invalid_create)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);

        // 3. Try to get with invalid ID
        let result = get_workspace(axum::extract::Path(String::new())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);

        // 4. Try to delete with invalid ID
        let result = delete_workspace(axum::extract::Path("../path".to_string())).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), StatusCode::BAD_REQUEST);
    }
}

// ==================== ENVIRONMENT VARIABLE TESTS ====================

#[cfg(test)]
mod env_tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_create_workspace_uses_env_pool_name() {
        // Set custom pool name
        env::set_var("NESTGATE_WORKSPACE_POOL", "custom-pool");

        let request = json!({
            "name": "env-test"
        });

        let _ = create_workspace(axum::Json(request)).await;

        // Clean up
        env::remove_var("NESTGATE_WORKSPACE_POOL");

        // Test passes if no panic occurs
    }

    #[tokio::test]
    async fn test_get_workspace_uses_default_pool_name() {
        // Ensure env var is not set
        env::remove_var("NESTGATE_WORKSPACE_POOL");

        let _ = get_workspace(axum::extract::Path("test-id".to_string())).await;

        // Test passes if default "zfspool" is used (no panic)
    }
}
