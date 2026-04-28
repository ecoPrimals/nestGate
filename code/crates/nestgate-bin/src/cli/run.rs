// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `Cli::run` dispatch implementation.

use super::runtime::{print_banner, setup_logging};
use super::{Cli, Commands};
use crate::error::BinErrorHelper;

/// Resolves multi-family socket ID: CLI `--family-id` wins over `NESTGATE_FAMILY_ID`.
///
/// Pass `env_family_id` as `std::env::var("NESTGATE_FAMILY_ID").ok()` at the call site; kept
/// injectable for unit tests without mutating process environment.
#[must_use]
pub(super) fn resolve_family_id(
    cli_family_id: Option<String>,
    env_family_id: Option<String>,
) -> Option<String> {
    cli_family_id.or(env_family_id)
}

impl Cli {
    /// Run the CLI application
    pub async fn run(self) -> crate::error::BinResult<()> {
        // Setup logging
        setup_logging(self.verbose);

        let auth_mode = std::env::var("NESTGATE_AUTH_MODE").unwrap_or_default();
        if auth_mode.eq_ignore_ascii_case("beardog") {
            tracing::info!(
                "Auth mode: beardog — JWT validation skipped, \
                 auth delegated to security capability provider"
            );
        } else {
            nestgate_core::jwt_validation::validate_jwt_secret_or_exit();
        }

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
                    resolve_family_id(family_id, std::env::var("NESTGATE_FAMILY_ID").ok());

                if let Some(ref fid) = resolved_family_id {
                    tracing::info!("Family ID: {fid} (creates nestgate-{fid}.sock)");
                }

                if r#abstract {
                    nestgate_core::env_process::set_var("NESTGATE_ABSTRACT_SOCKET", "1");
                    tracing::info!("Abstract socket mode enabled (Android/SELinux substrate)");
                }

                if enable_http {
                    tracing::info!("Starting NestGate with HTTP server enabled");
                } else {
                    tracing::info!("Starting NestGate in socket-only mode (default)");
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

#[cfg(test)]
mod tests {
    use super::resolve_family_id;

    #[test]
    fn resolve_family_id_prefers_cli_over_env() {
        assert_eq!(
            resolve_family_id(Some("from-cli".into()), Some("from-env".into())).as_deref(),
            Some("from-cli")
        );
    }

    #[test]
    fn resolve_family_id_falls_back_to_env_value() {
        assert_eq!(
            resolve_family_id(None, Some("env-only".into())).as_deref(),
            Some("env-only")
        );
    }

    #[test]
    fn resolve_family_id_none_when_no_cli_and_no_env() {
        assert!(resolve_family_id(None, None).is_none());
    }
}
