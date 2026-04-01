// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Compliance API handlers

use super::types::*;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::{DateTime, Utc};
use tracing::info;
use uuid::Uuid;
/// Get compliance dashboard
pub async fn get_compliance_dashboard(
    State(state): State<ComplianceState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let compliance = state.read().await;
    let report = compliance.generate_compliance_report();
    Ok(Json(serde_json::json!({
        "status": "success",
        "data": {
            "report": report,
            "total_audit_events": compliance.audit_logs.len(),
            "active_frameworks": compliance.regulatory_frameworks.len(),
        }
    })))
}

/// Get retention policies
pub async fn get_retention_policies(
    State(state): State<ComplianceState>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let compliance = state.read().await;
    let policies: Vec<&RetentionPolicy> = compliance.retention_policies.values().collect();
    Ok(Json(serde_json::json!({
        "status": "success",
        "data": {
            "policies": policies,
            "total": policies.len(),
        }
    })))
}

/// Create retention policy
pub async fn create_retention_policy(
    State(state): State<ComplianceState>,
    Json(mut policy): Json<RetentionPolicy>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    policy.id = Uuid::new_v4().to_string();
    policy.created_at = Utc::now();
    policy.updated_at = Utc::now();
    let mut compliance = state.write().await;
    compliance.add_retention_policy(policy.clone());

    Ok(Json(serde_json::json!({
        "status": "success",
        "data": {
            "policy": policy,
            "message": "Retention policy created successfully"
        }
    })))
}

/// Get audit logs
pub async fn get_audit_logs(
    State(state): State<ComplianceState>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let compliance = state.read().await;
    let limit = _params
        .get("limit")
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap_or(100);
    let logs: Vec<&AuditEvent> = compliance.audit_logs.iter().rev().take(limit).collect();

    Ok(Json(serde_json::json!({
        "status": "success",
        "data": {
            "logs": logs,
            "total": compliance.audit_logs.len(),
            "returned": logs.len(),
        }
    })))
}

/// Get compliance violations
pub async fn get_violations(
    State(state): State<ComplianceState>,
    Query(_params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let compliance = state.read().await;
    let status_filter = _params.get("status");
    let violations: Vec<&ComplianceViolation> = compliance
        .violations
        .iter()
        .filter(|v| {
            if let Some(status) = status_filter {
                matches!(
                    (&v.resolution_status, status.as_str()),
                    (ResolutionStatus::Open, "open")
                        | (ResolutionStatus::InProgress, "in_progress")
                        | (ResolutionStatus::Resolved, "resolved")
                        | (ResolutionStatus::Closed, "closed")
                        | (ResolutionStatus::Escalated, "escalated")
                )
            } else {
                true
            }
        })
        .collect();

    Ok(Json(serde_json::json!({
        "status": "success",
        "data": {
            "violations": violations,
            "total": violations.len(),
        }
    })))
}

/// Create compliance routes
pub fn create_compliance_routes() -> Router<ComplianceState> {
    Router::new()
        .route("/dashboard", get(get_compliance_dashboard))
        .route("/policies/retention", get(get_retention_policies))
        .route("/policies/retention", post(create_retention_policy))
        .route("/audit-logs", get(get_audit_logs))
        .route("/violations", get(get_violations))
}
/// Initialize compliance manager with default frameworks
pub fn initialize_compliance_manager() -> ComplianceManager {
    let mut manager = ComplianceManager::new();
    // Add default GDPR framework
    let gdpr_framework = RegulatoryFramework {
        id: "gdpr".to_string(),
        name: "General Data Protection Regulation".to_string(),
        framework_type: RegulatoryType::GDPR,
        required_controls: vec![
            ComplianceControl {
                id: "gdpr-consent".to_string(),
                name: "Data Subject Consent".to_string(),
                description: "Ensure valid consent for data processing".to_string(),
                control_type: ControlType::Preventive,
                implementation_status: ImplementationStatus::PartiallyImplemented,
                last_assessment: None,
                next_assessment_due: Some(Utc::now() + ChronoDuration::days(90)),
            },
            ComplianceControl {
                id: "gdpr-retention".to_string(),
                name: "Data Retention Limits".to_string(),
                description: "Implement data retention and deletion policies".to_string(),
                control_type: ControlType::Preventive,
                implementation_status: ImplementationStatus::FullyImplemented,
                last_assessment: Some(Utc::now() - ChronoDuration::days(30)),
                next_assessment_due: Some(Utc::now() + ChronoDuration::days(335)),
            },
        ],
        audit_frequency_days: 365,
        last_audit: None,
        compliance_status: ComplianceStatus::PartiallyCompliant,
    };

    manager.add_regulatory_framework(gdpr_framework);

    // Add default retention policy
    let default_retention = RetentionPolicy {
        id: "default-retention".to_string(),
        name: "Default Data Retention".to_string(),
        data_classification: DataClassification::Internal,
        retention_days: 2555,           // 7 years
        archive_after_days: Some(1095), // 3 years
        auto_delete: false,
        legal_hold: false,
        data_types: vec!["general".to_string(), "logs".to_string()],
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    manager.add_retention_policy(default_retention);

    info!("Compliance manager initialized with default frameworks");
    manager
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;
    use chrono::Duration as ChronoDuration;

    /// Creates  Test State
    fn create_test_state() -> ComplianceState {
        Arc::new(RwLock::new(ComplianceManager::new()))
    }

    /// Creates  Test Retention Policy
    fn create_test_retention_policy() -> RetentionPolicy {
        RetentionPolicy {
            id: "test-policy".to_string(),
            name: "Test Policy".to_string(),
            data_classification: DataClassification::Confidential,
            retention_days: 365,
            archive_after_days: Some(180),
            auto_delete: true,
            legal_hold: false,
            data_types: vec!["test_data".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_get_compliance_dashboard_success() {
        let state = create_test_state();
        let result = get_compliance_dashboard(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["status"], "success");
        assert!(json["data"].is_object());
    }

    #[tokio::test]
    async fn test_get_compliance_dashboard_has_report() {
        let state = create_test_state();
        let result = get_compliance_dashboard(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert!(json["data"]["report"].is_object());
    }

    #[tokio::test]
    async fn test_get_compliance_dashboard_has_metrics() {
        let state = create_test_state();
        let result = get_compliance_dashboard(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert!(json["data"]["total_audit_events"].is_number());
        assert!(json["data"]["active_frameworks"].is_number());
    }

    #[tokio::test]
    async fn test_get_retention_policies_empty() {
        let state = create_test_state();
        let result = get_retention_policies(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["status"], "success");
        assert_eq!(json["data"]["total"], 0);
    }

    #[tokio::test]
    async fn test_get_retention_policies_with_data() {
        let state = create_test_state();
        {
            let mut manager = state.write().await;
            manager.add_retention_policy(create_test_retention_policy());
        }
        let result = get_retention_policies(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["data"]["total"], 1);
    }

    #[tokio::test]
    async fn test_create_retention_policy_success() {
        let state = create_test_state();
        let policy = create_test_retention_policy();
        let result = create_retention_policy(State(state.clone()), Json(policy)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["status"], "success");
        assert!(json["data"]["policy"].is_object());
    }

    #[tokio::test]
    async fn test_create_retention_policy_generates_id() {
        let state = create_test_state();
        let mut policy = create_test_retention_policy();
        policy.id = String::new(); // Empty ID should be replaced
        let result = create_retention_policy(State(state), Json(policy)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert!(!json["data"]["policy"]["id"].as_str().expect("Operation failed").is_empty());
    }

    #[tokio::test]
    async fn test_create_retention_policy_sets_timestamps() {
        let state = create_test_state();
        let policy = create_test_retention_policy();
        let before = Utc::now();
        let result = create_retention_policy(State(state), Json(policy)).await;
        let after = Utc::now();
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        let created_at = json["data"]["policy"]["created_at"].as_str().expect("Operation failed");
        let created: DateTime<Utc> = created_at.parse().expect("Failed to parse value");
        assert!(created >= before && created <= after);
    }

    #[tokio::test]
    async fn test_get_audit_logs_empty() {
        let state = create_test_state();
        let params = Query(HashMap::new());
        let result = get_audit_logs(State(state), params).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["status"], "success");
        assert_eq!(json["data"]["total"], 0);
    }

    #[tokio::test]
    async fn test_get_audit_logs_with_limit() {
        let state = create_test_state();
        let mut params = HashMap::new();
        params.insert("limit".to_string(), "50".to_string());
        let result = get_audit_logs(State(state), Query(params)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_audit_logs_default_limit() {
        let state = create_test_state();
        let params = Query(HashMap::new());
        let result = get_audit_logs(State(state), params).await;
        assert!(result.is_ok());
        // Default limit is 100
    }

    #[tokio::test]
    async fn test_get_violations_empty() {
        let state = create_test_state();
        let params = Query(HashMap::new());
        let result = get_violations(State(state), params).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["status"], "success");
        assert_eq!(json["data"]["total"], 0);
    }

    #[tokio::test]
    async fn test_get_violations_with_status_filter() {
        let state = create_test_state();
        let mut params = HashMap::new();
        params.insert("status".to_string(), "open".to_string());
        let result = get_violations(State(state), Query(params)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_violations_various_statuses() {
        let statuses = vec!["open", "in_progress", "resolved", "closed", "escalated"];
        for status in statuses {
            let state = create_test_state();
            let mut params = HashMap::new();
            params.insert("status".to_string(), status.to_string());
            let result = get_violations(State(state), Query(params)).await;
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_initialize_compliance_manager_creates_manager() {
        let manager = initialize_compliance_manager();
        assert!(manager.regulatory_frameworks!debug_str.is_empty());
    }

    #[test]
    fn test_initialize_compliance_manager_has_gdpr() {
        let manager = initialize_compliance_manager();
        assert!(manager.regulatory_frameworks.contains_key("gdpr"));
    }

    #[test]
    fn test_initialize_compliance_manager_has_default_retention() {
        let manager = initialize_compliance_manager();
        assert!(manager.retention_policies.contains_key("default-retention"));
    }

    #[test]
    fn test_initialize_compliance_manager_gdpr_controls() {
        let manager = initialize_compliance_manager();
        let gdpr = &manager.regulatory_frameworks["gdpr"];
        assert_eq!(gdpr.required_controls.len(), 2);
    }

    #[test]
    fn test_initialize_compliance_manager_retention_policy_values() {
        let manager = initialize_compliance_manager();
        let policy = &manager.retention_policies["default-retention"];
        assert_eq!(policy.retention_days, 2555); // 7 years
        assert_eq!(policy.archive_after_days, Some(1095)); // 3 years
    }

    #[test]
    fn test_create_compliance_routes_returns_router() {
        let _router = create_compliance_routes();
        // Just verify it compiles and returns
    }

    #[tokio::test]
    async fn test_retention_policy_data_types() {
        let policy = create_test_retention_policy();
        assert_eq!(policy.data_types.len(), 1);
        assert_eq!(policy.data_types[0], "test_data");
    }

    #[tokio::test]
    async fn test_retention_policy_auto_delete() {
        let mut policy = create_test_retention_policy();
        policy.auto_delete = true;
        assert!(policy.auto_delete);
        
        policy.auto_delete = false;
        assert!(!policy.auto_delete);
    }

    #[tokio::test]
    async fn test_retention_policy_legal_hold() {
        let mut policy = create_test_retention_policy();
        policy.legal_hold = true;
        assert!(policy.legal_hold);
        
        policy.legal_hold = false;
        assert!(!policy.legal_hold);
    }

    #[test]
    fn test_compliance_manager_multiple_policies() {
        let mut manager = ComplianceManager::new();
        let policy1 = create_test_retention_policy();
        let mut policy2 = create_test_retention_policy();
        policy2.id = "policy2".to_string();
        
        manager.add_retention_policy(policy1);
        manager.add_retention_policy(policy2);
        
        assert_eq!(manager.retention_policies.len(), 2);
    }

    #[tokio::test]
    async fn test_get_retention_policies_returns_all() {
        let state = create_test_state();
        {
            let mut manager = state.write().await;
            for i in 0..5 {
                let mut policy = create_test_retention_policy();
                policy.id = format!("policy-{}", i);
                manager.add_retention_policy(policy);
            }
        }
        let result = get_retention_policies(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["data"]["total"], 5);
    }

    #[tokio::test]
    async fn test_create_retention_policy_preserves_data() {
        let state = create_test_state();
        let mut policy = create_test_retention_policy();
        policy.name = "Unique Policy Name".to_string();
        policy.retention_days = 999;
        
        let result = create_retention_policy(State(state), Json(policy)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert_eq!(json["data"]["policy"]["name"], "Unique Policy Name");
        assert_eq!(json["data"]["policy"]["retention_days"], 999);
    }

    #[tokio::test]
    async fn test_get_audit_logs_invalid_limit() {
        let state = create_test_state();
        let mut params = HashMap::new();
        params.insert("limit".to_string(), "invalid".to_string());
        let result = get_audit_logs(State(state), Query(params)).await;
        // Should use default limit (100) when parse fails
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_compliance_dashboard_with_frameworks() {
        let state = create_test_state();
        {
            let mut manager = state.write().await;
            let manager_init = initialize_compliance_manager();
            *manager = manager_init;
        }
        let result = get_compliance_dashboard(State(state)).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert!(json["data"]["active_frameworks"].as_u64().expect("Operation failed") > 0);
    }

    #[test]
    fn test_initialize_compliance_manager_audit_frequency() {
        let manager = initialize_compliance_manager();
        let gdpr = &manager.regulatory_frameworks["gdpr"];
        assert_eq!(gdpr.audit_frequency_days, 365);
    }

    #[test]
    fn test_initialize_compliance_manager_compliance_status() {
        let manager = initialize_compliance_manager();
        let gdpr = &manager.regulatory_frameworks["gdpr"];
        assert!(matches!(gdpr.compliance_status, ComplianceStatus::PartiallyCompliant));
    }

    #[test]
    fn test_retention_policy_classification() {
        let policy = create_test_retention_policy();
        assert!(matches!(policy.data_classification, DataClassification::Confidential));
    }

    #[tokio::test]
    async fn test_get_violations_no_filter() {
        let state = create_test_state();
        let params = Query(HashMap::new());
        let result = get_violations(State(state), params).await;
        assert!(result.is_ok());
        let json = result.expect("Operation failed").0;
        assert!(json["data"]["violations"].is_array());
    }
}
