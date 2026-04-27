// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![forbid(unsafe_code)]

//! `NestGate` binary main entry point
//!
//! **TRUE `UniBin` Architecture**: One binary, multiple modes via subcommands
//! - Primary: `nestgate <subcommand>` - Modern CLI interface
//! - Legacy: Detects `nestgate-server` and `nestgate-client` symlinks for backward compatibility
//!
//! ## Modes
//! - `nestgate server` - Run as server (primary mode)
//! - `nestgate status` - Check service status
//! - `nestgate health` - Health check
//! - `nestgate discover` - Discover primals
//! - `nestgate version` - Show version
//! - More subcommands available via `nestgate --help`

use clap::Parser;
use nestgate_bin::{cli::Cli, error::BinResult};
use std::path::Path;
use tracing::info;

#[tokio::main]
async fn main() -> BinResult<()> {
    // ═══════════════════════════════════════════════════════════════════════
    // UNIBIN: Binary name detection for backward compatibility
    // ═══════════════════════════════════════════════════════════════════════

    let bin_name = std::env::args()
        .next()
        .and_then(|p| {
            Path::new(&p)
                .file_name()
                .and_then(|n| n.to_str())
                .map(std::string::ToString::to_string)
        })
        .unwrap_or_else(|| "nestgate".to_string());

    // Backward compatibility: Auto-daemon mode for 'nestgate-server' symlink
    if bin_name == "nestgate-server" {
        info!("NestGate invoked as 'nestgate-server' (legacy symlink detected)");
        info!("Recommended: use 'nestgate server' for the modern UniBin interface");

        // Setup basic logging for daemon mode
        tracing_subscriber::fmt()
            .with_env_filter("nestgate=info")
            .with_target(false)
            .init();

        // Run daemon with socket-only mode as default (PRIMAL_DEPLOYMENT_STANDARD)
        // CRITICAL: run_daemon's `enable_http` is NOT `socket_only`
        // enable_http = false means socket-only mode (the correct default)
        return nestgate_bin::commands::service::run_daemon(
            None, // no explicit `--port` — Unix socket only (same as `nestgate daemon` default)
            nestgate_core::defaults::network::DEFAULT_BIND_ADDRESS,
            None, // listen: legacy symlink has no CLI
            false,
            false, // enable_http = false (socket-only is default)
            None,  // family_id: discovered at runtime
        )
        .await;
    }

    // Backward compatibility: 'nestgate-client' symlink -> just use normal CLI
    if bin_name == "nestgate-client" {
        info!("NestGate invoked as 'nestgate-client' (legacy symlink detected)");
        info!("Recommended: use 'nestgate <command>' directly for the modern UniBin interface");
    }

    // Modern UniBin: Parse CLI and execute subcommand
    let cli = Cli::parse();
    cli.run().await
}
