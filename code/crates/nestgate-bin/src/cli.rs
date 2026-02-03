//
// Modern command-line interface for NestGate operations:
// - ZFS filesystem management
// - Storage configuration and monitoring
// - Service management and deployment
// - System diagnostics and troubleshooting

//! Cli module

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// `NestGate` - Universal ZFS and Storage Management
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
  nestgate service start --port $NESTGATE_API_PORT

  # Check system health
  nestgate doctor --comprehensive

  # Configure storage backend
  nestgate storage configure --backend filesystem

  # Access ZFS features via API:
  curl -X POST $NESTGATE_API_ENDPOINT/api/v1/zfs/datasets \\
    -H 'Content-Type: application/json' \\
    -d '{\"name\": \"tank/data\", \"compression\": true}'

For more information: https://github.com/your-org/nestgate
")]
pub struct Cli {
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    /// Verbose
    pub verbose: bool,

    /// Configuration file path
    #[arg(short, long, global = true)]
    /// Configuration for
    pub config: Option<PathBuf>,

    /// Output format (json, yaml, table)
    #[arg(long, global = true, default_value = "table")]
    /// Output
    pub output: String,

    #[command(subcommand)]
    /// Command
    pub command: Commands,
}

/// Read port from environment with fallback chain (UniBin compliance)
/// Priority: NESTGATE_API_PORT → NESTGATE_HTTP_PORT → NESTGATE_PORT → default
fn port_from_env_or_default() -> u16 {
    std::env::var("NESTGATE_API_PORT")
        .or_else(|_| std::env::var("NESTGATE_HTTP_PORT"))
        .or_else(|_| std::env::var("NESTGATE_PORT"))
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(nestgate_core::defaults::network::DEFAULT_API_PORT)
}

/// Read bind address from environment with fallback (UniBin compliance)
/// Priority: NESTGATE_BIND → NESTGATE_BIND_ADDRESS → NESTGATE_HOST → default
fn bind_from_env_or_default() -> String {
    std::env::var("NESTGATE_BIND")
        .or_else(|_| std::env::var("NESTGATE_BIND_ADDRESS"))
        .or_else(|_| std::env::var("NESTGATE_HOST"))
        .ok()
        .unwrap_or_else(|| nestgate_core::defaults::network::DEFAULT_BIND_ADDRESS.to_string())
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run NestGate as a daemon (server mode) - UniBin pattern
    #[command(name = "daemon", alias = "server")]
    #[command(about = "Run NestGate daemon (server mode)")]
    Daemon {
        /// Port to bind to (only used with --enable-http)
        /// Reads from: NESTGATE_API_PORT, NESTGATE_HTTP_PORT, or NESTGATE_PORT
        #[arg(short, long, default_value_t = port_from_env_or_default())]
        port: u16,
        /// Bind address (only used with --enable-http)
        /// Reads from: NESTGATE_BIND, NESTGATE_BIND_ADDRESS, or NESTGATE_HOST
        #[arg(long, default_value_t = bind_from_env_or_default())]
        bind: String,
        /// Enable development mode
        #[arg(long)]
        dev: bool,
        /// Enable HTTP server (socket-only is default for TRUE ecoBin compliance)
        /// Socket-only mode: Zero external dependencies, perfect for NUCLEUS atomic patterns
        /// HTTP mode: Enables REST API and web endpoints (use only when needed)
        #[arg(long)]
        enable_http: bool,
    },

    /// Show daemon status (UniBin)
    #[command(name = "status")]
    #[command(about = "Check daemon status")]
    Status,

    /// Health check (UniBin)
    #[command(name = "health")]
    #[command(about = "Health check for all components")]
    Health,

    /// Show version information (UniBin)
    #[command(name = "version")]
    #[command(about = "Show version and build information")]
    Version,

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
    /// ZFS filesystem operations
    #[command(name = "zfs")]
    #[command(about = "ZFS dataset and pool management")]
    Zfs {
        #[command(subcommand)]
        command: crate::commands::zfs::ZfsCommands,
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

    /// Discovery operations (UniBin)
    #[command(name = "discover")]
    #[command(about = "Discover primals and services")]
    Discover {
        #[command(subcommand)]
        target: DiscoverTarget,
    },
}

#[derive(Debug, Subcommand)]
pub enum ServiceAction {
    /// Start `NestGate` service
    Start {
        /// Port to bind to (can be overridden with `NESTGATE_API_PORT`)
        #[arg(short, long, default_value_t = nestgate_core::defaults::network::DEFAULT_API_PORT)]
        port: u16,
        /// Bind address (can be overridden with `NESTGATE_BIND_ADDRESS`)
        #[arg(long, default_value = nestgate_core::defaults::network::DEFAULT_BIND_ADDRESS)]
        bind: String,
        /// Run in background
        #[arg(short, long)]
        daemon: bool,
    },
    /// Stop `NestGate` service
    Stop,
    /// Restart `NestGate` service
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
pub enum DiscoverTarget {
    /// List discovered primals
    Primals,
    /// List discovered services
    Services,
    /// List available capabilities
    Capabilities,
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
impl Cli {
    /// Run the CLI application
    pub async fn run(self) -> crate::error::BinResult<()> {
        use crate::error::BinErrorHelper;

        // Setup logging
        setup_logging(self.verbose);

        // 🔒 CRITICAL SECURITY: Validate JWT secret before starting
        // This prevents production deployment with insecure default values
        nestgate_core::jwt_validation::validate_jwt_secret_or_exit();

        // Print banner
        print_banner();

        // Handle commands
        match self.command {
            // UniBin: Daemon mode command
            Commands::Daemon { port, bind, dev, enable_http } => {
                if enable_http {
                    tracing::info!("🌐 Starting NestGate with HTTP server enabled");
                } else {
                    tracing::info!("🔌 Starting NestGate in socket-only mode (TRUE ecoBin - default)");
                }
                crate::commands::service::run_daemon(port, &bind, dev, enable_http)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("daemon".to_string()))
                    })?;
            }

            // UniBin: Status command
            Commands::Status => {
                crate::commands::service::show_status().await.map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some("status".to_string()))
                })?;
            }

            // UniBin: Health command
            Commands::Health => {
                crate::commands::service::show_health().await.map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some("health".to_string()))
                })?;
            }

            // UniBin: Version command
            Commands::Version => {
                crate::commands::service::show_version()
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("version".to_string()))
                    })?;
            }

            // UniBin: Discover command
            Commands::Discover { target } => match target {
                DiscoverTarget::Primals => {
                    println!("🔍 Discovered Primals:");
                    println!("   (Discovery commands coming soon)");
                }
                DiscoverTarget::Services => {
                    println!("🔍 Discovered Services:");
                    println!("   (Discovery commands coming soon)");
                }
                DiscoverTarget::Capabilities => {
                    println!("🔍 Available Capabilities:");
                    println!("   (Discovery commands coming soon)");
                }
            },

            Commands::Zfs { command } => {
                let mut zfs_handler = crate::commands::zfs::ZfsHandler::new();
                zfs_handler.execute(command).await.map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some("zfs_command".to_string()))
                })?;
            }
            Commands::Service { action } => {
                let mut service_manager = crate::commands::service::ServiceManager::new();
                service_manager.execute(action).await.map_err(|e| {
                    BinErrorHelper::runtime_error(
                        e.to_string(),
                        Some("service_command".to_string()),
                    )
                })?;
            }
            Commands::Doctor { comprehensive, fix } => {
                println!(
                    "🩺 System diagnostics not yet implemented - comprehensive: {comprehensive}, fix: {fix}"
                );
            }
            Commands::Storage { action } => {
                println!("💾 Storage management not yet implemented: {action:?}");
            }
            Commands::Config { action } => {
                println!("⚙️  Configuration management not yet implemented: {action:?}");
            }
            Commands::Monitor {
                interval,
                output,
                duration,
            } => {
                println!("📊 Performance monitoring not yet implemented - interval: {interval:?}, output: {output:?}, duration: {duration:?}");
            }
        }

        Ok(())
    }
}

/// Initialize CLI and parse arguments
#[must_use]
pub fn parse_args() -> Cli {
    Cli::parse()
}
/// Setup logging based on CLI arguments
pub fn setup_logging(verbose: bool) {
    let level = if verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("nestgate={level}"))
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
