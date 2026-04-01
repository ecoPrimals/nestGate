// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::float_cmp,
        clippy::uninlined_format_args,
        clippy::cast_precision_loss,
        clippy::items_after_statements,
    )
)]
#![allow(
    deprecated,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::doc_markdown,
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::no_effect_underscore_binding,
    clippy::manual_string_new,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::needless_pass_by_value,
    clippy::unnecessary_debug_formatting,
    clippy::unused_async,
    clippy::needless_pass_by_ref_mut,
    clippy::redundant_clone,
    clippy::unnecessary_wraps,
    dead_code,
    clippy::struct_field_names,
    clippy::trivially_copy_pass_by_ref,
    clippy::too_many_lines
)]

//! Main module

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
use nestgate_core::error::{ConfigurationErrorDetails, NestGateUnifiedError};

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
    /// Install `NestGate` with interactive wizard
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
    /// Uninstall `NestGate`
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

/// Setup Logging
fn setup_logging(verbose: bool) -> nestgate_core::Result<()> {
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
async fn main() -> nestgate_core::Result<()> {
    let cli = Cli::parse();

    setup_logging(cli.verbose)?;

    info!("NestGate Installer starting...");

    let mut installer = NestGateInstaller::new(cli.install_dir.clone()).map_err(|e| {
        NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
            field: "installer".into(),
            message: format!("Failed to create installer: {e}").into(),
            currentvalue: None,
            expected: None,
            user_error: false,
        }))
    })?;

    match cli.command {
        Some(Commands::Install {
            force: _,
            service: _,
            skip_zfs: _,
        }) => {
            let config = crate::config::installer_config_factory::development();
            installer.install(&config).map_err(|e| {
                NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                    field: "install".into(),
                    message: format!("Installation failed: {e}").into(),
                    currentvalue: None,
                    expected: None,
                    user_error: false,
                }))
            })?;
        }

        Some(Commands::Uninstall {
            remove_config,
            remove_data,
        }) => {
            installer
                .uninstall(remove_config, remove_data, cli.yes)
                .map_err(|e| {
                    NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                        field: "uninstall".into(),
                        message: format!("Uninstallation failed: {e}").into(),
                        currentvalue: None,
                        expected: None,
                        user_error: false,
                    }))
                })?;
        }

        Some(Commands::Update { version }) => {
            installer.update(version, cli.yes).await.map_err(|e| {
                NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                    field: "update".into(),
                    message: format!("Update failed: {e}").into(),
                    currentvalue: None,
                    expected: None,
                    user_error: false,
                }))
            })?;
        }

        Some(Commands::Configure { config, wizard }) => {
            if wizard {
                installer.run_configuration_wizard().map_err(|e| {
                    NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                        field: "wizard".into(),
                        message: format!("Configuration wizard failed: {e}").into(),
                        currentvalue: None,
                        expected: None,
                        user_error: false,
                    }))
                })?;
            } else {
                installer.configure(config).map_err(|e| {
                    NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                        field: "configure".into(),
                        message: format!("Configuration failed: {e}").into(),
                        currentvalue: None,
                        expected: None,
                        user_error: false,
                    }))
                })?;
            }
        }

        Some(Commands::Doctor) => {
            installer.doctor().await.map_err(|e| {
                NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                    field: "doctor".into(),
                    message: format!("Doctor check failed: {e}").into(),
                    currentvalue: None,
                    expected: None,
                    user_error: false,
                }))
            })?;
        }

        // GUI installer removed - using API endpoints for UI primals instead
        None => {
            // Default: run installation wizard
            info!("No command specified, running installation wizard...");
            let config = crate::config::installer_config_factory::development();
            installer.install(&config).map_err(|e| {
                NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                    field: "default_install".into(),
                    message: format!("Installation failed: {e}").into(),
                    currentvalue: None,
                    expected: None,
                    user_error: false,
                }))
            })?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod main_cli_round3_tests {
    use super::Cli;
    use clap::Parser;

    #[test]
    fn parses_install_doctor_update_flags() {
        let _ = Cli::try_parse_from(["nestgate-installer", "install", "--force", "--service"])
            .expect("install");
        let _ = Cli::try_parse_from(["nestgate-installer", "doctor"]).expect("doctor");
        let _ = Cli::try_parse_from(["nestgate-installer", "update", "--version", "1.2.3"])
            .expect("update");
        let _ = Cli::try_parse_from(["nestgate-installer", "-y", "uninstall", "--remove-config"])
            .expect("uninstall");
    }

    #[test]
    fn parses_global_flags() {
        let c = Cli::try_parse_from([
            "nestgate-installer",
            "-v",
            "--install-dir",
            "/opt/ng",
            "configure",
            "--wizard",
        ])
        .expect("cfg");
        assert!(c.verbose);
        assert_eq!(c.install_dir, Some(std::path::PathBuf::from("/opt/ng")));
    }
}
