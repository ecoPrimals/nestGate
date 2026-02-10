//! **AI-First Example Tests**
//!
//! Comprehensive test suite for AI-First response format and handlers.
//! Moved from `ai_first_example.rs` to maintain file size discipline.

#[cfg(test)]
mod tests {
    use super::super::ai_first_example::*;
    use axum::extract::Query;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[test]
    fn test_ai_first_response_creation() {
        let response: AIFirstResponse<String> = AIFirstResponse {
            data: "test_data".to_string(),
            success: true,
            message: "test message".to_string(),
            confidence_score: 0.95,
        };

        assert_eq!(response.data, "test_data");
        assert!(response.success);
        assert_eq!(response.confidence_score, 0.95);
    }

    #[test]
    fn test_ai_success_with_confidence() {
        let data = vec![1, 2, 3];
        let response = ai_success_with_confidence(data.clone(), 0.88);

        assert_eq!(response.data, data);
        assert!(response.success);
        assert_eq!(response.confidence_score, 0.88);
        assert_eq!(response.message, "Operation completed successfully");
    }

    #[test]
    fn test_ai_response_with_actions() {
        let data = "test";
        let actions = vec![];
        let response = ai_response_with_actions(data, actions);

        assert_eq!(response.data, "test");
        assert!(response.success);
        assert_eq!(response.confidence_score, 0.85);
    }

    #[test]
    fn test_storage_info_creation() {
        let info = StorageInfo {
            pool_name: "test-pool".to_string(),
            total_size_gb: 100,
            used_size_gb: 40,
            available_size_gb: 60,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-01-01".to_string()),
        };

        assert_eq!(info.pool_name, "test-pool");
        assert_eq!(info.total_size_gb, 100);
        assert_eq!(info.used_size_gb, 40);
    }

    #[test]
    fn test_storage_info_serialization() {
        let info = StorageInfo {
            pool_name: "test".to_string(),
            total_size_gb: 100,
            used_size_gb: 50,
            available_size_gb: 50,
            health_status: "ONLINE".to_string(),
            last_scrub: None,
        };

        let json = serde_json::to_string(&info);
        assert!(json.is_ok());
    }

    #[test]
    fn test_pool_info_creation() {
        let info = PoolInfo {
            pool_name: "test-pool".to_string(),
            total_size_gb: 500,
            used_size_gb: 200,
            available_size_gb: 300,
            health_status: "HEALTHY".to_string(),
            last_scrub: Some("2025-01-15T10:30:00Z".to_string()),
        };

        assert_eq!(info.pool_name, "test-pool");
        assert_eq!(info.available_size_gb, 300);
    }

    #[test]
    fn test_pool_operation_request_creation() {
        let request = PoolOperationRequest {
            b_operation: "scrub".to_string(),
            pool_name: "main-pool".to_string(),
            parameters: None,
        };

        assert_eq!(request.b_operation, "scrub");
        assert_eq!(request.pool_name, "main-pool");
        assert!(request.parameters.is_none());
    }

    #[test]
    fn test_suggested_action_creation() {
        let action = SuggestedAction {
            action_id: "action_001".to_string(),
            action_type: ActionType::Optimize,
            description: "Optimize pool".to_string(),
            confidence: 0.92,
            parameters: HashMap::new(),
            priority: 5,
            estimated_duration_ms: Some(5000),
            dependencies: vec![],
        };

        assert_eq!(action.action_id, "action_001");
        assert_eq!(action.confidence, 0.92);
        assert_eq!(action.priority, 5);
    }

    #[test]
    fn test_action_type_serialization() {
        let types = vec![
            ActionType::Optimize,
            ActionType::Monitor,
            ActionType::Alert,
            ActionType::Backup,
            ActionType::Scale,
            ActionType::Repair,
        ];

        for action_type in types {
            let json = serde_json::to_string(&action_type);
            assert!(json.is_ok());
        }
    }

    #[test]
    fn test_ai_first_response_serialization() {
        let response: AIFirstResponse<i32> = AIFirstResponse {
            data: 42,
            success: true,
            message: "Success".to_string(),
            confidence_score: 0.99,
        };

        let json = serde_json::to_string(&response);
        assert!(json.is_ok());

        let json_str = json.unwrap();
        let deserialized: AIFirstResponse<i32> = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.data, 42);
    }

    #[tokio::test]
    async fn test_get_storage_info_no_filter() {
        let params = StorageQuery {
            pool: None,
            detailed: None,
        };

        let result = get_storage_info(Query(params)).await;
        assert!(result.is_ok());

        let pools = result.unwrap().0;
        assert_eq!(pools.len(), 2);
        assert_eq!(pools[0].pool_name, "main-pool");
    }

    #[tokio::test]
    async fn test_get_storage_info_with_filter() {
        let params = StorageQuery {
            pool: Some("backup".to_string()),
            detailed: None,
        };

        let result = get_storage_info(Query(params)).await;
        assert!(result.is_ok());

        let pools = result.unwrap().0;
        assert_eq!(pools.len(), 1);
        assert_eq!(pools[0].pool_name, "backup-pool");
    }

    #[tokio::test]
    async fn test_demo_suggested_actions() {
        let result = demo_suggested_actions().await;
        let response = result.0;

        assert!(response.success);
        assert!(!response.data.is_empty());
        assert!(response.confidence_score > 0.0);
    }

    #[test]
    fn test_pool_info_serialization() {
        let info = PoolInfo {
            pool_name: "test-pool".to_string(),
            total_size_gb: 1000,
            used_size_gb: 400,
            available_size_gb: 600,
            health_status: "Healthy".to_string(),
            last_scrub: Some("2024-01-15T10:30:00Z".to_string()),
        };

        let json = serde_json::to_string(&info);
        assert!(json.is_ok());
    }

    #[test]
    fn test_suggested_action_with_dependencies() {
        let action = SuggestedAction {
            action_id: "action_002".to_string(),
            action_type: ActionType::Backup,
            description: "Backup pool".to_string(),
            confidence: 0.85,
            parameters: HashMap::new(),
            priority: 3,
            estimated_duration_ms: Some(10000),
            dependencies: vec!["action_001".to_string()],
        };

        assert_eq!(action.dependencies.len(), 1);
        assert_eq!(action.dependencies[0], "action_001");
    }

    // ==================== ACTIONTYPE TESTS ====================

    #[test]
    fn test_action_type_optimize_serialization() {
        let action_type = ActionType::Optimize;
        let json = serde_json::to_string(&action_type).expect("Should serialize");
        assert!(json.contains("Optimize"));
    }

    #[test]
    fn test_action_type_monitor_serialization() {
        let action_type = ActionType::Monitor;
        let json = serde_json::to_string(&action_type).expect("Should serialize");
        let deserialized: ActionType = serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(deserialized, ActionType::Monitor));
    }

    #[test]
    fn test_action_type_alert_variant() {
        let action_type = ActionType::Alert;
        let json = serde_json::to_string(&action_type).expect("Should serialize");
        assert!(!json.is_empty());
    }

    #[test]
    fn test_action_type_backup_variant() {
        let action_type = ActionType::Backup;
        let json = serde_json::to_string(&action_type).expect("Should serialize");
        assert!(!json.is_empty());
    }

    #[test]
    fn test_action_type_scale_variant() {
        let action_type = ActionType::Scale;
        assert!(matches!(action_type, ActionType::Scale));
    }

    #[test]
    fn test_action_type_repair_variant() {
        let action_type = ActionType::Repair;
        assert!(matches!(action_type, ActionType::Repair));
    }

    // ==================== SUGGESTED ACTION EDGE CASES ====================

    #[test]
    fn test_suggested_action_no_dependencies() {
        let action = SuggestedAction {
            action_id: "solo_action".to_string(),
            action_type: ActionType::Monitor,
            description: "Monitor solo".to_string(),
            confidence: 1.0,
            parameters: HashMap::new(),
            priority: 5,
            estimated_duration_ms: None,
            dependencies: vec![],
        };

        assert!(action.dependencies.is_empty());
        assert_eq!(action.confidence, 1.0);
        assert!(action.estimated_duration_ms.is_none());
    }

    #[test]
    fn test_suggested_action_with_parameters() {
        let mut params = HashMap::new();
        params.insert("key1".to_string(), serde_json::json!("value1"));
        params.insert("key2".to_string(), serde_json::json!(42));

        let action = SuggestedAction {
            action_id: "param_action".to_string(),
            action_type: ActionType::Optimize,
            description: "Optimize with params".to_string(),
            confidence: 0.75,
            parameters: params.clone(),
            priority: 3,
            estimated_duration_ms: Some(5000),
            dependencies: vec![],
        };

        assert_eq!(action.parameters.len(), 2);
        assert!(action.parameters.contains_key("key1"));
        assert!(action.parameters.contains_key("key2"));
    }

    #[test]
    fn test_suggested_action_high_priority() {
        let action = SuggestedAction {
            action_id: "urgent".to_string(),
            action_type: ActionType::Alert,
            description: "Urgent alert".to_string(),
            confidence: 0.99,
            parameters: HashMap::new(),
            priority: 10,
            estimated_duration_ms: Some(100),
            dependencies: vec![],
        };

        assert_eq!(action.priority, 10);
        assert!(action.confidence > 0.9);
    }

    #[test]
    fn test_suggested_action_serialization() {
        let action = SuggestedAction {
            action_id: "test_id".to_string(),
            action_type: ActionType::Backup,
            description: "Test backup".to_string(),
            confidence: 0.85,
            parameters: HashMap::new(),
            priority: 2,
            estimated_duration_ms: Some(30000),
            dependencies: vec!["dep1".to_string()],
        };

        let json = serde_json::to_string(&action).expect("Should serialize");
        let deserialized: SuggestedAction =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.action_id, "test_id");
        assert_eq!(deserialized.priority, 2);
        assert_eq!(deserialized.dependencies.len(), 1);
    }

    // ==================== AI FIRST RESPONSE EDGE CASES ====================

    #[test]
    fn test_ai_first_response_zero_confidence() {
        let response: AIFirstResponse<String> = AIFirstResponse {
            data: "low confidence".to_string(),
            success: false,
            message: "Operation uncertain".to_string(),
            confidence_score: 0.0,
        };

        assert_eq!(response.confidence_score, 0.0);
        assert!(!response.success);
    }

    #[test]
    fn test_ai_first_response_max_confidence() {
        let response = ai_success_with_confidence(vec![1, 2, 3], 1.0);
        assert_eq!(response.confidence_score, 1.0);
        assert!(response.success);
    }

    #[test]
    fn test_ai_first_response_with_complex_data() {
        #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
        struct ComplexData {
            field1: String,
            field2: Vec<i32>,
            field3: HashMap<String, String>,
        }

        let mut map = HashMap::new();
        map.insert("key".to_string(), "value".to_string());

        let complex = ComplexData {
            field1: "test".to_string(),
            field2: vec![1, 2, 3],
            field3: map,
        };

        let response = ai_success_with_confidence(complex.clone(), 0.95);
        assert_eq!(response.data, complex);
    }

    #[test]
    fn test_ai_response_with_multiple_actions() {
        let actions = vec![
            SuggestedAction {
                action_id: "act1".to_string(),
                action_type: ActionType::Monitor,
                description: "First".to_string(),
                confidence: 0.9,
                parameters: HashMap::new(),
                priority: 1,
                estimated_duration_ms: None,
                dependencies: vec![],
            },
            SuggestedAction {
                action_id: "act2".to_string(),
                action_type: ActionType::Optimize,
                description: "Second".to_string(),
                confidence: 0.85,
                parameters: HashMap::new(),
                priority: 2,
                estimated_duration_ms: Some(1000),
                dependencies: vec!["act1".to_string()],
            },
        ];

        let response = ai_response_with_actions("test_data", actions);
        assert!(response.success);
        assert_eq!(response.confidence_score, 0.85);
    }

    // ==================== STORAGE INFO TESTS ====================

    #[test]
    fn test_storage_info_zero_available() {
        let info = StorageInfo {
            pool_name: "full-pool".to_string(),
            total_size_gb: 1000,
            used_size_gb: 1000,
            available_size_gb: 0,
            health_status: "DEGRADED".to_string(),
            last_scrub: None,
        };

        assert_eq!(info.available_size_gb, 0);
        assert_eq!(info.used_size_gb, info.total_size_gb);
    }

    #[test]
    fn test_storage_info_unhealthy_status() {
        let info = StorageInfo {
            pool_name: "error-pool".to_string(),
            total_size_gb: 500,
            used_size_gb: 250,
            available_size_gb: 250,
            health_status: "FAULTED".to_string(),
            last_scrub: Some("2024-01-01T00:00:00Z".to_string()),
        };

        assert_eq!(info.health_status, "FAULTED");
        assert!(info.last_scrub.is_some());
    }

    #[test]
    fn test_storage_info_no_scrub_history() {
        let info = StorageInfo {
            pool_name: "new-pool".to_string(),
            total_size_gb: 2000,
            used_size_gb: 100,
            available_size_gb: 1900,
            health_status: "ONLINE".to_string(),
            last_scrub: None,
        };

        assert!(info.last_scrub.is_none());
        assert_eq!(info.available_size_gb, 1900);
    }

    // ==================== POOL INFO TESTS ====================

    #[test]
    fn test_pool_info_deserialization() {
        let json = r#"{"pool_name":"test","total_size_gb":100,"used_size_gb":50,"available_size_gb":50,"health_status":"ONLINE","last_scrub":null}"#;
        let info: PoolInfo = serde_json::from_str(json).expect("Should deserialize");

        assert_eq!(info.pool_name, "test");
        assert_eq!(info.total_size_gb, 100);
        assert!(info.last_scrub.is_none());
    }

    #[test]
    fn test_pool_info_with_scrub_timestamp() {
        let info = PoolInfo {
            pool_name: "scrubbed-pool".to_string(),
            total_size_gb: 800,
            used_size_gb: 300,
            available_size_gb: 500,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-02-01T12:00:00Z".to_string()),
        };

        assert!(info.last_scrub.is_some());
        assert!(info.last_scrub.unwrap().contains("2025-02-01"));
    }

    // ==================== REQUEST/QUERY TESTS ====================

    #[test]
    fn test_storage_query_default_values() {
        let query = StorageQuery {
            pool: None,
            detailed: None,
        };

        assert!(query.pool.is_none());
        assert!(query.detailed.is_none());
    }

    #[test]
    fn test_storage_query_with_pool_filter() {
        let query = StorageQuery {
            pool: Some("main".to_string()),
            detailed: Some(true),
        };

        assert!(query.pool.is_some());
        assert_eq!(query.pool.unwrap(), "main");
        assert!(query.detailed.unwrap());
    }

    #[test]
    fn test_storage_request_with_parameters() {
        let mut params = HashMap::new();
        params.insert("force".to_string(), serde_json::json!(true));

        let request = StorageRequest {
            b_operation: "destroy".to_string(),
            pool_name: "old-pool".to_string(),
            parameters: Some(params),
        };

        assert!(request.parameters.is_some());
        let params = request.parameters.unwrap();
        assert!(params.contains_key("force"));
    }

    #[test]
    fn test_pool_operation_request_serialization() {
        let request = PoolOperationRequest {
            b_operation: "snapshot".to_string(),
            pool_name: "data-pool".to_string(),
            parameters: None,
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: PoolOperationRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(deserialized.b_operation, "snapshot");
        assert_eq!(deserialized.pool_name, "data-pool");
    }

    #[test]
    fn test_pool_operation_request_with_params() {
        let mut params = HashMap::new();
        params.insert("recursive".to_string(), serde_json::json!(true));
        params.insert("depth".to_string(), serde_json::json!(5));

        let request = PoolOperationRequest {
            b_operation: "list".to_string(),
            pool_name: "tank".to_string(),
            parameters: Some(params),
        };

        assert!(request.parameters.is_some());
        let params = request.parameters.unwrap();
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_pool_query_detailed_flag() {
        let query1 = PoolQuery {
            pool: Some("test".to_string()),
            detailed: Some(true),
        };

        let query2 = PoolQuery {
            pool: Some("test".to_string()),
            detailed: Some(false),
        };

        assert!(query1.detailed.unwrap());
        assert!(!query2.detailed.unwrap());
    }

    // ==================== HANDLER FUNCTION TESTS ====================

    #[tokio::test]
    async fn test_get_storage_info_empty_filter() {
        let params = StorageQuery {
            pool: Some("nonexistent".to_string()),
            detailed: None,
        };

        let result = get_storage_info(Query(params)).await;
        assert!(result.is_ok());

        let pools = result.unwrap().0;
        // Should be empty since no pools match "nonexistent"
        assert!(pools.is_empty());
    }

    #[tokio::test]
    async fn test_demo_confidence_levels() {
        let result = demo_confidence_levels().await;
        let response = result.0;

        assert!(response.success);
        assert!(!response.data.is_empty());
        assert!(response.confidence_score >= 0.0 && response.confidence_score <= 1.0);

        // Should have multiple confidence examples
        assert!(response.data.len() > 1);
    }

    #[tokio::test]
    async fn test_demo_suggested_actions_structure() {
        let result = demo_suggested_actions().await;
        let response = result.0;

        assert!(response.success);
        assert!(!response.data.is_empty());

        // Verify the first action has proper structure
        let first_capability = &response.data[0];
        assert!(!first_capability.category.is_empty());
        assert!(!first_capability.description.is_empty());
        assert!(!first_capability.automation_level.is_empty());
    }

    // ==================== INTEGRATION TESTS ====================

    #[test]
    fn test_create_routes_returns_router() {
        let router = create_routes();
        // Just verify router can be created without panic
        let _ = router;
    }

    #[test]
    fn test_create_handler_returns_router() {
        let router = create_handler();
        // Verify handler can be created without panic
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

        // Verify capacity math is consistent
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
}
