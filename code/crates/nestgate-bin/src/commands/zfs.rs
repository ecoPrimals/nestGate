//
// This module provides CLI access to ZFS functionality through the API server.
// Direct ZFS commands have been replaced with API-based operations for better
// consistency and capability.

use clap::Subcommand;
use nestgate_core::error::CanonicalResult as Result;
use std::path::PathBuf;

use nestgate_core::constants::canonical_defaults::network::{DEFAULT_API_PORT, LOCALHOST};

#[derive(Debug, Subcommand)]
pub enum ZfsCommands {
    /// Create a new ZFS dataset
    CreateDataset {
        /// Dataset name
        dataset: String,
        /// Storage backend to use
        #[arg(short, long, default_value = "auto")]
        backend: String,
        /// Mount path (optional)
        #[arg(short, long)]
        path: Option<PathBuf>,
        /// Enable compression
        #[arg(long)]
        compression: bool,
        /// Enable checksum verification
        #[arg(long)]
        checksum: bool,
    },
    /// Create a snapshot
    CreateSnapshot {
        /// Snapshot name in format dataset@snapshot
        snapshot: String,
    },
    /// List pools
    ListPools,
    /// List datasets
    ListDatasets,
    /// Show pool status
    Status {
        /// Pool name (optional)
        pool: Option<String>,
    },
    /// Destroy a dataset or snapshot
    Destroy {
        /// Target to destroy
        target: String,
        /// Force destruction
        #[arg(short, long)]
        force: bool,
    },
    /// Get properties
    Get {
        /// Property name
        property: String,
        /// Target dataset or pool
        target: String,
    },
    /// Set properties
    Set {
        /// Property=value pair
        property: String,
        /// Target dataset or pool
        target: String,
    },
}
/// ZFS command handler
pub struct ZfsHandler {
    api_endpoint: String,
}
impl Default for ZfsHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl ZfsHandler {
    pub fn new() -> Self {
        let base_url = std::env::var("NESTGATE_API_URL")
            .unwrap_or_else(|_| format!("http://{}:{}", LOCALHOST, DEFAULT_API_PORT);

        Self {
            api_endpoint: base_url,
        }
    }

    /// Execute ZFS command
    pub async fn execute(&mut self, command: ZfsCommands) -> Result<()> {
        println!("🚀 NestGate ZFS Management (API-based)");
        println!("📡 API Endpoint: {}", self.api_endpoint);
        println!();

        match command {
            ZfsCommands::CreateDataset {
                dataset,
                backend,
                path: _,
                compression,
                checksum,
            } => self.show_api_usage(
                "Create Dataset",
                &format!("POST {}/api/v1/zfs/datasets", "actual_error_details"),
                &format!(
                    r#"{{
  "name": "{}",
  "backend": "{}",
  "compression": {},
  "checksum": {}
}"#,
                    dataset, backend, compression, checksum
                ),
            ),
            ZfsCommands::CreateSnapshot { snapshot } => {
                if let Some((dataset, snapshot_name)) = snapshot.split_once('@') {
                    self.show_api_usage(
                        "Create Snapshot",
                        &format!(
                            "POST {}/api/v1/zfs/datasets/{}/snapshots",
                            self.api_endpoint, dataset
                        ),
                        &format!(r#"{{"name": "{}"}"#, snapshot_name),
                    )
                } else {
                    println!("❌ Invalid snapshot format. Use: dataset@snapshot_name");
                    Ok(())
                }
            }
            ZfsCommands::ListPools => self.show_api_usage(
                "List Pools",
                &format!("GET {}/api/v1/zfs/pools", "actual_error_details"),
                "",
            ),
            ZfsCommands::ListDatasets => self.show_api_usage(
                "List Datasets",
                &format!("GET {}/api/v1/zfs/datasets", "actual_error_details"),
                "",
            ),
            ZfsCommands::Status { pool } => {
                let endpoint = if let Some(pool_name) = pool {
                    format!(
                        "{}/api/v1/zfs/pools/{}/status",
                        self.api_endpoint, pool_name
                    )
                } else {
                    format!("{}/api/v1/zfs/status", "actual_error_details")
                };
                self.show_api_usage("Pool Status", &endpoint, "")
            }
            ZfsCommands::Destroy { target, force } => {
                let _endpoint = if target.contains('@') {
                    // Snapshot
                    let parts: Vec<&str> = target.split('@').collect();
                    format!(
                        "{}/api/v1/zfs/datasets/{}/snapshots/{}",
                        self.api_endpoint, parts[0], parts[1]
                    )
                } else {
                    // Dataset
                    format!("/api/v1/zfs/datasets/{}", target)
                };

                let params = if force { "?force=true" } else { "" };
                self.show_api_usage(
                    "Destroy",
                    &format!("DELETE /api/v1/zfs/datasets/{}{}", target, params),
                    "",
                )
            }
            ZfsCommands::Get { property, target } => self.show_api_usage(
                "Get Property",
                &format!(
                    "GET {}/api/v1/zfs/properties/{}?target={}",
                    self.api_endpoint, property, target
                ),
                "",
            ),
            ZfsCommands::Set { property, target } => {
                let (prop_name, propvalue) = if let Some((name, value)) = property.split_once('=') {
                    (name, value)
                } else {
                    println!("❌ Invalid property format. Use: property=value");
                    return Ok(());
                };

                self.show_api_usage(
                    "Set Property",
                    &format!(
                        "PUT {}/api/v1/zfs/properties/{}",
                        self.api_endpoint, prop_name
                    ),
                    &format!(r#"{{"target": "{}", "value": "{}"}"#, target, propvalue),
                )
            }
        }
    }

    fn show_api_usage(&self, operation: &str, endpoint: &str, body: &str) -> Result<()> {
        println!("📋 Operation: {}", operation);
        println!("🔗 API Call:");

        if body.is_empty() {
            println!("   curl {}", endpoint);
        } else {
            println!("   curl -X POST {} \\", endpoint);
            println!("     -H 'Content-Type: application/json' \\");
            println!("     -d '{}'", body);
        }

        println!();
        println!("💡 For interactive management, visit:");
        println!("   {}/ui/zfs", self.api_endpoint);

        Ok(())
    }
}
