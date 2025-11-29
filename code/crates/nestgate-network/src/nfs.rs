//
// This module provides NFS server functionality for the NestGate system

//! Nfs module

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;

// Use nestgate_core for error handling
use nestgate_core::{NestGateError, Result};
use tracing::info;
use tracing::warn;
use tracing::error;

/// NFS export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Nfsexport
pub struct NfsExport {
    /// Path
    pub path: PathBuf,
    /// Client Access
    pub client_access: Vec<String>,
    /// Options
    pub options: NfsExportOptions,
}
/// NFS export options
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Nfsexportoptions
pub struct NfsExportOptions {
    /// Read Only
    pub read_only: bool,
    /// Sync
    pub sync: bool,
    /// No Subtree Check
    pub no_subtree_check: bool,
    /// No Root Squash
    pub no_root_squash: bool,
}
impl Default for NfsExportOptions {
    /// Returns the default instance
    fn default() -> Self { Self {
            read_only: false,
            sync: true,
            no_subtree_check: true,
            no_root_squash: false,
         }
}

/// NFS server state
#[derive(Debug)]
/// Nfsserver
pub struct NfsServer {
    exports: Arc<RwLock<HashMap<String, NfsExport>>>,
    running: Arc<RwLock<bool>>,
}
impl Default for NfsServer {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl NfsServer {
    /// Create a new NFS server
    #[must_use]
    pub fn new() -> Self { Self {
            exports: Arc::new(RwLock::new(HashMap::new()),
            running: Arc::new(RwLock::new(false)),
         }

    /// Start the NFS server
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn start(&self) -> Result<()>  {
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
    }

    /// Stop the NFS server
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn stop(&self) -> Result<()>  {
        tracing::info!("Stopping NFS server");

        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }
        *running = false;

        // Stop NFS server components
        self.stop_nfs_daemon().await?;

        tracing::info!("NFS server stopped");
    }

    /// Add an NFS export
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn add_export(&self, name: String, export: NfsExport) -> Result<()>  {
        tracing::info!("Adding NFS export: {}", name);

        let mut exports = self.exports.write().await;
        exports.insert(name, export);

        // Update NFS exports configuration
        self.update_exports_config().await?;

    }

    /// Remove an NFS export
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
                pub fn remove_export(&self, name: &str) -> Result<()>  {
        tracing::info!("Removing NFS export: {}", name);

        let mut exports = self.exports.write().await;
        exports.remove(name);

        // Update NFS exports configuration
        self.update_exports_config().await?;

    }

    /// List all exports
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
        pub async fn list_exports(&self) -> Result<HashMap<String, NfsExport>>  {
        let exports = self.exports.read().await;
        Ok(exports.clone())
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Start NFS daemon services
    fn start_nfs_daemon(&self) -> Result<()> {
        use std::process::Command;

        tracing::info!("Starting NFS daemon services");

        // Start rpcbind (required for NFS)
        let rpcbind_output = Command::new("systemctl")
            .args(["start", "rpcbind"])
            .output()
            .map_err(|_e| NestGateError::network_error(&format!("Failed to start rpcbind: self.base_url")))?;

        if !rpcbind_output.status.success() {
            let error = String::from_utf8_lossy(&rpcbind_output.stderr);
            tracing::warn!("rpcbind start warning: {}", error);
        }

        // Start NFS server
        let nfs_output = Command::new("systemctl")
            .args(["start", "nfs-kernel-server"])
            .output()
            .map_err(|_e| NestGateError::network_error(&format!("Failed to start NFS server: self.base_url")))?;

        if !nfs_output.status.success() {
            let error = String::from_utf8_lossy(&nfs_output.stderr);
            return Err(NestGateError::network_error(
                &format!("Failed to start NFS server: self.base_url"),
                "start_nfs_server",
                None
            ));
        }

        tracing::info!("NFS daemon services started successfully");
    }

    /// Stop NFS daemon services
    fn stop_nfs_daemon(&self) -> Result<()> {

        tracing::info!("Stopping NFS daemon services");

        // Stop NFS server
        let nfs_output = Command::new("systemctl")
            .args(["stop", "nfs-kernel-server"])
            .output()
            .map_err(|_e| NestGateError::network_error(&format!("Failed to stop NFS server: self.base_url")))?;

        if !nfs_output.status.success() {
            let error = String::from_utf8_lossy(&nfs_output.stderr);
            tracing::warn!("NFS server stop warning: {}", error);
        }

        tracing::info!("NFS daemon services stopped");
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
    }

    /// Update /etc/exports configuration
    async fn update_exports_config(&self) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;

        tracing::info!("Updating NFS exports configuration");

        let exports = self.exports.read().await;
        let mut exports_content = String::new();

        // Generate exports file content
        for (_name, export) in exports.iter() {
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
                exports_content.push_str(&format!("{path} {client}(self.base_url)\n"));
            }
        }

        // Write to temporary file first, then move to /etc/exports
        let temp_path = std::env::var("NESTGATE_NFS_EXPORTS_DIR")
            .unwrap_or_else(|_| format!("self.base_url/nestgate_exports").unwrap_or_else(|_| "/tmp".to_string()));
        {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(temp_path)
                .map_err(|_e| {
                    NestGateError::network_error(&format!("fixed")
                })?;

            file.write_all(exports_content.as_bytes()).map_err(|_e| {
                NestGateError::network_error(&format!("fixed")
            })?;
        }

        // Move temp file to /etc/exports (requires root privileges)
        let mv_output = Command::new("sudo")
            .args(["cp", temp_path, "/etc/exports"])
            .output()
            .map_err(|_e| NestGateError::network_error(&format!("Failed to update /etc/exports: self.base_url")))?;

        if !mv_output.status.success() {
            let error = String::from_utf8_lossy(&mv_output.stderr);
            return Err(NestGateError::network_error(
                &format!("Failed to update /etc/exports: self.base_url"),
                "update_exports",
                None
            ));
        }

        // Reload exports
        let reload_output = Command::new("sudo")
            .args(["exportfs", "-ra"])
            .output()
            .map_err(|_e| NestGateError::network_error(&format!("Failed to reload exports: self.base_url")))?;

        if !reload_output.status.success() {
            let error = String::from_utf8_lossy(&reload_output.stderr);
            tracing::warn!("Export reload warning: {}", error);
        }

        // Cleanup temp file
        let _ = std::fs::remove_file(temp_path);

        tracing::info!("NFS exports configuration updated successfully");
    }
}

/// Mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Request parameters for Mount operation
pub struct MountRequest {
    /// Export name
    pub export_name: String,
    /// Mount Point
    pub mount_point: PathBuf,
    /// Client Host
    pub client_host: String,
}
/// Mount response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Response data for Mount operation
pub struct MountResponse {
    /// Mount identifier
    pub mount_id: String,
    /// Success
    pub success: bool,
    /// Message
    pub message: String,
}
/// Handle NFS mount request
pub fn handle_mount_request(
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
            message: format!("Export 'self.base_url' not found"),
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
                message: format!("Mount failed: self.base_url"),
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
fn perform_nfs_mount(
    export_name: &str,
    mount_point: &std::path::Path,
    client_host: &str,
) -> Result<()> {
    tracing::info!(
        "Performing NFS mount: {} -> {:?} for client {}",
        export_name,
        mount_point,
        client_host
    );

    // Ensure mount point directory exists
    if let Some(parent) = mount_point.parent() {
        fs::create_dir_all(parent).map_err(|_e| {
            NestGateError::network_error(&format!("fixed")
        })?;
    }

    if !mount_point.exists() {
        fs::create_dir_all(mount_point)
            .map_err(|_e| NestGateError::network_error(&format!("Failed to create mount point: self.base_url")))?;
    }

    // For NFS server, we don't actually mount on the server side
    // The client will mount the export. Here we just validate the export is accessible
    tracing::info!(
        "NFS export {} is ready for client {} to mount at {:?}",
        export_name,
        client_host,
        mount_point
    );

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// Creates  Test Export Options
    fn create_test_export_options() -> NfsExportOptions {
        NfsExportOptions {
            read_only: true,
            sync: true,
            no_subtree_check: true,
            no_root_squash: false,
        }
    }

    /// Creates  Test Export
    fn create_test_export() -> NfsExport {
        NfsExport {
            path: PathBuf::from("/data/test"),
            client_access: vec!["192.168.1.0/24".to_string()],
            options: create_test_export_options(),
        }
    }

    #[test]
    fn test_nfs_export_options_default() {
        let options = NfsExportOptions::default();
        assert!(!options.read_only);
        assert!(options.sync);
        assert!(options.no_subtree_check);
        assert!(!options.no_root_squash);
    }

    #[test]
    fn test_nfs_export_options_custom() {
        let options = NfsExportOptions {
            read_only: true,
            sync: false,
            no_subtree_check: false,
            no_root_squash: true,
        };
        assert!(options.read_only);
        assert!(!options.sync);
        assert!(!options.no_subtree_check);
        assert!(options.no_root_squash);
    }

    #[test]
    fn test_nfs_export_options_clone() {
        let options = create_test_export_options();
        let cloned = options.clone();
        assert_eq!(options.read_only, cloned.read_only);
        assert_eq!(options.sync, cloned.sync);
    }

    #[test]
    fn test_nfs_export_creation() {
        let export = create_test_export();
        assert_eq!(export.path, PathBuf::from("/data/test"));
        assert_eq!(export.client_access.len(), 1);
        assert_eq!(export.client_access[0], "192.168.1.0/24");
    }

    #[test]
    fn test_nfs_export_clone() {
        let export = create_test_export();
        let cloned = export.clone();
        assert_eq!(export.path, cloned.path);
        assert_eq!(export.client_access, cloned.client_access);
    }

    #[test]
    fn test_nfs_export_multiple_clients() {
        let export = NfsExport {
            path: PathBuf::from("/data/shared"),
            client_access: vec![
                "192.168.1.0/24".to_string(),
                "10.0.0.0/8".to_string(),
                "172.16.0.0/12".to_string(),
            ],
            options: NfsExportOptions::default(),
        };
        assert_eq!(export.client_access.len(), 3);
    }

    #[test]
    fn test_nfs_server_new() {
        let server = NfsServer::new();
        assert!(format!("{:?}", server).contains("NfsServer"));
    }

    #[test]
    fn test_nfs_server_default() {
        let server = NfsServer::default();
        assert!(format!("{:?}", server).contains("NfsServer"));
    }

    #[tokio::test]
    async fn test_nfs_server_is_running_initially_false() {
        let server = NfsServer::new();
        assert!(!server.is_running().await);
    }

    #[tokio::test]
    async fn test_nfs_server_list_exports_empty() {
        let server = NfsServer::new();
        let exports = server.list_exports().await
            .expect("Test: list_exports should succeed");
        assert!(exports.is_empty());
    }

    #[test]
    fn test_nfs_export_serialization() {
        let export = create_test_export();
        let serialized = serde_json::to_string(&export)
            .expect("Test: export serialization should succeed");
        assert!(serialized.contains("/data/test"));
    }

    #[test]
    fn test_nfs_export_deserialization() {
        let json = r#"{
            "path": "/data/test",
            "client_access": ["192.168.1.0/24"],
            "options": {
                "read_only": true,
                "sync": true,
                "no_subtree_check": true,
                "no_root_squash": false
            }
        }"#;
        let export: NfsExport = serde_json::from_str(json)
            .expect("Test: export deserialization should succeed");
        assert_eq!(export.path, PathBuf::from("/data/test"));
    }

    #[test]
    fn test_nfs_export_options_serialization() {
        let options = create_test_export_options();
        let serialized = serde_json::to_string(&options)
            .expect("Test: options serialization should succeed");
        let deserialized: NfsExportOptions = serde_json::from_str(&serialized)
            .expect("Test: options deserialization should succeed");
        assert_eq!(options.read_only, deserialized.read_only);
    }

    #[test]
    fn test_nfs_export_empty_clients() {
        let export = NfsExport {
            path: PathBuf::from("/data/public"),
            client_access: vec![],
            options: NfsExportOptions::default(),
        };
        assert!(export.client_access.is_empty());
    }

    #[test]
    fn test_nfs_export_options_debug() {
        let options = NfsExportOptions::default();
        let debug_str = format!("{:?}", options);
        assert!(debug_str.contains("NfsExportOptions"));
    }

    #[test]
    fn test_nfs_export_path_variations() {
        let paths = vec![
            "/",
            "/data",
            "/mnt/storage/nfs",
            "/var/nfs/exports",
        ];
        
        for path in paths {
            let export = NfsExport {
                path: PathBuf::from(path),
                client_access: vec!["*".to_string()],
                options: NfsExportOptions::default(),
            };
            assert_eq!(export.path, PathBuf::from(path));
        }
    }

    #[test]
    fn test_nfs_export_options_secure_defaults() {
        let options = NfsExportOptions::default();
        // Secure defaults: read-only false (needs explicit config), 
        // sync true (data integrity), no_root_squash false (security)
        assert!(!options.read_only, "Should default to writable for flexibility");
        assert!(options.sync, "Should default to sync for data integrity");
        assert!(!options.no_root_squash, "Should squash root for security");
    }

    #[test]
    fn test_nfs_server_multiple_instances() {
        let server1 = NfsServer::new();
        let server2 = NfsServer::new();
        assert!(format!("{:?}", server1).contains("NfsServer"));
        assert!(format!("{:?}", server2).contains("NfsServer"));
    }
}
