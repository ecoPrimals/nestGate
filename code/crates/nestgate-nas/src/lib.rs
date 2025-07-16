//! NestGate NAS
//!
//! Network Attached Storage functionality for NestGate

use nestgate_core::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tracing::info;

/// NAS server configuration
#[derive(Debug, Clone)]
pub struct NasConfig {
    pub smb_enabled: bool,
    pub nfs_enabled: bool,
    pub http_enabled: bool,
    pub bind_address: String,
    pub smb_port: u16,
    pub nfs_port: u16,
    pub http_port: u16,
    pub share_root: PathBuf,
}

impl Default for NasConfig {
    fn default() -> Self {
        Self {
            smb_enabled: true,
            nfs_enabled: true,
            http_enabled: true,
            bind_address: std::env::var("NESTGATE_NAS_BIND_ADDRESS")
                .unwrap_or_else(|_| "192.168.1.100".to_string()),
            smb_port: nestgate_core::constants::network::smb_port(),
            nfs_port: nestgate_core::constants::network::nfs_port(),
            http_port: nestgate_core::constants::network::api_port(),
            share_root: PathBuf::from("/nas/shares"),
        }
    }
}

/// NAS share definition
#[derive(Debug, Clone)]
pub struct NasShare {
    pub name: String,
    pub path: PathBuf,
    pub read_only: bool,
    pub allowed_users: Vec<String>,
    pub protocols: Vec<ShareProtocol>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ShareProtocol {
    SMB,
    NFS,
    HTTP,
}

/// Main NAS server
pub struct NasServer {
    config: NasConfig,
    shares: HashMap<String, NasShare>,
    smb_server: Option<SmbServer>,
    nfs_server: Option<NfsServer>,
    http_server: Option<HttpServer>,
}

impl NasServer {
    /// Create a new NAS server
    pub fn new(config: NasConfig) -> Self {
        Self {
            config,
            shares: HashMap::new(),
            smb_server: None,
            nfs_server: None,
            http_server: None,
        }
    }

    /// Initialize NAS server with protocol handlers
    pub async fn initialize(&mut self) -> Result<()> {
        info!("🚀 Initializing NAS server...");

        // Ensure share root directory exists
        if !self.config.share_root.exists() {
            tokio::fs::create_dir_all(&self.config.share_root)
                .await
                .map_err(|e| nestgate_core::NestGateError::Io(e.to_string()))?;
            info!(
                "📁 Created share root directory: {:?}",
                self.config.share_root
            );
        }

        // Initialize SMB server if enabled
        if self.config.smb_enabled {
            self.smb_server = Some(SmbServer::new(&self.config)?);
            info!("📁 SMB server initialized on port {}", self.config.smb_port);
        }

        // Initialize NFS server if enabled
        if self.config.nfs_enabled {
            self.nfs_server = Some(NfsServer::new(&self.config)?);
            info!("📁 NFS server initialized on port {}", self.config.nfs_port);
        }

        // Initialize HTTP server if enabled
        if self.config.http_enabled {
            self.http_server = Some(HttpServer::new(&self.config)?);
            info!(
                "🌐 HTTP server initialized on port {}",
                self.config.http_port
            );
        }

        info!("✅ NAS server initialization complete");
        Ok(())
    }

    /// Start all enabled services
    pub async fn start(&mut self) -> Result<()> {
        info!("🚀 Starting NAS services...");

        // Start SMB server
        if let Some(smb) = &mut self.smb_server {
            smb.start().await?;
            info!("✅ SMB server started");
        }

        // Start NFS server
        if let Some(nfs) = &mut self.nfs_server {
            nfs.start().await?;
            info!("✅ NFS server started");
        }

        // Start HTTP server
        if let Some(http) = &mut self.http_server {
            http.start().await?;
            info!("✅ HTTP server started");
        }

        info!("🎉 All NAS services started successfully");
        Ok(())
    }

    /// Add a new share
    pub async fn add_share(&mut self, share: NasShare) -> Result<()> {
        info!("📁 Adding share: {}", share.name);

        // Validate share path exists
        if !share.path.exists() {
            tokio::fs::create_dir_all(&share.path)
                .await
                .map_err(|e| nestgate_core::NestGateError::Io(e.to_string()))?;
            info!("📁 Created share directory: {:?}", share.path);
        }

        // Configure share in each enabled protocol
        for protocol in &share.protocols {
            match protocol {
                ShareProtocol::SMB => {
                    if let Some(smb) = &mut self.smb_server {
                        smb.add_share(&share).await?;
                    }
                }
                ShareProtocol::NFS => {
                    if let Some(nfs) = &mut self.nfs_server {
                        nfs.add_share(&share).await?;
                    }
                }
                ShareProtocol::HTTP => {
                    if let Some(http) = &mut self.http_server {
                        http.add_share(&share).await?;
                    }
                }
            }
        }

        self.shares.insert(share.name.clone(), share);
        info!("✅ Share added successfully");
        Ok(())
    }
}

// Protocol-specific server implementations

struct SmbServer {
    config: NasConfig,
    listener: Option<TcpListener>,
}

impl SmbServer {
    fn new(config: &NasConfig) -> nestgate_core::Result<Self> {
        Ok(Self {
            config: config.clone(),
            listener: None,
        })
    }

    async fn start(&mut self) -> nestgate_core::Result<()> {
        let bind_addr = format!("{}:{}", self.config.bind_address, self.config.smb_port);
        self.listener = Some(
            TcpListener::bind(&bind_addr)
                .await
                .map_err(|e| nestgate_core::NestGateError::Io(e.to_string()))?,
        );
        info!("📁 SMB server listening on {}", bind_addr);

        // Start accepting connections in background
        tokio::spawn(async move {
            // SMB protocol handling would go here
            // For now, just log connections
            info!("SMB server ready to accept connections");
        });

        Ok(())
    }

    async fn add_share(&mut self, share: &NasShare) -> nestgate_core::Result<()> {
        info!("📁 Configuring SMB share: {}", share.name);
        // SMB share configuration would go here
        Ok(())
    }
}

struct NfsServer {
    config: NasConfig,
}

impl NfsServer {
    fn new(config: &NasConfig) -> nestgate_core::Result<Self> {
        Ok(Self {
            config: config.clone(),
        })
    }

    async fn start(&mut self) -> nestgate_core::Result<()> {
        info!("📁 Starting NFS server on port {}", self.config.nfs_port);
        // NFS server startup would go here
        Ok(())
    }

    async fn add_share(&mut self, share: &NasShare) -> nestgate_core::Result<()> {
        info!("📁 Configuring NFS export: {}", share.name);
        // NFS export configuration would go here
        Ok(())
    }
}

struct HttpServer {
    config: NasConfig,
    listener: Option<TcpListener>,
}

impl HttpServer {
    fn new(config: &NasConfig) -> nestgate_core::Result<Self> {
        Ok(Self {
            config: config.clone(),
            listener: None,
        })
    }

    async fn start(&mut self) -> nestgate_core::Result<()> {
        let bind_addr = format!("{}:{}", self.config.bind_address, self.config.http_port);
        self.listener = Some(
            TcpListener::bind(&bind_addr)
                .await
                .map_err(|e| nestgate_core::NestGateError::Io(e.to_string()))?,
        );
        info!("🌐 HTTP server listening on {}", bind_addr);

        // Start HTTP service in background
        tokio::spawn(async move {
            info!("HTTP file server ready");
        });

        Ok(())
    }

    async fn add_share(&mut self, share: &NasShare) -> nestgate_core::Result<()> {
        info!("🌐 Configuring HTTP share: {}", share.name);
        // HTTP share configuration would go here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_nas_config_default() {
        let config = NasConfig::default();

        assert!(config.smb_enabled);
        assert!(config.nfs_enabled);
        assert!(config.http_enabled);
        assert_eq!(config.bind_address, "192.168.1.100");
        assert_eq!(config.smb_port, 445);
        assert_eq!(config.nfs_port, 2049);
        assert_eq!(config.http_port, 8080);
        assert_eq!(config.share_root, PathBuf::from("/nas/shares"));
    }

    #[test]
    fn test_nas_config_creation() {
        let config = NasConfig {
            smb_enabled: false,
            nfs_enabled: true,
            http_enabled: false,
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 9080,
            share_root: PathBuf::from("/custom/nas"),
        };

        assert!(!config.smb_enabled);
        assert!(config.nfs_enabled);
        assert!(!config.http_enabled);
        assert_eq!(config.bind_address, "127.0.0.1");
        assert_eq!(config.smb_port, 8445);
        assert_eq!(config.nfs_port, 8049);
        assert_eq!(config.http_port, 9080);
        assert_eq!(config.share_root, PathBuf::from("/custom/nas"));
    }

    #[test]
    fn test_share_protocol_variants() {
        let protocols = vec![ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP];

        assert_eq!(protocols.len(), 3);

        // Test that protocols can be cloned and compared
        for protocol in protocols {
            let cloned = protocol;
            assert_eq!(protocol, cloned);

            // Test debug formatting
            let debug_str = format!("{protocol:?}");
            assert!(!debug_str.is_empty());
        }
    }

    #[test]
    fn test_nas_share_creation() {
        let share = NasShare {
            name: "shared_docs".to_string(),
            path: PathBuf::from("/nas/documents"),
            read_only: false,
            allowed_users: vec!["user1".to_string(), "user2".to_string()],
            protocols: vec![ShareProtocol::SMB],
        };

        assert_eq!(share.name, "shared_docs");
        assert_eq!(share.path, PathBuf::from("/nas/documents"));
        assert!(!share.read_only);
        assert_eq!(share.protocols, vec![ShareProtocol::SMB]);
    }

    #[test]
    fn test_nas_share_read_only() {
        let share = NasShare {
            name: "read_only_share".to_string(),
            path: PathBuf::from("/nas/readonly"),
            read_only: true,
            allowed_users: vec!["admin".to_string()],
            protocols: vec![ShareProtocol::NFS],
        };

        assert_eq!(share.name, "read_only_share");
        assert_eq!(share.path, PathBuf::from("/nas/readonly"));
        assert!(share.read_only);
        assert_eq!(share.protocols, vec![ShareProtocol::NFS]);
    }

    #[test]
    fn test_nas_share_multiple_protocols() {
        let protocols = vec![ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP];

        let share = NasShare {
            name: "multi_protocol_share".to_string(),
            path: PathBuf::from("/nas/multi"),
            read_only: false,
            allowed_users: vec![
                "user1".to_string(),
                "user2".to_string(),
                "user3".to_string(),
            ],
            protocols: protocols.clone(),
        };

        assert_eq!(share.name, "multi_protocol_share");
        assert_eq!(share.path, PathBuf::from("/nas/multi"));
        assert!(!share.read_only);
        assert_eq!(share.protocols, protocols);
        assert_eq!(share.protocols.len(), 3);
    }

    #[test]
    fn test_nas_share_different_protocols() {
        let all_protocols = [ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP];

        for (i, protocol) in all_protocols.iter().enumerate() {
            let share = NasShare {
                name: format!("share_{i}"),
                path: PathBuf::from(format!("/nas/share_{i}")),
                read_only: i % 2 == 0, // Alternate read-only
                allowed_users: vec![format!("user_{}", i)],
                protocols: vec![*protocol],
            };

            assert_eq!(share.name, format!("share_{i}"));
            assert_eq!(share.path, PathBuf::from(format!("/nas/share_{}", i)));
            assert_eq!(share.read_only, i % 2 == 0);
            assert_eq!(share.protocols, vec![*protocol]);
        }
    }

    #[test]
    fn test_nas_server_creation() {
        let config = NasConfig::default();
        let _server = NasServer::new(config);

        // Server should be created successfully
        // We can't inspect internal state but creation shouldn't panic

        // Test that we can create multiple servers
        let config2 = NasConfig {
            smb_enabled: false,
            nfs_enabled: true,
            http_enabled: false,
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 9080,
            share_root: PathBuf::from("/custom/nas"),
        };
        let _server2 = NasServer::new(config2);
    }

    #[test]
    fn test_nas_server_with_custom_config() {
        let config = NasConfig {
            smb_enabled: true,
            nfs_enabled: false,
            http_enabled: true,
            bind_address: std::env::var("NESTGATE_NAS_BIND_ADDRESS")
                .unwrap_or_else(|_| "192.168.1.100".to_string()),
            smb_port: 445,
            nfs_port: 2049,
            http_port: 8080,
            share_root: PathBuf::from("/srv/nas"),
        };

        let _server = NasServer::new(config);

        // Server should be created successfully with custom config
        // We can't inspect internal state but creation shouldn't panic
    }

    #[test]
    fn test_nas_server_with_disabled_protocols() {
        let config = NasConfig {
            smb_enabled: false,
            nfs_enabled: false,
            http_enabled: false,
            bind_address: "127.0.0.1".to_string(),
            smb_port: 445,
            nfs_port: 2049,
            http_port: 8080,
            share_root: PathBuf::from("/nas"),
        };

        let _server = NasServer::new(config);

        // Server should be created even with all protocols disabled
        // This might be useful for testing or maintenance modes
    }

    #[test]
    fn test_nas_config_port_validation() {
        // Test with various port configurations
        let configs = vec![
            (445, 2049, 8080),     // Standard ports
            (8445, 8049, 9080),    // Custom ports
            (1024, 1025, 1026),    // Low custom ports
            (65535, 65534, 65533), // High ports
        ];

        for (smb_port, nfs_port, http_port) in configs {
            let config = NasConfig {
                smb_enabled: true,
                nfs_enabled: true,
                http_enabled: true,
                bind_address: "0.0.0.0".to_string(),
                smb_port,
                nfs_port,
                http_port,
                share_root: PathBuf::from("/nas"),
            };

            assert_eq!(config.smb_port, smb_port);
            assert_eq!(config.nfs_port, nfs_port);
            assert_eq!(config.http_port, http_port);

            // Should be able to create server with any valid port configuration
            let _server = NasServer::new(config);
        }
    }

    #[test]
    fn test_nas_config_bind_addresses() {
        let specific_ip = std::env::var("NESTGATE_NAS_SPECIFIC_IP")
            .unwrap_or_else(|_| "192.168.1.100".to_string());
        let addresses = vec![
            "0.0.0.0",    // All interfaces
            "127.0.0.1",  // Localhost
            &specific_ip, // Specific IP
            "::",         // IPv6 all interfaces
            "::1",        // IPv6 localhost
        ];

        for address in addresses {
            let config = NasConfig {
                smb_enabled: true,
                nfs_enabled: true,
                http_enabled: true,
                bind_address: address.to_string(),
                smb_port: 445,
                nfs_port: 2049,
                http_port: 8080,
                share_root: PathBuf::from("/nas"),
            };

            assert_eq!(config.bind_address, address);

            // Should be able to create server with any bind address
            let _server = NasServer::new(config);
        }
    }

    #[test]
    fn test_nas_config_share_root_paths() {
        let paths = vec![
            "/nas",
            "/srv/nas",
            "/home/shares",
            "/mnt/storage/nas",
            "/opt/nestgate/shares",
        ];

        for path in paths {
            let config = NasConfig {
                smb_enabled: true,
                nfs_enabled: true,
                http_enabled: true,
                bind_address: "0.0.0.0".to_string(),
                smb_port: 445,
                nfs_port: 2049,
                http_port: 8080,
                share_root: PathBuf::from(path),
            };

            assert_eq!(config.share_root, PathBuf::from(path));

            // Should be able to create server with any share root path
            let _server = NasServer::new(config);
        }
    }

    #[test]
    fn test_protocol_combinations() {
        // Test various combinations of enabled/disabled protocols
        let combinations = vec![
            (true, true, true),    // All enabled
            (true, true, false),   // SMB + NFS
            (true, false, true),   // SMB + HTTP
            (false, true, true),   // NFS + HTTP
            (true, false, false),  // SMB only
            (false, true, false),  // NFS only
            (false, false, true),  // HTTP only
            (false, false, false), // All disabled
        ];

        for (smb, nfs, http) in combinations {
            let config = NasConfig {
                smb_enabled: smb,
                nfs_enabled: nfs,
                http_enabled: http,
                bind_address: "0.0.0.0".to_string(),
                smb_port: 445,
                nfs_port: 2049,
                http_port: 8080,
                share_root: PathBuf::from("/nas"),
            };

            assert_eq!(config.smb_enabled, smb);
            assert_eq!(config.nfs_enabled, nfs);
            assert_eq!(config.http_enabled, http);

            // Should be able to create server with any protocol combination
            let _server = NasServer::new(config);
        }
    }

    #[test]
    fn test_share_protocol_ordering() {
        // Test that protocol order in Vec doesn't matter for functionality
        let protocols1 = vec![ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP];
        let protocols2 = vec![ShareProtocol::HTTP, ShareProtocol::SMB, ShareProtocol::NFS];

        let share1 = NasShare {
            name: "test_share_1".to_string(),
            path: PathBuf::from("/nas/test1"),
            read_only: false,
            allowed_users: vec!["user1".to_string()],
            protocols: protocols1,
        };

        let share2 = NasShare {
            name: "test_share_2".to_string(),
            path: PathBuf::from("/nas/test2"),
            read_only: false,
            allowed_users: vec!["user2".to_string()],
            protocols: protocols2,
        };

        // Both shares should be valid regardless of protocol order
        assert_eq!(share1.protocols.len(), 3);
        assert_eq!(share2.protocols.len(), 3);

        // Both should contain all three protocols
        assert!(share1.protocols.contains(&ShareProtocol::SMB));
        assert!(share1.protocols.contains(&ShareProtocol::NFS));
        assert!(share1.protocols.contains(&ShareProtocol::HTTP));

        assert!(share2.protocols.contains(&ShareProtocol::SMB));
        assert!(share2.protocols.contains(&ShareProtocol::NFS));
        assert!(share2.protocols.contains(&ShareProtocol::HTTP));
    }

    #[test]
    fn test_empty_protocols_list() {
        // Test that a share can be created with no protocols (might be useful for disabled shares)
        let share = NasShare {
            name: "disabled_share".to_string(),
            path: PathBuf::from("/nas/disabled"),
            read_only: true,
            allowed_users: vec![],
            protocols: vec![],
        };

        assert_eq!(share.name, "disabled_share");
        assert_eq!(share.path, PathBuf::from("/nas/disabled"));
        assert!(share.read_only);
        assert!(share.protocols.is_empty());
    }

    #[test]
    fn test_single_protocol_shares() {
        // Test shares with single protocols
        let smb_share = NasShare {
            name: "smb_only".to_string(),
            path: PathBuf::from("/nas/smb"),
            read_only: false,
            allowed_users: vec!["smb_user".to_string()],
            protocols: vec![ShareProtocol::SMB],
        };

        let nfs_share = NasShare {
            name: "nfs_only".to_string(),
            path: PathBuf::from("/nas/nfs"),
            read_only: false,
            allowed_users: vec!["nfs_user".to_string()],
            protocols: vec![ShareProtocol::NFS],
        };

        let http_share = NasShare {
            name: "http_only".to_string(),
            path: PathBuf::from("/nas/http"),
            read_only: true,
            allowed_users: vec!["http_user".to_string()],
            protocols: vec![ShareProtocol::HTTP],
        };

        assert_eq!(smb_share.protocols, vec![ShareProtocol::SMB]);
        assert_eq!(nfs_share.protocols, vec![ShareProtocol::NFS]);
        assert_eq!(http_share.protocols, vec![ShareProtocol::HTTP]);

        assert!(!smb_share.read_only);
        assert!(!nfs_share.read_only);
        assert!(http_share.read_only);
    }
}
