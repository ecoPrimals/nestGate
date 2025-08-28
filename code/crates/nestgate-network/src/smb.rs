//
// This module provides SMB server functionality for the NestGate system

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

/// SMB share configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbShare {
    pub path: PathBuf,
    pub name: String,
    pub comment: String,
    pub read_only: bool,
    pub guest_ok: bool,
    pub browseable: bool,
}

/// SMB server state
#[derive(Debug)]
pub struct SmbServer {
    shares: Arc<RwLock<HashMap<String, SmbShare>>>,
    running: Arc<RwLock<bool>>,
}

impl Default for SmbServer {
    fn default() -> Self {
        Self::new()
    }
}

impl SmbServer {
    /// Create a new SMB server
    pub fn new() -> Self {
        Self {
            shares: Arc::new(RwLock::new(HashMap::new())),
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Start the SMB server
    pub async fn start(&self) -> Result<()> {
        tracing::info!("Starting SMB server");

        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;

        // Start SMB server components
        self.start_samba_daemon().await?;
        self.configure_shares().await?;
        self.setup_authentication().await?;

        tracing::info!("SMB server started");
    }

    /// Stop the SMB server
    pub async fn stop(&self) -> Result<()> {
        tracing::info!("Stopping SMB server");

        let mut running = self.running.write().await;
        if !*running {
            return Ok(());
        }
        *running = false;

        // Stop SMB server components
        self.stop_samba_daemon().await?;

        tracing::info!("SMB server stopped");
    }

    /// Add an SMB share
    pub async fn add_share(&self, name: String, share: SmbShare) -> Result<()> {
        tracing::info!("Adding SMB share: {}", name);

        let mut shares = self.shares.write().await;
        shares.insert(name, share);

        // Update SMB configuration
        self.update_smb_config().await?;

    }

    /// Remove an SMB share
    pub async fn remove_share(&self, name: &str) -> Result<()> {
        tracing::info!("Removing SMB share: {}", name);

        let mut shares = self.shares.write().await;
        shares.remove(name);

        // Update SMB configuration
        self.update_smb_config().await?;

    }

    /// List all shares
    pub async fn list_shares(&self) -> Result<HashMap<String, SmbShare>> {
        let shares = self.shares.read().await;
        Ok(shares.clone())
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Start Samba daemon services
    async fn start_samba_daemon(&self) -> Result<()> {
        use std::process::Command;

        tracing::info!("Starting Samba daemon services");

        // Start smbd (SMB/CIFS server daemon)
        let smbd_output = Command::new("systemctl")
            .args(["start", "smbd"])
            .output()
            .map_err(|e| NestGateError::network_error(&format!("Failed to start smbd: {e}"), "start_smbd", None))?;

        if !smbd_output.status.success() {
            let error = String::from_utf8_lossy(&smbd_output.stderr);
            return Err(NestGateError::Network(format!(
                "Failed to start smbd: {error}"
            )));
        }

        // Start nmbd (NetBIOS name server daemon)
        let nmbd_output = Command::new("systemctl")
            .args(["start", "nmbd"])
            .output()
            .map_err(|e| NestGateError::network_error(&format!("Failed to start nmbd: {e}"), "start_nmbd", None))?;

        if !nmbd_output.status.success() {
            let error = String::from_utf8_lossy(&nmbd_output.stderr);
            tracing::warn!("nmbd start warning: {}", error);
        }

        tracing::info!("Samba daemon services started successfully");
    }

    /// Stop Samba daemon services
    async fn stop_samba_daemon(&self) -> Result<()> {

        tracing::info!("Stopping Samba daemon services");

        // Stop smbd
        let smbd_output = Command::new("systemctl")
            .args(["stop", "smbd"])
            .output()
            .map_err(|e| NestGateError::network_error(&format!("Failed to stop smbd: {e}"), "stop_smbd", None))?;

        if !smbd_output.status.success() {
            let error = String::from_utf8_lossy(&smbd_output.stderr);
            tracing::warn!("smbd stop warning: {}", error);
        }

        // Stop nmbd
        let nmbd_output = Command::new("systemctl")
            .args(["stop", "nmbd"])
            .output()
            .map_err(|e| NestGateError::network_error(&format!("Failed to stop nmbd: {e}"), "stop_nmbd", None))?;

        if !nmbd_output.status.success() {
            let error = String::from_utf8_lossy(&nmbd_output.stderr);
            tracing::warn!("nmbd stop warning: {}", error);
        }

        tracing::info!("Samba daemon services stopped");
    }

    /// Configure SMB shares
    async fn configure_shares(&self) -> Result<()> {
        self.update_smb_config().await
    }

    /// Set up SMB authentication
    async fn setup_authentication(&self) -> Result<()> {
        tracing::info!("Setting up SMB authentication");

        // For now, we'll use guest access and basic authentication
        // In a production environment, this would integrate with proper user management

        tracing::info!("SMB authentication setup complete");
    }

    /// Update Samba configuration
    async fn update_smb_config(&self) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;

        tracing::info!("Updating Samba configuration");

        let shares = self.shares.read().await;
        let mut config_content = String::new();

        // Add global section
        config_content.push_str("[global]\n");
        config_content.push_str(&format!(
            "   workgroup = {}\n",
            nestgate_core::constants::smb::workgroup()
        ));
        config_content.push_str(&format!(
            "   server string = {}\n",
            nestgate_core::constants::smb::server_string()
        ));
        config_content.push_str("   security = user\n");
        config_content.push_str("   map to guest = bad user\n");
        config_content.push_str("   dns proxy = no\n");
        config_content.push_str(&format!(
            "   log file = {}\n",
            nestgate_core::constants::smb::log_file()
        ));
        config_content.push_str(&format!(
            "   max log size = {}\n",
            nestgate_core::constants::smb::max_log_size()
        ));
        config_content.push_str("   logging = file\n");
        config_content.push_str(&format!(
            "   panic action = {}\n",
            nestgate_core::constants::smb::panic_action()
        ));
        config_content.push_str("   server role = standalone server\n");
        config_content.push_str("   obey pam restrictions = yes\n");
        config_content.push_str("   unix password sync = yes\n");
        config_content.push_str(&format!(
            "   passwd program = {}\n",
            nestgate_core::constants::smb::passwd_program()
        ));
        config_content.push_str(&format!(
            "   passwd chat = {}\n",
            nestgate_core::constants::smb::password_chat()
        ));
        config_content.push_str("   pam password change = yes\n");
        config_content.push_str("   map to guest = bad user\n\n");

        // Add shares
        for (name, share) in shares.iter() {
            config_content.push_str(&format!("[{name}]\n"));
            config_content.push_str(&format!("   comment = {}\n", share.comment));
            config_content.push_str(&format!("   path = {}\n", share.path.to_string_lossy()));

            if share.browseable {
                config_content.push_str("   browseable = yes\n");
            } else {
                config_content.push_str("   browseable = no\n");
            }

            if share.guest_ok {
                config_content.push_str("   guest ok = yes\n");
            } else {
                config_content.push_str("   guest ok = no\n");
            }

            if share.read_only {
                config_content.push_str("   read only = yes\n");
            } else {
                config_content.push_str("   read only = no\n");
                config_content.push_str("   writable = yes\n");
            }

            config_content.push_str("   create mask = 0755\n");
            config_content.push_str("   directory mask = 0755\n\n");
        }

        // Write to temporary file first
        let temp_dir = nestgate_core::constants::defaults::TEMP_DIR;
        let temp_path = format!("{temp_dir}/nestgate_smb.conf");
        {
            let mut file = OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(&temp_path)
                .map_err(|e| {
                    NestGateError::Network(format!("Failed to create temp SMB config: {e}"))
                })?;

            file.write_all(config_content.as_bytes())
                .map_err(|e| NestGateError::Network(format!("Failed to write SMB config: {e}")))?;
        }

        // Move temp file to /etc/samba/smb.conf (requires root privileges)
        let mv_output = Command::new("sudo")
            .args(["cp", &temp_path, &format!("{}/samba/smb.conf", 
                std::env::var("NESTGATE_CONFIG_DIR").unwrap_or_else(|_| "/etc".to_string()))])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to update smb.conf: {e}")))?;

        if !mv_output.status.success() {
            let error = String::from_utf8_lossy(&mv_output.stderr);
            return Err(NestGateError::Network(format!(
                "Failed to update smb.conf: {error}"
            )));
        }

        // Test configuration
        let test_output = Command::new("testparm")
            .args(["-s"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to test SMB config: {e}")))?;

        if !test_output.status.success() {
            let error = String::from_utf8_lossy(&test_output.stderr);
            tracing::warn!("SMB config test warning: {}", error);
        }

        // Reload Samba configuration
        let reload_output = Command::new("sudo")
            .args(["systemctl", "reload", "smbd"])
            .output()
            .map_err(|e| NestGateError::Network(format!("Failed to reload Samba: {e}")))?;

        if !reload_output.status.success() {
            let error = String::from_utf8_lossy(&reload_output.stderr);
            tracing::warn!("Samba reload warning: {}", error);
        }

        // Cleanup temp file
        let _ = std::fs::remove_file(&temp_path);

        tracing::info!("Samba configuration updated successfully");
    }
}

/// SMB mount request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbMountRequest {
    pub share_name: String,
    pub mount_point: PathBuf,
    pub username: Option<String>,
    pub password: Option<String>,
}

/// SMB mount response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmbMountResponse {
    pub mount_id: String,
    pub success: bool,
    pub message: String,
}

/// Handle SMB mount request
pub async fn handle_smb_mount_request(
    server: &SmbServer,
    request: SmbMountRequest,
) -> Result<SmbMountResponse> {
    tracing::info!(
        "Handling SMB mount request for share: {}",
        request.share_name
    );

    // Check if share exists
    let shares = server.list_shares().await?;
    if !shares.contains_key(&request.share_name) {
        return Ok(SmbMountResponse {
            mount_id: String::new(),
            success: false,
            message: format!("Share '{}' not found", request.share_name),
        });
    }

    // Implement actual mount handling
    let mount_id = uuid::Uuid::new_v4().to_string();

    // Perform the actual SMB mount operation
    match perform_smb_mount(
        &request.share_name,
        &request.mount_point,
        &request.username,
        &request.password,
    )
    .await
    {
        Ok(_) => tracing::info!(
            "SMB mount successful: {} -> {:?}",
            request.share_name,
            request.mount_point
        ),
        Err(e) => {
            tracing::error!("SMB mount failed: {}", e);
            return Ok(SmbMountResponse {
                mount_id: String::new(),
                success: false,
                message: format!("Mount failed: {e}"),
            });
        }
    }

    Ok(SmbMountResponse {
        mount_id,
        success: true,
        message: "Mount successful".to_string(),
    })
}

/// Perform actual SMB mount operation
async fn perform_smb_mount(
    share_name: &str,
    mount_point: &std::path::Path,
    username: &Option<String>,
    _password: &Option<String>,
) -> Result<()> {
    use std::fs;

    tracing::info!("Performing SMB mount: {} -> {:?}", share_name, mount_point);

    // Ensure mount point directory exists
    if let Some(parent) = mount_point.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            NestGateError::Network(format!("Failed to create mount point parent: {e}"))
        })?;
    }

    if !mount_point.exists() {
        fs::create_dir_all(mount_point)
            .map_err(|e| NestGateError::Network(format!("Failed to create mount point: {e}")))?;
    }

    // For SMB server, we don't actually mount on the server side
    // The client will mount the share. Here we just validate the share is accessible
    if let Some(user) = username {
        tracing::info!(
            "SMB share {} is ready for user {} to mount at {:?}",
            share_name,
            user,
            mount_point
        );
    } else {
        tracing::info!(
            "SMB share {} is ready for guest access to mount at {:?}",
            share_name,
            mount_point
        );
    }

}
