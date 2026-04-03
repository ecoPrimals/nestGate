// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Modern command-line interface for NestGate operations:
// - ZFS filesystem management
// - Storage configuration and monitoring
// - Service management and deployment
// - System diagnostics and troubleshooting

//! Cli module

mod run;
mod runtime;
mod subcommands;

pub use runtime::{print_banner, setup_logging};
pub use subcommands::{Commands, ConfigAction, DiscoverTarget, ServiceAction, StorageAction};

use clap::Parser;
use std::path::PathBuf;

/// Re-export for `crate::cli::port_from_env_or_default` (`service` and external consistency).
pub(crate) use crate::commands::env::port_from_env_or_default;

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

/// Initialize CLI and parse arguments
#[must_use]
pub fn parse_args() -> Cli {
    Cli::parse()
}

#[cfg(test)]
mod tests;
