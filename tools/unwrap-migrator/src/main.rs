//! # `NestGate` Advanced Unwrap Migrator
//!
//! Enhanced panic migration tool with advanced pattern detection and team-friendly features.
//!
//! ## Features
//! - Smart pattern detection and categorization
//! - Safe automated fixes with confidence scoring
//! - Comprehensive reporting and analytics
//! - Integration with `NestGate` error patterns
//! - Team workflow support

#![allow(dead_code)] // Utility tool with many optional features
#![allow(clippy::disallowed_types)] // Allow HashMap in utility crate

use clap::{Arg, ArgAction, Command};
use std::path::Path;
use tracing::{error, info, warn};
use walkdir::WalkDir;

mod advanced_panic_migrator;
mod enhanced_migrator;
mod nestgate_patterns;
mod refined_nestgate_migrator;
mod reporter;
mod scanner;
mod systematic_migrator;

use crate::advanced_panic_migrator::AdvancedNestGatePanicMigrator;
use crate::enhanced_migrator::EnhancedUnwrapMigrator;

#[tokio::main]
#[allow(clippy::type_complexity)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing with better formatting
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    let matches = Command::new("unwrap-migrator")
        .version("0.3.0")
        .about("🚀 NestGate Advanced Unwrap & Panic Pattern Migrator")
        .long_about(
            "Advanced tool for migrating unsafe panic patterns to safe error handling.\n\
             Designed specifically for NestGate development with team workflow integration.",
        )
        .arg(
            Arg::new("path")
                .value_name("PATH")
                .help("Path to analyze (directory or file)")
                .default_value("./")
                .index(1),
        )
        .arg(
            Arg::new("analyze")
                .short('a')
                .long("analyze")
                .help("🔍 Analyze patterns without making changes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fix")
                .short('f')
                .long("fix")
                .help("🔧 Apply safe automatic fixes")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("🎯 Interactive mode - review each fix")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("report")
                .short('r')
                .long("report")
                .help("📊 Generate detailed HTML report")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("confidence")
                .short('c')
                .long("confidence")
                .value_name("LEVEL")
                .help("🎯 Minimum confidence level (50-100)")
                .default_value("80"),
        )
        .arg(
            Arg::new("priority")
                .short('p')
                .long("priority")
                .value_name("LEVEL")
                .help("⚡ Priority filter: high, medium, low, all")
                .default_value("medium"),
        )
        .arg(
            Arg::new("include-tests")
                .long("include-tests")
                .help("🧪 Include test files in analysis")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("exclude-pattern")
                .long("exclude")
                .value_name("PATTERN")
                .help("🚫 Exclude files matching pattern (regex)")
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("📁 Output report to file"),
        )
        .arg(
            Arg::new("format")
                .long("format")
                .value_name("FORMAT")
                .help("📋 Output format: json, markdown, html")
                .default_value("markdown"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("📢 Verbose output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .help("🔇 Quiet mode - minimal output")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("advanced")
                .long("advanced")
                .help("🎯 Use advanced pattern detection")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("nestgate-mode")
                .long("nestgate-mode")
                .help("🏠 Enable NestGate-specific patterns")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let analyze = matches.get_flag("analyze");
    let fix = matches.get_flag("fix");
    let interactive = matches.get_flag("interactive");
    let report = matches.get_flag("report");
    let include_tests = matches.get_flag("include-tests");
    let verbose = matches.get_flag("verbose");
    let quiet = matches.get_flag("quiet");
    let advanced = matches.get_flag("advanced");
    let nestgate_mode = matches.get_flag("nestgate-mode");

    let confidence = matches
        .get_one::<String>("confidence")
        .unwrap()
        .parse::<u8>()?;

    let priority = matches.get_one::<String>("priority").unwrap();
    let format = matches.get_one::<String>("format").unwrap();
    let output_file = matches.get_one::<String>("output");

    // Validate confidence level
    if !(50..=100).contains(&confidence) {
        error!("❌ Confidence level must be between 50 and 100");
        std::process::exit(1);
    }

    // Print banner
    if !quiet {
        print_banner();
        info!("🚀 Starting NestGate Unwrap Migration Tool v0.3.0");
        info!("📁 Target path: {}", path);
        info!("🎯 Confidence threshold: {}%", confidence);
        info!("⚡ Priority filter: {}", priority);
    }

    // Determine operation mode
    let operation_mode = determine_mode(analyze, fix, interactive, report);

    if !quiet {
        info!("🔍 Operation mode: {}", operation_mode);
    }

    // Validate path
    let target_path = Path::new(path);
    if !target_path.exists() {
        error!("❌ Path does not exist: {}", path);
        std::process::exit(1);
    }

    // Run the appropriate operation
    match operation_mode.as_str() {
        "analyze" => run_analysis(target_path, confidence, include_tests, verbose).await?,
        "fix" => {
            run_fixes(
                target_path,
                confidence,
                include_tests,
                advanced,
                nestgate_mode,
                verbose,
            )
            .await?
        }
        "interactive" => run_interactive(target_path, confidence, include_tests, verbose).await?,
        "report" => run_report(target_path, format, output_file, include_tests, verbose).await?,
        _ => {
            warn!("⚠️  No specific mode selected, defaulting to analysis");
            run_analysis(target_path, confidence, include_tests, verbose).await?;
        }
    }

    if !quiet {
        info!("✅ Operation completed successfully!");
    }

    Ok(())
}

fn print_banner() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║                🚀 NestGate Unwrap Migrator v0.3.0             ║");
    println!("║                                                               ║");
    println!("║  Safe automated migration from panic patterns to robust       ║");
    println!("║  error handling. Built for development teams.                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
}

fn determine_mode(analyze: bool, fix: bool, interactive: bool, report: bool) -> String {
    match (analyze, fix, interactive, report) {
        (_, _, true, _) => "interactive".to_string(),
        (_, true, _, _) => "fix".to_string(),
        (_, _, _, true) => "report".to_string(),
        (true, _, _, _) | (false, false, false, false) => "analyze".to_string(),
    }
}

async fn run_analysis(
    path: &Path,
    confidence: u8,
    include_tests: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔍 Running pattern analysis...");

    let stats = analyze_patterns(path, include_tests).await?;

    info!("📊 Analysis Results:");
    info!("   📁 Files scanned: {}", stats.files_scanned);
    info!("   🎯 Total patterns found: {}", stats.total_patterns());
    info!("   ⚠️  Unwrap calls: {}", stats.unwrap_calls);
    info!("   📝 Expect calls: {}", stats.expect_calls);
    info!("   💥 Panic calls: {}", stats.panic_calls);
    info!("   📋 TODO calls: {}", stats.todo_calls);
    info!("   🚫 Unimplemented calls: {}", stats.unimplemented_calls);

    if stats.total_patterns() > 0 {
        let risk_level = calculate_risk_level(&stats);
        info!("🎯 Risk Assessment: {}", risk_level);

        if verbose {
            print_recommendations(&stats, confidence);
        }
    } else {
        info!("🎉 No problematic patterns found! Your code is already safe.");
    }

    Ok(())
}

async fn run_fixes(
    path: &Path,
    _confidence: u8,
    _include_tests: bool,
    advanced: bool,
    _nestgate_mode: bool,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔧 Running automated fixes...");

    if advanced {
        info!("🎯 Using advanced pattern migrator");
        let mut migrator = AdvancedNestGatePanicMigrator::new(false)?;
        let results = migrator.migrate_directory(path).await?;

        info!("✅ Advanced migration completed:");
        info!("   📁 Files processed: {}", results.files_scanned);
        info!("   🎯 Patterns fixed: {}", results.migrations_applied);
        info!(
            "   📊 Success rate: {:.1}%",
            (results.migrations_applied as f32 / results.panic_patterns_found.max(1) as f32)
                * 100.0
        );
    } else {
        info!("🎯 Using standard enhanced migrator");
        let migrator = EnhancedUnwrapMigrator::new(false);
        let results = migrator.migrate_crate(path).await?;

        info!("✅ Standard migration completed:");
        info!("   🔧 Migrations applied: {}", results.migrations_applied);
    }

    info!("💡 Tip: Run with --analyze to verify changes");
    Ok(())
}

async fn run_interactive(
    _path: &Path,
    _confidence: u8,
    _include_tests: bool,
    _verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    warn!("🚧 Interactive mode is under development");
    info!("📋 This feature will allow reviewing each fix before applying");
    info!("💡 For now, use --analyze to review patterns, then --fix to apply");
    Ok(())
}

async fn run_report(
    path: &Path,
    format: &str,
    output_file: Option<&String>,
    include_tests: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("📊 Generating detailed report...");

    let stats = analyze_patterns(path, include_tests).await?;
    let report_content = generate_report(&stats, format, verbose)?;

    match output_file {
        Some(file) => {
            tokio::fs::write(file, report_content).await?;
            info!("📁 Report saved to: {}", file);
        }
        None => {
            println!("{report_content}");
        }
    }

    Ok(())
}

async fn analyze_patterns(
    path: &Path,
    include_tests: bool,
) -> Result<PatternStats, Box<dyn std::error::Error>> {
    let mut stats = PatternStats::default();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    // Skip test files unless explicitly included
                    if !include_tests && is_test_file(entry.path()) {
                        continue;
                    }

                    let content = tokio::fs::read_to_string(entry.path()).await?;
                    stats.files_scanned += 1;

                    // Count different pattern types
                    stats.unwrap_calls += content.matches(".unwrap()").count();
                    stats.expect_calls += content.matches(".expect(").count();
                    stats.panic_calls += content.matches("panic!(").count();
                    stats.todo_calls += content.matches("todo!(").count();
                    stats.unimplemented_calls += content.matches("unimplemented!").count();

                    if stats.has_patterns(&content) {
                        stats.files_with_patterns += 1;
                    }
                }
            }
        }
    }

    Ok(stats)
}

fn is_test_file(path: &Path) -> bool {
    path.to_string_lossy().contains("/test")
        || path.to_string_lossy().contains("_test.rs")
        || path.to_string_lossy().contains("/tests/")
        || path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("test_"))
}

fn calculate_risk_level(stats: &PatternStats) -> &'static str {
    let total = stats.total_patterns();
    let panic_ratio = stats.panic_calls as f32 / total.max(1) as f32;

    match (total, panic_ratio) {
        (0..=10, _) => "🟢 LOW",
        (11..=50, p) if p < 0.1 => "🟡 MEDIUM",
        (11..=50, _) => "🟠 HIGH",
        (_, p) if p > 0.2 => "🔴 CRITICAL",
        _ => "🟠 HIGH",
    }
}

fn print_recommendations(stats: &PatternStats, confidence: u8) {
    info!("💡 Recommendations:");

    if stats.unwrap_calls > 0 {
        info!("   🔧 Replace .unwrap() with proper error handling");
    }

    if stats.expect_calls > 0 {
        info!("   📝 Review .expect() calls for better error messages");
    }

    if stats.panic_calls > 0 {
        info!("   💥 Replace panic!() with Result-based error handling");
    }

    if stats.todo_calls > 0 {
        info!("   📋 Implement TODO items or convert to compile-time warnings");
    }

    info!(
        "   🎯 Run with --fix --confidence {} to apply safe fixes",
        confidence
    );
    info!("   📊 Run with --report to get detailed analysis");
}

fn generate_report(
    stats: &PatternStats,
    format: &str,
    verbose: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    match format {
        "json" => Ok(serde_json::to_string_pretty(stats)?),
        "markdown" => Ok(generate_markdown_report(stats, verbose)),
        "html" => Ok(generate_html_report(stats, verbose)),
        _ => {
            warn!("⚠️  Unknown format '{}', using markdown", format);
            Ok(generate_markdown_report(stats, verbose))
        }
    }
}

fn generate_markdown_report(stats: &PatternStats, verbose: bool) -> String {
    let mut report = String::new();

    report.push_str("# 🚀 NestGate Unwrap Migration Report\n\n");
    report.push_str(&format!(
        "**Generated**: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));

    report.push_str("## 📊 Summary\n\n");
    report.push_str(&format!("- **Files Scanned**: {}\n", stats.files_scanned));
    report.push_str(&format!(
        "- **Files with Patterns**: {}\n",
        stats.files_with_patterns
    ));
    report.push_str(&format!(
        "- **Total Patterns**: {}\n\n",
        stats.total_patterns()
    ));

    report.push_str("## 🎯 Pattern Breakdown\n\n");
    report.push_str("| Pattern Type | Count | Risk Level |\n");
    report.push_str("|--------------|-------|------------|\n");
    report.push_str(&format!(
        "| `.unwrap()` | {} | {} |\n",
        stats.unwrap_calls,
        pattern_risk(stats.unwrap_calls)
    ));
    report.push_str(&format!(
        "| `.expect()` | {} | {} |\n",
        stats.expect_calls,
        pattern_risk(stats.expect_calls)
    ));
    report.push_str(&format!(
        "| `panic!()` | {} | {} |\n",
        stats.panic_calls,
        panic_risk(stats.panic_calls)
    ));
    report.push_str(&format!(
        "| `todo!()` | {} | {} |\n",
        stats.todo_calls,
        todo_risk(stats.todo_calls)
    ));
    report.push_str(&format!(
        "| `unimplemented!()` | {} | {} |\n\n",
        stats.unimplemented_calls,
        unimpl_risk(stats.unimplemented_calls)
    ));

    if verbose {
        report.push_str("## 💡 Recommendations\n\n");
        report.push_str("1. **Priority**: Address `panic!()` calls first\n");
        report.push_str("2. **Safety**: Use `--confidence 80` for conservative fixes\n");
        report.push_str("3. **Testing**: Run with `--include-tests` for complete analysis\n");
        report.push_str("4. **Verification**: Always test after applying fixes\n\n");
    }

    report.push_str("---\n");
    report.push_str("*Generated by NestGate Unwrap Migrator v0.3.0*\n");

    report
}

fn generate_html_report(stats: &PatternStats, _verbose: bool) -> String {
    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>NestGate Unwrap Migration Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 40px; }}
        .header {{ color: #2563eb; }}
        .stats {{ background: #f3f4f6; padding: 20px; border-radius: 8px; }}
        .risk-high {{ color: #dc2626; }}
        .risk-medium {{ color: #d97706; }}
        .risk-low {{ color: #16a34a; }}
    </style>
</head>
<body>
    <h1 class="header">🚀 NestGate Unwrap Migration Report</h1>
    <div class="stats">
        <h2>📊 Summary</h2>
        <p><strong>Files Scanned:</strong> {}</p>
        <p><strong>Files with Patterns:</strong> {}</p>
        <p><strong>Total Patterns:</strong> {}</p>
        <h3>Pattern Breakdown</h3>
        <ul>
            <li>Unwrap calls: {}</li>
            <li>Expect calls: {}</li>
            <li>Panic calls: {}</li>
            <li>TODO calls: {}</li>
            <li>Unimplemented calls: {}</li>
        </ul>
    </div>
    <p><em>Generated by NestGate Unwrap Migrator v0.3.0</em></p>
</body>
</html>"#,
        stats.files_scanned,
        stats.files_with_patterns,
        stats.total_patterns(),
        stats.unwrap_calls,
        stats.expect_calls,
        stats.panic_calls,
        stats.todo_calls,
        stats.unimplemented_calls
    )
}

fn pattern_risk(count: usize) -> &'static str {
    match count {
        0 => "✅",
        1..=5 => "🟡 Low",
        6..=20 => "🟠 Medium",
        _ => "🔴 High",
    }
}

fn panic_risk(count: usize) -> &'static str {
    match count {
        0 => "✅",
        1..=2 => "🟠 Medium",
        _ => "🔴 High",
    }
}

fn todo_risk(count: usize) -> &'static str {
    match count {
        0 => "✅",
        1..=10 => "🟡 Low",
        _ => "🟠 Medium",
    }
}

fn unimpl_risk(count: usize) -> &'static str {
    match count {
        0 => "✅",
        1..=3 => "🟡 Low",
        _ => "🟠 Medium",
    }
}

#[derive(Debug, Default, serde::Serialize)]
struct PatternStats {
    files_scanned: usize,
    files_with_patterns: usize,
    unwrap_calls: usize,
    expect_calls: usize,
    panic_calls: usize,
    todo_calls: usize,
    unimplemented_calls: usize,
}

impl PatternStats {
    fn total_patterns(&self) -> usize {
        self.unwrap_calls
            + self.expect_calls
            + self.panic_calls
            + self.todo_calls
            + self.unimplemented_calls
    }

    fn has_patterns(&self, content: &str) -> bool {
        content.contains(".unwrap()")
            || content.contains(".expect(")
            || content.contains("panic!(")
            || content.contains("todo!(")
            || content.contains("unimplemented!")
    }
}
