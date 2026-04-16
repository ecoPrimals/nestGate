// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! AI-First handler and integration tests (split from `ai_first_example_tests.rs`).

#[cfg(test)]
mod tests {
    use super::super::ai_first_example::*;
    use axum::Json;
    use axum::extract::{Path, Query};
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_get_storage_info_empty_filter() {
        let params = StorageQuery {
            pool: Some("nonexistent".to_string()),
            detailed: None,
        };

        let result = get_storage_info(Query(params)).await;
        assert!(result.is_ok());

        let pools = result.unwrap().0;
        assert!(pools.is_empty());
    }

    #[tokio::test]
    async fn test_demo_confidence_levels() {
        let result = demo_confidence_levels().await;
        let response = result.0;

        assert!(response.success);
        assert!(!response.data.is_empty());
        assert!(response.confidence_score >= 0.0 && response.confidence_score <= 1.0);
        assert!(response.data.len() > 1);
    }

    #[tokio::test]
    async fn test_demo_suggested_actions_structure() {
        let result = demo_suggested_actions().await;
        let response = result.0;

        assert!(response.success);
        assert!(!response.data.is_empty());

        let first_capability = &response.data[0];
        assert!(!first_capability.category.is_empty());
        assert!(!first_capability.description.is_empty());
        assert!(!first_capability.automation_level.is_empty());
    }

    #[test]
    fn test_create_routes_returns_router() {
        let router = create_routes();
        let _ = router;
    }

    #[test]
    fn test_create_handler_returns_router() {
        let router = create_handler();
        let _ = router;
    }

    #[test]
    fn test_storage_info_capacity_calculation() {
        let info = StorageInfo {
            pool_name: "calc-pool".to_string(),
            total_size_gb: 1000,
            used_size_gb: 650,
            available_size_gb: 350,
            health_status: "ONLINE".to_string(),
            last_scrub: None,
        };

        assert_eq!(
            info.used_size_gb + info.available_size_gb,
            info.total_size_gb
        );
    }

    #[test]
    fn test_multiple_action_types_in_vec() {
        let actions = [
            SuggestedAction {
                action_id: "1".to_string(),
                action_type: ActionType::Optimize,
                description: "Opt".to_string(),
                confidence: 0.9,
                parameters: HashMap::new(),
                priority: 1,
                estimated_duration_ms: None,
                dependencies: vec![],
            },
            SuggestedAction {
                action_id: "2".to_string(),
                action_type: ActionType::Monitor,
                description: "Mon".to_string(),
                confidence: 0.85,
                parameters: HashMap::new(),
                priority: 2,
                estimated_duration_ms: None,
                dependencies: vec![],
            },
            SuggestedAction {
                action_id: "3".to_string(),
                action_type: ActionType::Backup,
                description: "Bak".to_string(),
                confidence: 0.95,
                parameters: HashMap::new(),
                priority: 3,
                estimated_duration_ms: Some(5000),
                dependencies: vec!["1".to_string(), "2".to_string()],
            },
        ];

        assert_eq!(actions.len(), 3);
        assert_eq!(actions[2].dependencies.len(), 2);
    }

    #[tokio::test]
    async fn get_storage_info_filters_by_pool_query() {
        let q = StorageQuery {
            pool: Some("backup".into()),
            detailed: None,
        };
        let Json(rows) = get_storage_info(Query(q)).await.expect("ok");
        assert_eq!(rows.len(), 1);
        assert!(rows[0].pool_name.contains("backup"));
    }

    #[tokio::test]
    async fn get_storage_info_returns_all_when_no_filter() {
        let q = StorageQuery {
            pool: None,
            detailed: Some(true),
        };
        let Json(rows) = get_storage_info(Query(q)).await.expect("ok");
        assert_eq!(rows.len(), 2);
    }

    #[tokio::test]
    async fn get_pool_info_known_and_unknown_pools() {
        let main = get_pool_info(Path("main-pool".into())).await;
        assert!(main.0.data.is_some());
        assert!(main.0.confidence_score > 0.9);

        let backup = get_pool_info(Path("backup-pool".into())).await;
        assert!(backup.0.data.is_some());
        assert!(backup.0.confidence_score < 0.9);

        let missing = get_pool_info(Path("unknown-pool-xyz".into())).await;
        assert!(missing.0.data.is_none());
        assert!((missing.0.confidence_score - 0.0).abs() < f64::EPSILON);
    }

    #[tokio::test]
    async fn execute_storage_operation_scrub_snapshot_export_and_unknown() {
        let scrub = execute_storage_operation(Json(PoolOperationRequest {
            b_operation: "scrub".into(),
            pool_name: "tank".into(),
            parameters: None,
        }))
        .await
        .expect("ok");
        assert!(scrub.0.data.contains("Scrub"));

        let snap = execute_storage_operation(Json(PoolOperationRequest {
            b_operation: "snapshot".into(),
            pool_name: "tank".into(),
            parameters: None,
        }))
        .await
        .expect("ok");
        assert!(snap.0.data.contains("Snapshot"));

        let exp = execute_storage_operation(Json(PoolOperationRequest {
            b_operation: "export".into(),
            pool_name: "tank".into(),
            parameters: None,
        }))
        .await
        .expect("ok");
        assert!(exp.0.data.contains("Export"));

        let bad = execute_storage_operation(Json(PoolOperationRequest {
            b_operation: "unknown-op".into(),
            pool_name: "tank".into(),
            parameters: None,
        }))
        .await
        .expect("ok");
        assert!(bad.0.data.contains("not supported"));
    }

    #[tokio::test]
    async fn demo_endpoints_return_ai_first_wrappers() {
        let conf = demo_confidence_levels().await;
        assert!(!conf.0.data.is_empty());
        let sug = demo_suggested_actions().await;
        assert!(!sug.0.data.is_empty());
    }

    #[test]
    fn ai_pool_operation_and_status_handlers_run() {
        let op = ai_pool_operation(Json(PoolOperationRequest {
            b_operation: "scrub".into(),
            pool_name: "zroot".into(),
            parameters: None,
        }));
        assert!(op.0.data.contains("scrub"));

        let st = ai_pool_status(Path("pool-a".into()));
        assert_eq!(st.0.data.pool_name, "pool-a");
    }
}
