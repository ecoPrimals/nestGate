//
// Compliance manager implementation
//

//! Manager module

use chrono::{Duration as ChronoDuration, Utc};
use nestgate_core::math::float_compare::approx_eq_f32;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use super::types::{
    AccessPolicy, AuditEvent, ComplianceControl, ComplianceReport, ComplianceStatus,
    ComplianceViolation, ControlType, DataClassification, ImplementationStatus,
    RegulatoryFramework, RegulatoryType, RetentionPolicy, ViolationSeverity,
};

/// Compliance manager state
pub type ComplianceState = Arc<RwLock<ComplianceManager>>;

/// Compliance manager
#[derive(Debug, Clone, Default)]
/// Manager for Compliance operations
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
    pub(crate) fn calculate_compliance_score(&self) -> f32 {
        if self.violations.is_empty() {
            return 100.0;
        }

        let total_controls = self
            .regulatory_frameworks
            .values()
            .map(|f| f.required_controls.len())
            .sum::<usize>() as f32;

        // ✅ MODERN: Use epsilon for zero check in production code
        if approx_eq_f32(total_controls, 0.0) {
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
    use chrono::Utc;

    /// Creates  Test Retention Policy
    fn create_test_retention_policy() -> RetentionPolicy {
        RetentionPolicy {
            id: "test-retention".to_string(),
            name: "Test Retention".to_string(),
            data_classification: DataClassification::Confidential,
            retention_days: 365,
            archive_after_days: Some(180),
            auto_delete: false,
            legal_hold: false,
            data_types: vec!["test_data".to_string(), "logs".to_string()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Creates  Test Access Policy
    fn create_test_access_policy() -> AccessPolicy {
        AccessPolicy {
            id: "test-access".to_string(),
            name: "Test Access".to_string(),
            required_permissions: vec!["read".to_string(), "write".to_string()],
            min_clearance_level: 3,
            time_restrictions: vec![],
            location_restrictions: vec!["us-east".to_string()],
            mfa_required: false,
            audit_access: true,
            created_at: Utc::now(),
        }
    }

    /// Creates  Test Audit Event
    fn create_test_audit_event() -> AuditEvent {
        use super::AuditEvent;
        use crate::handlers::compliance::types::{AuditEventType, AuditResult};
        use std::collections::HashMap;

        AuditEvent {
            id: "audit-1".to_string(),
            timestamp: Utc::now(),
            event_type: AuditEventType::DataAccess,
            user_id: Some("user-123".to_string()),
            path: "/data/resource-456".to_string(),
            action: "read".to_string(),
            result: AuditResult::Success,
            details: HashMap::new(),
            source_ip: Some("192.168.1.1".to_string()),
            user_agent: Some("test-agent/1.0".to_string()),
        }
    }

    /// Creates  Test Violation
    fn create_test_violation() -> ComplianceViolation {
        use crate::handlers::compliance::types::{ResolutionStatus, ViolationType};

        ComplianceViolation {
            id: "violation-1".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::High,
            description: "Data retained beyond policy".to_string(),
            path: "/data/resource-789".to_string(),
            framework: "gdpr".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        }
    }

    #[test]
    fn test_compliance_manager_new() {
        let manager = ComplianceManager::new();
        assert!(manager.retention_policies.is_empty());
        assert!(manager.access_policies.is_empty());
        assert!(manager.audit_logs.is_empty());
        assert!(manager.regulatory_frameworks.is_empty());
        assert!(manager.violations.is_empty());
    }

    #[test]
    fn test_compliance_manager_default() {
        let manager = ComplianceManager::default();
        assert!(manager.retention_policies.is_empty());
        assert!(manager.access_policies.is_empty());
    }

    #[test]
    fn test_add_retention_policy() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_retention_policy();
        let policy_id = policy.id.clone();

        manager.add_retention_policy(policy);
        assert_eq!(manager.retention_policies.len(), 1);
        assert!(manager.retention_policies.contains_key(&policy_id));
    }

    #[test]
    fn test_add_access_policy() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_access_policy();
        let policy_id = policy.id.clone();

        manager.add_access_policy(policy);
        assert_eq!(manager.access_policies.len(), 1);
        assert!(manager.access_policies.contains_key(&policy_id));
    }

    #[test]
    fn test_log_audit_event() {
        let mut manager = ComplianceManager::new();
        let event = create_test_audit_event();

        manager.log_audit_event(event.clone());
        assert_eq!(manager.audit_logs.len(), 1);
        assert_eq!(manager.audit_logs[0].user_id, event.user_id);
    }

    #[test]
    fn test_record_violation() {
        let mut manager = ComplianceManager::new();
        let violation = create_test_violation();

        manager.record_violation(violation.clone());
        assert_eq!(manager.violations.len(), 1);
        assert_eq!(manager.violations[0].id, violation.id);
    }

    #[test]
    fn test_check_data_retention_compliant() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_retention_policy();
        manager.add_retention_policy(policy);

        // Data age within retention period
        assert!(manager.check_data_retention("test_data", 100));
        assert!(manager.check_data_retention("logs", 365));
    }

    #[test]
    fn test_check_data_retention_non_compliant() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_retention_policy();
        manager.add_retention_policy(policy);

        // Data age exceeds retention period
        assert!(!manager.check_data_retention("test_data", 400));
    }

    #[test]
    fn test_check_data_retention_legal_hold() {
        let mut manager = ComplianceManager::new();
        let mut policy = create_test_retention_policy();
        policy.legal_hold = true;
        manager.add_retention_policy(policy);

        // Legal hold overrides retention period
        assert!(manager.check_data_retention("test_data", 10000));
    }

    #[test]
    fn test_check_data_retention_no_policy() {
        let manager = ComplianceManager::new();

        // No policy found, should be non-compliant
        assert!(!manager.check_data_retention("unknown_type", 10));
    }

    #[test]
    fn test_check_access_compliance_valid() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_access_policy();
        manager.add_access_policy(policy);

        let user_permissions = vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
        ];
        assert!(manager.check_access_compliance(&user_permissions, 5));
    }

    #[test]
    fn test_check_access_compliance_insufficient_clearance() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_access_policy();
        manager.add_access_policy(policy);

        let user_permissions = vec!["read".to_string(), "write".to_string()];
        assert!(!manager.check_access_compliance(&user_permissions, 2));
    }

    #[test]
    fn test_check_access_compliance_missing_permissions() {
        let mut manager = ComplianceManager::new();
        let policy = create_test_access_policy();
        manager.add_access_policy(policy);

        let user_permissions = vec!["read".to_string()]; // Missing 'write'
        assert!(!manager.check_access_compliance(&user_permissions, 5));
    }

    #[test]
    fn test_generate_compliance_report() {
        let mut manager = ComplianceManager::new();
        manager.add_retention_policy(create_test_retention_policy());
        manager.add_access_policy(create_test_access_policy());
        manager.record_violation(create_test_violation());

        let report = manager.generate_compliance_report();
        assert_eq!(report.total_policies, 2);
        assert_eq!(report.total_violations, 1);
        assert_eq!(report.critical_violations, 0);
        assert!(report.compliance_score >= 0.0 && report.compliance_score <= 100.0);
    }

    #[test]
    fn test_calculate_compliance_score_no_violations() {
        let manager = ComplianceManager::new();
        assert_eq!(manager.calculate_compliance_score(), 100.0);
    }

    #[test]
    fn test_calculate_compliance_score_with_violations() {
        let mut manager = ComplianceManager::new();
        let framework = RegulatoryFramework {
            id: "test-framework".to_string(),
            name: "Test Framework".to_string(),
            framework_type: RegulatoryType::Custom("Test Framework".to_string()),
            required_controls: vec![ComplianceControl {
                id: "control-1".to_string(),
                name: "Control 1".to_string(),
                description: "Test control".to_string(),
                control_type: ControlType::Detective,
                implementation_status: ImplementationStatus::FullyImplemented,
                last_assessment: None,
                next_assessment_due: None,
            }],
            audit_frequency_days: 365,
            last_audit: None,
            compliance_status: ComplianceStatus::Compliant,
        };
        manager.add_regulatory_framework(framework);

        let mut violation = create_test_violation();
        violation.severity = ViolationSeverity::Critical;
        manager.record_violation(violation);

        let score = manager.calculate_compliance_score();
        assert!((0.0..=100.0).contains(&score));
        assert!(score < 100.0); // Should be less than perfect with violations
    }

    #[test]
    fn test_calculate_compliance_score_critical_violation() {
        let mut manager = ComplianceManager::new();
        let framework = RegulatoryFramework {
            id: "test-framework".to_string(),
            name: "Test Framework".to_string(),
            framework_type: RegulatoryType::GDPR,
            required_controls: vec![ComplianceControl {
                id: "control-1".to_string(),
                name: "Control 1".to_string(),
                description: "Test control".to_string(),
                control_type: ControlType::Corrective,
                implementation_status: ImplementationStatus::FullyImplemented,
                last_assessment: None,
                next_assessment_due: None,
            }],
            audit_frequency_days: 90,
            last_audit: None,
            compliance_status: ComplianceStatus::NonCompliant,
        };
        manager.add_regulatory_framework(framework);

        let mut violation = create_test_violation();
        violation.severity = ViolationSeverity::Critical;
        manager.record_violation(violation);

        let score = manager.calculate_compliance_score();
        assert!(score < 100.0);
    }

    #[test]
    fn test_initialize_compliance_manager() {
        let manager = initialize_compliance_manager();

        // Should have default GDPR framework
        assert_eq!(manager.regulatory_frameworks.len(), 1);
        assert!(manager.regulatory_frameworks.contains_key("gdpr"));

        // Should have default retention policy
        assert_eq!(manager.retention_policies.len(), 1);
        assert!(manager.retention_policies.contains_key("default-retention"));

        // Should have no violations initially
        assert!(manager.violations.is_empty());
    }

    #[test]
    fn test_compliance_manager_cloning() {
        let mut manager = ComplianceManager::new();
        manager.add_retention_policy(create_test_retention_policy());

        let cloned = manager.clone();
        assert_eq!(
            cloned.retention_policies.len(),
            manager.retention_policies.len()
        );
    }

    #[test]
    fn test_multiple_violations_severity_weighting() {
        let mut manager = ComplianceManager::new();
        let framework = RegulatoryFramework {
            id: "test-framework".to_string(),
            name: "Test Framework".to_string(),
            framework_type: RegulatoryType::HIPAA,
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

        // Add violations of different severities
        let mut critical_violation = create_test_violation();
        critical_violation.severity = ViolationSeverity::Critical;
        manager.record_violation(critical_violation);

        let mut low_violation = create_test_violation();
        low_violation.id = "violation-2".to_string();
        low_violation.severity = ViolationSeverity::Low;
        manager.record_violation(low_violation);

        let score = manager.calculate_compliance_score();
        assert!((0.0..100.0).contains(&score));
    }

    #[test]
    fn test_compliance_report_recent_violations() {
        let mut manager = ComplianceManager::new();

        // Add more than 10 violations
        for i in 0..15 {
            let mut violation = create_test_violation();
            violation.id = format!("violation-{i}");
            manager.record_violation(violation);
        }

        let report = manager.generate_compliance_report();
        assert_eq!(report.total_violations, 15);
        assert_eq!(report.recent_violations.len(), 10); // Should only include 10 most recent
    }

    #[test]
    fn test_compliance_state_type() {
        let manager = ComplianceManager::new();
        let state: ComplianceState = Arc::new(RwLock::new(manager));
        assert!(Arc::strong_count(&state) == 1);
    }
}
