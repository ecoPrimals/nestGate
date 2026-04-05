// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **AI FIRST EXAMPLE - COVERAGE BOOST**
//!
//! Comprehensive tests to boost coverage for ai_first_example handlers.

use super::*;
use axum::extract::{Json, Path};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_pool_info_main_pool() {
        let result = get_pool_info(Path("main-pool".to_string())).await;
        let response = result.0;

        assert!(response.success);
        assert!(response.data.is_some());

        let pool = response.data.unwrap();
        assert_eq!(pool.pool_name, "main-pool");
        assert_eq!(pool.total_size_gb, 1000);
        assert_eq!(pool.used_size_gb, 650);
        assert_eq!(pool.available_size_gb, 350);
        assert_eq!(pool.health_status, "ONLINE");
        assert!(pool.last_scrub.is_some());
    }

    #[tokio::test]
    async fn test_get_pool_info_backup_pool() {
        let result = get_pool_info(Path("backup-pool".to_string())).await;
        let response = result.0;

        assert!(response.success);
        assert!(response.data.is_some());

        let pool = response.data.unwrap();
        assert_eq!(pool.pool_name, "backup-pool");
        assert_eq!(pool.health_status, "DEGRADED");
        // Confidence is lower for degraded pools
        assert!(response.confidence_score < 0.95);
    }

    #[tokio::test]
    async fn test_get_pool_info_nonexistent_pool() {
        let result = get_pool_info(Path("nonexistent-pool".to_string())).await;
        let response = result.0;

        // Non-existent pool returns None with 0.0 confidence
        assert!(response.data.is_none());
        assert_eq!(response.confidence_score, 0.0);
    }

    #[tokio::test]
    async fn test_execute_storage_operation_scrub() {
        let request = PoolOperationRequest {
            pool_name: "test-pool".to_string(),
            b_operation: "scrub".to_string(),
            parameters: None,
        };

        let result = execute_storage_operation(Json(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert!(response.success);
        // Scrub operation returns "Operation completed with suggested actions"
        assert!(response.message.contains("Operation completed"));
        assert_eq!(response.confidence_score, 0.85); // Default for ai_response_with_actions
    }

    #[tokio::test]
    async fn test_execute_storage_operation_snapshot() {
        let request = PoolOperationRequest {
            pool_name: "test-pool".to_string(),
            b_operation: "snapshot".to_string(),
            parameters: None,
        };

        let result = execute_storage_operation(Json(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert!(response.success);
        assert!(response.confidence_score >= 0.9);
    }

    #[tokio::test]
    async fn test_execute_storage_operation_export() {
        let request = PoolOperationRequest {
            pool_name: "test-pool".to_string(),
            b_operation: "export".to_string(),
            parameters: None,
        };

        let result = execute_storage_operation(Json(request)).await;
        assert!(result.is_ok());

        let response = result.unwrap().0;
        assert!(response.success);
    }

    #[tokio::test]
    async fn test_execute_storage_operation_invalid() {
        let request = PoolOperationRequest {
            pool_name: "test-pool".to_string(),
            b_operation: "invalid-operation".to_string(),
            parameters: None,
        };

        let result = execute_storage_operation(Json(request)).await;
        // Invalid operations return OK but with very low confidence
        assert!(result.is_ok());
        let response = result.unwrap().0;
        assert_eq!(response.confidence_score, 0.1);
    }

    #[tokio::test]
    async fn test_execute_storage_operation_with_parameters() {
        let mut params = std::collections::HashMap::new();
        params.insert("force".to_string(), serde_json::json!(true));

        let request = PoolOperationRequest {
            pool_name: "test-pool".to_string(),
            b_operation: "scrub".to_string(),
            parameters: Some(params),
        };

        let result = execute_storage_operation(Json(request)).await;
        assert!(result.is_ok());
    }

    #[test]
    fn test_ai_pool_operation() {
        let request = PoolOperationRequest {
            pool_name: "main-pool".to_string(),
            b_operation: "optimize".to_string(),
            parameters: None,
        };

        let result = ai_pool_operation(Json(request));
        let response = result.0;

        assert!(response.success);
        assert!(response.data.contains("Executed"));
        assert!(response.data.contains("optimize"));
        assert!(response.data.contains("main-pool"));
        assert_eq!(response.confidence_score, 0.92);
    }

    #[test]
    fn test_ai_pool_status() {
        let result = ai_pool_status(Path("test-pool".to_string()));
        let response = result.0;

        assert!(response.success);
        assert_eq!(response.data.pool_name, "test-pool");
        assert_eq!(response.data.health_status, "Healthy");
        assert_eq!(response.confidence_score, 0.95);
    }

    #[test]
    fn test_ai_success_with_confidence_high() {
        let response = ai_success_with_confidence("test data".to_string(), 0.95);
        assert!(response.success);
        assert_eq!(response.data, "test data");
        assert_eq!(response.confidence_score, 0.95);
    }

    #[test]
    fn test_ai_success_with_confidence_low() {
        let response = ai_success_with_confidence(42, 0.25);
        assert!(response.success);
        assert_eq!(response.data, 42);
        assert_eq!(response.confidence_score, 0.25);
    }

    #[test]
    fn test_ai_response_with_actions() {
        let actions = vec![
            SuggestedAction {
                action_id: "test-action-1".to_string(),
                action_type: ActionType::Optimize,
                description: "Test optimization".to_string(),
                confidence: 0.9,
                parameters: std::collections::HashMap::new(),
                priority: 1,
                estimated_duration_ms: Some(1000),
                dependencies: vec![],
            },
            SuggestedAction {
                action_id: "test-action-2".to_string(),
                action_type: ActionType::Monitor,
                description: "Test monitoring".to_string(),
                confidence: 0.85,
                parameters: std::collections::HashMap::new(),
                priority: 2,
                estimated_duration_ms: None,
                dependencies: vec!["test-action-1".to_string()],
            },
        ];

        let response = ai_response_with_actions("operation complete".to_string(), actions);
        assert!(response.success);
        assert_eq!(response.data, "operation complete");
        assert_eq!(response.confidence_score, 0.85);
    }

    #[test]
    fn test_action_type_variants() {
        // Test all ActionType variants for serialization/deserialization
        let optimize = ActionType::Optimize;
        let monitor = ActionType::Monitor;
        let alert = ActionType::Alert;
        let backup = ActionType::Backup;
        let scale = ActionType::Scale;
        let repair = ActionType::Repair;

        let optimize_json = serde_json::to_string(&optimize).unwrap();
        let monitor_json = serde_json::to_string(&monitor).unwrap();
        let alert_json = serde_json::to_string(&alert).unwrap();
        let backup_json = serde_json::to_string(&backup).unwrap();
        let scale_json = serde_json::to_string(&scale).unwrap();
        let repair_json = serde_json::to_string(&repair).unwrap();

        assert!(optimize_json.contains("Optimize"));
        assert!(monitor_json.contains("Monitor"));
        assert!(alert_json.contains("Alert"));
        assert!(backup_json.contains("Backup"));
        assert!(scale_json.contains("Scale"));
        assert!(repair_json.contains("Repair"));
    }

    #[test]
    fn test_suggested_action_with_dependencies() {
        let action = SuggestedAction {
            action_id: "dependent-action".to_string(),
            action_type: ActionType::Scale,
            description: "Scale based on monitoring".to_string(),
            confidence: 0.88,
            parameters: std::collections::HashMap::new(),
            priority: 3,
            estimated_duration_ms: Some(5000),
            dependencies: vec!["monitor-action".to_string(), "analyze-action".to_string()],
        };

        assert_eq!(action.dependencies.len(), 2);
        assert!(action.dependencies.contains(&"monitor-action".to_string()));
        assert!(action.dependencies.contains(&"analyze-action".to_string()));
    }

    #[test]
    fn test_suggested_action_with_parameters() {
        let mut params = std::collections::HashMap::new();
        params.insert("threshold".to_string(), serde_json::json!(80));
        params.insert("interval".to_string(), serde_json::json!(60000));
        params.insert("enabled".to_string(), serde_json::json!(true));

        let action = SuggestedAction {
            action_id: "config-action".to_string(),
            action_type: ActionType::Optimize,
            description: "Configure optimization".to_string(),
            confidence: 0.92,
            parameters: params,
            priority: 1,
            estimated_duration_ms: Some(2000),
            dependencies: vec![],
        };

        assert_eq!(action.parameters.len(), 3);
        assert!(action.parameters.contains_key("threshold"));
        assert!(action.parameters.contains_key("interval"));
        assert!(action.parameters.contains_key("enabled"));
    }

    #[test]
    fn test_pool_operation_request_scrub() {
        let request = PoolOperationRequest {
            pool_name: "data-pool".to_string(),
            b_operation: "scrub".to_string(),
            parameters: None,
        };

        assert_eq!(request.pool_name, "data-pool");
        assert_eq!(request.b_operation, "scrub");
        assert!(request.parameters.is_none());
    }

    #[test]
    fn test_pool_operation_request_with_parameters() {
        let mut params = std::collections::HashMap::new();
        params.insert("force".to_string(), serde_json::json!(true));

        let request = PoolOperationRequest {
            pool_name: "test-pool".to_string(),
            b_operation: "export".to_string(),
            parameters: Some(params),
        };

        assert!(request.parameters.is_some());
    }

    #[test]
    fn test_pool_info_serialization() {
        let pool = PoolInfo {
            pool_name: "main-pool".to_string(),
            total_size_gb: 2000,
            used_size_gb: 1200,
            available_size_gb: 800,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-11-22T12:00:00Z".to_string()),
        };

        let json = serde_json::to_string(&pool).unwrap();
        let deserialized: PoolInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(pool.pool_name, deserialized.pool_name);
        assert_eq!(pool.total_size_gb, deserialized.total_size_gb);
        assert_eq!(pool.health_status, deserialized.health_status);
    }

    #[test]
    fn test_pool_info_without_scrub() {
        let pool = PoolInfo {
            pool_name: "new-pool".to_string(),
            total_size_gb: 500,
            used_size_gb: 0,
            available_size_gb: 500,
            health_status: "ONLINE".to_string(),
            last_scrub: None,
        };

        assert!(pool.last_scrub.is_none());
    }

    #[test]
    fn test_storage_info() {
        let storage = StorageInfo {
            pool_name: "storage-pool".to_string(),
            total_size_gb: 5000,
            used_size_gb: 3000,
            available_size_gb: 2000,
            health_status: "ONLINE".to_string(),
            last_scrub: Some("2025-11-22T12:00:00Z".to_string()),
        };

        assert_eq!(storage.total_size_gb, 5000);
        assert_eq!(storage.used_size_gb, 3000);
        assert_eq!(storage.available_size_gb, 2000);
    }

    #[test]
    fn test_ai_first_response_ext_with_confidence() {
        let response = AIFirstResponse {
            data: "test".to_string(),
            success: true,
            message: "test message".to_string(),
            confidence_score: 0.5,
        };

        let updated = response.with_confidence(0.95);
        assert_eq!(updated.confidence_score, 0.95);
    }

    #[test]
    fn test_ai_first_response_ext_clamp_high() {
        let response = AIFirstResponse {
            data: 42,
            success: true,
            message: "test".to_string(),
            confidence_score: 0.5,
        };

        let updated = response.with_confidence(1.5); // Should clamp to 1.0
        assert_eq!(updated.confidence_score, 1.0);
    }

    #[test]
    fn test_ai_first_response_ext_clamp_low() {
        let response = AIFirstResponse {
            data: vec![1, 2, 3],
            success: true,
            message: "test".to_string(),
            confidence_score: 0.5,
        };

        let updated = response.with_confidence(-0.5); // Should clamp to 0.0
        assert_eq!(updated.confidence_score, 0.0);
    }

    #[test]
    fn test_optimization_scenario() {
        let scenario = OptimizationScenario {
            scenario: "caching-optimization".to_string(),
            confidence: 0.88,
            description: "Enable caching for frequently accessed data".to_string(),
        };

        assert!(scenario.confidence > 0.0 && scenario.confidence <= 1.0);
        assert!(!scenario.description.is_empty());
    }

    #[test]
    fn test_automation_capability() {
        let capability = AutomationCapability {
            category: "Performance".to_string(),
            description: "Automatic performance tuning".to_string(),
            automation_level: "Full".to_string(),
        };

        assert_eq!(capability.category, "Performance");
        assert_eq!(capability.automation_level, "Full");
    }

    #[tokio::test]
    async fn test_example_handler() {
        let result = example_handler().await;
        let response = result.0;

        assert!(response.success);
        assert_eq!(response.data, "AI First Example");
        assert_eq!(response.message, "Example working");
        assert_eq!(response.confidence_score, 0.95);
    }

    #[test]
    fn test_create_handler() {
        let router = create_handler();
        // Just verify it creates without panic
        assert!(format!("{router:?}").contains("Router"));
    }
}
