// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **WORKSPACE OPTIMIZATION TESTS**
//!
//! Tests for workspace storage optimization including:
//! - ZFS optimization (compression, recordsize, cache, dedup)
//! - Storage pattern analysis
//! - AI-assisted optimization
//! - Error handling

use super::optimization::*;
use axum::extract::Path;

// ==================== HANDLER TESTS ====================

#[cfg(test)]
mod handler_tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_workspace_returns_success() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert_eq!(json["status"], "success");
    }

    #[tokio::test]
    async fn test_optimize_workspace_has_workspace_id() {
        let workspace_id = "my-test-workspace".to_string();
        let result = optimize_workspace(Path(workspace_id.clone())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert_eq!(json["workspace_id"], workspace_id);
    }

    #[tokio::test]
    async fn test_optimize_workspace_has_optimizations_list() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert!(json["optimizations_applied"].is_array());
    }

    #[tokio::test]
    async fn test_optimize_workspace_has_pattern_analysis() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert!(json["pattern_analysis"].is_object());
    }

    #[tokio::test]
    async fn test_optimize_workspace_has_stats() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert!(json["optimization_stats"].is_object());
    }

    #[tokio::test]
    async fn test_optimize_workspace_has_message() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert!(json["message"].is_string());
        assert!(!json["message"].as_str().unwrap().is_empty());
    }
}

// ==================== VALIDATION TESTS ====================

#[cfg(test)]
mod validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_with_simple_id() {
        let result = optimize_workspace(Path("simple".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_optimize_with_hyphenated_id() {
        let result = optimize_workspace(Path("my-workspace-123".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_optimize_with_underscored_id() {
        let result = optimize_workspace(Path("my_workspace_456".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_optimize_with_long_id() {
        let long_id = "workspace-".to_string() + &"a".repeat(100);
        let result = optimize_workspace(Path(long_id)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_optimize_with_empty_id() {
        let result = optimize_workspace(Path(String::new())).await;
        // Should handle gracefully even with empty ID
        assert!(result.is_ok());
    }
}

// ==================== PATTERN ANALYSIS TESTS ====================

#[cfg(test)]
mod pattern_analysis_tests {
    use super::*;

    #[tokio::test]
    async fn test_pattern_analysis_includes_file_size_distribution() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let pattern = &json["pattern_analysis"];
        assert!(pattern["file_size_distribution"].is_string());
    }

    #[tokio::test]
    async fn test_pattern_analysis_includes_file_type_distribution() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let pattern = &json["pattern_analysis"];
        assert!(pattern["file_type_distribution"].is_object());
    }

    #[tokio::test]
    async fn test_pattern_analysis_includes_duplicate_ratio() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let pattern = &json["pattern_analysis"];
        assert!(pattern["duplicate_ratio"].is_number() || pattern["duplicate_ratio"].is_f64());
    }

    #[tokio::test]
    async fn test_pattern_analysis_includes_sequential_vs_random() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let pattern = &json["pattern_analysis"];
        assert!(
            pattern["sequential_vs_random"].is_number() || pattern["sequential_vs_random"].is_f64()
        );
    }

    #[tokio::test]
    async fn test_pattern_analysis_includes_read_write_ratio() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let pattern = &json["pattern_analysis"];
        assert!(pattern["read_write_ratio"].is_number() || pattern["read_write_ratio"].is_f64());
    }
}

// ==================== OPTIMIZATION TYPES TESTS ====================

#[cfg(test)]
mod optimization_types_tests {
    use super::*;

    #[tokio::test]
    async fn test_optimizations_applied_is_array() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let optimizations = &json["optimizations_applied"];
        assert!(optimizations.is_array());
    }

    #[tokio::test]
    async fn test_optimizations_may_include_compression() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let _optimizations = json["optimizations_applied"].as_array().unwrap();

        // Successfully got optimizations array
        // (Exact count depends on ZFS availability, just verify array exists)
    }

    #[tokio::test]
    async fn test_warnings_array_present() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert!(json["warnings"].is_array());
    }
}

// ==================== ERROR HANDLING TESTS ====================

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_no_panics_with_zfs_unavailable() {
        // When ZFS is not available, should still return success with appropriate handling
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handles_invalid_dataset_gracefully() {
        let result = optimize_workspace(Path("invalid/dataset/path".to_string())).await;
        // Should handle invalid paths gracefully
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handles_special_characters() {
        let special_ids = vec!["ws@123", "ws#456", "ws$789"];

        for special_id in special_ids {
            let result = optimize_workspace(Path(special_id.to_string())).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_handles_unicode_characters() {
        let unicode_ids = vec!["espace-français", "工作区-中文"];

        for unicode_id in unicode_ids {
            let result = optimize_workspace(Path(unicode_id.to_string())).await;
            assert!(result.is_ok());
        }
    }
}

// ==================== INTEGRATION TESTS ====================

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_multiple_optimizations_sequential() {
        let workspace_ids = vec!["ws-1", "ws-2", "ws-3"];

        for workspace_id in workspace_ids {
            let result = optimize_workspace(Path(workspace_id.to_string())).await;
            assert!(result.is_ok());

            let json = result.unwrap().0;
            assert_eq!(json["status"], "success");
            assert_eq!(json["workspace_id"], workspace_id);
        }
    }

    #[tokio::test]
    async fn test_concurrent_optimizations() {
        use tokio::task;

        let mut handles = vec![];

        for i in 0..3 {
            let workspace_id = format!("concurrent-ws-{i}");
            let handle = task::spawn(async move { optimize_workspace(Path(workspace_id)).await });
            handles.push(handle);
        }

        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok());
            assert!(result.unwrap().is_ok());
        }
    }

    #[tokio::test]
    async fn test_optimization_response_consistency() {
        // Run multiple times, verify consistent structure
        for _ in 0..3 {
            let result = optimize_workspace(Path("test-ws".to_string())).await;
            assert!(result.is_ok());

            let json = result.unwrap().0;

            // Verify consistent structure
            assert!(json["status"].is_string());
            assert!(json["message"].is_string());
            assert!(json["workspace_id"].is_string());
            assert!(json["optimizations_applied"].is_array());
            assert!(json["warnings"].is_array());
            assert!(json["pattern_analysis"].is_object());
            assert!(json["optimization_stats"].is_object());
        }
    }
}

// ==================== AI OPTIMIZATION TESTS ====================

#[cfg(test)]
mod ai_optimization_tests {
    use super::*;

    #[tokio::test]
    async fn test_optimization_works_without_ai() {
        // Optimization should work even when AI is not available
        // (Which is the common case in testing)
        nestgate_core::env_process::remove_var("NESTGATE_AI_ENDPOINT");

        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        assert_eq!(json["status"], "success");
    }

    #[tokio::test]
    async fn test_optimization_with_ai_endpoint_set() {
        // Set AI endpoint (even if it doesn't exist)
        nestgate_core::env_process::set_var("NESTGATE_AI_ENDPOINT", "http://localhost:9999");

        let result = optimize_workspace(Path("test-ws".to_string())).await;

        // Should still succeed even if AI endpoint is unreachable
        assert!(result.is_ok());

        nestgate_core::env_process::remove_var("NESTGATE_AI_ENDPOINT");
    }
}

// ==================== RESPONSE STRUCTURE TESTS ====================

#[cfg(test)]
mod response_structure_tests {
    use super::*;

    #[tokio::test]
    async fn test_response_has_all_required_fields() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;

        // Required fields
        assert!(json.get("status").is_some());
        assert!(json.get("message").is_some());
        assert!(json.get("workspace_id").is_some());
        assert!(json.get("optimizations_applied").is_some());
        assert!(json.get("warnings").is_some());
        assert!(json.get("pattern_analysis").is_some());
        assert!(json.get("optimization_stats").is_some());
    }

    #[tokio::test]
    async fn test_pattern_analysis_structure() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let pattern = &json["pattern_analysis"];

        // Pattern analysis fields
        assert!(pattern.get("file_size_distribution").is_some());
        assert!(pattern.get("file_type_distribution").is_some());
        assert!(pattern.get("duplicate_ratio").is_some());
        assert!(pattern.get("sequential_vs_random").is_some());
        assert!(pattern.get("read_write_ratio").is_some());
    }

    #[tokio::test]
    async fn test_optimization_stats_structure() {
        let result = optimize_workspace(Path("test-ws".to_string())).await;
        assert!(result.is_ok());

        let json = result.unwrap().0;
        let stats = &json["optimization_stats"];

        // Stats should be an object (content varies based on ZFS availability)
        assert!(stats.is_object());
    }
}

// ==================== EDGE CASE TESTS ====================

#[cfg(test)]
mod edge_case_tests {
    use super::*;

    #[tokio::test]
    async fn test_optimize_with_very_long_workspace_id() {
        let long_id = "a".repeat(1000);
        let result = optimize_workspace(Path(long_id)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_optimize_with_whitespace() {
        let whitespace_ids = vec![
            "workspace with spaces",
            "  leading-spaces",
            "trailing-spaces  ",
        ];

        for ws_id in whitespace_ids {
            let result = optimize_workspace(Path(ws_id.to_string())).await;
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_optimize_idempotency() {
        // Running optimization multiple times should be safe
        let workspace_id = "idempotent-ws";

        let result1 = optimize_workspace(Path(workspace_id.to_string())).await;
        let result2 = optimize_workspace(Path(workspace_id.to_string())).await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        // Both should succeed
        assert_eq!(result1.unwrap().0["status"], "success");
        assert_eq!(result2.unwrap().0["status"], "success");
    }
}
