//! Systematic Unwrap Migrator - CLI Tool
//!
//! Command-line interface for the systematic migration tool that eliminates
//! unwrap/expect calls using graceful error handling.

use clap::{Arg, Command};
use tracing::{info, warn};
use tracing_subscriber;

mod systematic_migrator;
mod nestgate_patterns;
mod enhanced_migrator;
mod scanner;
mod reporter;

use unwrap_migrator::{
    enhanced_migrator::EnhancedUnwrapMigrator,
    systematic_migrator::SystematicUnwrapMigrator,
    context_fixer::ContextAwareFixer,
    compilation_fixer::CompilationFixer,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let matches = Command::new("unwrap-migrator")
        .version("1.0.0")
        .about("🔄 Systematic Unwrap/Expect Migrator - Eliminate crashes with graceful error handling")
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Root path to scan for Rust files")
                .default_value("./code/crates")
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show what would be changed without applying changes")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("apply")
                .long("apply")
                .help("Apply the migration changes to files")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("stats-only")
                .long("stats-only")
                .help("Show statistics without performing migration")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("nestgate-mode")
                .long("nestgate-mode")
                .help("Use NestGate-specific migration patterns")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("include-tests")
                .long("include-tests")
                .help("Include test files in migration (normally excluded)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("production-only")
                .long("production-only")
                .help("Only process production code (default behavior)")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show detailed file-by-file analysis")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("context-fix")
                .long("context-fix")
                .help("Fix context-specific issues from previous migration")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("compilation-fix")
                .long("compilation-fix")
                .help("Fix compilation errors identified in audit")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("focused")
                .long("focused")
                .help("Focus on files with highest unwrap counts first")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("max-files")
                .long("max-files")
                .value_name("COUNT")
                .help("Maximum number of files to process in focused mode")
                .default_value("10")
        )
        .get_matches();

    let default_path = "./code/crates".to_string();
    let path = matches.get_one::<String>("path").unwrap_or(&default_path);
    let dry_run = matches.get_flag("dry-run");
    let apply = matches.get_flag("apply");
    let stats_only = matches.get_flag("stats-only");
    let nestgate_mode = matches.get_flag("nestgate-mode");
    let include_tests = matches.get_flag("include-tests");
    let production_only = matches.get_flag("production-only");
    let verbose = matches.get_flag("verbose");
    let context_fix = matches.get_flag("context-fix");
    let compilation_fix = matches.get_flag("compilation-fix");
    let focused = matches.get_flag("focused");
    let max_files = matches.get_one::<String>("max-files").unwrap().parse::<usize>().unwrap();

    info!("🚀 Starting Systematic Unwrap Migration");
    info!("📁 Target path: {}", path);
    if nestgate_mode {
        info!("🧬 Using NestGate-specific patterns");
    }
    if include_tests {
        info!("🧪 Including test files in analysis");
    }
    if production_only {
        info!("🏭 Production code only");
    }
    if context_fix {
        info!("🔧 Context-aware fix mode");
    }
    if compilation_fix {
        info!("🛠️ Compilation fix mode");
    }
    if focused {
        info!("🎯 Focusing on files with highest unwrap counts (max {} files)", max_files);
    }
    
    let mut migrator = SystematicUnwrapMigrator::new();
    
    // Add NestGate-specific patterns if requested
    if nestgate_mode {
        migrator.add_nestgate_patterns();
    }
    
    // Handle each mode separately and return early
    if stats_only {
        handle_stats_mode(&migrator)?;
        return Ok(());
    }

    if context_fix {
        handle_context_fix_mode(path, dry_run).await?;
        return Ok(());
    }

    if compilation_fix {
        handle_compilation_fix_mode(path, dry_run).await?;
        return Ok(());
    }
    
    if dry_run {
        handle_enhanced_dry_run_mode(path, nestgate_mode, include_tests, verbose).await?;
        return Ok(());
    }

    if apply {
        handle_apply_mode(&migrator, path, include_tests, production_only).await?;
        return Ok(());
    }

    // Default case
    info!("ℹ️  No action specified. Use --dry-run to preview or --apply to execute migration");
    info!("   Use --stats-only to see tool statistics");
    info!("   Use --nestgate-mode for NestGate-specific patterns");
    Ok(())
}

async fn handle_context_fix_mode(path: &str, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔧 Context Fix Mode - Analyzing and fixing migration issues");
    
    let fixer = ContextAwareFixer::new(dry_run);
    let mut all_fixes = Vec::new();
    
    // Find all Rust files in the path
    let mut rust_files = Vec::new();
    collect_rust_files(std::path::Path::new(path), &mut rust_files).await?;
    
    info!("📁 Found {} Rust files to analyze", rust_files.len());
    
    for file_path in rust_files {
        match fixer.fix_file(&file_path).await {
            Ok(fixes) => {
                if !fixes.is_empty() {
                    info!("🔍 Found {} issues in {}", fixes.len(), file_path.display());
                    all_fixes.extend(fixes);
                }
            }
            Err(e) => {
                warn!("⚠️ Failed to analyze {}: {}", file_path.display(), e);
            }
        }
    }
    
    if all_fixes.is_empty() {
        info!("✨ No context issues found!");
        return Ok(());
    }
    
    info!("📊 Found {} total context issues across {} files", 
          all_fixes.len(), 
          all_fixes.iter().map(|f| &f.file_path).collect::<std::collections::HashSet<_>>().len());
    
    // Group fixes by type for reporting
    let mut fix_counts = std::collections::HashMap::new();
    for fix in &all_fixes {
        *fix_counts.entry(format!("{:?}", fix.fix_type)).or_insert(0) += 1;
    }
    
    println!("\n📋 CONTEXT FIX ANALYSIS");
    println!("=======================");
    for (fix_type, count) in fix_counts {
        println!("🔧 {}: {} issues", fix_type, count);
    }
    
    if !dry_run {
        info!("🚀 Applying context fixes...");
        fixer.apply_fixes(&all_fixes).await?;
        info!("✅ Context fixes applied successfully!");
    } else {
        info!("🔍 DRY RUN: Use --apply to execute fixes");
    }
    
    Ok(())
}

async fn handle_compilation_fix_mode(path: &str, dry_run: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("🛠️ Compilation Fix Mode - Fixing critical compilation errors");
    
    let fixer = CompilationFixer::new(dry_run);
    
    // Apply compilation fixes
    let results = fixer.fix_compilation_errors(std::path::Path::new(path)).await?;
    
    println!("{}", results.generate_summary());
    
    if results.fixes_applied > 0 {
        println!("\n🎉 SUCCESS: {} compilation fixes applied!", results.fixes_applied);
        println!("🔄 Next: Run 'cargo check --all-features' to verify the fixes");
    } else {
        println!("ℹ️ No compilation errors found that match our fix patterns");
    }
    
    Ok(())
}

async fn collect_rust_files(dir: &std::path::Path, files: &mut Vec<std::path::PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let mut entries = tokio::fs::read_dir(dir).await?;
    
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        
        if path.is_dir() {
            // Skip certain directories
            if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                if dir_name == "target" || dir_name == ".git" || dir_name == "node_modules" {
                    continue;
                }
            }
            Box::pin(collect_rust_files(&path, files)).await?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path);
        }
    }
    
    Ok(())
}

fn handle_stats_mode(migrator: &SystematicUnwrapMigrator) -> Result<(), Box<dyn std::error::Error>> {
    info!("📊 Statistics Mode - No changes will be made");
    let stats = migrator.get_statistics();
    println!("\n🔧 MIGRATION TOOL STATISTICS");
    println!("============================");
    println!("Available Patterns: {}", stats.available_patterns);
    println!("Files Processed: {}", stats.files_processed);
    println!("Patterns Applied: {}", stats.patterns_applied);
    Ok(())
}

async fn handle_enhanced_dry_run_mode(path: &str, nestgate_mode: bool, include_tests: bool, verbose: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔍 Enhanced Dry Run Mode - Scanning for actual patterns");
    
    let mut enhanced_migrator = EnhancedUnwrapMigrator::new(true); // dry_run = true
    
    // Configure the migrator based on flags
    if include_tests {
        enhanced_migrator.set_include_tests(true);
    }
    
    let report = enhanced_migrator.generate_report(std::path::Path::new(path)).await?;
    
    println!("\n📋 DETAILED MIGRATION ANALYSIS");
    println!("==============================");
    println!("📁 Files Scanned: {}", report.total_files_scanned);
    println!("🔍 Patterns Found: {}", report.total_patterns_found);
    
    if include_tests {
        println!("🧪 Test files included in analysis");
    } else {
        println!("🏭 Production code only (use --include-tests for full analysis)");
    }
    
    if report.total_patterns_found == 0 {
        println!("✨ No unwrap/expect patterns found in target code!");
        println!("🎉 Your codebase is already panic-safe!");
        return Ok(());
    }
    
    println!("\n📊 SEVERITY BREAKDOWN:");
    for (severity, count) in &report.patterns_by_severity {
        let emoji = match severity.as_str() {
            "Critical" => "🔴",
            "High" => "🟠", 
            "Medium" => "🟡",
            "Low" => "🟢",
            _ => "⚪",
        };
        println!("  {} {}: {} patterns", emoji, severity, count);
    }
    
    println!("\n📈 TOP FILES WITH ISSUES:");
    let display_count = if verbose { report.file_statistics.len() } else { 10.min(report.file_statistics.len()) };
    for (file, count) in report.file_statistics.iter().take(display_count) {
        println!("  📄 {}: {} patterns", file, count);
    }
    
    if !verbose && report.file_statistics.len() > 10 {
        println!("  ... and {} more files (use --verbose to see all)", report.file_statistics.len() - 10);
    }
    
    if nestgate_mode {
        println!("\n🧬 NestGate-Specific Patterns Available:");
        println!("  ✅ Service start/stop unwraps → Service error handling");
        println!("  ✅ Health check unwraps → Health error handling"); 
        println!("  ✅ Storage operation unwraps → Storage error handling");
        println!("  ✅ RwLock unwraps → Poison recovery patterns");
        println!("  ✅ Config expect calls → Configuration error handling");
        println!("  ✅ Runtime creation unwraps → Resource error handling");
    }
    
    println!("\n🎯 RECOMMENDATIONS:");
    if report.total_patterns_found > 50 {
        println!("  🔥 High pattern count detected - consider batch processing");
        println!("  📋 Focus on Critical and High severity patterns first");
        println!("  🧪 Test production code changes before applying to full codebase");
    } else {
        println!("  ✅ Manageable pattern count - safe to apply migration");
    }
    
    println!("\n🔄 Next Steps:");
    println!("  1. Review the patterns above");
    println!("  2. Run with --apply to execute the migration");
    println!("  3. Test your code after migration");
    println!("  4. Run clippy and fmt for final cleanup");
    
    Ok(())
}

fn handle_dry_run_mode(path: &str, nestgate_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("🔍 Dry Run Mode - Scanning for migration opportunities");
    
    info!("📋 MIGRATION PREVIEW");
    info!("===================");
    info!("This would scan {} for unwrap/expect patterns", path);
    
    if nestgate_mode {
        info!("🧬 NestGate-Specific Patterns:");
        info!("✅ Service start/stop unwraps → Service error handling");
        info!("✅ Health check unwraps → Health error handling"); 
        info!("✅ Storage operation unwraps → Storage error handling");
        info!("✅ RwLock unwraps → Poison recovery patterns");
        info!("✅ Config expect calls → Configuration error handling");
        info!("✅ Runtime creation unwraps → Resource error handling");
    } else {
        info!("✅ Configuration-related unwraps → Environment error handling");
        info!("✅ Lock unwraps → Poison recovery patterns");
        info!("✅ JSON parsing unwraps → Validation error handling");
        info!("✅ HTTP request unwraps → Network error handling");
        info!("✅ File I/O unwraps → I/O error handling");
    }
    
    println!("\n🔄 Run with --apply to execute the migration");
    Ok(())
}

async fn handle_apply_mode(migrator: &SystematicUnwrapMigrator, path: &str, include_tests: bool, production_only: bool) -> Result<(), Box<dyn std::error::Error>> {
    info!("⚡ Apply Mode - Executing systematic migration");
    
    // Use the enhanced migrator for actual migration
    let mut enhanced_migrator = EnhancedUnwrapMigrator::new(false); // dry_run = false
    if include_tests {
        enhanced_migrator.set_include_tests(true);
    }
    
    // Execute the migration
    let report = enhanced_migrator.generate_report(std::path::Path::new(path)).await?;
    
    if report.total_patterns_found == 0 {
        info!("✨ No unwrap/expect patterns found - codebase is already clean!");
        return Ok(());
    }
    
    info!("✅ Migration completed successfully!");
    info!("📊 Processed {} files and fixed {} patterns", report.total_files_scanned, report.total_patterns_found);
    
    println!("\n📋 MIGRATION RESULTS");
    println!("====================");
    println!("Files modified: {}", report.file_statistics.len());
    println!("Total patterns fixed: {}", report.total_patterns_found);
    
    for (severity, count) in &report.patterns_by_severity {
        let emoji = match severity.as_str() {
            "Critical" => "🔴",
            "High" => "🟠", 
            "Medium" => "🟡",
            "Low" => "🟢",
            _ => "⚪",
        };
        println!("  {} {}: {} patterns fixed", emoji, severity, count);
    }
    
    println!("\n🧪 NEXT STEPS:");
    println!("1. ✅ Migration completed - patterns have been replaced");
    println!("2. 🔍 Run 'cargo check --all-features' to validate syntax");
    println!("3. 🧪 Run 'cargo test --all-features' to ensure functionality");
    println!("4. 🎨 Run 'cargo clippy' for additional improvements");
    println!("5. 📝 Review changes and commit to version control");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_creation() {
        let cmd = Command::new("test-unwrap-migrator")
            .version("1.0.0")
            .about("Test CLI");
        
        assert_eq!(cmd.get_name(), "test-unwrap-migrator");
    }
}
