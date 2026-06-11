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

/// Resolve whether HTTP should be enabled for `server` mode.
///
/// Default: HTTP enabled (guideStone standard).
/// `--socket-only` disables HTTP.
/// `PRIMAL_BIND_MODE` overrides both: `tcp_only`/`tcp` forces HTTP on,
/// `uds_only` forces HTTP off.
#[must_use]
pub(super) fn resolve_enable_http(socket_only_flag: bool) -> bool {
    let bind_mode = std::env::var("PRIMAL_BIND_MODE")
        .unwrap_or_default()
        .to_lowercase();

    match bind_mode.as_str() {
        "tcp_only" | "tcp" => true,
        "uds_only" | "uds" => false,
        _ => !socket_only_flag,
    }
}

impl Cli {
    /// Run the CLI application
    pub async fn run(self) -> crate::error::BinResult<()> {
        // Setup logging
        setup_logging(self.verbose);

        let auth_mode = std::env::var("NESTGATE_AUTH_MODE").unwrap_or_default();
        let delegated = auth_mode.eq_ignore_ascii_case("delegated")
            || auth_mode.eq_ignore_ascii_case("external");
        let btsp_composition = nestgate_core::rpc::btsp_server_handshake::is_btsp_required();
        if delegated {
            tracing::info!(
                "Auth mode: delegated — JWT validation skipped, \
                 auth delegated to security capability provider"
            );
        } else if btsp_composition {
            tracing::info!(
                "BTSP composition detected (FAMILY_ID set) — JWT validation \
                 skipped, auth delegated to security capability provider via BTSP"
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
                socket,
                port,
                bind,
                listen,
                dev,
                socket_only,
                family_id,
                r#abstract,
            } => {
                if let Some(ref sock_path) = socket {
                    nestgate_core::env_process::set_var(
                        "NESTGATE_SOCKET",
                        sock_path.to_string_lossy().as_ref(),
                    );
                    tracing::info!("Socket path (CLI): {}", sock_path.display());
                }

                let resolved_family_id =
                    resolve_family_id(family_id, std::env::var("NESTGATE_FAMILY_ID").ok());

                if let Some(ref fid) = resolved_family_id {
                    tracing::info!("Family ID: {fid} (creates nestgate-{fid}.sock)");
                }

                if r#abstract {
                    nestgate_core::env_process::set_var("NESTGATE_ABSTRACT_SOCKET", "1");
                    tracing::info!("Abstract socket mode enabled (Android/SELinux substrate)");
                }

                let enable_http = resolve_enable_http(socket_only);

                if enable_http {
                    tracing::info!("Starting NestGate with HTTP server enabled (default)");
                } else {
                    tracing::info!("Starting NestGate in socket-only mode (--socket-only)");
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
                    BinErrorHelper::runtime_error(e.to_string(), Some(String::from("server")))
                })?;
            }

            // UniBin: Status command
            Commands::Status => {
                crate::commands::service::show_status().await.map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some(String::from("status")))
                })?;
            }

            // UniBin: Health command
            Commands::Health => {
                crate::commands::service::show_health().await.map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some(String::from("health")))
                })?;
            }

            // UniBin: Version command
            Commands::Version => {
                crate::commands::service::show_version()
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some(String::from("version")))
                    })?;
            }

            // UniBin: Discover command (EVOLVED: Real implementations)
            Commands::Discover { target } => {
                crate::commands::discover::execute(target)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some(String::from("discover")))
                    })?;
            }

            Commands::Zfs { command } => {
                let mut zfs_handler = crate::commands::zfs::ZfsHandler::new();
                zfs_handler.execute(command).await.map_err(|e| {
                    BinErrorHelper::runtime_error(e.to_string(), Some(String::from("zfs_command")))
                })?;
            }
            Commands::Service { action } => {
                let mut service_manager = crate::commands::service::ServiceManager::new();
                service_manager.execute(action).await.map_err(|e| {
                    BinErrorHelper::runtime_error(
                        e.to_string(),
                        Some(String::from("service_command")),
                    )
                })?;
            }
            Commands::Doctor { comprehensive, fix } => {
                crate::commands::doctor::execute(comprehensive, fix)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some(String::from("doctor")))
                    })?;
            }
            Commands::Storage { action } => {
                crate::commands::storage::execute(action)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some(String::from("storage")))
                    })?;
            }
            Commands::Config { action } => {
                crate::commands::config::execute(action)
                    .await
                    .map_err(|e| {
                        BinErrorHelper::runtime_error(e.to_string(), Some(String::from("config")))
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
                        BinErrorHelper::runtime_error(e.to_string(), Some(String::from("monitor")))
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

    #[test]
    fn btsp_composition_detected_when_family_id_set() {
        temp_env::with_vars(
            [
                ("FAMILY_ID", Some("test-nucleus")),
                ("BIOMEOS_INSECURE", None::<&str>),
            ],
            || {
                assert!(
                    nestgate_core::rpc::btsp_server_handshake::is_btsp_required(),
                    "BTSP should be required when FAMILY_ID is set"
                );
            },
        );
    }

    #[test]
    fn btsp_composition_not_detected_standalone() {
        temp_env::with_vars(
            [
                ("FAMILY_ID", None::<&str>),
                ("BIOMEOS_FAMILY_ID", None::<&str>),
                ("NESTGATE_FAMILY_ID", None::<&str>),
            ],
            || {
                assert!(
                    !nestgate_core::rpc::btsp_server_handshake::is_btsp_required(),
                    "BTSP should not be required in standalone mode"
                );
            },
        );
    }
}
