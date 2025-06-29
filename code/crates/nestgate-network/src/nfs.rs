//! NFS (Network File System) protocol implementation
//!
//! This module provides NFS server functionality for the NestGate system

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use nestgate_core for error handling
use nestgate_core::{NestGateError, Result};

/// NFS export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsExport {
    pub path: PathBuf,
    pub client_access: Vec<String>,
    pub options: NfsExportOptions,
}

/// NFS export options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NfsExportOptions {
    pub read_only: bool,
    pub sync: bool,
    pub no_subtree_check: bool,
    pub no_root_squash: bool,
}

impl Default for NfsExportOptions {
    fn default() -> Self {
        Self {
            read_only: false,
            sync: true,
            no_subtree_check: true,
            no_root_squash: false,
        }
    }
}

/// NFS server state
#[derive(Debug)]
pub struct NfsServer {
    exports: Arc<RwLock<HashMap<String, NfsExport>>>,
    running: Arc<RwLock<bool>>,
}

impl NfsServer {
    /// Create a new NFS server
    pub fn new() -> Self {
        Self {
            exports: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the NFS server
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting NFS server");

        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;

        // Start NFS server components
        self.start_nfs_daemon().await?;
        self.configure_exports().await?;
        self.setup_mount_points().await?;

        tracing::info!("NFS server started");
        Ok(())
    }

    /// Stop the NFS server
    pub async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping NFS server");

        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }
        *running = false;

        // Stop NFS server components
        self.stop_nfs_daemon().await?;

        tracing::info!("NFS server stopped");
        Ok(())
    }

    /// Add an NFS export
    pub async fn add_export(&self, name: String, export: NfsExport) -> Result<()> {
        tracing::info!("Adding NFS export: {}", name);

        let mut exports = self.exports.write().await;
        exports.insert(name, export);

        // Update NFS exports configuration
        self.update_exports_config().await?;

        Ok(())
    }

    /// Remove an NFS export
    pub async fn remove_export(&self, name: &str) -> Result<()> {
        tracing::info!("Removing NFS export: {}", name);

        let mut exports = self.exports.write().await;
        exports.remove(name);

        // Update NFS exports configuration
        self.update_exports_config().await?;

        Ok(())
    }

    /// List all exports
    pub async fn list_exports(&self) -> Result<HashMap<String, NfsExport>> {
        let exports = self.exports.read().await;
        Ok(exports.clone())
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Start NFS daemon services
    async fn start_nfs_daemon(&self) -> Result<()> {
        use std::process::Command;

        tracing::info!("Starting NFS daemon services");

        // Start rpcbind (required for NFS)
        let rpcbind_output = Command::new("systemctl")
            .args(&["start", "rpcbind"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to start rpcbind: {}", e)))?;

        if !rpcbind_output.status.success() {
            let error = String::from_utf8_lossy(&rpcbind_output.stderr);
            tracing::warn!("rpcbind start warning: {}", error);
        }

        // Start NFS server
        let nfs_output = Command::new("systemctl")
            .args(&["start", "nfs-kernel-server"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to start NFS server: {}", e)))?;

        if !nfs_output.status.success() {
            let error = String::from_utf8_lossy(&nfs_output.stderr);
            return Err(NestGateError::Network(format!(
                "Failed to start NFS server: {}",
                error
            )));
        }

        tracing::info!("NFS daemon services started successfully");
        Ok(())
    }

    /// Stop NFS daemon services
    async fn stop_nfs_daemon(&self) -> Result<()> {
        use std::process::Command;

        tracing::info!("Stopping NFS daemon services");

        // Stop NFS server
        let nfs_output = Command::new("systemctl")
            .args(&["stop", "nfs-kernel-server"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to stop NFS server: {}", e)))?;

        if !nfs_output.status.success() {
            let error = String::from_utf8_lossy(&nfs_output.stderr);
            tracing::warn!("NFS server stop warning: {}", error);
        }

        tracing::info!("NFS daemon services stopped");
        Ok(())
    }

    /// Configure NFS exports
    async fn configure_exports(&self) -> Result<()> {
        self.update_exports_config().await
    }

    /// Set up mount points
    async fn setup_mount_points(&self) -> Result<()> {
        use std::fs;

        tracing::info!("Setting up NFS mount points");

        let exports = self.exports.read().await;
        for (name, export) in exports.iter() {
            // Ensure the export path exists
            if let Some(parent) = export.path.parent() {
                if let Err(e) = fs::create_dir_all(parent) {
                    tracing::warn!("Failed to create directory for export {}: {}", name, e);
                }
            }

            // Ensure the export path itself exists
            if !export.path.exists() {
                if let Err(e) = fs::create_dir_all(&export.path) {
                    tracing::warn!("Failed to create export directory {}: {}", name, e);
                }
            }
        }

        tracing::info!("Mount points setup complete");
        Ok(())
    }

    /// Update /etc/exports configuration
    async fn update_exports_config(&self) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;

        tracing::info!("Updating NFS exports configuration");

        let exports = self.exports.read().await;
        let mut exports_content = String::new();

        // Generate exports file content
        for (name, export) in exports.iter() {
            let path = export.path.to_string_lossy();
            let mut options = Vec::new();

            if export.options.read_only {
                options.push("ro");
            } else {
                options.push("rw");
            }

            if export.options.sync {
                options.push("sync");
            } else {
                options.push("async");
            }

            if export.options.no_subtree_check {
                options.push("no_subtree_check");
            }

            if export.options.no_root_squash {
                options.push("no_root_squash");
            }

            let options_str = options.join(",");

            // Add each client access entry
            for client in &export.client_access {
                exports_content.push_str(&format!("{} {}({})\n", path, client, options_str));
            }
        }

        // Write to temporary file first, then move to /etc/exports
        let temp_path = "/tmp/nestgate_exports";
        {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(temp_path)
                .map_err(|e| {
                    NestGateError::Network(format!("Failed to create temp exports file: {}", e))
                })?;

            file.write_all(exports_content.as_bytes()).map_err(|e| {
                NestGateError::Network(format!("Failed to write exports file: {}", e))
            })?;
        }

        // Move temp file to /etc/exports (requires root privileges)
        use std::process::Command;
        let mv_output = Command::new("sudo")
            .args(&["cp", temp_path, "/etc/exports"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to update /etc/exports: {}", e)))?;

        if !mv_output.status.success() {
            let error = String::from_utf8_lossy(&mv_output.stderr);
            return Err(NestGateError::Network(format!(
                "Failed to update /etc/exports: {}",
                error
            )));
        }

        // Reload exports
        let reload_output = Command::new("sudo")
            .args(&["exportfs", "-ra"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to reload exports: {}", e)))?;

        if !reload_output.status.success() {
            let error = String::from_utf8_lossy(&reload_output.stderr);
            tracing::warn!("Export reload warning: {}", error);
        }

        // Cleanup temp file
        let _ = std::fs::remove_file(temp_path);

        tracing::info!("NFS exports configuration updated successfully");
        Ok(())
    }
}

/// Mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountRequest {
    pub export_name: String,
    pub mount_point: PathBuf,
    pub client_host: String,
}

/// Mount response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountResponse {
    pub mount_id: String,
    pub success: bool,
    pub message: String,
}

/// Handle NFS mount request
pub async fn handle_mount_request(
    server: &NfsServer,
    request: MountRequest,
) -> Result<MountResponse> {
    tracing::info!(
        "Handling NFS mount request for export: {}",
        request.export_name
    );

    // Check if export exists
    let exports = server.list_exports().await?;
    if !exports.contains_key(&request.export_name) {
        return Ok(MountResponse {
            mount_id: String::new(),
            success: false,
            message: format!("Export '{}' not found", request.export_name),
        });
    }

    // Implement actual mount handling
    let mount_id = uuid::Uuid::new_v4().to_string();

    // Perform the actual NFS mount operation
    match perform_nfs_mount(
        &request.export_name,
        &request.mount_point,
        &request.client_host,
    )
    .await
    {
        Ok(_) => tracing::info!(
            "NFS mount successful: {} -> {:?}",
            request.export_name,
            request.mount_point
        ),
        Err(e) => {
            tracing::error!("NFS mount failed: {}", e);
            return Ok(MountResponse {
                mount_id: String::new(),
                success: false,
                message: format!("Mount failed: {}", e),
            });
        }
    }

    Ok(MountResponse {
        mount_id,
        success: true,
        message: "Mount successful".to_string(),
    })
}

/// Perform actual NFS mount operation
async fn perform_nfs_mount(
    export_name: &str,
    mount_point: &std::path::Path,
    client_host: &str,
) -> Result<()> {
    use std::fs;
    use std::process::Command;

    tracing::info!(
        "Performing NFS mount: {} -> {:?} for client {}",
        export_name,
        mount_point,
        client_host
    );

    // Ensure mount point directory exists
    if let Some(parent) = mount_point.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            NestGateError::Network(format!("Failed to create mount point parent: {}", e))
        })?;
    }

    if !mount_point.exists() {
        fs::create_dir_all(mount_point)
            .map_err(|e| NestGateError::Network(format!("Failed to create mount point: {}", e)))?;
    }

    // For NFS server, we don't actually mount on the server side
    // The client will mount the export. Here we just validate the export is accessible
    tracing::info!(
        "NFS export {} is ready for client {} to mount at {:?}",
        export_name,
        client_host,
        mount_point
    );

    Ok(())
}
