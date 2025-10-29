//! Compliance types and data structures
//!
//! This module implements comprehensive compliance features for enterprise storage systems
//! including data retention policies, access control compliance, audit logging, and
//! regulatory compliance (GDPR, HIPAA, SOX, etc.).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compliance_manager_new() {
        let manager = ComplianceManager::new();
        assert_eq!(manager.retention_policies.len(), 0);
        assert_eq!(manager.access_policies.len(), 0);
        assert_eq!(manager.audit_logs.len(), 0);
        assert_eq!(manager.regulatory_frameworks.len(), 0);
        assert_eq!(manager.violations.len(), 0);
    }
    
    #[test]
    fn test_compliance_manager_default() {
        let manager = ComplianceManager::default();
        assert_eq!(manager.retention_policies.len(), 0);
    }
    
    #[test]
    fn test_add_retention_policy() {
        let mut manager = ComplianceManager::new();
        let policy = RetentionPolicy {
            id: "pol1".to_string(),
            name: "Test Policy".to_string(),
            data_classification: DataClassification::Confidential,
            retention_days: 365,
            archive_after_days: Some(180),
            auto_delete: false,
            legal_hold: false,
            data_types: vec!["documents".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        manager.add_retention_policy(policy.clone());
        assert_eq!(manager.retention_policies.len(), 1);
        assert_eq!(manager.retention_policies.get("pol1").unwrap().name, "Test Policy");
    }
    
    #[test]
    fn test_add_access_policy() {
        let mut manager = ComplianceManager::new();
        let policy = AccessPolicy {
            id: "acc1".to_string(),
            name: "Access Policy".to_string(),
            required_permissions: vec!["read".to_string(), "write".to_string()],
            min_clearance_level: 3,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: true,
            audit_access: true,
            created_at: Utc::now(),
        };
        
        manager.add_access_policy(policy.clone());
        assert_eq!(manager.access_policies.len(), 1);
    }
    
    #[test]
    fn test_log_audit_event() {
        let mut manager = ComplianceManager::new();
        let event = AuditEvent {
            id: "evt1".to_string(),
            timestamp: Utc::now(),
            event_type: AuditEventType::DataAccess,
            user_id: Some("user123".to_string()),
            path: "/data/file.txt".to_string(),
            action: "read".to_string(),
            result: AuditResult::Success,
            details: HashMap::new(),
            source_ip: Some("192.168.1.1".to_string()),
            user_agent: Some("Mozilla/5.0".to_string()),
        };
        
        manager.log_audit_event(event);
        assert_eq!(manager.audit_logs.len(), 1);
    }
    
    #[test]
    fn test_record_violation() {
        let mut manager = ComplianceManager::new();
        let violation = ComplianceViolation {
            id: "vio1".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::High,
            description: "Retention policy violated".to_string(),
            path: "/data/old_file.txt".to_string(),
            framework: "GDPR".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };
        
        manager.record_violation(violation);
        assert_eq!(manager.violations.len(), 1);
    }
    
    #[test]
    fn test_check_data_retention_compliant() {
        let mut manager = ComplianceManager::new();
        let policy = RetentionPolicy {
            id: "pol1".to_string(),
            name: "Test Policy".to_string(),
            data_classification: DataClassification::Confidential,
            retention_days: 365,
            archive_after_days: Some(180),
            auto_delete: false,
            legal_hold: false,
            data_types: vec!["documents".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        manager.add_retention_policy(policy);
        
        // Data is 100 days old, should be compliant
        assert!(manager.check_data_retention("documents", 100));
        
        // Data is 400 days old, should not be compliant
        assert!(!manager.check_data_retention("documents", 400));
    }
    
    #[test]
    fn test_check_data_retention_legal_hold() {
        let mut manager = ComplianceManager::new();
        let policy = RetentionPolicy {
            id: "pol1".to_string(),
            name: "Legal Hold Policy".to_string(),
            data_classification: DataClassification::Confidential,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: true,
            data_types: vec!["legal_docs".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        manager.add_retention_policy(policy);
        
        // With legal hold, any age should be compliant
        assert!(manager.check_data_retention("legal_docs", 1000));
    }
    
    #[test]
    fn test_check_access_compliance() {
        let mut manager = ComplianceManager::new();
        let policy = AccessPolicy {
            id: "acc1".to_string(),
            name: "Access Policy".to_string(),
            required_permissions: vec!["read".to_string(), "write".to_string()],
            min_clearance_level: 3,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: true,
            audit_access: true,
            created_at: Utc::now(),
        };
        
        manager.add_access_policy(policy);
        
        let permissions = vec!["read".to_string(), "write".to_string(), "delete".to_string()];
        assert!(manager.check_access_compliance(&permissions, 3));
        
        // Too low clearance level
        assert!(!manager.check_access_compliance(&permissions, 2));
        
        // Missing required permission
        let limited_permissions = vec!["read".to_string()];
        assert!(!manager.check_access_compliance(&limited_permissions, 5));
    }
    
    #[test]
    fn test_generate_compliance_report() {
        let mut manager = ComplianceManager::new();
        
        let policy = RetentionPolicy {
            id: "pol1".to_string(),
            name: "Test Policy".to_string(),
            data_classification: DataClassification::Confidential,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: false,
            data_types: vec!["documents".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        manager.add_retention_policy(policy);
        
        let violation = ComplianceViolation {
            id: "vio1".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::Critical,
            description: "Critical violation".to_string(),
            path: "/data/file.txt".to_string(),
            framework: "GDPR".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };
        manager.record_violation(violation);
        
        let report = manager.generate_compliance_report();
        assert_eq!(report.total_policies, 1);
        assert_eq!(report.total_violations, 1);
        assert_eq!(report.critical_violations, 1);
        assert!(report.compliance_score <= 100.0);
    }
    
    #[test]
    fn test_audit_event_type_display() {
        assert_eq!(AuditEventType::DataAccess.to_string(), "Data Access");
        assert_eq!(AuditEventType::DataModification.to_string(), "Data Modification");
        assert_eq!(AuditEventType::DataDeletion.to_string(), "Data Deletion");
        assert_eq!(AuditEventType::PolicyChange.to_string(), "Policy Change");
        assert_eq!(AuditEventType::Authentication.to_string(), "Authentication");
        assert_eq!(AuditEventType::Authorization.to_string(), "Authorization");
        assert_eq!(AuditEventType::SystemConfiguration.to_string(), "System Configuration");
        assert_eq!(AuditEventType::ComplianceViolation.to_string(), "Compliance Violation");
    }
    
    #[test]
    fn test_violation_type_display() {
        assert_eq!(ViolationType::DataRetention.to_string(), "Data Retention");
        assert_eq!(ViolationType::AccessControl.to_string(), "Access Control");
        assert_eq!(ViolationType::Encryption.to_string(), "Encryption");
        assert_eq!(ViolationType::AuditLogging.to_string(), "Audit Logging");
        assert_eq!(ViolationType::DataResidency.to_string(), "Data Residency");
        assert_eq!(ViolationType::Backup.to_string(), "Backup");
        assert_eq!(ViolationType::Documentation.to_string(), "Documentation");
    }
    
    #[test]
    fn test_data_classification_serialization() {
        let classifications = vec![
            DataClassification::Public,
            DataClassification::Internal,
            DataClassification::Confidential,
            DataClassification::Restricted,
            DataClassification::TopSecret,
        ];
        
        for classification in classifications {
            let json = serde_json::to_string(&classification).unwrap();
            let deserialized = serde_json::from_str::<DataClassification>(&json).unwrap();
            assert!(matches!(deserialized, DataClassification::Public | DataClassification::Internal | DataClassification::Confidential | DataClassification::Restricted | DataClassification::TopSecret));
        }
    }
    
    #[test]
    fn test_regulatory_type_serialization() {
        let reg_type = RegulatoryType::GDPR;
        let json = serde_json::to_string(&reg_type).unwrap();
        let deserialized = serde_json::from_str::<RegulatoryType>(&json).unwrap();
        assert!(matches!(deserialized, RegulatoryType::GDPR));
    }
    
    #[test]
    fn test_regulatory_type_custom() {
        let custom = RegulatoryType::Custom("Custom Framework".to_string());
        let json = serde_json::to_string(&custom).unwrap();
        let deserialized = serde_json::from_str::<RegulatoryType>(&json).unwrap();
        
        if let RegulatoryType::Custom(name) = deserialized {
            assert_eq!(name, "Custom Framework");
        } else {
            panic!("Expected Custom variant");
        }
    }
    
    #[test]
    fn test_compliance_score_no_violations() {
        let manager = ComplianceManager::new();
        let report = manager.generate_compliance_report();
        assert_eq!(report.compliance_score, 100.0);
    }
    
    #[test]
    fn test_time_restriction_creation() {
        let restriction = TimeRestriction {
            day_of_week: 1,
            start_time: "09:00".to_string(),
            end_time: "17:00".to_string(),
            timezone: "UTC".to_string(),
        };
        
        assert_eq!(restriction.day_of_week, 1);
        assert_eq!(restriction.start_time, "09:00");
        assert_eq!(restriction.end_time, "17:00");
    }
    
    #[test]
    fn test_compliance_control_creation() {
        let control = ComplianceControl {
            id: "ctrl1".to_string(),
            name: "Access Control".to_string(),
            description: "Control for data access".to_string(),
            control_type: ControlType::Preventive,
            implementation_status: ImplementationStatus::FullyImplemented,
            last_assessment: Some(Utc::now()),
            next_assessment_due: None,
        };
        
        assert_eq!(control.id, "ctrl1");
        assert!(matches!(control.control_type, ControlType::Preventive));
    }
    
    #[test]
    fn test_regulatory_framework_creation() {
        let framework = RegulatoryFramework {
            id: "gdpr1".to_string(),
            name: "GDPR Compliance".to_string(),
            framework_type: RegulatoryType::GDPR,
            required_controls: vec![],
            audit_frequency_days: 90,
            last_audit: None,
            compliance_status: ComplianceStatus::Compliant,
        };
        
        assert_eq!(framework.name, "GDPR Compliance");
        assert_eq!(framework.audit_frequency_days, 90);
    }
}
