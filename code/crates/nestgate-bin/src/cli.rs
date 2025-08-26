//
// Modern command-line interface for NestGate operations:
// - ZFS filesystem management
// - Storage configuration and monitoring
// - Service management and deployment
// - System diagnostics and troubleshooting

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// NestGate - Universal ZFS and Storage Management
#[derive(Debug, Parser)]
#[command(name = "nestgate")]
#[command(about = "Universal ZFS and Storage Management System")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(long_about = "
🏠 NestGate - Sovereign Storage System

NestGate provides ZFS capabilities through a modern API-based architecture:
• Universal ZFS features accessible via REST API
• Works with any storage backend (local, cloud, network, memory)
• Copy-on-Write, compression, checksumming, snapshots
• Intelligent auto-configuration and optimization
• Production-ready performance and reliability

EXAMPLES:
  # Start NestGate service
  nestgate service start --port 8080

  # Check system health
  nestgate doctor --comprehensive

  # Configure storage backend
  nestgate storage configure --backend filesystem

  # Access ZFS features via API:
  curl -X POST http://localhost:8080/api/v1/zfs/datasets \\
    -H 'Content-Type: application/json' \\
    -d '{\"name\": \"tank/data\", \"compression\": true}'

For more information: https://github.com/your-org/nestgate
")]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    /// Output format (json, yaml, table)
    #[arg(long, global = true, default_value = "table")]
    pub output: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Service management
    #[command(name = "service")]
    #[command(about = "Start/stop NestGate services")]
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },

    /// Storage management
    #[command(name = "storage")]
    #[command(about = "Storage backend configuration")]
    Storage {
        #[command(subcommand)]
        action: StorageAction,
    },

    /// System diagnostics
    #[command(name = "doctor")]
    #[command(about = "System health check and diagnostics")]
    Doctor {
        /// Run comprehensive checks
        #[arg(long)]
        comprehensive: bool,
        /// Fix issues automatically
        #[arg(long)]
        fix: bool,
    },

    /// Configuration management
    #[command(name = "config")]
    #[command(about = "Configuration management")]
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },

    /// Performance monitoring
    #[command(name = "monitor")]
    #[command(about = "Performance monitoring and statistics")]
    Monitor {
        /// Monitoring interval in seconds
        #[arg(short, long, default_value = "5")]
        interval: u64,
        /// Output file for metrics
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Duration to monitor (seconds)
        #[arg(short, long)]
        duration: Option<u64>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ServiceAction {
    /// Start NestGate service
    Start {
        /// Port to bind to
        #[arg(short, long, default_value = "8080")]
        port: u16,
        /// Bind address
        #[arg(long, default_value = "0.0.0.0")]
        bind: String,
        /// Run in background
        #[arg(short, long)]
        daemon: bool,
    },
    /// Stop NestGate service
    Stop,
    /// Restart NestGate service
    Restart,
    /// Show service status
    Status,
    /// Show service logs
    Logs {
        /// Number of lines to show
        #[arg(short, long, default_value = "100")]
        lines: u32,
        /// Follow log output
        #[arg(short, long)]
        follow: bool,
    },
}

#[derive(Debug, Subcommand)]
pub enum StorageAction {
    /// List available storage backends
    List,
    /// Scan for available storage
    Scan {
        /// Path to scan
        #[arg(short, long, default_value = ".")]
        path: PathBuf,
        /// Include cloud storage
        #[arg(long)]
        cloud: bool,
        /// Include network storage
        #[arg(long)]
        network: bool,
    },
    /// Test storage performance
    Benchmark {
        /// Storage backend to test
        backend: String,
        /// Test duration in seconds
        #[arg(short, long, default_value = "30")]
        duration: u64,
        /// Test file size in MB
        #[arg(short, long, default_value = "100")]
        size: u64,
    },
    /// Configure storage backend
    Configure {
        /// Backend type
        backend: String,
        /// Configuration key=value pairs
        #[arg(short, long)]
        set: Vec<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ConfigAction {
    /// Show current configuration
    Show,
    /// Set configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    /// Get configuration value
    Get {
        /// Configuration key
        key: String,
    },
    /// Reset configuration to defaults
    Reset {
        /// Confirm reset
        #[arg(long)]
        confirm: bool,
    },
    /// Validate configuration
    Validate,
    /// Export configuration
    Export {
        /// Output file
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Export format (json, yaml, toml)
        #[arg(short, long, default_value = "yaml")]
        format: String,
    },
    /// Import configuration
    Import {
        /// Input file
        input: PathBuf,
    },
}

/// Initialize CLI and parse arguments
pub fn parse_args() -> Cli {
    Cli::parse()
}

/// Setup logging based on CLI arguments
pub fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(format!("{level},nestgate=debug"))
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}

/// Print welcome banner
pub fn print_banner() {
    println!(
        "🏠 NestGate v{} - Universal ZFS & Storage Management",
        env!("CARGO_PKG_VERSION")
    );
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🌟 ZFS features on ANY storage backend");
    println!("📦 Local, Cloud, Network, Memory support");
    println!("⚡ Production-ready performance");
    println!("🔒 Enterprise-grade data integrity");
    println!();
}
