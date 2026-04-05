// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Compliance handler types split by subdomain (retention/access, audit, regulatory, violations, reporting).

mod audit;
mod regulatory;
mod report;
mod retention_access;
mod violations;

pub use audit::{AuditEvent, AuditEventType, AuditResult};
pub use regulatory::{
    ComplianceControl, ComplianceStatus, ControlType, ImplementationStatus, RegulatoryFramework,
    RegulatoryType,
};
pub use report::ComplianceReport;
pub use retention_access::{AccessPolicy, DataClassification, RetentionPolicy, TimeRestriction};
pub use violations::{ComplianceViolation, ResolutionStatus, ViolationSeverity, ViolationType};

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_audit_event_type_display() {
        assert_eq!(AuditEventType::DataAccess.to_string(), "Data Access");
        assert_eq!(
            AuditEventType::DataModification.to_string(),
            "Data Modification"
        );
        assert_eq!(AuditEventType::DataDeletion.to_string(), "Data Deletion");
        assert_eq!(AuditEventType::PolicyChange.to_string(), "Policy Change");
        assert_eq!(AuditEventType::Authentication.to_string(), "Authentication");
        assert_eq!(AuditEventType::Authorization.to_string(), "Authorization");
        assert_eq!(
            AuditEventType::SystemConfiguration.to_string(),
            "System Configuration"
        );
        assert_eq!(
            AuditEventType::ComplianceViolation.to_string(),
            "Compliance Violation"
        );
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
            let json = serde_json::to_string(&classification).expect("String operation failed");
            let deserialized = serde_json::from_str::<DataClassification>(&json)
                .expect("Failed to convert from string");
            assert!(matches!(
                deserialized,
                DataClassification::Public
                    | DataClassification::Internal
                    | DataClassification::Confidential
                    | DataClassification::Restricted
                    | DataClassification::TopSecret
            ));
        }
    }

    #[test]
    fn test_regulatory_type_serialization() {
        let reg_type = RegulatoryType::GDPR;
        let json = serde_json::to_string(&reg_type).expect("String operation failed");
        let deserialized =
            serde_json::from_str::<RegulatoryType>(&json).expect("Failed to convert from string");
        assert!(matches!(deserialized, RegulatoryType::GDPR));
    }

    #[test]
    fn test_regulatory_type_custom() {
        let custom = RegulatoryType::Custom("Custom Framework".to_string());
        let json = serde_json::to_string(&custom).expect("String operation failed");
        let deserialized =
            serde_json::from_str::<RegulatoryType>(&json).expect("Failed to convert from string");

        if let RegulatoryType::Custom(name) = deserialized {
            assert_eq!(name, "Custom Framework");
        } else {
            panic!("Expected Custom variant");
        }
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
