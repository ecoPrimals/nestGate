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
                category: String::from("ZFS Availability"),
                description: String::from("ZFS is not available on this system"),
                severity: FindingSeverity::Critical,
                blocking: true,
            });
            report
                .recommendations
                .push(String::from("Install ZFS kernel modules and utilities"));
        }

        if !report.real_hardware_detected {
            report.findings.push(ReadinessFinding {
                category: String::from("Hardware Detection"),
                description: String::from("Running in mock/virtual mode — not real hardware"),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push(String::from("Run on real hardware for production workloads"));
        }

        if !report.mock_dependencies.is_empty() {
            report.findings.push(ReadinessFinding {
                category: String::from("Mock Dependencies"),
                description: format!(
                    "Mock dependencies active: {}",
                    report.mock_dependencies.join(", ")
                ),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push(String::from("Disable mock mode for production: unset NESTGATE_MOCK_MODE"));
        }

        if !report.performance_validated {
            report.findings.push(ReadinessFinding {
                category: String::from("Performance"),
                description: String::from("Performance validation failed — insufficient resources"),
                severity: FindingSeverity::Warning,
                blocking: false,
            });
            report
                .recommendations
                .push(String::from("Ensure sufficient memory and CPU for production workloads"));
        }

        if !report.security_validated {
            report.findings.push(ReadinessFinding {
                category: String::from("Security"),
                description: String::from("Security validation failed — encryption not available"),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push(String::from("Enable ZFS encryption support for secure mode"));
        }

        if !report.configuration_validated {
            report.findings.push(ReadinessFinding {
                category: String::from("Configuration"),
                description: String::from("Required directories could not be created"),
                severity: FindingSeverity::Error,
                blocking: true,
            });
            report
                .recommendations
                .push(String::from("Verify NESTGATE_DATA_DIR and NESTGATE_CONFIG_DIR are writable"));
        }

        // Add general production recommendations
        report
            .recommendations
            .push(String::from("Review logs for any warnings during operation"));
        report
            .recommendations
            .push(String::from("Monitor ZFS pool health and performance metrics"));
        report
            .recommendations
            .push(String::from("Ensure backup and recovery procedures are tested"));

        Ok(())
    }
}
