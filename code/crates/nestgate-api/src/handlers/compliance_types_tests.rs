//! **COMPREHENSIVE TESTS FOR COMPLIANCE TYPES**
//!
//! Test coverage for compliance.rs (898 lines)
//! This module tests compliance data structures, validation, and type conversions.

#[cfg(test)]
mod compliance_types_tests {
    use crate::handlers::compliance::*;
    use chrono::Utc;
    use serde_json;
    use std::collections::HashMap;

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
    fn test_compliance_status_pending() {
        let status = ComplianceStatus::PendingReview;
        assert_eq!(format!("{:?}", status), "PendingReview");
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

    // ==================== COMPLIANCE LEVEL TESTS ====================

    #[test]
    fn test_compliance_level_critical() {
        let level = ComplianceLevel::Critical;
        assert_eq!(format!("{:?}", level), "Critical");
    }

    #[test]
    fn test_compliance_level_high() {
        let level = ComplianceLevel::High;
        assert_eq!(format!("{:?}", level), "High");
    }

    #[test]
    fn test_compliance_level_medium() {
        let level = ComplianceLevel::Medium;
        assert_eq!(format!("{:?}", level), "Medium");
    }

    #[test]
    fn test_compliance_level_low() {
        let level = ComplianceLevel::Low;
        assert_eq!(format!("{:?}", level), "Low");
    }

    #[test]
    fn test_compliance_level_serialization() {
        let level = ComplianceLevel::Critical;
        let json = serde_json::to_string(&level).expect("Should serialize");
        assert!(json.contains("Critical"));
    }

    #[test]
    fn test_compliance_level_deserialization() {
        let json = r#""High""#;
        let level: ComplianceLevel = serde_json::from_str(json).expect("Should deserialize");
        assert!(matches!(level, ComplianceLevel::High));
    }

    // ==================== COMPLIANCE FRAMEWORK TESTS ====================

    #[test]
    fn test_compliance_framework_gdpr() {
        let framework = ComplianceFramework::GDPR;
        assert_eq!(format!("{:?}", framework), "GDPR");
    }

    #[test]
    fn test_compliance_framework_hipaa() {
        let framework = ComplianceFramework::HIPAA;
        assert_eq!(format!("{:?}", framework), "HIPAA");
    }

    #[test]
    fn test_compliance_framework_sox() {
        let framework = ComplianceFramework::SOX;
        assert_eq!(format!("{:?}", framework), "SOX");
    }

    #[test]
    fn test_compliance_framework_pci_dss() {
        let framework = ComplianceFramework::PCIDSS;
        assert_eq!(format!("{:?}", framework), "PCIDSS");
    }

    #[test]
    fn test_compliance_framework_iso27001() {
        let framework = ComplianceFramework::ISO27001;
        assert_eq!(format!("{:?}", framework), "ISO27001");
    }

    #[test]
    fn test_compliance_framework_serialization() {
        let framework = ComplianceFramework::GDPR;
        let json = serde_json::to_string(&framework).expect("Should serialize");
        assert!(json.contains("GDPR"));
    }

    #[test]
    fn test_compliance_framework_deserialization() {
        let json = r#""HIPAA""#;
        let framework: ComplianceFramework =
            serde_json::from_str(json).expect("Should deserialize");
        assert!(matches!(framework, ComplianceFramework::HIPAA));
    }

    // ==================== COMPLIANCE REPORT TESTS ====================

    #[test]
    fn test_compliance_report_default() {
        let report = ComplianceReport::default();
        assert!(!report.id.is_empty());
        assert_eq!(report.passed_checks, 0);
        assert_eq!(report.failed_checks, 0);
    }

    #[test]
    fn test_compliance_report_creation() {
        let report = ComplianceReport {
            id: "test-report-001".to_string(),
            framework: ComplianceFramework::GDPR,
            status: ComplianceStatus::Compliant,
            passed_checks: 10,
            failed_checks: 0,
            total_checks: 10,
            compliance_percentage: 100.0,
            findings: vec![],
            recommendations: vec![],
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(report.id, "test-report-001");
        assert_eq!(report.passed_checks, 10);
        assert_eq!(report.compliance_percentage, 100.0);
    }

    #[test]
    fn test_compliance_report_with_findings() {
        let finding = ComplianceFinding {
            id: "finding-001".to_string(),
            check_id: "check-001".to_string(),
            status: ComplianceStatus::NonCompliant,
            level: ComplianceLevel::Critical,
            description: "Test finding".to_string(),
            remediation: Some("Fix immediately".to_string()),
            affected_resources: vec!["resource-1".to_string()],
        };

        let report = ComplianceReport {
            id: "test-report-002".to_string(),
            framework: ComplianceFramework::HIPAA,
            status: ComplianceStatus::NonCompliant,
            passed_checks: 8,
            failed_checks: 2,
            total_checks: 10,
            compliance_percentage: 80.0,
            findings: vec![finding],
            recommendations: vec!["Improve security".to_string()],
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.recommendations.len(), 1);
        assert_eq!(report.failed_checks, 2);
    }

    #[test]
    fn test_compliance_report_serialization() {
        let report = ComplianceReport::default();
        let json = serde_json::to_string(&report).expect("Should serialize");
        assert!(json.contains("id"));
        assert!(json.contains("framework"));
    }

    #[test]
    fn test_compliance_report_deserialization() {
        let report = ComplianceReport::default();
        let json = serde_json::to_string(&report).expect("Should serialize");
        let deserialized: ComplianceReport =
            serde_json::from_str(&json).expect("Should deserialize");
        assert_eq!(report.id, deserialized.id);
    }

    // ==================== COMPLIANCE FINDING TESTS ====================

    #[test]
    fn test_compliance_finding_critical() {
        let finding = ComplianceFinding {
            id: "crit-001".to_string(),
            check_id: "check-sec-001".to_string(),
            status: ComplianceStatus::NonCompliant,
            level: ComplianceLevel::Critical,
            description: "Critical security issue".to_string(),
            remediation: Some("Immediate action required".to_string()),
            affected_resources: vec!["server-1".to_string(), "server-2".to_string()],
        };

        assert_eq!(finding.level, ComplianceLevel::Critical);
        assert_eq!(finding.affected_resources.len(), 2);
        assert!(finding.remediation.is_some());
    }

    #[test]
    fn test_compliance_finding_without_remediation() {
        let finding = ComplianceFinding {
            id: "low-001".to_string(),
            check_id: "check-info-001".to_string(),
            status: ComplianceStatus::PendingReview,
            level: ComplianceLevel::Low,
            description: "Informational notice".to_string(),
            remediation: None,
            affected_resources: vec![],
        };

        assert!(finding.remediation.is_none());
        assert_eq!(finding.affected_resources.len(), 0);
    }

    #[test]
    fn test_compliance_finding_clone() {
        let finding1 = ComplianceFinding {
            id: "test-001".to_string(),
            check_id: "check-001".to_string(),
            status: ComplianceStatus::Compliant,
            level: ComplianceLevel::Medium,
            description: "Test".to_string(),
            remediation: None,
            affected_resources: vec![],
        };

        let finding2 = finding1.clone();
        assert_eq!(finding1.id, finding2.id);
        assert_eq!(finding1.check_id, finding2.check_id);
    }

    // ==================== COMPLIANCE CHECK TESTS ====================

    #[test]
    fn test_compliance_check_creation() {
        let check = ComplianceCheck {
            id: "check-001".to_string(),
            name: "Encryption Verification".to_string(),
            description: "Verify data encryption at rest".to_string(),
            framework: ComplianceFramework::GDPR,
            level: ComplianceLevel::Critical,
            automated: true,
            frequency: CheckFrequency::Daily,
        };

        assert_eq!(check.id, "check-001");
        assert!(check.automated);
    }

    #[test]
    fn test_compliance_check_manual() {
        let check = ComplianceCheck {
            id: "manual-001".to_string(),
            name: "Manual Review".to_string(),
            description: "Requires human review".to_string(),
            framework: ComplianceFramework::SOX,
            level: ComplianceLevel::High,
            automated: false,
            frequency: CheckFrequency::Quarterly,
        };

        assert!(!check.automated);
        assert_eq!(check.frequency, CheckFrequency::Quarterly);
    }

    // ==================== CHECK FREQUENCY TESTS ====================

    #[test]
    fn test_check_frequency_continuous() {
        let freq = CheckFrequency::Continuous;
        assert_eq!(format!("{:?}", freq), "Continuous");
    }

    #[test]
    fn test_check_frequency_daily() {
        let freq = CheckFrequency::Daily;
        assert_eq!(format!("{:?}", freq), "Daily");
    }

    #[test]
    fn test_check_frequency_weekly() {
        let freq = CheckFrequency::Weekly;
        assert_eq!(format!("{:?}", freq), "Weekly");
    }

    #[test]
    fn test_check_frequency_monthly() {
        let freq = CheckFrequency::Monthly;
        assert_eq!(format!("{:?}", freq), "Monthly");
    }

    #[test]
    fn test_check_frequency_quarterly() {
        let freq = CheckFrequency::Quarterly;
        assert_eq!(format!("{:?}", freq), "Quarterly");
    }

    #[test]
    fn test_check_frequency_annual() {
        let freq = CheckFrequency::Annual;
        assert_eq!(format!("{:?}", freq), "Annual");
    }

    // ==================== COMPLIANCE CONTEXT TESTS ====================

    #[test]
    fn test_compliance_context_default() {
        let context = ComplianceContext::default();
        assert!(context.metadata.is_empty());
    }

    #[test]
    fn test_compliance_context_with_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("region".to_string(), "eu-west-1".to_string());
        metadata.insert("environment".to_string(), "production".to_string());

        let context = ComplianceContext {
            resource_id: "resource-001".to_string(),
            resource_type: "storage".to_string(),
            frameworks: vec![ComplianceFramework::GDPR, ComplianceFramework::ISO27001],
            metadata,
        };

        assert_eq!(context.frameworks.len(), 2);
        assert_eq!(context.metadata.len(), 2);
        assert_eq!(context.metadata.get("region").unwrap(), "eu-west-1");
    }

    // ==================== COMPLIANCE SCAN REQUEST TESTS ====================

    #[test]
    fn test_scan_request_single_framework() {
        let request = ComplianceScanRequest {
            resource_id: "storage-pool-1".to_string(),
            frameworks: vec![ComplianceFramework::GDPR],
            include_recommendations: true,
            deep_scan: false,
        };

        assert_eq!(request.frameworks.len(), 1);
        assert!(request.include_recommendations);
        assert!(!request.deep_scan);
    }

    #[test]
    fn test_scan_request_multiple_frameworks() {
        let request = ComplianceScanRequest {
            resource_id: "database-1".to_string(),
            frameworks: vec![
                ComplianceFramework::GDPR,
                ComplianceFramework::HIPAA,
                ComplianceFramework::SOX,
            ],
            include_recommendations: true,
            deep_scan: true,
        };

        assert_eq!(request.frameworks.len(), 3);
        assert!(request.deep_scan);
    }

    // ==================== EDGE CASES & VALIDATION ====================

    #[test]
    fn test_compliance_report_zero_checks() {
        let report = ComplianceReport {
            id: "empty-report".to_string(),
            framework: ComplianceFramework::PCIDSS,
            status: ComplianceStatus::PendingReview,
            passed_checks: 0,
            failed_checks: 0,
            total_checks: 0,
            compliance_percentage: 0.0,
            findings: vec![],
            recommendations: vec![],
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(report.total_checks, 0);
        assert_eq!(report.compliance_percentage, 0.0);
    }

    #[test]
    fn test_compliance_report_perfect_score() {
        let report = ComplianceReport {
            id: "perfect-report".to_string(),
            framework: ComplianceFramework::ISO27001,
            status: ComplianceStatus::Compliant,
            passed_checks: 100,
            failed_checks: 0,
            total_checks: 100,
            compliance_percentage: 100.0,
            findings: vec![],
            recommendations: vec![],
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(report.compliance_percentage, 100.0);
        assert_eq!(report.status, ComplianceStatus::Compliant);
    }

    #[test]
    fn test_finding_multiple_resources() {
        let finding = ComplianceFinding {
            id: "multi-res-001".to_string(),
            check_id: "check-001".to_string(),
            status: ComplianceStatus::NonCompliant,
            level: ComplianceLevel::High,
            description: "Multiple resources affected".to_string(),
            remediation: Some("Fix all resources".to_string()),
            affected_resources: vec![
                "res-1".to_string(),
                "res-2".to_string(),
                "res-3".to_string(),
                "res-4".to_string(),
                "res-5".to_string(),
            ],
        };

        assert_eq!(finding.affected_resources.len(), 5);
    }

    #[test]
    fn test_empty_compliance_context() {
        let context = ComplianceContext {
            resource_id: "".to_string(),
            resource_type: "".to_string(),
            frameworks: vec![],
            metadata: HashMap::new(),
        };

        assert!(context.resource_id.is_empty());
        assert!(context.frameworks.is_empty());
        assert!(context.metadata.is_empty());
    }
}
