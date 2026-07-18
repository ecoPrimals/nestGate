// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Compliance module for enterprise storage systems
//!
//! This module implements comprehensive compliance features including data retention
//! policies, access control compliance, audit logging, and regulatory compliance
//! (GDPR, HIPAA, SOX, etc.).

/// Handlers for compliance-related HTTP endpoints
pub mod handlers;
/// Manager for compliance state and operations
pub mod manager;
pub mod types;

// Re-export public API
pub use handlers::{
    create_compliance_routes, create_retention_policy, get_audit_logs, get_compliance_dashboard,
    get_retention_policies, get_violations,
};
pub use manager::{ComplianceManager, ComplianceState, initialize_compliance_manager};
pub use types::*;

#[cfg(test)]
mod handlers_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration as ChronoDuration, Utc};
    use std::collections::HashMap;

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
            id: "test-policy".into(),
            name: "Test Policy".into(),
            data_classification: DataClassification::Internal,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: true,
            legal_hold: false,
            data_types: vec!["test-data".into()],
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
            id: "test-access".into(),
            name: "Test Access Policy".into(),
            required_permissions: vec!["read".into(), "write".into()],
            min_clearance_level: 3,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: true,
            audit_access: true,
            created_at: Utc::now(),
        };

        manager.add_access_policy(policy);

        // User with sufficient permissions and clearance should be compliant
        assert!(
            manager.check_access_compliance(&["read".into(), "write".into(), "admin".into()], 5)
        );

        // User with insufficient permissions should be non-compliant
        assert!(!manager.check_access_compliance(&["read".into()], 5));

        // User with insufficient clearance should be non-compliant
        assert!(!manager.check_access_compliance(&["read".into(), "write".into()], 2));
    }

    #[test]
    fn test_compliance_score_calculation() {
        let mut manager = ComplianceManager::new();

        // Add a framework with controls
        let framework = RegulatoryFramework {
            id: "test-framework".into(),
            name: "Test Framework".into(),
            framework_type: RegulatoryType::Custom("test".into()),
            required_controls: vec![ComplianceControl {
                id: "control-1".into(),
                name: "Control 1".into(),
                description: "Test control".into(),
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
            id: "test-violation".into(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::Medium,
            description: "Test violation".into(),
            path: "test-resource".into(),
            framework: "test-framework".into(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };

        manager.record_violation(violation);

        // Score should be reduced due to violation
        assert!(manager.calculate_compliance_score() < 100.0);
    }

    #[test]
    fn test_retention_policy_with_legal_hold() {
        let mut manager = ComplianceManager::new();
        let policy = RetentionPolicy {
            id: "legal-hold".into(),
            name: "Legal Hold Policy".into(),
            data_classification: DataClassification::Restricted,
            retention_days: 30,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: true,
            data_types: vec!["legal-data".into()],
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
                id: format!("policy-{i}"),
                name: format!("Policy {i}"),
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
            start_time: "09:00".into(),
            end_time: "17:00".into(),
            timezone: "UTC".into(),
        };

        let policy = AccessPolicy {
            id: "time-restricted".into(),
            name: "Business Hours Only".into(),
            required_permissions: vec!["read".into()],
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
                id: format!("event-{i}"),
                timestamp: Utc::now(),
                /// Event Type
                event_type,
                user_id: Some(format!("user-{i}")),
                path: format!("/path/{i}"),
                action: format!("action-{i}"),
                result: AuditResult::Success,
                details: HashMap::new(),
                source_ip: Some("192.168.1.1".into()),
                user_agent: None,
            };
            manager.log_audit_event(event);
        }

        assert_eq!(manager.audit_logs.len(), 5);
    }

    #[test]
    fn test_regulatory_framework_types() {
        let frameworks = [
            RegulatoryType::GDPR,
            RegulatoryType::HIPAA,
            RegulatoryType::SOX,
            RegulatoryType::PCIDSS,
            RegulatoryType::ISO27001,
            RegulatoryType::FedRAMP,
            RegulatoryType::Custom("Custom Framework".into()),
        ];

        assert_eq!(frameworks.len(), 7);
    }

    #[test]
    fn test_compliance_violation_tracking() {
        let mut manager = ComplianceManager::new();

        let violation = ComplianceViolation {
            id: "v1".into(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::High,
            description: "Data retention policy violated".into(),
            path: "/data/sensitive".into(),
            framework: "gdpr".into(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: Some(Utc::now() + ChronoDuration::days(7)),
            assigned_to: Some("compliance-team".into()),
        };

        manager.record_violation(violation);

        assert_eq!(manager.violations.len(), 1);
        assert_eq!(
            manager.violations[0].assigned_to,
            Some("compliance-team".into())
        );
    }

    #[test]
    fn test_compliance_score_with_multiple_violations() {
        let mut manager = ComplianceManager::new();

        let framework = RegulatoryFramework {
            id: "multi-test".into(),
            name: "Multi Violation Test".into(),
            framework_type: RegulatoryType::HIPAA,
            required_controls: vec![ComplianceControl {
                id: "ctrl-1".into(),
                name: "Control 1".into(),
                description: "Test control".into(),
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
            id: "v1".into(),
            timestamp: Utc::now(),
            violation_type: ViolationType::AccessControl,
            severity: ViolationSeverity::Low,
            description: "Minor access issue".into(),
            path: "/test1".into(),
            framework: "multi-test".into(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        });

        manager.record_violation(ComplianceViolation {
            id: "v2".into(),
            timestamp: Utc::now(),
            violation_type: ViolationType::Encryption,
            severity: ViolationSeverity::Critical,
            description: "Encryption failure".into(),
            path: "/test2".into(),
            framework: "multi-test".into(),
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
        let classifications = [
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
            id: "r1".into(),
            name: "Retention 1".into(),
            data_classification: DataClassification::Internal,
            retention_days: 365,
            archive_after_days: None,
            auto_delete: false,
            legal_hold: false,
            data_types: vec!["logs".into()],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        });

        manager.add_access_policy(AccessPolicy {
            id: "a1".into(),
            name: "Access 1".into(),
            required_permissions: vec!["read".into()],
            min_clearance_level: 1,
            time_restrictions: vec![],
            location_restrictions: vec![],
            mfa_required: false,
            audit_access: true,
            created_at: Utc::now(),
        });

        manager.add_access_policy(AccessPolicy {
            id: "a2".into(),
            name: "Access 2".into(),
            required_permissions: vec!["write".into()],
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
