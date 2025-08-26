use clap::{Parser, Subcommand};
use tracing::info;
// Removed unused tracing import

mod config;
mod download;
mod installer;
mod platform;
mod wizard;

// GUI feature removed - using API endpoints for UI primals instead

use crate::installer::NestGateInstaller;

#[derive(Parser)]
#[command(name = "nestgate-installer")]
#[command(about = "NestGate Installation and Configuration Tool")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// Installation directory (defaults to system appropriate location)
    #[arg(long)]
    install_dir: Option<std::path::PathBuf>,

    /// Skip confirmation prompts
    #[arg(short, long)]
    yes: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Install NestGate with interactive wizard
    Install {
        /// Force reinstallation even if already installed
        #[arg(long)]
        force: bool,

        /// Install as system service
        #[arg(long)]
        service: bool,

        /// Skip ZFS setup
        #[arg(long)]
        skip_zfs: bool,
    },

    /// Uninstall NestGate
    Uninstall {
        /// Remove configuration files
        #[arg(long)]
        remove_config: bool,

        /// Remove data files
        #[arg(long)]
        remove_data: bool,
    },

    /// Update existing installation
    Update {
        /// Update to specific version
        #[arg(long)]
        version: Option<String>,
    },

    /// Configure existing installation
    Configure {
        /// Configuration file path
        #[arg(long)]
        config: Option<std::path::PathBuf>,

        /// Run configuration wizard
        #[arg(long)]
        wizard: bool,
    },

    /// Check system requirements
    Doctor,
    // GUI installer removed - using API endpoints for UI primals instead
}

fn setup_logging(verbose: bool) -> Result<()> {
    let log_level = if verbose { "debug" } else { "info" };

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(log_level)),
        )
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    setup_logging(cli.verbose)?;

    info!("NestGate Installer starting...");

    let mut installer = NestGateInstaller::new(cli.install_dir.clone())?;

    match cli.command {
        Some(Commands::Install {
            force: _,
            service: _,
            skip_zfs: _,
        }) => {
            let config = crate::config::installer_config_factory::development();
            installer.install(&config).await?;
        }

        Some(Commands::Uninstall {
            remove_config,
            remove_data,
        }) => {
            installer
                .uninstall(remove_config, remove_data, cli.yes)
                .await?;
        }

        Some(Commands::Update { version }) => {
            installer.update(version, cli.yes).await?;
        }

        Some(Commands::Configure { config, wizard }) => {
            if wizard {
                installer.run_configuration_wizard().await?;
            } else {
                installer.configure(config).await?;
            }
        }

        Some(Commands::Doctor) => {
            installer.doctor().await?;
        }

        // GUI installer removed - using API endpoints for UI primals instead
        None => {
            // Default: run installation wizard
            info!("No command specified, running installation wizard...");
            let config = crate::config::installer_config_factory::development();
            installer.install(&config).await?;
        }
    }

    Ok(())
}
