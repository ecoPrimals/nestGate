// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Clap subcommand and nested action definitions.

use crate::commands::env::bind_from_env_or_default;
use clap::Subcommand;
use std::net::SocketAddr;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Run `NestGate` as a server (primary mode) - `UniBin` pattern
    #[command(name = "server", alias = "daemon")]
    #[command(about = "Run NestGate server")]
    Server {
        /// Port for TCP JSON-RPC listener (alongside Unix socket). When omitted, TCP activates if
        /// `NESTGATE_API_PORT`, `NESTGATE_HTTP_PORT`, or `NESTGATE_PORT` is explicitly set, or if
        /// `NESTGATE_JSONRPC_TCP` is truthy (`1`/`true`/`yes`/`on`) for the default API port
        /// (see `nestgate-config` / `DEFAULT_API_PORT`).
        #[arg(short, long)]
        port: Option<u16>,
        /// Bind address for TCP JSON-RPC (`bind:port`; `--listen host:port` overrides).
        /// Reads from: `NESTGATE_BIND`, `NESTGATE_BIND_ADDRESS`, or `NESTGATE_HOST`
        #[arg(long, default_value_t = bind_from_env_or_default())]
        bind: String,
        /// Listen address as `host:port` (`UniBin` v1.2). Takes precedence over `--bind` and `--port`.
        #[arg(long)]
        listen: Option<SocketAddr>,
        /// Enable development mode
        #[arg(long)]
        dev: bool,
        /// Run in Unix socket-only mode (no HTTP server, no external dependencies)
        /// Perfect for NUCLEUS atomic patterns and inter-primal communication
        /// NOTE: Socket-only is now the DEFAULT per `PRIMAL_DEPLOYMENT_STANDARD`
        #[arg(long, default_value_t = true)]
        socket_only: bool,
        /// Enable HTTP server mode (legacy/standalone mode)
        /// Only use when HTTP API is explicitly required
        #[arg(long, conflicts_with = "socket_only")]
        enable_http: bool,
        /// Family ID for multi-family socket support
        /// Creates family-scoped socket: nestgate-{family_id}.sock
        /// Reads from: `NESTGATE_FAMILY_ID` environment variable if not specified
        #[arg(long)]
        family_id: Option<String>,
        /// Use Linux abstract namespace socket (`\0` prefix) instead of filesystem socket.
        /// Required for Android/SELinux substrates (`UniBin` v1.2).
        #[arg(long)]
        r#abstract: bool,
    },

    /// Show daemon status (`UniBin`)
    #[command(name = "status")]
    #[command(about = "Check daemon status")]
    Status,

    /// Health check (`UniBin`)
    #[command(name = "health")]
    #[command(about = "Health check for all components")]
    Health,

    /// Show version information (`UniBin`)
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

    /// Discovery operations (`UniBin`)
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
        #[arg(short, long, default_value_t = nestgate_core::constants::get_api_port())]
        port: u16,
        /// Bind address (can be overridden with `NESTGATE_BIND_ADDRESS`)
        #[arg(long, default_value = nestgate_core::constants::DEFAULT_BIND_ADDRESS)]
        bind: String,
        /// Listen address as `host:port` (`UniBin` v1.2). Takes precedence over `--bind` and `--port`.
        #[arg(long)]
        listen: Option<SocketAddr>,
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
