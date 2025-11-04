//
// Compliance manager implementation
//

use chrono::{Duration as ChronoDuration, Utc};
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
