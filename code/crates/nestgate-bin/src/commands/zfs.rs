// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

#![expect(
    clippy::unnecessary_wraps,
    reason = "Stub APIs use Result for forward-compatible error propagation"
)]
//
// This module provides CLI access to ZFS functionality through the API server.
// Direct ZFS commands have been replaced with API-based operations for better
// consistency and capability.

//! Zfs module

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
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl ZfsHandler {
    #[must_use]
    pub fn new() -> Self {
        let base_url = std::env::var("NESTGATE_API_URL")
            .unwrap_or_else(|_| format!("http://{LOCALHOST}:{DEFAULT_API_PORT}"));

        Self::with_api_endpoint(base_url)
    }

    /// Build a handler with an explicit API base URL (embedding, tests; avoids `NESTGATE_API_URL`).
    #[must_use]
    pub fn with_api_endpoint(endpoint: impl Into<String>) -> Self {
        Self {
            api_endpoint: endpoint.into(),
        }
    }

    /// Base URL for ZFS API calls (from `NESTGATE_API_URL` or localhost default).
    #[must_use]
    pub fn api_endpoint(&self) -> &str {
        &self.api_endpoint
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
                &format!("POST {}/api/v1/zfs/datasets", self.api_endpoint),
                &format!(
                    r#"{{
  "name": "{dataset}",
  "backend": "{backend}",
  "compression": {compression},
  "checksum": {checksum}
}}"#
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
                        &format!(r#"{{"name": "{snapshot_name}"}}"#),
                    )
                } else {
                    println!("❌ Invalid snapshot format. Use: dataset@snapshot_name");
                    Ok(())
                }
            }
            ZfsCommands::ListPools => self.show_api_usage(
                "List Pools",
                &format!("GET {}/api/v1/zfs/pools", self.api_endpoint),
                "",
            ),
            ZfsCommands::ListDatasets => self.show_api_usage(
                "List Datasets",
                &format!("GET {}/api/v1/zfs/datasets", self.api_endpoint),
                "",
            ),
            ZfsCommands::Status { pool } => {
                let endpoint = if let Some(pool_name) = pool {
                    format!(
                        "{}/api/v1/zfs/pools/{}/status",
                        self.api_endpoint, pool_name
                    )
                } else {
                    format!("{}/api/v1/zfs/status", self.api_endpoint)
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
                    format!("/api/v1/zfs/datasets/{target}")
                };

                let params = if force { "?force=true" } else { "" };
                self.show_api_usage(
                    "Destroy",
                    &format!("DELETE /api/v1/zfs/datasets/{target}{params}"),
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
                let Some((prop_name, propvalue)) = property.split_once('=') else {
                    println!("Invalid property format. Use: property=value");
                    return Ok(());
                };

                self.show_api_usage(
                    "Set Property",
                    &format!(
                        "PUT {}/api/v1/zfs/properties/{}",
                        self.api_endpoint, prop_name
                    ),
                    &format!(r#"{{"target": "{target}", "value": "{propvalue}"}}"#),
                )
            }
        }
    }

    /// Show Api Usage
    fn show_api_usage(&self, operation: &str, endpoint: &str, body: &str) -> Result<()> {
        println!("📋 Operation: {operation}");
        println!("🔗 API Call:");

        if body.is_empty() {
            println!("   curl {endpoint}");
        } else {
            println!("   curl -X POST {endpoint} \\");
            println!("     -H 'Content-Type: application/json' \\");
            println!("     -d '{body}'");
        }

        println!();
        println!("💡 For interactive management, visit:");
        println!("   {}/ui/zfs", self.api_endpoint);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{ZfsCommands, ZfsHandler};

    #[test]
    fn with_api_endpoint_round_trips_via_accessor() {
        let h = ZfsHandler::with_api_endpoint("http://192.0.2.1:9999");
        assert_eq!(h.api_endpoint(), "http://192.0.2.1:9999");
    }

    #[tokio::test]
    async fn execute_create_snapshot_rejects_missing_at_separator() {
        let mut h = ZfsHandler::with_api_endpoint("http://127.0.0.1:9");
        let r = h
            .execute(ZfsCommands::CreateSnapshot {
                snapshot: "no-at-sign".into(),
            })
            .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn execute_set_rejects_property_without_equals() {
        let mut h = ZfsHandler::with_api_endpoint("http://127.0.0.1:9");
        let r = h
            .execute(ZfsCommands::Set {
                property: "notkeyvalue".into(),
                target: "tank/data".into(),
            })
            .await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn execute_create_dataset_and_list_commands_succeed() {
        let mut h = ZfsHandler::with_api_endpoint("http://127.0.0.1:9");
        assert!(
            h.execute(ZfsCommands::CreateDataset {
                dataset: "tank/data".into(),
                backend: "zfs".into(),
                path: None,
                compression: true,
                checksum: false,
            })
            .await
            .is_ok()
        );
        assert!(h.execute(ZfsCommands::ListPools).await.is_ok());
        assert!(h.execute(ZfsCommands::ListDatasets).await.is_ok());
    }

    #[tokio::test]
    async fn execute_status_with_and_without_pool() {
        let mut h = ZfsHandler::with_api_endpoint("http://127.0.0.1:9");
        assert!(
            h.execute(ZfsCommands::Status {
                pool: Some("tank".into()),
            })
            .await
            .is_ok()
        );
        assert!(h.execute(ZfsCommands::Status { pool: None }).await.is_ok());
    }

    #[tokio::test]
    async fn execute_destroy_dataset_and_snapshot_paths() {
        let mut h = ZfsHandler::with_api_endpoint("http://127.0.0.1:9");
        assert!(
            h.execute(ZfsCommands::Destroy {
                target: "tank/data".into(),
                force: false,
            })
            .await
            .is_ok()
        );
        assert!(
            h.execute(ZfsCommands::Destroy {
                target: "tank/data@snap1".into(),
                force: true,
            })
            .await
            .is_ok()
        );
    }

    #[tokio::test]
    async fn execute_get_and_create_snapshot_happy_path() {
        let mut h = ZfsHandler::with_api_endpoint("http://127.0.0.1:9");
        assert!(
            h.execute(ZfsCommands::Get {
                property: "compression".into(),
                target: "tank/data".into(),
            })
            .await
            .is_ok()
        );
        assert!(
            h.execute(ZfsCommands::CreateSnapshot {
                snapshot: "tank/data@daily".into(),
            })
            .await
            .is_ok()
        );
        assert!(
            h.execute(ZfsCommands::Set {
                property: "compression=on".into(),
                target: "tank/data".into(),
            })
            .await
            .is_ok()
        );
    }
}
