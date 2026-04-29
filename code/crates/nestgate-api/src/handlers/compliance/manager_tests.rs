// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

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

/// Violations with no framework controls: score treats total_controls as zero → 100.0.
#[test]
fn test_calculate_compliance_score_violations_without_framework_controls() {
    let mut manager = ComplianceManager::new();
    manager.record_violation(create_test_violation());
    assert_eq!(manager.calculate_compliance_score(), 100.0);
}

#[test]
fn test_calculate_compliance_score_medium_severity_weight() {
    let mut manager = ComplianceManager::new();
    let framework = RegulatoryFramework {
        id: "fw".to_string(),
        name: "FW".to_string(),
        framework_type: RegulatoryType::SOX,
        required_controls: vec![ComplianceControl {
            id: "c1".to_string(),
            name: "C1".to_string(),
            description: "d".to_string(),
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

    let mut v = create_test_violation();
    v.severity = ViolationSeverity::Medium;
    manager.record_violation(v);

    let score = manager.calculate_compliance_score();
    assert!((0.0..100.0).contains(&score));
}

#[test]
fn test_calculate_compliance_score_clamps_to_zero() {
    let mut manager = ComplianceManager::new();
    let framework = RegulatoryFramework {
        id: "fw".to_string(),
        name: "FW".to_string(),
        framework_type: RegulatoryType::PCIDSS,
        required_controls: vec![ComplianceControl {
            id: "only".to_string(),
            name: "Only".to_string(),
            description: "d".to_string(),
            control_type: ControlType::Preventive,
            implementation_status: ImplementationStatus::FullyImplemented,
            last_assessment: None,
            next_assessment_due: None,
        }],
        audit_frequency_days: 180,
        last_audit: None,
        compliance_status: ComplianceStatus::NonCompliant,
    };
    manager.add_regulatory_framework(framework);

    let mut v = create_test_violation();
    v.severity = ViolationSeverity::Critical; // weight 10 vs 1 control → raw score negative
    manager.record_violation(v);

    assert_eq!(manager.calculate_compliance_score(), 0.0);
}

#[test]
fn test_add_regulatory_framework_inserts_by_id() {
    let mut manager = ComplianceManager::new();
    let fw = RegulatoryFramework {
        id: "custom-fw".to_string(),
        name: "Custom".to_string(),
        framework_type: RegulatoryType::ISO27001,
        required_controls: vec![],
        audit_frequency_days: 90,
        last_audit: None,
        compliance_status: ComplianceStatus::Unknown,
    };
    manager.add_regulatory_framework(fw);
    assert_eq!(manager.regulatory_frameworks.len(), 1);
    assert!(manager.regulatory_frameworks.contains_key("custom-fw"));
}

#[test]
fn test_check_access_compliance_no_policies() {
    let manager = ComplianceManager::new();
    assert!(!manager.check_access_compliance(&["read".to_string()], 9));
}

#[test]
fn test_check_access_compliance_empty_required_permissions() {
    let mut manager = ComplianceManager::new();
    let mut policy = create_test_access_policy();
    policy.required_permissions.clear();
    policy.min_clearance_level = 1;
    manager.add_access_policy(policy);

    assert!(manager.check_access_compliance(&[], 2));
}

#[test]
fn test_generate_compliance_report_counts_critical_violations() {
    let mut manager = ComplianceManager::new();
    let mut crit = create_test_violation();
    crit.id = "c1".to_string();
    crit.severity = ViolationSeverity::Critical;
    manager.record_violation(crit);

    let mut high = create_test_violation();
    high.id = "h1".to_string();
    high.severity = ViolationSeverity::High;
    manager.record_violation(high);

    let report = manager.generate_compliance_report();
    assert_eq!(report.critical_violations, 1);
    assert_eq!(report.total_violations, 2);
}

#[test]
fn test_check_data_retention_no_matching_data_types() {
    let mut manager = ComplianceManager::new();
    let mut policy = create_test_retention_policy();
    policy.data_types = vec!["only-this".to_string()];
    manager.add_retention_policy(policy);

    assert!(!manager.check_data_retention("other_type", 1));
}

#[test]
fn generate_compliance_report_empty_manager_defaults() {
    let manager = ComplianceManager::new();
    let report = manager.generate_compliance_report();
    assert_eq!(report.total_policies, 0);
    assert_eq!(report.total_violations, 0);
    assert_eq!(report.critical_violations, 0);
    assert_eq!(report.compliance_score, 100.0);
    assert!(report.frameworks.is_empty());
    assert!(report.recent_violations.is_empty());
}

#[test]
fn check_access_compliance_matches_any_satisfying_policy() {
    let mut manager = ComplianceManager::new();
    let mut strict = create_test_access_policy();
    strict.id = "strict".to_string();
    strict.min_clearance_level = 9;

    let mut relaxed = create_test_access_policy();
    relaxed.id = "relaxed".to_string();
    relaxed.min_clearance_level = 1;
    relaxed.required_permissions = vec!["read".to_string()];

    manager.add_access_policy(strict);
    manager.add_access_policy(relaxed);

    assert!(manager.check_access_compliance(&["read".to_string()], 5));
}

#[test]
fn check_data_retention_uses_policy_matching_data_type() {
    let mut manager = ComplianceManager::new();
    let mut short = create_test_retention_policy();
    short.id = "short".to_string();
    short.data_types = vec!["alpha".to_string()];
    short.retention_days = 10;

    let mut long = create_test_retention_policy();
    long.id = "long".to_string();
    long.data_types = vec!["beta".to_string()];
    long.retention_days = 100;

    manager.add_retention_policy(short);
    manager.add_retention_policy(long);

    assert!(manager.check_data_retention("alpha", 5));
    assert!(!manager.check_data_retention("alpha", 20));
    assert!(manager.check_data_retention("beta", 50));
}
