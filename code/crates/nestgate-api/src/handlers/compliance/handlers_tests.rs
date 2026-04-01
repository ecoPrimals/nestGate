// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for compliance API handlers

use super::*;
use crate::handlers::compliance::manager::{ComplianceManager, ComplianceState};
use crate::handlers::compliance::types::*;
use axum::Json;
use axum::extract::{Query, State};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Helper to create test compliance state
fn create_test_state() -> ComplianceState {
    Arc::new(RwLock::new(ComplianceManager::new()))
}

/// Helper to create initialized test state with data
fn create_initialized_state() -> ComplianceState {
    let manager = crate::handlers::compliance::manager::initialize_compliance_manager();
    Arc::new(RwLock::new(manager))
}

#[tokio::test]
async fn test_get_compliance_dashboard_empty() {
    let state = create_test_state();
    let result = get_compliance_dashboard(State(state)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert!(json["data"].is_object());
}

#[tokio::test]
async fn test_get_compliance_dashboard_with_data() {
    let state = create_initialized_state();
    let result = get_compliance_dashboard(State(state)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert!(json["data"]["total_audit_events"].is_number());
    assert!(json["data"]["active_frameworks"].is_number());
}

#[tokio::test]
async fn test_get_retention_policies_empty() {
    let state = create_test_state();
    let result = get_retention_policies(State(state)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["total"], 0);
    assert!(json["data"]["policies"].is_array());
}

#[tokio::test]
async fn test_get_retention_policies_with_policies() {
    let state = create_initialized_state();
    let result = get_retention_policies(State(state)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    let total = json["data"]["total"].as_u64().unwrap();
    assert!(total > 0);
}

#[tokio::test]
async fn test_create_retention_policy_valid() {
    let state = create_test_state();

    let policy = RetentionPolicy {
        id: String::new(), // Will be generated
        name: "Test Policy".to_string(),
        data_classification: DataClassification::Internal,
        retention_days: 365,
        archive_after_days: Some(180),
        auto_delete: true,
        legal_hold: false,
        data_types: vec!["test-data".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let result = create_retention_policy(State(state.clone()), Json(policy)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert!(!json["data"]["policy"]["id"].as_str().unwrap().is_empty());
    assert_eq!(json["data"]["policy"]["name"], "Test Policy");

    // Verify policy was added to state
    let compliance = state.read().await;
    assert_eq!(compliance.retention_policies.len(), 1);
}

#[tokio::test]
async fn test_create_multiple_retention_policies() {
    let state = create_test_state();

    for i in 1..=3 {
        let policy = RetentionPolicy {
            id: String::new(),
            name: format!("Policy {i}"),
            data_classification: DataClassification::Internal,
            retention_days: i * 365,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: false,
            data_types: vec![format!("type-{}", i)],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = create_retention_policy(State(state.clone()), Json(policy)).await;
        assert!(result.is_ok());
    }

    let compliance = state.read().await;
    assert_eq!(compliance.retention_policies.len(), 3);
}

#[tokio::test]
async fn test_create_retention_policy_with_legal_hold() {
    let state = create_test_state();

    let policy = RetentionPolicy {
        id: String::new(),
        name: "Legal Hold Policy".to_string(),
        data_classification: DataClassification::Restricted,
        retention_days: 3650, // 10 years
        archive_after_days: None,
        auto_delete: false,
        legal_hold: true,
        data_types: vec!["legal-documents".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let result = create_retention_policy(State(state.clone()), Json(policy)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["data"]["policy"]["legal_hold"], true);
    assert_eq!(json["data"]["policy"]["auto_delete"], false);
}

#[tokio::test]
async fn test_get_violations_empty() {
    let state = create_test_state();
    let params = HashMap::new();
    let result = get_violations(State(state), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["total"], 0);
    assert!(json["data"]["violations"].is_array());
}

#[tokio::test]
async fn test_get_violations_with_data() {
    let state = create_test_state();

    // Add a violation
    {
        let mut compliance = state.write().await;
        let violation = ComplianceViolation {
            id: "test-violation".to_string(),
            timestamp: chrono::Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::High,
            description: "Test violation".to_string(),
            path: "/test/path".to_string(),
            framework: "gdpr".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };
        compliance.record_violation(violation);
    }

    let params = HashMap::new();
    let result = get_violations(State(state), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["total"], 1);
}

#[tokio::test]
async fn test_get_audit_logs_empty() {
    let state = create_test_state();
    let params = HashMap::new();
    let result = get_audit_logs(State(state), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["total"], 0);
    assert!(json["data"]["logs"].is_array());
}

#[tokio::test]
async fn test_get_audit_logs_with_entries() {
    let state = create_test_state();

    // Add audit events
    {
        let mut compliance = state.write().await;
        for i in 1..=5 {
            let event = AuditEvent {
                id: format!("event-{i}"),
                timestamp: chrono::Utc::now(),
                event_type: AuditEventType::DataAccess,
                user_id: Some(format!("user-{i}")),
                path: format!("/path/{i}"),
                action: "READ".to_string(),
                result: AuditResult::Success,
                details: HashMap::new(),
                source_ip: Some("192.168.1.1".to_string()),
                user_agent: None,
            };
            compliance.log_audit_event(event);
        }
    }

    let params = HashMap::new();
    let result = get_audit_logs(State(state), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert_eq!(json["data"]["total"], 5);
}

#[tokio::test]
async fn test_compliance_routes_creation() {
    let _state = create_test_state();
    let _router = create_compliance_routes();

    // Router creation succeeds - test passes if no panic occurs
}

#[tokio::test]
async fn test_retention_policy_timestamps() {
    let state = create_test_state();

    let before = chrono::Utc::now();

    let policy = RetentionPolicy {
        id: String::new(),
        name: "Timestamp Test".to_string(),
        data_classification: DataClassification::Internal,
        retention_days: 365,
        archive_after_days: None,
        auto_delete: true,
        legal_hold: false,
        data_types: vec!["test".to_string()],
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let result = create_retention_policy(State(state), Json(policy)).await;

    let after = chrono::Utc::now();

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    // Timestamps should be set and within test execution window
    let created_str = json["data"]["policy"]["created_at"].as_str().unwrap();
    let created_at = chrono::DateTime::parse_from_rfc3339(created_str).unwrap();

    assert!(created_at.timestamp() >= before.timestamp());
    assert!(created_at.timestamp() <= after.timestamp());
}

#[tokio::test]
async fn test_concurrent_policy_creation() {
    let state = create_test_state();

    let mut handles = vec![];

    for i in 1..=10 {
        let state_clone = state.clone();
        let handle = tokio::spawn(async move {
            let policy = RetentionPolicy {
                id: String::new(),
                name: format!("Concurrent Policy {i}"),
                data_classification: DataClassification::Internal,
                retention_days: 365,
                archive_after_days: None,
                auto_delete: true,
                legal_hold: false,
                data_types: vec![format!("concurrent-{}", i)],
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            create_retention_policy(State(state_clone), Json(policy)).await
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.await.unwrap();
        assert!(result.is_ok());
    }

    let compliance = state.read().await;
    assert_eq!(compliance.retention_policies.len(), 10);
}

#[tokio::test]
async fn test_policy_with_all_data_classifications() {
    let state = create_test_state();

    let classifications = vec![
        DataClassification::Public,
        DataClassification::Internal,
        DataClassification::Confidential,
        DataClassification::Restricted,
        DataClassification::TopSecret,
    ];

    for classification in classifications {
        let policy = RetentionPolicy {
            id: String::new(),
            name: format!("Policy {classification:?}"),
            data_classification: classification,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: true,
            legal_hold: false,
            data_types: vec!["test".to_string()],
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = create_retention_policy(State(state.clone()), Json(policy)).await;
        assert!(result.is_ok());
    }

    let compliance = state.read().await;
    assert_eq!(compliance.retention_policies.len(), 5);
}

#[tokio::test]
async fn test_dashboard_response_structure() {
    let state = create_initialized_state();
    let result = get_compliance_dashboard(State(state)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    // Verify response structure
    assert!(json.is_object());
    assert!(json["status"].is_string());
    assert!(json["data"].is_object());
    assert!(json["data"]["report"].is_object());
    assert!(json["data"]["total_audit_events"].is_number());
    assert!(json["data"]["active_frameworks"].is_number());
}

#[tokio::test]
async fn test_violations_response_structure() {
    let state = create_test_state();
    let params = HashMap::new();
    let result = get_violations(State(state), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert!(json["status"].is_string());
    assert!(json["data"]["violations"].is_array());
    assert!(json["data"]["total"].is_number());
}

#[tokio::test]
async fn test_audit_logs_response_structure() {
    let state = create_test_state();
    let params = HashMap::new();
    let result = get_audit_logs(State(state), Query(params)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert!(json["status"].is_string());
    assert!(json["data"]["logs"].is_array());
    assert!(json["data"]["total"].is_number());
}

#[tokio::test]
async fn test_retention_policies_response_structure() {
    let state = create_test_state();
    let result = get_retention_policies(State(state)).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    let json = response.0;

    assert_eq!(json["status"], "success");
    assert!(json["data"]["policies"].is_array());
    assert!(json["data"]["total"].is_number());
}
