// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use nestgate_core::Result;

use super::{
    FindingSeverity, ProductionReadinessReport, ProductionReadinessValidator, ReadinessFinding,
};

impl ProductionReadinessValidator {
    /// Generate Findings And Recommendations
    pub(crate) fn generate_findings_and_recommendations(
        &self,
        report: &mut ProductionReadinessReport,
    ) -> Result<()> {
        if !report.zfs_available {
            report.findings.push(ReadinessFinding {
                category: "ZFS Availability".to_string(),
                description: "ZFS is not available on this system".to_string(),
                severity: FindingSeverity::Critical,
                blocking: true,
            });
            report
                .recommendations
                .push("Install ZFS kernel modules and utilities".to_string());
        }

        if !report.real_hardware_detected {
            report.findings.push(ReadinessFinding {
                category: "Hardware Detection".to_string(),
                description: "Running in mock/virtual mode — not real hardware".to_string(),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push("Run on real hardware for production workloads".to_string());
        }

        if !report.mock_dependencies.is_empty() {
            report.findings.push(ReadinessFinding {
                category: "Mock Dependencies".to_string(),
                description: format!(
                    "Mock dependencies active: {}",
                    report.mock_dependencies.join(", ")
                ),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Disable mock mode for production: unset NESTGATE_MOCK_MODE".to_string());
        }

        if !report.performance_validated {
            report.findings.push(ReadinessFinding {
                category: "Performance".to_string(),
                description: "Performance validation failed — insufficient resources".to_string(),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push("Ensure sufficient memory and CPU for production workloads".to_string());
        }

        if !report.security_validated {
            report.findings.push(ReadinessFinding {
                category: "Security".to_string(),
                description: "Security validation failed — encryption not available".to_string(),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Enable ZFS encryption support for secure mode".to_string());
        }

        if !report.configuration_validated {
            report.findings.push(ReadinessFinding {
                category: "Configuration".to_string(),
                description: "Required directories could not be created".to_string(),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Verify NESTGATE_DATA_DIR and NESTGATE_CONFIG_DIR are writable".to_string());
        }

        // Add general production recommendations
        report
            .recommendations
            .push("Review logs for any warnings during operation".to_string());
        report
            .recommendations
            .push("Monitor ZFS pool health and performance metrics".to_string());
        report
            .recommendations
            .push("Ensure backup and recovery procedures are tested".to_string());

        Ok(())
    }
}
