// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **PERFORMANCE REPORT GENERATION**
//!
//! Generate comprehensive performance reports from analysis results.

use super::analyzer::{AnalysisResult, PerformanceStatus};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Performance report generator
#[derive(Debug)]
/// Reportgenerator
pub struct ReportGenerator {
    /// Report configuration
    pub config: ReportConfig,
}

impl ReportGenerator {
    /// Create new report generator
    #[must_use]
    pub const fn new(config: ReportConfig) -> Self {
        Self { config }
    }

    /// Generate comprehensive performance report
    #[must_use]
    pub fn generate_report(&self, analysis: &AnalysisResult) -> PerformanceReport {
        PerformanceReport {
            overall_score: analysis.overall_score,
            status: self.determine_overall_status(analysis),
            summary: self.generate_summary(analysis),
            component_reports: self.generate_component_reports(analysis),
            recommendations: analysis.recommendations.clone(),
            generated_at: std::time::SystemTime::now(),
            report_id: uuid::Uuid::new_v4().to_string(),
        }
    }

    /// Generate report in multiple formats
    #[must_use]
    pub fn generate_multi_format(&self, analysis: &AnalysisResult) -> MultiFormatReport {
        let base_report = self.generate_report(analysis);

        MultiFormatReport {
            json: serde_json::to_string_pretty(&base_report).unwrap_or_default(),
            markdown: self.generate_markdown_report(&base_report),
            html: self.generate_html_report(&base_report),
            csv: self.generate_csv_report(&base_report),
        }
    }

    /// Determine Overall Status
    fn determine_overall_status(&self, analysis: &AnalysisResult) -> OverallStatus {
        let components = [
            &analysis.cpu_analysis.status,
            &analysis.memory_analysis.status,
            &analysis.disk_analysis.status,
            &analysis.network_analysis.status,
        ];

        let has_critical = components
            .iter()
            .any(|s| matches!(s, PerformanceStatus::Critical));
        let has_warning = components
            .iter()
            .any(|s| matches!(s, PerformanceStatus::Warning));

        if has_critical && self.config.include_detailed_analysis {
            OverallStatus::Critical
        } else if has_critical || has_warning {
            OverallStatus::Warning
        } else {
            OverallStatus::Healthy
        }
    }

    /// Generate Summary
    fn generate_summary(&self, analysis: &AnalysisResult) -> String {
        let trends_note = if self.config.include_trends {
            " Trend series omitted in this build."
        } else {
            ""
        };
        format!(
            "System performance score: {:.1}/100. {} components analyzed with {} recommendations. Default export format: {:?}.{}{}",
            analysis.overall_score,
            4, // CPU, Memory, Disk, Network
            analysis.recommendations.len(),
            self.config.default_format,
            trends_note,
            if self.config.include_detailed_analysis {
                " Full component detail enabled."
            } else {
                " Summary-only mode (detailed component analysis disabled in report config)."
            }
        )
    }

    /// Generate Component Reports
    fn generate_component_reports(
        &self,
        analysis: &AnalysisResult,
    ) -> HashMap<String, ComponentReport> {
        let mut reports = HashMap::new();

        let detail = |s: String| -> String {
            if self.config.include_detailed_analysis {
                s
            } else {
                s.chars().take(120).collect::<String>()
            }
        };

        reports.insert(
            "cpu".to_string(),
            ComponentReport {
                score: analysis.cpu_analysis.score,
                status: analysis.cpu_analysis.status.clone(),
                details: detail(analysis.cpu_analysis.details.clone()),
            },
        );

        reports.insert(
            "memory".to_string(),
            ComponentReport {
                score: analysis.memory_analysis.score,
                status: analysis.memory_analysis.status.clone(),
                details: detail(analysis.memory_analysis.details.clone()),
            },
        );

        reports.insert(
            "disk".to_string(),
            ComponentReport {
                score: analysis.disk_analysis.score,
                status: analysis.disk_analysis.status.clone(),
                details: detail(analysis.disk_analysis.details.clone()),
            },
        );

        reports.insert(
            "network".to_string(),
            ComponentReport {
                score: analysis.network_analysis.score,
                status: analysis.network_analysis.status.clone(),
                details: detail(analysis.network_analysis.details.clone()),
            },
        );

        reports
    }

    /// Generate Markdown Report
    fn generate_markdown_report(&self, report: &PerformanceReport) -> String {
        format!(
            "# Performance Report\n\n**Report ID**: {}\n**Preferred format**: {:?}\n**Generated**: {:?}\n**Overall Score**: {:.1}/100\n**Status**: {:?}\n\n## Summary\n{}\n\n## Recommendations\n{}\n",
            report.report_id,
            self.config.default_format,
            report.generated_at,
            report.overall_score,
            report.status,
            report.summary,
            report
                .recommendations
                .iter()
                .map(|r| format!("- {r}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Generate Html Report
    fn generate_html_report(&self, report: &PerformanceReport) -> String {
        format!(
            "<html><head><title>Performance Report</title><meta name=\"report-format\" content=\"{:?}\" /></head><body><h1>Performance Report</h1><p><strong>Score:</strong> {:.1}/100</p><p><strong>Status:</strong> {:?}</p><p>{}</p></body></html>",
            self.config.default_format, report.overall_score, report.status, report.summary
        )
    }

    /// Generate Csv Report
    fn generate_csv_report(&self, report: &PerformanceReport) -> String {
        let rows = report
            .component_reports
            .iter()
            .map(|(name, comp)| {
                format!(
                    "{},{:.1},{:?},\"{}\"",
                    name,
                    comp.score,
                    comp.status,
                    comp.details.replace('\"', "\"\"")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "# nestgate report config: include_trends={} include_detailed_analysis={} default_format={:?}\nComponent,Score,Status,Details\n{rows}",
            self.config.include_trends,
            self.config.include_detailed_analysis,
            self.config.default_format
        )
    }
}

/// Report generation configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::ReportConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ReportConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
/// Configuration for Report
pub struct ReportConfig {
    /// Include detailed component analysis
    pub include_detailed_analysis: bool,
    /// Include historical trends
    pub include_trends: bool,
    /// Report format preferences
    pub default_format: ReportFormat,
}

impl Default for ReportConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            include_detailed_analysis: true,
            include_trends: false,
            default_format: ReportFormat::Json,
        }
    }
}

/// Report format options
#[derive(Debug, Clone)]
/// Reportformat
pub enum ReportFormat {
    /// JSON format for API consumption
    Json,
    /// Markdown format for documentation
    Markdown,
    /// HTML format for web display
    Html,
    /// CSV format for data analysis
    Csv,
}

/// Complete performance report
#[derive(Debug, Serialize, Deserialize)]
/// Performancereport
pub struct PerformanceReport {
    /// Overall system performance score (0.0-100.0)
    pub overall_score: f64,
    /// Overall system health status
    pub status: OverallStatus,
    /// Executive summary of performance analysis
    pub summary: String,
    /// Detailed reports for each system component
    pub component_reports: HashMap<String, ComponentReport>,
    /// Performance improvement recommendations
    pub recommendations: Vec<String>,
    /// When this report was generated
    pub generated_at: std::time::SystemTime,
    /// Unique identifier for this report
    pub report_id: String,
}

/// Component-specific report
#[derive(Debug, Serialize, Deserialize)]
/// Componentreport
pub struct ComponentReport {
    /// Component performance score (0.0-100.0)
    pub score: f64,
    /// Current performance status level
    pub status: PerformanceStatus,
    /// Detailed analysis and recommendations for this component
    pub details: String,
}

/// Overall system status
#[derive(Debug, Serialize, Deserialize)]
/// Status values for Overall
pub enum OverallStatus {
    /// System is performing optimally
    Healthy,
    /// System has performance issues but is functional
    Warning,
    /// System has critical performance problems requiring immediate attention
    Critical,
}

/// Multi-format report output
#[derive(Debug)]
/// Multiformatreport
pub struct MultiFormatReport {
    /// Report in JSON format for API consumption
    pub json: String,
    /// Report in Markdown format for documentation
    pub markdown: String,
    /// Report in HTML format for web display
    pub html: String,
    /// Report in CSV format for data analysis
    pub csv: String,
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
/// Type alias for Reportconfigcanonical
pub type ReportConfigCanonical =
    nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ReportConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
