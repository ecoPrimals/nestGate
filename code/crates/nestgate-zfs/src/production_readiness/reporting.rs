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
                category: "ZFS Availability".into(),
                description: "ZFS is not available on this system".into(),
                severity: FindingSeverity::Critical,
                blocking: true,
            });
            report
                .recommendations
                .push("Install ZFS kernel modules and utilities".into());
        }

        if !report.real_hardware_detected {
            report.findings.push(ReadinessFinding {
                category: "Hardware Detection".into(),
                description: "Running in mock/virtual mode — not real hardware".into(),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push("Run on real hardware for production workloads".into());
        }

        if !report.mock_dependencies.is_empty() {
            report.findings.push(ReadinessFinding {
                category: "Mock Dependencies".into(),
                description: format!(
                    "Mock dependencies active: {}",
                    report.mock_dependencies.join(", ")
                ),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Disable mock mode for production: unset NESTGATE_MOCK_MODE".into());
        }

        if !report.performance_validated {
            report.findings.push(ReadinessFinding {
                category: "Performance".into(),
                description: "Performance validation failed — insufficient resources".into(),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push("Ensure sufficient memory and CPU for production workloads".into());
        }

        if !report.security_validated {
            report.findings.push(ReadinessFinding {
                category: "Security".into(),
                description: "Security validation failed — encryption not available".into(),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Enable ZFS encryption support for secure mode".into());
        }

        if !report.configuration_validated {
            report.findings.push(ReadinessFinding {
                category: "Configuration".into(),
                description: "Required directories could not be created".into(),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push("Verify NESTGATE_DATA_DIR and NESTGATE_CONFIG_DIR are writable".into());
        }

        // Add general production recommendations
        report
            .recommendations
            .push("Review logs for any warnings during operation".into());
        report
            .recommendations
            .push("Monitor ZFS pool health and performance metrics".into());
        report
            .recommendations
            .push("Ensure backup and recovery procedures are tested".into());

        Ok(())
    }
}
