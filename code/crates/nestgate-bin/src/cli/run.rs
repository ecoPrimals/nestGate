// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `Cli::run` dispatch implementation.

use super::runtime::{print_banner, setup_logging};
use super::{Cli, Commands};
use crate::error::BinErrorHelper;

impl Cli {
    /// Run the CLI application
    pub async fn run(self) -> crate::error::BinResult<()> {
        // Setup logging
        setup_logging(self.verbose);

        // 🔒 CRITICAL SECURITY: Validate JWT secret before starting
        // This prevents production deployment with insecure default values
        nestgate_core::jwt_validation::validate_jwt_secret_or_exit();

        // Print banner
        print_banner();

        // Handle commands
        match self.command {
            // UniBin: Server mode command (primary)
            Commands::Server {
                port,
                bind,
                listen,
                dev,
                enable_http,
                family_id,
                socket_only: _,
                r#abstract,
            } => {
                // Multi-family support: CLI flag > env var > default
                let resolved_family_id =
                    family_id.or_else(|| std::env::var("NESTGATE_FAMILY_ID").ok());

                if let Some(ref fid) = resolved_family_id {
                    tracing::info!("👪 Family ID: {} (creates nestgate-{}.sock)", fid, fid);
                }

                if r#abstract {
                    nestgate_core::env_process::set_var("NESTGATE_ABSTRACT_SOCKET", "1");
                    tracing::info!("📱 Abstract socket mode enabled (Android/SELinux substrate)");
                }

                if enable_http {
                    tracing::info!("🌐 Starting NestGate with HTTP server enabled");
                } else {
                    tracing::info!(
                        "🔌 Starting NestGate in socket-only mode (TRUE ecoBin - default)"
                    );
                }
                crate::commands::service::run_daemon(
                    port,
                    bind.as_str(),
                    listen,
                    dev,
                    enable_http,
                    resolved_family_id.as_deref(),
                )
                .await
                .map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some("server".to_string()))
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

            // UniBin: Discover command (EVOLVED: Real implementations)
            Commands::Discover { target } => {
                crate::commands::discover::execute(target)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("discover".to_string()))
                    })?;
            }

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
                crate::commands::doctor::execute(comprehensive, fix)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("doctor".to_string()))
                    })?;
            }
            Commands::Storage { action } => {
                crate::commands::storage::execute(action)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("storage".to_string()))
                    })?;
            }
            Commands::Config { action } => {
                crate::commands::config::execute(action)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("config".to_string()))
                    })?;
            }
            Commands::Monitor {
                interval,
                output,
                duration,
            } => {
                crate::commands::monitor::execute(interval, output, duration)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some("monitor".to_string()))
                    })?;
            }
        }

        Ok(())
    }
}
