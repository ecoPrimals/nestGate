use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::time::timeout;

use crates::network::nestgate_mcp::{
    config::{NfsConfig, SmbConfig, IscsiConfig},
    protocol::{
        nfs::{NfsHandler},
        smb::{SmbHandler},
        iscsi::{IscsiHandler},
        ProtocolHandler,
    },
    error::Error,
    types::{MountOptions, PerformancePreference, CachePolicy, NfsVersion, SmbVersion},
};

// Constants for timeouts
pub const MOUNT_TIMEOUT: Duration = Duration::from_secs(5);
pub const UNMOUNT_TIMEOUT: Duration = Duration::from_secs(5);
pub const FILE_OPERATION_TIMEOUT: Duration = Duration::from_secs(2);

// Define a Result type alias
pub type Result<T> = std::result::Result<T, Error>;

// TimeoutHandler trait to add timeout functionality to protocol handlers
pub trait TimeoutHandler: ProtocolHandler {
    async fn mount_with_timeout(&self, options: &MountOptions) -> Result<PathBuf> {
        match timeout(MOUNT_TIMEOUT, self.mount(options)).await {
            Ok(result) => result,
            Err(_) => Err(Error::TimeoutError("Mount operation timed out".to_string())),
        }
    }

    async fn unmount_with_timeout(&self, path: &PathBuf) -> Result<()> {
        match timeout(UNMOUNT_TIMEOUT, self.unmount(path)).await {
            Ok(result) => result,
            Err(_) => Err(Error::TimeoutError("Unmount operation timed out".to_string())),
        }
    }
}

// Implement TimeoutHandler for all protocol handlers
impl TimeoutHandler for NfsHandler {}
impl TimeoutHandler for SmbHandler {}
impl TimeoutHandler for IscsiHandler {}

// Test fixture to manage test directories and files
pub struct TestFixture {
    mount_point: PathBuf,
    mount_path: Option<PathBuf>,
}

impl TestFixture {
    pub async fn new() -> Result<Self> {
        let mount_point = PathBuf::from("/tmp/nestgate_test");
        Ok(Self {
            mount_point,
            mount_path: None,
        })
    }

    pub async fn create_test_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.mount_point).await?;
        Ok(())
    }

    pub async fn cleanup(&self) -> Result<()> {
        if let Some(mount_path) = &self.mount_path {
            // In a real implementation, we would unmount if still mounted
            if mount_path.exists() {
                // Just for safety to make sure we're not removing something important
                if mount_path.starts_with("/tmp/") || mount_path.starts_with("/mnt/") {
                    let _ = fs::remove_dir_all(mount_path).await;
                }
            }
        }
        
        if self.mount_point.exists() {
            fs::remove_dir_all(&self.mount_point).await?;
        }
        
        Ok(())
    }

    pub async fn create_test_file(&self, content: &str) -> Result<()> {
        let filename = format!("test_file_{}.txt", "nfs");
        let file_path = self.mount_point.join(&filename);
        fs::write(&file_path, content.as_bytes()).await?;
        Ok(())
    }

    pub async fn verify_file_contents(&self, expected_content: &str) -> Result<bool> {
        let filename = format!("test_file_{}.txt", "nfs");
        let file_path = self.mount_point.join(&filename);
        
        match timeout(FILE_OPERATION_TIMEOUT, fs::read(&file_path)).await {
            Ok(Ok(content)) => Ok(content == expected_content.as_bytes()),
            Ok(Err(e)) => Err(Error::from(e)),
            Err(_) => Err(Error::TimeoutError("File read operation timed out".to_string())),
        }
    }

    pub fn mount_point(&self) -> Option<&PathBuf> {
        self.mount_path.as_ref()
    }

    pub async fn mount_nfs(&mut self, options: &MountOptions) -> Result<Box<dyn ProtocolHandler>> {
        let nfs_config = create_nfs_config();
        let config = to_config_nfs(&nfs_config);
        let handler = Box::new(NfsHandler::new(config));
        let mount_path = handler.mount(options).await?;
        self.mount_path = Some(mount_path);
        Ok(handler)
    }

    pub async fn mount_smb(&mut self, options: &MountOptions) -> Result<Box<dyn ProtocolHandler>> {
        let smb_config = create_smb_config();
        let config = to_config_smb(&smb_config);
        let handler = Box::new(SmbHandler::new(config));
        let mount_path = handler.mount(options).await?;
        self.mount_path = Some(mount_path);
        Ok(handler)
    }

    pub async fn mount_iscsi(&mut self, options: &MountOptions) -> Result<Box<dyn ProtocolHandler>> {
        let iscsi_config = create_iscsi_config();
        let config = to_config_iscsi(&iscsi_config);
        let handler = Box::new(IscsiHandler::new(config));
        let mount_path = handler.mount(options).await?;
        self.mount_path = Some(mount_path);
        Ok(handler)
    }
}

// Create test configurations for the protocols
pub fn create_nfs_config() -> NfsConfig {
    NfsConfig {
        server_address: String::from("127.0.0.1"),
        export_path: String::from("/exports/test"),
        version: String::from("4.1"),
        use_kerberos: false,
        enable_acls: false,
        enable_xattrs: false,
    }
}

// Convert protocol NfsConfig to config NfsConfig
pub fn to_config_nfs(protocol_config: &NfsConfig) -> NfsConfig {
    NfsConfig {
        server_address: protocol_config.server_address.clone(),
        export_path: protocol_config.export_path.clone(),
        version: protocol_config.version.clone(),
        use_kerberos: protocol_config.use_kerberos,
        enable_acls: protocol_config.enable_acls,
        enable_xattrs: protocol_config.enable_xattrs,
    }
}

pub fn create_smb_config() -> SmbConfig {
    SmbConfig {
        server_address: String::from("127.0.0.1"),
        share_name: String::from("test-share"),
        share_path: String::from("/shares/test"),
        version: String::from("3.0"),
        enable_signing: false,
        enable_encryption: false,
        enable_extended_security: false,
    }
}

// Convert protocol SmbConfig to config SmbConfig
pub fn to_config_smb(protocol_config: &SmbConfig) -> SmbConfig {
    SmbConfig {
        server_address: protocol_config.server_address.clone(),
        share_name: protocol_config.share_name.clone(),
        share_path: protocol_config.share_path.clone(),
        version: protocol_config.version.clone(),
        enable_signing: protocol_config.enable_signing,
        enable_encryption: protocol_config.enable_encryption,
        enable_extended_security: protocol_config.enable_extended_security,
    }
}

pub fn create_iscsi_config() -> IscsiConfig {
    IscsiConfig {
        target_address: String::from("127.0.0.1"),
        iqn: String::from("iqn.2023-01.com.example:storage"),
        port: 3260,
    }
}

// Convert protocol IscsiConfig to config IscsiConfig
pub fn to_config_iscsi(protocol_config: &IscsiConfig) -> IscsiConfig {
    IscsiConfig {
        target_address: protocol_config.target_address.clone(),
        iqn: protocol_config.iqn.clone(),
        port: protocol_config.port,
    }
}

pub fn create_test_mount_options(protocol: &str) -> MountOptions {
    MountOptions {
        fs_type: Some(format!("{}", protocol)),
        mount_flags: vec!["rw".to_string(), "sync".to_string()],
        read_only: false,
        performance: PerformancePreference::Balanced,
        cache_policy: CachePolicy::Default,
        protocol: Some(protocol.to_string()),
        source: Some("127.0.0.1".to_string()),
        username: Some("test-user".to_string()),
        password: Some("test-pass".to_string()),
    }
} 