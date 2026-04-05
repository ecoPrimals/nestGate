// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **COMPREHENSIVE TESTS FOR COMPLIANCE TYPES**
//!
//! Test coverage for compliance types - matches compliance/types.rs struct definitions.

#[cfg(test)]
mod compliance_types_tests {
    use crate::handlers::compliance::types::{
        ComplianceReport, ComplianceStatus, ComplianceViolation, RegulatoryType, ResolutionStatus,
        ViolationSeverity, ViolationType,
    };
    use chrono::Utc;

    // ==================== COMPLIANCE STATUS TESTS ====================

    #[test]
    fn test_compliance_status_creation() {
        let status = ComplianceStatus::Compliant;
        assert_eq!(format!("{:?}", status), "Compliant");
    }

    #[test]
    fn test_compliance_status_non_compliant() {
        let status = ComplianceStatus::NonCompliant;
        assert_eq!(format!("{:?}", status), "NonCompliant");
    }

    #[test]
    fn test_compliance_status_partial() {
        let status = ComplianceStatus::PartiallyCompliant;
        assert_eq!(format!("{:?}", status), "PartiallyCompliant");
    }

    #[test]
    fn test_compliance_status_under_assessment() {
        let status = ComplianceStatus::UnderAssessment;
        assert_eq!(format!("{:?}", status), "UnderAssessment");
    }

    #[test]
    fn test_compliance_status_serialization() {
        let status = ComplianceStatus::Compliant;
        let json = serde_json::to_string(&status).expect("Should serialize");
        assert!(json.contains("Compliant"));
    }

    #[test]
    fn test_compliance_status_deserialization() {
        let json = r#""NonCompliant""#;
        let status: ComplianceStatus = serde_json::from_str(json).expect("Should deserialize");
        assert!(matches!(status, ComplianceStatus::NonCompliant));
    }

    #[test]
    fn test_compliance_status_clone() {
        let status1 = ComplianceStatus::Compliant;
        let status2 = status1.clone();
        assert_eq!(format!("{:?}", status1), format!("{:?}", status2));
    }

    // ==================== REGULATORY TYPE TESTS ====================

    #[test]
    fn test_regulatory_type_gdpr() {
        let framework = RegulatoryType::GDPR;
        assert_eq!(format!("{:?}", framework), "GDPR");
    }

    #[test]
    fn test_regulatory_type_hipaa() {
        let framework = RegulatoryType::HIPAA;
        assert_eq!(format!("{:?}", framework), "HIPAA");
    }

    #[test]
    fn test_regulatory_type_sox() {
        let framework = RegulatoryType::SOX;
        assert_eq!(format!("{:?}", framework), "SOX");
    }

    #[test]
    fn test_regulatory_type_serialization() {
        let framework = RegulatoryType::GDPR;
        let json = serde_json::to_string(&framework).expect("Should serialize");
        assert!(json.contains("GDPR"));
    }

    #[test]
    fn test_regulatory_type_deserialization() {
        let json = r#""HIPAA""#;
        let framework: RegulatoryType = serde_json::from_str(json).expect("Should deserialize");
        assert!(matches!(framework, RegulatoryType::HIPAA));
    }

    // ==================== COMPLIANCE REPORT TESTS ====================

    #[test]
    fn test_compliance_report_creation() {
        let report = ComplianceReport {
            timestamp: Utc::now(),
            total_policies: 10,
            total_violations: 0,
            critical_violations: 0,
            compliance_score: 100.0,
            frameworks: vec![],
            recent_violations: vec![],
        };

        assert_eq!(report.total_policies, 10);
        assert_eq!(report.total_violations, 0);
        assert_eq!(report.compliance_score, 100.0);
    }

    #[test]
    fn test_compliance_report_with_violations() {
        let violation = ComplianceViolation {
            id: "vio-001".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::DataRetention,
            severity: ViolationSeverity::Critical,
            description: "Test violation".to_string(),
            path: "/data/file".to_string(),
            framework: "GDPR".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };

        let report = ComplianceReport {
            timestamp: Utc::now(),
            total_policies: 5,
            total_violations: 2,
            critical_violations: 1,
            compliance_score: 80.0,
            frameworks: vec![],
            recent_violations: vec![violation],
        };

        assert_eq!(report.total_violations, 2);
        assert_eq!(report.critical_violations, 1);
        assert_eq!(report.recent_violations.len(), 1);
        assert!(matches!(
            report.recent_violations[0].severity,
            ViolationSeverity::Critical
        ));
    }

    #[test]
    fn test_compliance_report_serialization() {
        let report = ComplianceReport {
            timestamp: Utc::now(),
            total_policies: 1,
            total_violations: 0,
            critical_violations: 0,
            compliance_score: 100.0,
            frameworks: vec![],
            recent_violations: vec![],
        };
        let json = serde_json::to_string(&report).expect("Should serialize");
        assert!(json.contains("total_policies"));
        assert!(json.contains("compliance_score"));
    }

    #[test]
    fn test_compliance_report_deserialization() {
        let report = ComplianceReport {
            timestamp: Utc::now(),
            total_policies: 3,
            total_violations: 1,
            critical_violations: 0,
            compliance_score: 90.0,
            frameworks: vec![],
            recent_violations: vec![],
        };
        let json = serde_json::to_string(&report).expect("Should serialize");
        let deserialized: ComplianceReport =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(report.total_policies, deserialized.total_policies);
        assert_eq!(report.compliance_score, deserialized.compliance_score);
    }

    // ==================== COMPLIANCE VIOLATION TESTS ====================

    #[test]
    fn test_compliance_violation_creation() {
        let violation = ComplianceViolation {
            id: "crit-001".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::Encryption,
            severity: ViolationSeverity::Critical,
            description: "Critical security issue".to_string(),
            path: "/data".to_string(),
            framework: "GDPR".to_string(),
            resolution_status: ResolutionStatus::Open,
            resolution_deadline: None,
            assigned_to: None,
        };

        assert!(matches!(violation.severity, ViolationSeverity::Critical));
        assert!(matches!(
            violation.violation_type,
            ViolationType::Encryption
        ));
    }

    #[test]
    fn test_compliance_violation_clone() {
        let violation1 = ComplianceViolation {
            id: "test-001".to_string(),
            timestamp: Utc::now(),
            violation_type: ViolationType::AuditLogging,
            severity: ViolationSeverity::Medium,
            description: "Test".to_string(),
            path: "/path".to_string(),
            framework: "HIPAA".to_string(),
            resolution_status: ResolutionStatus::InProgress,
            resolution_deadline: None,
            assigned_to: None,
        };

        let violation2 = violation1.clone();
        assert_eq!(violation1.id, violation2.id);
        assert_eq!(violation1.description, violation2.description);
    }

    // ==================== VIOLATION TYPE TESTS ====================

    #[test]
    fn test_violation_type_display() {
        assert_eq!(ViolationType::DataRetention.to_string(), "Data Retention");
        assert_eq!(ViolationType::AccessControl.to_string(), "Access Control");
        assert_eq!(ViolationType::Encryption.to_string(), "Encryption");
    }

    // ==================== EDGE CASES ====================

    #[test]
    fn test_compliance_report_zero_policies() {
        let report = ComplianceReport {
            timestamp: Utc::now(),
            total_policies: 0,
            total_violations: 0,
            critical_violations: 0,
            compliance_score: 0.0,
            frameworks: vec![],
            recent_violations: vec![],
        };

        assert_eq!(report.total_policies, 0);
        assert_eq!(report.compliance_score, 0.0);
    }

    #[test]
    fn test_compliance_report_perfect_score() {
        let report = ComplianceReport {
            timestamp: Utc::now(),
            total_policies: 100,
            total_violations: 0,
            critical_violations: 0,
            compliance_score: 100.0,
            frameworks: vec![],
            recent_violations: vec![],
        };

        assert_eq!(report.compliance_score, 100.0);
        assert_eq!(report.total_violations, 0);
    }
}
