//! BYOB (Bring Your Own Biome) Storage Management
//!
//! Integrates with orchestration coordination and compute execution modules.
//!
//! This module provides workspace management and storage provisioning for
//! the BYOB (Bring Your Own Biome) system, allowing users to bring their own
//! development environments and data.

use crate::manager::ZfsManager;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::error;
use tracing::info;
use tracing::warn;
// Removed unused tracing import

/// BYOB storage manager
pub struct ByobManager {
    zfs_manager: Arc<ZfsManager>,
    orchestration_endpoint: Option<String>,
    compute_endpoint: Option<String>,
}

/// BYOB storage request from orchestration module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageRequest {
    /// Request ID for tracking
    pub request_id: String,
    /// Workspace name
    pub workspace_name: String,
    /// Storage size in GB
    pub storage_size_gb: u64,
    /// Storage tier (hot, warm, cold)
    pub tier: String,
    /// Mount point for the workspace
    pub mount_point: String,
    /// Additional configuration
    pub config: HashMap<String, String>,
}

/// BYOB storage response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByobStorageResponse {
    /// Request ID that this response corresponds to
    pub request_id: String,
    /// Success status
    pub success: bool,
    /// Dataset name created
    pub dataset_name: Option<String>,
    /// Mount point
    pub mount_point: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl ByobManager {
    /// Create a new BYOB manager
    pub fn new(zfs_manager: Arc<ZfsManager>) -> Self {
        let orchestration_endpoint = std::env::var("ORCHESTRATION_URL").ok();
        let compute_endpoint = std::env::var("COMPUTE_URL").ok();

        Self {
            zfs_manager,
            orchestration_endpoint,
            compute_endpoint,
        }
    }

    /// Process a BYOB storage request
    pub async fn process_storage_request(
        &self,
        request: ByobStorageRequest,
    ) -> Result<ByobStorageResponse> {
        info!("🏗️ Processing BYOB storage request: {}", request.request_id);

        // Validate request
        if request.workspace_name.is_empty() {
            return Ok(ByobStorageResponse {
                request_id: request.request_id,
                success: false,
                dataset_name: None,
                mount_point: None,
                error: Some("Workspace name cannot be empty".to_string()),
                metadata: HashMap::new(),
            });
        }

        // Create dataset name
        let dataset_name = format!("nestpool/byob/{}", request.workspace_name);

        // Create the dataset
        match self.create_workspace_dataset(&dataset_name, &request).await {
            Ok(()) => {
                info!(
                    "✅ Successfully created BYOB workspace: {}",
                    request.workspace_name
                );

                // Notify orchestration module if available
                if let Some(ref orchestration_endpoint) = self.orchestration_endpoint {
                    self.notify_orchestration_module(orchestration_endpoint, &request)
                        .await?;
                }

                // Notify compute module if available
                if let Some(ref compute_endpoint) = self.compute_endpoint {
                    self.notify_compute_module(compute_endpoint, &request)
                        .await?;
                }

                Ok(ByobStorageResponse {
                    request_id: request.request_id,
                    success: true,
                    dataset_name: Some(dataset_name),
                    mount_point: Some(request.mount_point),
                    error: None,
                    metadata: HashMap::new(),
                })
            }
            Err(e) => {
                error!("❌ Failed to create BYOB workspace: {}", e);
                Ok(ByobStorageResponse {
                    request_id: request.request_id,
                    success: false,
                    dataset_name: None,
                    mount_point: None,
                    error: Some(e.to_string()),
                    metadata: HashMap::new(),
                })
            }
        }
    }

    /// Create a workspace dataset
    async fn create_workspace_dataset(
        &self,
        dataset_name: &str,
        request: &ByobStorageRequest,
    ) -> Result<()> {
        info!("📦 Creating workspace dataset: {}", dataset_name);

        // Create dataset with appropriate settings based on tier
        let tier = match request.tier.as_str() {
            "hot" => nestgate_core::StorageTier::Hot,
            "warm" => nestgate_core::StorageTier::Warm,
            "cold" => nestgate_core::StorageTier::Cold,
            _ => nestgate_core::StorageTier::Warm,
        };

        // Extract parent and child from dataset name
        let parts: Vec<&str> = dataset_name.split('/').collect();
        let parent = parts[0..parts.len() - 1].join("/");
        let child = parts[parts.len() - 1];

        // Create the dataset
        self.zfs_manager
            .create_dataset(child, &parent, tier)
            .await?;

        // Set quota using zfs command directly if specified
        if request.storage_size_gb > 0 {
            let quota_bytes = request.storage_size_gb * 1024 * 1024 * 1024;
            let output = tokio::process::Command::new("zfs")
                .args(["set", &format!("quota={quota_bytes}"), dataset_name])
                .output()
                .await?;

            if !output.status.success() {
                return Err(anyhow::anyhow!(
                    "Failed to set quota: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }

        // Set mount point using zfs command directly
        let output = tokio::process::Command::new("zfs")
            .args([
                "set",
                &format!("mountpoint={}", request.mount_point),
                dataset_name,
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to set mountpoint: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    /// Notify orchestration module of storage creation
    async fn notify_orchestration_module(
        &self,
        orchestration_endpoint: &str,
        request: &ByobStorageRequest,
    ) -> Result<()> {
        info!("📡 Notifying orchestration module about workspace creation");

        let notification = serde_json::json!({
            "event": "workspace_created",
            "workspace_name": request.workspace_name,
            "storage_size_gb": request.storage_size_gb,
            "tier": request.tier,
            "mount_point": request.mount_point,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        // Send notification to orchestration module
        match reqwest::Client::new()
            .post(format!("{orchestration_endpoint}/api/v1/notifications"))
            .json(&notification)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    info!("✅ Successfully notified orchestration module");
                } else {
                    warn!(
                        "⚠️ Orchestration module notification failed: {}",
                        response.status()
                    );
                }
            }
            Err(e) => {
                warn!("⚠️ Failed to notify orchestration module: {}", e);
            }
        }

        Ok(())
    }

    /// Notify compute module of storage creation
    async fn notify_compute_module(
        &self,
        compute_endpoint: &str,
        request: &ByobStorageRequest,
    ) -> Result<()> {
        info!("💻 Notifying compute module about workspace creation");

        let notification = serde_json::json!({
            "event": "storage_ready",
            "workspace_name": request.workspace_name,
            "mount_point": request.mount_point,
            "storage_size_gb": request.storage_size_gb,
            "tier": request.tier,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        // Send notification to compute module
        match reqwest::Client::new()
            .post(format!("{compute_endpoint}/api/v1/notifications"))
            .json(&notification)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    info!("✅ Successfully notified compute module");
                } else {
                    warn!(
                        "⚠️ Compute module notification failed: {}",
                        response.status()
                    );
                }
            }
            Err(e) => {
                warn!("⚠️ Failed to notify compute module: {}", e);
            }
        }

        Ok(())
    }

    /// Clean up workspace storage
    pub async fn cleanup_workspace(&self, workspace_name: &str) -> Result<()> {
        info!("🧹 Cleaning up BYOB workspace: {}", workspace_name);

        let dataset_name = format!("nestpool/byob/{workspace_name}");

        // Destroy the dataset using zfs command directly
        let output = tokio::process::Command::new("zfs")
            .args(["destroy", "-r", &dataset_name])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to destroy dataset: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Notify modules about cleanup
        if let Some(ref orchestration_endpoint) = self.orchestration_endpoint {
            self.notify_workspace_cleanup(orchestration_endpoint, workspace_name)
                .await?;
        }

        if let Some(ref compute_endpoint) = self.compute_endpoint {
            self.notify_workspace_cleanup(compute_endpoint, workspace_name)
                .await?;
        }

        info!(
            "✅ Successfully cleaned up BYOB workspace: {}",
            workspace_name
        );
        Ok(())
    }

    /// Notify modules about workspace cleanup
    async fn notify_workspace_cleanup(&self, endpoint: &str, workspace_name: &str) -> Result<()> {
        let notification = serde_json::json!({
            "event": "workspace_destroyed",
            "workspace_name": workspace_name,
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        match reqwest::Client::new()
            .post(format!("{endpoint}/api/v1/notifications"))
            .json(&notification)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    info!("✅ Successfully notified module about cleanup");
                } else {
                    warn!(
                        "⚠️ Module cleanup notification failed: {}",
                        response.status()
                    );
                }
            }
            Err(e) => {
                warn!("⚠️ Failed to notify module about cleanup: {}", e);
            }
        }

        Ok(())
    }
}
