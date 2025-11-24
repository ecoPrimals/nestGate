//
// Compliance API handlers
//

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use std::collections::HashMap;
use uuid::Uuid;

use super::manager::ComplianceState;
use super::types::{AuditEvent, ComplianceViolation, ResolutionStatus, RetentionPolicy};

#[cfg(test)]
#[path = "handlers_tests.rs"]
mod handlers_tests;

/// Get compliance dashboard
///
/// # Errors
///
/// Returns `StatusCode` error if the compliance state cannot be accessed.
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
///
/// # Errors
///
/// Returns `StatusCode` error if the retention policies cannot be retrieved.
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
///
/// # Errors
///
/// Returns `StatusCode` error if the policy cannot be created or state cannot be accessed.
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
///
/// # Errors
///
/// Returns `StatusCode` error if audit logs cannot be retrieved or state cannot be accessed.
pub async fn get_audit_logs(
    State(state): State<ComplianceState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let compliance = state.read().await;
    let limit = params
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
///
/// # Errors
///
/// Returns `StatusCode` error if violations cannot be retrieved or state cannot be accessed.
pub async fn get_violations(
    State(state): State<ComplianceState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let compliance = state.read().await;
    let status_filter = params.get("status");
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
