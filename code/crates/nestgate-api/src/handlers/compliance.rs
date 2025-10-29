//
// This module implements comprehensive compliance features for enterprise storage systems
// including data retention policies, access control compliance, audit logging, and
// regulatory compliance (GDPR, HIPAA, SOX, etc.).

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Duration as ChronoDuration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tracing::{info, warn};
// Removed unused tracing import
use uuid::Uuid;

/// Compliance manager state
pub type ComplianceState = Arc<RwLock<ComplianceManager>>;
/// Compliance manager
#[derive(Debug, Clone, Default)]
pub struct ComplianceManager {
    /// Data retention policies
    pub retention_policies: HashMap<String, RetentionPolicy>,
    /// Access control policies
    pub access_policies: HashMap<String, AccessPolicy>,
    /// Audit logs
    pub audit_logs: Vec<AuditEvent>,
    /// Regulatory frameworks
    pub regulatory_frameworks: HashMap<String, RegulatoryFramework>,
    /// Compliance violations
    pub violations: Vec<ComplianceViolation>,
}
/// Data retention policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Data classification
    pub data_classification: DataClassification,
    /// Retention period in days
    pub retention_days: u32,
    /// Archive after days
    pub archive_after_days: Option<u32>,
    /// Auto-delete after retention
    pub auto_delete: bool,
    /// Legal hold override
    pub legal_hold: bool,
    /// Applicable data types
    pub data_types: Vec<String>,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
    /// Updated timestamp
    pub updated_at: DateTime<Utc>,
}
/// Access control policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Required permissions
    pub required_permissions: Vec<String>,
    /// Minimum clearance level
    pub min_clearance_level: u8,
    /// Access time restrictions
    pub time_restrictions: Vec<TimeRestriction>,
    /// Location restrictions
    pub location_restrictions: Vec<String>,
    /// MFA required
    pub mfa_required: bool,
    /// Audit access
    pub audit_access: bool,
    /// Created timestamp
    pub created_at: DateTime<Utc>,
}
/// Time restriction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    /// Day of week (0-6, Sunday = 0)
    pub day_of_week: u8,
    /// Start time (24-hour format)
    pub start_time: String,
    /// End time (24-hour format)
    pub end_time: String,
    /// Timezone
    pub timezone: String,
}
/// Data classification levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataClassification {
    /// Public data
    Public,
    /// Internal data
    Internal,
    /// Confidential data
    Confidential,
    /// Restricted data
    Restricted,
    /// Top secret data
    TopSecret,
}
/// Audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    /// Event ID
    pub id: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event type
    pub event_type: AuditEventType,
    /// User ID
    pub user_id: Option<String>,
    /// Resource accessed
    pub path: String,
    /// Action performed
    pub action: String,
    /// Result status
    pub result: AuditResult,
    /// Additional details
    pub details: HashMap<String, String>,
    /// Source IP address
    pub source_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
}
/// Audit event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    /// Data access
    DataAccess,
    /// Data modification
    DataModification,
    /// Data deletion
    DataDeletion,
    /// Policy change
    PolicyChange,
    /// Authentication
    Authentication,
    /// Authorization
    Authorization,
    /// System configuration
    SystemConfiguration,
    /// Compliance violation
    ComplianceViolation,
}

impl std::fmt::Display for AuditEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataAccess => write!(f, "Data Access"),
            Self::DataModification => write!(f, "Data Modification"),
            Self::DataDeletion => write!(f, "Data Deletion"),
            Self::PolicyChange => write!(f, "Policy Change"),
            Self::Authentication => write!(f, "Authentication"),
            Self::Authorization => write!(f, "Authorization"),
            Self::SystemConfiguration => write!(f, "System Configuration"),
            Self::ComplianceViolation => write!(f, "Compliance Violation"),
        }
    }
}

/// Audit result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditResult {
    /// Success
    Success,
    /// Failure
    Failure,
    /// Unauthorized
    Unauthorized,
    /// Forbidden
    Forbidden,
    /// Error
    Error,
}
/// Regulatory framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegulatoryFramework {
    /// Framework ID
    pub id: String,
    /// Framework name
    pub name: String,
    /// Framework type
    pub framework_type: RegulatoryType,
    /// Required controls
    pub required_controls: Vec<ComplianceControl>,
    /// Audit frequency
    pub audit_frequency_days: u32,
    /// Last audit date
    pub last_audit: Option<DateTime<Utc>>,
    /// Compliance status
    pub compliance_status: ComplianceStatus,
}
/// Regulatory framework types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RegulatoryType {
    /// GDPR (General Data Protection Regulation)
    GDPR,
    /// HIPAA (Health Insurance Portability and Accountability Act)
    HIPAA,
    /// SOX (Sarbanes-Oxley Act)
    SOX,
    /// PCI DSS (Payment Card Industry Data Security Standard)
    PCIDSS,
    /// ISO 27001
    ISO27001,
    /// `FedRAMP`
    FedRAMP,
    /// Custom framework
    Custom(String),
}
/// Compliance control
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceControl {
    /// Control ID
    pub id: String,
    /// Control name
    pub name: String,
    /// Control description
    pub description: String,
    /// Control type
    pub control_type: ControlType,
    /// Implementation status
    pub implementation_status: ImplementationStatus,
    /// Last assessment date
    pub last_assessment: Option<DateTime<Utc>>,
    /// Next assessment due
    pub next_assessment_due: Option<DateTime<Utc>>,
}
/// Control types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ControlType {
    /// Preventive control
    Preventive,
    /// Detective control
    Detective,
    /// Corrective control
    Corrective,
    /// Compensating control
    Compensating,
}
/// Implementation status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationStatus {
    /// Not implemented
    NotImplemented,
    /// Partially implemented
    PartiallyImplemented,
    /// Fully implemented
    FullyImplemented,
    /// Under review
    UnderReview,
    /// Non-compliant
    NonCompliant,
}
/// Compliance status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ComplianceStatus {
    /// Compliant
    Compliant,
    /// Non-compliant
    NonCompliant,
    /// Partially compliant
    PartiallyCompliant,
    /// Under assessment
    UnderAssessment,
    /// Unknown
    Unknown,
}
/// Compliance violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceViolation {
    /// Violation ID
    pub id: String,
    /// Violation timestamp
    pub timestamp: DateTime<Utc>,
    /// Violation type
    pub violation_type: ViolationType,
    /// Severity level
    pub severity: ViolationSeverity,
    /// Description
    pub description: String,
    /// Affected resource
    pub path: String,
    /// Regulatory framework
    pub framework: String,
    /// Resolution status
    pub resolution_status: ResolutionStatus,
    /// Resolution deadline
    pub resolution_deadline: Option<DateTime<Utc>>,
    /// Assigned to
    pub assigned_to: Option<String>,
}
/// Violation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    /// Data retention violation
    DataRetention,
    /// Access control violation
    AccessControl,
    /// Encryption violation
    Encryption,
    /// Audit logging violation
    AuditLogging,
    /// Data residency violation
    DataResidency,
    /// Backup violation
    Backup,
    /// Documentation violation
    Documentation,
}

impl std::fmt::Display for ViolationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DataRetention => write!(f, "Data Retention"),
            Self::AccessControl => write!(f, "Access Control"),
            Self::Encryption => write!(f, "Encryption"),
            Self::AuditLogging => write!(f, "Audit Logging"),
            Self::DataResidency => write!(f, "Data Residency"),
            Self::Backup => write!(f, "Backup"),
            Self::Documentation => write!(f, "Documentation"),
        }
    }
}

/// Violation severity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}
/// Resolution status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionStatus {
    /// Open
    Open,
    /// In progress
    InProgress,
    /// Resolved
    Resolved,
    /// Closed
    Closed,
    /// Escalated
    Escalated,
}
impl ComplianceManager {
    /// Create new compliance manager
    #[must_use]
    pub fn new() -> Self {
        Self {
            retention_policies: HashMap::new(),
            access_policies: HashMap::new(),
            audit_logs: Vec::new(),
            regulatory_frameworks: HashMap::new(),
            violations: Vec::new(),
        }
    }

    /// Add retention policy
    pub fn add_retention_policy(&mut self, policy: RetentionPolicy) {
        info!("Adding retention policy: {}", policy.name);
        self.retention_policies.insert(policy.id.clone(), policy);
    }

    /// Add access policy
    pub fn add_access_policy(&mut self, policy: AccessPolicy) {
        info!("Adding access policy: {}", policy.name);
        self.access_policies.insert(policy.id.clone(), policy);
    }

    /// Log audit event
    pub fn log_audit_event(&mut self, event: AuditEvent) {
        info!(
            "Logging audit event: {} - {}",
            event.event_type, event.action
        );
        self.audit_logs.push(event);
    }

    /// Add regulatory framework
    pub fn add_regulatory_framework(&mut self, framework: RegulatoryFramework) {
        info!("Adding regulatory framework: {}", framework.name);
        self.regulatory_frameworks
            .insert(framework.id.clone(), framework);
    }

    /// Record compliance violation
    pub fn record_violation(&mut self, violation: ComplianceViolation) {
        warn!(
            "Recording compliance violation: {} - {}",
            violation.violation_type, violation.description
        );
        self.violations.push(violation);
    }

    /// Check data retention compliance
    #[must_use]
    pub fn check_data_retention(&self, data_type: &str, data_age_days: u32) -> bool {
        for policy in self.retention_policies.values() {
            if policy.data_types.contains(&data_type.to_string()) {
                if policy.legal_hold {
                    return true; // Legal hold overrides retention
                }
                return data_age_days <= policy.retention_days;
            }
        }
        false // No policy found, assume non-compliant
    }

    /// Check access compliance
    #[must_use]
    pub fn check_access_compliance(
        &self,
        user_permissions: &[String],
        clearance_level: u8,
    ) -> bool {
        for policy in self.access_policies.values() {
            if clearance_level >= policy.min_clearance_level {
                let has_required_permissions = policy
                    .required_permissions
                    .iter()
                    .all(|perm| user_permissions.contains(perm));
                if has_required_permissions {
                    return true;
                }
            }
        }
        false
    }

    /// Generate compliance report
    #[must_use]
    pub fn generate_compliance_report(&self) -> ComplianceReport {
        let total_policies = self.retention_policies.len() + self.access_policies.len();
        let total_violations = self.violations.len();
        let critical_violations = self
            .violations
            .iter()
            .filter(|v| matches!(v.severity, ViolationSeverity::Critical))
            .count();

        ComplianceReport {
            timestamp: Utc::now(),
            total_policies,
            total_violations,
            critical_violations,
            compliance_score: self.calculate_compliance_score(),
            frameworks: self.regulatory_frameworks.values().cloned().collect(),
            recent_violations: self.violations.iter().rev().take(10).cloned().collect(),
        }
    }

    /// Calculate compliance score (0-100)
    fn calculate_compliance_score(&self) -> f32 {
        if self.violations.is_empty() {
            return 100.0;
        }

        let total_controls = self
            .regulatory_frameworks
            .values()
            .map(|f| f.required_controls.len())
            .sum::<usize>() as f32;

        if total_controls == 0.0 {
            return 100.0;
        }

        let violation_weight = self
            .violations
            .iter()
            .map(|v| match v.severity {
                ViolationSeverity::Critical => 10.0,
                ViolationSeverity::High => 5.0,
                ViolationSeverity::Medium => 2.0,
                ViolationSeverity::Low => 1.0,
            })
            .sum::<f32>();

        let score = (violation_weight / total_controls).mul_add(-100.0, 100.0);
        score.max(0.0).min(100.0)
    }
}

/// Compliance report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceReport {
    /// Report timestamp
    pub timestamp: DateTime<Utc>,
    /// Total policies
    pub total_policies: usize,
    /// Total violations
    pub total_violations: usize,
    /// Critical violations
    pub critical_violations: usize,
    /// Compliance score (0-100)
    pub compliance_score: f32,
    /// Regulatory frameworks
    pub frameworks: Vec<RegulatoryFramework>,
    /// Recent violations
    pub recent_violations: Vec<ComplianceViolation>,
}
/// API Routes
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

    #[test]
    fn test_compliance_manager_creation() {
        let manager = ComplianceManager::new();
        assert!(manager.retention_policies.is_empty());
        assert!(manager.access_policies.is_empty());
        assert!(manager.audit_logs.is_empty());
        assert!(manager.regulatory_frameworks.is_empty());
        assert!(manager.violations.is_empty());
    }
    #[test]
    fn test_data_retention_compliance() {
        let mut manager = ComplianceManager::new();

        let policy = RetentionPolicy {
            id: "test-policy".to_string(),
            name: "Test Policy".to_string(),
            data_classification: DataClassification::Internal,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: true,
            legal_hold: false,
            data_types: vec!["test-data".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        manager.add_retention_policy(policy);

        // Data within retention period should be compliant
        assert!(manager.check_data_retention("test-data", 300));

        // Data beyond retention period should be non-compliant
        assert!(!manager.check_data_retention("test-data", 400));

        // Unknown data type should be non-compliant
        assert!(!manager.check_data_retention("unknown-data", 100));
    }

    #[test]
    fn test_access_compliance() {
        let mut manager = ComplianceManager::new();

        let policy = AccessPolicy {
            id: "test-access".to_string(),
            name: "Test Access Policy".to_string(),
            required_permissions: vec!["read".to_string(), "write".to_string()],
            min_clearance_level: 3,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: true,
            audit_access: true,
            created_at: Utc::now(),
        };

        manager.add_access_policy(policy);

        // User with sufficient permissions and clearance should be compliant
        assert!(manager.check_access_compliance(
            &["read".to_string(), "write".to_string(), "admin".to_string()],
            5
        ));

        // User with insufficient permissions should be non-compliant
        assert!(!manager.check_access_compliance(&["read".to_string()], 5));

        // User with insufficient clearance should be non-compliant
        assert!(!manager.check_access_compliance(&["read".to_string(), "write".to_string()], 2));
    }

    #[test]
    fn test_compliance_score_calculation() {
        let mut manager = ComplianceManager::new();

        // Add a framework with controls
        let framework = RegulatoryFramework {
            id: "test-framework".to_string(),
            name: "Test Framework".to_string(),
            framework_type: RegulatoryType::Custom("test".to_string()),
            required_controls: vec![ComplianceControl {
                id: "control-1".to_string(),
                name: "Control 1".to_string(),
                description: "Test control".to_string(),
                control_type: ControlType::Preventive,
                implementation_status: ImplementationStatus::FullyImplemented,
                last_assessment: None,
                next_assessment_due: None,
            }],
            audit_frequency_days: 365,
            last_audit: None,
            compliance_status: ComplianceStatus::Compliant,
        };

        manager.add_regulatory_framework(framework);

        // No violations should give 100% score
        assert_eq!(manager.calculate_compliance_score(), 100.0);

        // Add a violation
        let violation = ComplianceViolation {
            id: "test-violation".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::Medium,
            description: "Test violation".to_string(),
            path: "test-resource".to_string(),
            framework: "test-framework".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };

        manager.record_violation(violation);

        // Score should be reduced due to violation
        assert!(manager.calculate_compliance_score() < 100.0);
    }

    // ==================== ADDITIONAL COMPREHENSIVE TESTS ====================

    #[test]
    fn test_retention_policy_with_legal_hold() {
        let mut manager = ComplianceManager::new();
        let policy = RetentionPolicy {
            id: "legal-hold".to_string(),
            name: "Legal Hold Policy".to_string(),
            data_classification: DataClassification::Restricted,
            retention_days: 30,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: true,
            data_types: vec!["legal-data".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        manager.add_retention_policy(policy);

        // Legal hold should override retention period
        assert!(manager.check_data_retention("legal-data", 1000));
    }

    #[test]
    fn test_multiple_retention_policies() {
        let mut manager = ComplianceManager::new();

        for i in 1..=5 {
            let policy = RetentionPolicy {
                id: format!("policy-{}", i),
                name: format!("Policy {}", i),
                data_classification: DataClassification::Internal,
                retention_days: i * 365,
                archive_after_days: Some(i * 180),
                auto_delete: true,
                legal_hold: false,
                data_types: vec![format!("type-{}", i)],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            manager.add_retention_policy(policy);
        }

        assert_eq!(manager.retention_policies.len(), 5);
    }

    #[test]
    fn test_access_policy_with_time_restrictions() {
        let time_restriction = TimeRestriction {
            day_of_week: 1, // Monday
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            timezone: "UTC".to_string(),
        };

        let policy = AccessPolicy {
            id: "time-restricted".to_string(),
            name: "Business Hours Only".to_string(),
            required_permissions: vec!["read".to_string()],
            min_clearance_level: 2,
            time_restrictions: vec![time_restriction],
            location_restrictions: vec![],
            mfa_required: false,
            audit_access: true,
            created_at: Utc::now(),
        };

        assert_eq!(policy.time_restrictions.len(), 1);
        assert_eq!(policy.time_restrictions[0].day_of_week, 1);
    }

    #[test]
    fn test_audit_event_logging_multiple() {
        let mut manager = ComplianceManager::new();

        let event_types = vec![
            AuditEventType::DataAccess,
            AuditEventType::DataModification,
            AuditEventType::DataDeletion,
            AuditEventType::PolicyChange,
            AuditEventType::Authentication,
        ];

        for (i, event_type) in event_types.into_iter().enumerate() {
            let event = AuditEvent {
                id: format!("event-{}", i),
                timestamp: Utc::now(),
                event_type,
                user_id: Some(format!("user-{}", i)),
                path: format!("/path/{}", i),
                action: format!("action-{}", i),
                result: AuditResult::Success,
                details: HashMap::new(),
                source_ip: Some("192.168.1.1".to_string()),
                user_agent: None,
            };
            manager.log_audit_event(event);
        }

        assert_eq!(manager.audit_logs.len(), 5);
    }

    #[test]
    fn test_regulatory_framework_types() {
        let frameworks = vec![
            RegulatoryType::GDPR,
            RegulatoryType::HIPAA,
            RegulatoryType::SOX,
            RegulatoryType::PCIDSS,
            RegulatoryType::ISO27001,
            RegulatoryType::FedRAMP,
            RegulatoryType::Custom("Custom Framework".to_string()),
        ];

        assert_eq!(frameworks.len(), 7);
    }

    #[test]
    fn test_compliance_violation_tracking() {
        let mut manager = ComplianceManager::new();

        let violation = ComplianceViolation {
            id: "v1".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::High,
            description: "Data retention policy violated".to_string(),
            path: "/data/sensitive".to_string(),
            framework: "gdpr".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: Some(Utc::now() + ChronoDuration::days(7)),
            assigned_to: Some("compliance-team".to_string()),
        };

        manager.record_violation(violation);

        assert_eq!(manager.violations.len(), 1);
        assert_eq!(
            manager.violations[0].assigned_to,
            Some("compliance-team".to_string())
        );
    }

    #[test]
    fn test_compliance_score_with_multiple_violations() {
        let mut manager = ComplianceManager::new();

        let framework = RegulatoryFramework {
            id: "multi-test".to_string(),
            name: "Multi Violation Test".to_string(),
            framework_type: RegulatoryType::HIPAA,
            required_controls: vec![ComplianceControl {
                id: "ctrl-1".to_string(),
                name: "Control 1".to_string(),
                description: "Test control".to_string(),
                control_type: ControlType::Preventive,
                implementation_status: ImplementationStatus::FullyImplemented,
                last_assessment: None,
                next_assessment_due: None,
            }],
            audit_frequency_days: 365,
            last_audit: None,
            compliance_status: ComplianceStatus::Compliant,
        };

        manager.add_regulatory_framework(framework);

        // Add violations of different severities
        manager.record_violation(ComplianceViolation {
            id: "v1".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::AccessControl,
            severity: ViolationSeverity::Low,
            description: "Minor access issue".to_string(),
            path: "/test1".to_string(),
            framework: "multi-test".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        });

        manager.record_violation(ComplianceViolation {
            id: "v2".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::Encryption,
            severity: ViolationSeverity::Critical,
            description: "Encryption failure".to_string(),
            path: "/test2".to_string(),
            framework: "multi-test".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        });

        let score = manager.calculate_compliance_score();
        assert!(score < 100.0);
        assert!(score >= 0.0);
    }

    #[test]
    fn test_data_classification_levels() {
        let classifications = vec![
            DataClassification::Public,
            DataClassification::Internal,
            DataClassification::Confidential,
            DataClassification::Restricted,
            DataClassification::TopSecret,
        ];

        assert_eq!(classifications.len(), 5);
    }

    #[test]
    fn test_compliance_report_generation_detailed() {
        let mut manager = ComplianceManager::new();

        // Add multiple policies
        manager.add_retention_policy(RetentionPolicy {
            id: "r1".to_string(),
            name: "Retention 1".to_string(),
            data_classification: DataClassification::Internal,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: false,
            data_types: vec!["logs".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        manager.add_access_policy(AccessPolicy {
            id: "a1".to_string(),
            name: "Access 1".to_string(),
            required_permissions: vec!["read".to_string()],
            min_clearance_level: 1,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: false,
            audit_access: true,
            created_at: Utc::now(),
        });

        manager.add_access_policy(AccessPolicy {
            id: "a2".to_string(),
            name: "Access 2".to_string(),
            required_permissions: vec!["write".to_string()],
            min_clearance_level: 3,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: true,
            audit_access: true,
            created_at: Utc::now(),
        });

        let report = manager.generate_compliance_report();

        assert_eq!(report.total_policies, 3); // 1 retention + 2 access
        assert_eq!(report.total_violations, 0);
        assert_eq!(report.compliance_score, 100.0);
    }

    #[test]
    fn test_violation_type_display() {
        assert_eq!(
            format!("{}", ViolationType::DataRetention),
            "Data Retention"
        );
        assert_eq!(
            format!("{}", ViolationType::AccessControl),
            "Access Control"
        );
        assert_eq!(format!("{}", ViolationType::Encryption), "Encryption");
        assert_eq!(format!("{}", ViolationType::AuditLogging), "Audit Logging");
        assert_eq!(
            format!("{}", ViolationType::DataResidency),
            "Data Residency"
        );
        assert_eq!(format!("{}", ViolationType::Backup), "Backup");
        assert_eq!(format!("{}", ViolationType::Documentation), "Documentation");
    }

    #[test]
    fn test_audit_event_type_display() {
        assert_eq!(format!("{}", AuditEventType::DataAccess), "Data Access");
        assert_eq!(
            format!("{}", AuditEventType::DataModification),
            "Data Modification"
        );
        assert_eq!(format!("{}", AuditEventType::DataDeletion), "Data Deletion");
        assert_eq!(format!("{}", AuditEventType::PolicyChange), "Policy Change");
        assert_eq!(
            format!("{}", AuditEventType::Authentication),
            "Authentication"
        );
        assert_eq!(
            format!("{}", AuditEventType::Authorization),
            "Authorization"
        );
        assert_eq!(
            format!("{}", AuditEventType::SystemConfiguration),
            "System Configuration"
        );
        assert_eq!(
            format!("{}", AuditEventType::ComplianceViolation),
            "Compliance Violation"
        );
    }

    #[test]
    fn test_initialize_compliance_manager_gdpr() {
        let manager = initialize_compliance_manager();

        // Check GDPR framework was added
        assert!(manager.regulatory_frameworks.contains_key("gdpr"));
        let gdpr = &manager.regulatory_frameworks["gdpr"];
        assert_eq!(gdpr.name, "General Data Protection Regulation");
        assert!(matches!(gdpr.framework_type, RegulatoryType::GDPR));

        // Check required controls
        assert!(!gdpr.required_controls.is_empty());
    }

    #[test]
    fn test_initialize_compliance_manager_default_retention() {
        let manager = initialize_compliance_manager();

        // Check default retention policy was added
        assert!(manager.retention_policies.contains_key("default-retention"));
        let policy = &manager.retention_policies["default-retention"];
        assert_eq!(policy.name, "Default Data Retention");
        assert_eq!(policy.retention_days, 2555); // 7 years
    }
}
