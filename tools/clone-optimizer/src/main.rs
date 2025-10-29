//! # Clone Optimizer CLI
//!
//! Advanced clone optimization and zero-copy migration tool for Rust codebases.

use clap::{Arg, Command};
use clone_optimizer::{CloneOptimizer, CloneOptimizerConfig};
use std::process;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let matches = Command::new("clone-optimizer")
        .version(clone_optimizer::VERSION)
        .about("Advanced clone optimization and zero-copy migration tool for Rust codebases")
        .arg(
            Arg::new("path")
                .long("path")
                .value_name("PATH")
                .help("Path to analyze")
                .default_value("./src"),
        )
        .arg(
            Arg::new("dry-run")
                .long("dry-run")
                .help("Show optimizations without applying changes")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("apply")
                .long("apply")
                .help("Apply safe optimizations")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("safety-level")
                .long("safety-level")
                .value_name("LEVEL")
                .help("Safety level: conservative, safe, moderate, aggressive")
                .default_value("safe"),
        )
        .arg(
            Arg::new("min-impact")
                .long("min-impact")
                .value_name("BYTES")
                .help("Minimum performance impact to consider (bytes)")
                .default_value("100"),
        )
        .arg(
            Arg::new("exclude-tests")
                .long("exclude-tests")
                .help("Skip test files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("report")
                .long("report")
                .help("Generate detailed report")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output")
                .long("output")
                .value_name("FILE")
                .help("Output file for reports (default: stdout)"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Verbose output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Parse command line arguments
    let path = matches.get_one::<String>("path").unwrap();
    let dry_run = matches.get_flag("dry-run");
    let apply = matches.get_flag("apply");
    let safety_level_str = matches.get_one::<String>("safety-level").unwrap();
    let min_impact_str = matches.get_one::<String>("min-impact").unwrap();
    let exclude_tests = matches.get_flag("exclude-tests");
    let generate_report = matches.get_flag("report");
    let output_file = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");

    // Validate arguments
    if apply && dry_run {
        eprintln!("Error: --apply and --dry-run are mutually exclusive");
        process::exit(1);
    }

    // Parse safety level
    let safety_level = match safety_level_str.parse() {
        Ok(level) => level,
        Err(e) => {
            eprintln!("Error: Invalid safety level '{}': {}", safety_level_str, e);
            process::exit(1);
        }
    };

    // Parse minimum impact
    let min_impact_bytes: u64 = match min_impact_str.parse() {
        Ok(bytes) => bytes,
        Err(_) => {
            eprintln!("Error: Invalid minimum impact value '{}'", min_impact_str);
            process::exit(1);
        }
    };

    // Create configuration
    let mut config = CloneOptimizerConfig::default();
    config.safety_level = safety_level;
    config.analysis.min_impact_bytes = min_impact_bytes;
    config.filtering.include_tests = !exclude_tests;
    config.optimization.auto_apply = apply && !dry_run;
    
    if let Some(output_path) = output_file {
        config.reporting.output_file = Some(output_path.into());
    }

    if verbose {
        config.reporting.verbosity = clone_optimizer::config::VerbosityLevel::Verbose;
    }

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("Error: Configuration validation failed: {}", e);
        process::exit(1);
    }

    info!("🚀 Starting Clone Optimizer v{}", clone_optimizer::VERSION);
    info!("📁 Analyzing path: {}", path);
    info!("🛡️  Safety level: {}", safety_level);
    info!("⚡ Minimum impact: {} bytes", min_impact_bytes);

    // Create and run optimizer
    let mut optimizer = match CloneOptimizer::new(config) {
        Ok(opt) => opt,
        Err(e) => {
            eprintln!("Error: Failed to create optimizer: {}", e);
            process::exit(1);
        }
    };

    // Analyze the code
    let results = match optimizer.analyze_directory(path).await {
        Ok(results) => results,
        Err(e) => {
            eprintln!("Error: Analysis failed: {}", e);
            process::exit(1);
        }
    };

    info!("🔍 Analysis completed. Found {} optimization opportunities", results.len());

    // Apply optimizations if requested
    if apply {
        match optimizer.apply_optimizations(&results).await {
            Ok(applied_count) => {
                info!("✅ Applied {} optimizations", applied_count);
            }
            Err(e) => {
                eprintln!("Error: Failed to apply optimizations: {}", e);
                process::exit(1);
            }
        }
    }

    // Generate report
    if generate_report || dry_run {
        let report = optimizer.generate_report();
        
        println!("\n🔍 Clone Optimization Report");
        println!("═══════════════════════════════════════════════════════════");
        println!();
        println!("📁 Files analyzed: {}", report.stats.files_analyzed);
        println!("🔍 Optimizations found: {}", report.stats.optimizations_found);
        println!("✅ Optimizations applied: {}", report.stats.optimizations_applied);
        
        if dry_run && !results.is_empty() {
            println!("\n🎯 Optimization Opportunities:");
            println!("─────────────────────────────────────────────────────────");
            for (i, result) in results.iter().enumerate() {
                println!("{}. {}", i + 1, result.description);
            }
        }
        
        if !apply && !results.is_empty() {
            println!("\n💡 Run with --apply to implement safe optimizations");
            println!("💡 Use --safety-level aggressive for more optimizations");
        }
    }

    if results.is_empty() {
        println!("\n🎉 No optimization opportunities found! Your code is already well-optimized.");
    }

    info!("🏁 Clone Optimizer completed successfully");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_is_set() {
        assert!(!clone_optimizer::VERSION.is_empty());
    }
} 