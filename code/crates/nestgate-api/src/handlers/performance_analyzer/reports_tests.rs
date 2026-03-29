// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! Tests for performance analyzer reports

use super::analyzer::*;
use super::reports::*;

/// Creates  Test Analysis
fn create_test_analysis() -> AnalysisResult {
    AnalysisResult {
        overall_score: 85.5,
        cpu_analysis: ComponentAnalysis {
            score: 90.0,
            status: PerformanceStatus::Good,
            details: "CPU is performing well".to_string(),
        },
        memory_analysis: ComponentAnalysis {
            score: 85.0,
            status: PerformanceStatus::Good,
            details: "Memory usage is acceptable".to_string(),
        },
        disk_analysis: ComponentAnalysis {
            score: 80.0,
            status: PerformanceStatus::Warning,
            details: "Disk I/O could be improved".to_string(),
        },
        network_analysis: ComponentAnalysis {
            score: 88.0,
            status: PerformanceStatus::Good,
            details: "Network performance is good".to_string(),
        },
        recommendations: vec![
            "Consider upgrading disk storage".to_string(),
            "Optimize I/O patterns".to_string(),
        ],
        timestamp: std::time::SystemTime::now(),
    }
}

#[test]
fn test_report_config_default() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    assert!(config.include_detailed_analysis);
    assert!(!config.include_trends);
}

#[test]
fn test_report_generator_new() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    assert!(generator.config.include_detailed_analysis);
}

#[test]
fn test_generate_report_basic() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let report = generator.generate_report(&analysis);

    assert_eq!(report.overall_score, 85.5);
    assert!(matches!(report.status, OverallStatus::Warning));
    assert_eq!(report.recommendations.len(), 2);
}

#[test]
fn test_generate_report_includes_all_components() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let report = generator.generate_report(&analysis);

    assert_eq!(report.component_reports.len(), 4);
    assert!(report.component_reports.contains_key("cpu"));
    assert!(report.component_reports.contains_key("memory"));
    assert!(report.component_reports.contains_key("disk"));
    assert!(report.component_reports.contains_key("network"));
}

#[test]
fn test_generate_report_component_scores() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let report = generator.generate_report(&analysis);

    assert_eq!(
        report
            .component_reports
            .get("cpu")
            .expect("CPU report should exist")
            .score,
        90.0
    );
    assert_eq!(
        report
            .component_reports
            .get("memory")
            .expect("Memory report should exist")
            .score,
        85.0
    );
    assert_eq!(
        report
            .component_reports
            .get("disk")
            .expect("Disk report should exist")
            .score,
        80.0
    );
    assert_eq!(
        report
            .component_reports
            .get("network")
            .expect("Network report should exist")
            .score,
        88.0
    );
}

#[test]
fn test_overall_status_determination_healthy() {
    let analysis = AnalysisResult {
        overall_score: 95.0,
        cpu_analysis: ComponentAnalysis {
            score: 95.0,
            status: PerformanceStatus::Good,
            details: String::new(),
        },
        memory_analysis: ComponentAnalysis {
            score: 95.0,
            status: PerformanceStatus::Good,
            details: String::new(),
        },
        disk_analysis: ComponentAnalysis {
            score: 95.0,
            status: PerformanceStatus::Good,
            details: String::new(),
        },
        network_analysis: ComponentAnalysis {
            score: 95.0,
            status: PerformanceStatus::Good,
            details: String::new(),
        },
        recommendations: vec![],
        timestamp: std::time::SystemTime::now(),
    };

    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let report = generator.generate_report(&analysis);

    assert!(matches!(report.status, OverallStatus::Healthy));
}

#[test]
fn test_overall_status_determination_warning() {
    let analysis = create_test_analysis();

    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let report = generator.generate_report(&analysis);

    assert!(matches!(report.status, OverallStatus::Warning));
}

#[test]
fn test_overall_status_determination_critical() {
    let analysis = AnalysisResult {
        overall_score: 30.0,
        cpu_analysis: ComponentAnalysis {
            score: 20.0,
            status: PerformanceStatus::Critical,
            details: String::new(),
        },
        memory_analysis: ComponentAnalysis {
            score: 40.0,
            status: PerformanceStatus::Warning,
            details: String::new(),
        },
        disk_analysis: ComponentAnalysis {
            score: 30.0,
            status: PerformanceStatus::Warning,
            details: String::new(),
        },
        network_analysis: ComponentAnalysis {
            score: 30.0,
            status: PerformanceStatus::Warning,
            details: String::new(),
        },
        recommendations: vec![],
        timestamp: std::time::SystemTime::now(),
    };

    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let report = generator.generate_report(&analysis);

    assert!(matches!(report.status, OverallStatus::Critical));
}

#[test]
fn test_report_summary_generation() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let report = generator.generate_report(&analysis);

    assert!(report.summary.contains("85.5"));
    assert!(report.summary.contains("4 components"));
    assert!(report.summary.contains("2 recommendations"));
}

#[test]
fn test_report_has_unique_id() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let report1 = generator.generate_report(&analysis);
    let report2 = generator.generate_report(&analysis);

    assert_ne!(report1.report_id, report2.report_id);
}

#[test]
fn test_generate_multi_format_report() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let multi_report = generator.generate_multi_format(&analysis);

    assert!(!multi_report.json.is_empty());
    assert!(!multi_report.markdown.is_empty());
    assert!(!multi_report.html.is_empty());
    assert!(!multi_report.csv.is_empty());
}

#[test]
fn test_markdown_report_format() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let multi_report = generator.generate_multi_format(&analysis);

    assert!(multi_report.markdown.contains("# Performance Report"));
    assert!(multi_report.markdown.contains("85.5/100"));
    assert!(multi_report.markdown.contains("## Summary"));
    assert!(multi_report.markdown.contains("## Recommendations"));
}

#[test]
fn test_html_report_format() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let multi_report = generator.generate_multi_format(&analysis);

    assert!(multi_report.html.contains("<html>"));
    assert!(
        multi_report
            .html
            .contains("<title>Performance Report</title>")
    );
    assert!(multi_report.html.contains("85.5/100"));
    assert!(multi_report.html.contains("</html>"));
}

#[test]
fn test_csv_report_format() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let multi_report = generator.generate_multi_format(&analysis);

    assert!(multi_report.csv.contains("Component,Score,Status,Details"));
    assert!(multi_report.csv.contains("cpu"));
    assert!(multi_report.csv.contains("memory"));
    assert!(multi_report.csv.contains("disk"));
    assert!(multi_report.csv.contains("network"));
}

#[test]
fn test_json_report_format() {
    #[allow(deprecated)]
    let config = ReportConfig::default();
    let generator = ReportGenerator::new(config);
    let analysis = create_test_analysis();

    let multi_report = generator.generate_multi_format(&analysis);

    assert!(multi_report.json.contains("overall_score"));
    assert!(multi_report.json.contains("component_reports"));
    assert!(multi_report.json.contains("recommendations"));
}

#[test]
fn test_component_report_structure() {
    let component = ComponentReport {
        score: 85.5,
        status: PerformanceStatus::Good,
        details: "Test details".to_string(),
    };

    assert_eq!(component.score, 85.5);
    assert!(matches!(component.status, PerformanceStatus::Good));
    assert_eq!(component.details, "Test details");
}

#[test]
fn test_report_config_with_custom_settings() {
    #[allow(deprecated)]
    let config = ReportConfig {
        include_detailed_analysis: false,
        include_trends: true,
        default_format: ReportFormat::Markdown,
    };

    assert!(!config.include_detailed_analysis);
    assert!(config.include_trends);
}
