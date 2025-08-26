//
// Tests the actual NAS functionality with real types

use nestgate_nas::{NasConfig, NasServer, NasShare, ShareProtocol};
use std::path::PathBuf;
use tempfile::TempDir;
// use tokio_test;

#[cfg(test)]
mod nas_config_tests {
    use super::*;

    #[test]
    fn test_nas_config_default() {
        let config = NasConfig::default();

        assert!(config.smb_enabled);
        assert!(config.nfs_enabled);
        assert!(config.http_enabled);
        assert_eq!(config.bind_address, "0.0.0.0");
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
            http_enabled: true,
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 8080,
            share_root: PathBuf::from("/custom/shares"),
        };

        assert!(!config.smb_enabled);
        assert!(config.nfs_enabled);
        assert!(config.http_enabled);
        assert_eq!(config.bind_address, "127.0.0.1");
        assert_eq!(config.smb_port, 8445);
        assert_eq!(config.nfs_port, 8049);
        assert_eq!(config.http_port, 8080);
        assert_eq!(config.share_root, PathBuf::from("/custom/shares"));
    }

    #[test]
    fn test_nas_config_port_ranges() {
        let config = NasConfig {
            smb_enabled: true,
            nfs_enabled: true,
            http_enabled: true,
            bind_address: "0.0.0.0".to_string(),
            smb_port: 1024,
            nfs_port: 1025,
            http_port: 1026,
            share_root: PathBuf::from("/test/shares"),
        };

        // Validate port ranges
        assert!(config.smb_port >= 1024);
        assert!(config.nfs_port >= 1024);
        assert!(config.http_port >= 1024);
        assert_ne!(config.smb_port, config.nfs_port);
        assert_ne!(config.nfs_port, config.http_port);
    }
}

#[cfg(test)]
mod share_protocol_tests {
    use super::*;

    #[test]
    fn test_share_protocol_variants() {
        let protocols = [ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP];

        assert_eq!(protocols.len(), 3);

        // Test equality
        assert_eq!(ShareProtocol::SMB, ShareProtocol::SMB);
        assert_eq!(ShareProtocol::NFS, ShareProtocol::NFS);
        assert_eq!(ShareProtocol::HTTP, ShareProtocol::HTTP);

        // Test inequality
        assert_ne!(ShareProtocol::SMB, ShareProtocol::NFS);
        assert_ne!(ShareProtocol::NFS, ShareProtocol::HTTP);
        assert_ne!(ShareProtocol::HTTP, ShareProtocol::SMB);
    }

    #[test]
    fn test_share_protocol_copy_trait() {
        let original = ShareProtocol::SMB;
        let copied = original;

        // Both should be equal after copy
        assert_eq!(original, copied);
        assert_eq!(original, ShareProtocol::SMB);
        assert_eq!(copied, ShareProtocol::SMB);
    }
}

#[cfg(test)]
mod nas_share_tests {
    use super::*;

    #[test]
    fn test_nas_share_creation() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            format!("TempDir creation failed: {:?}", e)
        })?;

        let share = NasShare {
            name: "test_share".to_string(),
            path: temp_dir.path().to_path_buf(),
            read_only: false,
            allowed_users: vec!["user1".to_string(), "user2".to_string()],
            protocols: vec![ShareProtocol::SMB, ShareProtocol::NFS],
        };

        assert_eq!(share.name, "test_share");
        assert_eq!(share.path, temp_dir.path().to_path_buf());
        assert!(!share.read_only);
        assert_eq!(share.allowed_users.len(), 2);
        assert_eq!(share.protocols.len(), 2);
        assert!(share.protocols.contains(&ShareProtocol::SMB));
        assert!(share.protocols.contains(&ShareProtocol::NFS));
        Ok(())
    }

    #[test]
    fn test_nas_share_read_only() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            format!("TempDir creation failed: {:?}", e)
        })?;

        let share = NasShare {
            name: "readonly_share".to_string(),
            path: temp_dir.path().to_path_buf(),
            read_only: true,
            allowed_users: vec!["readonly_user".to_string()],
            protocols: vec![ShareProtocol::HTTP],
        };

        assert_eq!(share.name, "readonly_share");
        assert!(share.read_only);
        assert_eq!(share.allowed_users.len(), 1);
        assert_eq!(share.allowed_users[0], "readonly_user");
        assert_eq!(share.protocols.len(), 1);
        assert_eq!(share.protocols[0], ShareProtocol::HTTP);
        Ok(())
    }

    #[test]
    fn test_nas_share_multiple_protocols() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
        })?;

        let share = NasShare {
            name: "multi_protocol_share".to_string(),
            path: temp_dir.path().to_path_buf(),
            read_only: false,
            allowed_users: vec!["admin".to_string()],
            protocols: vec![ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP],
        };

        assert_eq!(share.protocols.len(), 3);
        assert!(share.protocols.contains(&ShareProtocol::SMB));
        assert!(share.protocols.contains(&ShareProtocol::NFS));
        assert!(share.protocols.contains(&ShareProtocol::HTTP));

        Ok(())
    }

    #[test]
    fn test_nas_share_no_users() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
        })?;

        let share = NasShare {
            name: "public_share".to_string(),
            path: temp_dir.path().to_path_buf(),
            read_only: true,
            allowed_users: vec![],
            protocols: vec![ShareProtocol::HTTP],
        };

        assert_eq!(share.name, "public_share");
        assert!(share.read_only);
        assert!(share.allowed_users.is_empty());
        assert_eq!(share.protocols.len(), 1);

        Ok(())
    }
}

#[cfg(test)]
mod nas_server_tests {
    use super::*;

    #[test]
    fn test_nas_server_creation() {
        let config = NasConfig::default();
        let _server = NasServer::new(config);

        // Server should be created successfully
        // We can't inspect internal state but creation shouldn't panic
    }

    #[test]
    fn test_nas_server_with_custom_config() {
        let config = NasConfig {
            smb_enabled: false,
            nfs_enabled: true,
            http_enabled: false,
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 8080,
            share_root: PathBuf::from("/tmp/test_shares"),
        };

        let _server = NasServer::new(config);

        // Server should be created with custom configuration
        // Creation should not panic or fail
    }

    #[test]
    fn test_nas_server_disabled_protocols() {
        let config = NasConfig {
            nfs_enabled: false,
            smb_enabled: false,
            http_enabled: false,
            bind_address: "127.0.0.1".to_string(),
            nfs_port: 2049,
            smb_port: 445,
            http_port: 8080,
            share_root: PathBuf::from("/tmp/disabled_shares"),
        };

        let _server = NasServer::new(config);

        // Server should be created even with all protocols disabled
        // This might be useful for testing or specific configurations
    }

    #[tokio::test]
    async fn test_nas_server_initialization() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
        })?;

        let config = NasConfig {
            smb_enabled: false,  // Disable to avoid port conflicts
            nfs_enabled: false,  // Disable to avoid port conflicts
            http_enabled: false, // Disable to avoid port conflicts
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 8081,
            share_root: temp_dir.path().to_path_buf(),
        };

        let mut server = NasServer::new(config);

        // Initialize should succeed
        let result = server.initialize().await;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_nas_server_add_share() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
        })?;
        let share_dir = temp_dir.path().join("test_share");

        let config = NasConfig {
            smb_enabled: false,  // Disable to avoid system operations
            nfs_enabled: false,  // Disable to avoid system operations
            http_enabled: false, // Disable to avoid system operations
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 8081,
            share_root: temp_dir.path().to_path_buf(),
        };

        let mut server = NasServer::new(config);
        let _ = server.initialize().await;

        let share = NasShare {
            name: "test_share".to_string(),
            path: share_dir,
            read_only: false,
            allowed_users: vec!["test_user".to_string()],
            protocols: vec![], // Empty protocols to avoid system operations
        };

        // Add share should succeed
        let result = server.add_share(share).await;
        assert!(result.is_ok());

        Ok(())
    }
}

#[cfg(test)]
mod integration_validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_nas_workflow() -> Result<(), Box<dyn std::error::Error>> {
        let temp_dir = TempDir::new().map_err(|e| {
            tracing::error!("TempDir creation failed: {:?}", e);
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
        })?;

        // Create NAS configuration
        let config = NasConfig {
            smb_enabled: false,  // Disable to avoid system dependencies
            nfs_enabled: false,  // Disable to avoid system dependencies
            http_enabled: false, // Disable to avoid system dependencies
            bind_address: "127.0.0.1".to_string(),
            smb_port: 8445,
            nfs_port: 8049,
            http_port: 8081,
            share_root: temp_dir.path().to_path_buf(),
        };

        // Create and initialize server
        let mut server = NasServer::new(config);
        assert!(server.initialize().await.is_ok());

        // Create test shares
        let share1 = NasShare {
            name: "documents".to_string(),
            path: temp_dir.path().join("documents"),
            read_only: false,
            allowed_users: vec!["user1".to_string(), "user2".to_string()],
            protocols: vec![], // Empty to avoid system operations
        };

        let share2 = NasShare {
            name: "media".to_string(),
            path: temp_dir.path().join("media"),
            read_only: true,
            allowed_users: vec!["guest".to_string()],
            protocols: vec![], // Empty to avoid system operations
        };

        // Add shares
        assert!(server.add_share(share1).await.is_ok());
        assert!(server.add_share(share2).await.is_ok());
    }

    #[test]
    fn test_protocol_combinations() {
        let temp_dir = TempDir::new().unwrap_or_else(|e| {
            tracing::error!("Unwrap failed: {:?}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Operation failed: {:?}", e),
            )
            .into());
        });

        // Test all possible protocol combinations
        let combinations = [
            vec![ShareProtocol::SMB],
            vec![ShareProtocol::NFS],
            vec![ShareProtocol::HTTP],
            vec![ShareProtocol::SMB, ShareProtocol::NFS],
            vec![ShareProtocol::SMB, ShareProtocol::HTTP],
            vec![ShareProtocol::NFS, ShareProtocol::HTTP],
            vec![ShareProtocol::SMB, ShareProtocol::NFS, ShareProtocol::HTTP],
        ];

        for (i, protocols) in combinations.iter().enumerate() {
            let share = NasShare {
                name: format!("test_share_{i}"),
                path: temp_dir.path().join(format!("share_{i}")),
                read_only: false,
                allowed_users: vec!["test_user".to_string()],
                protocols: protocols.clone(),
            };

            assert_eq!(share.protocols.len(), protocols.len());
            for protocol in protocols {
                assert!(share.protocols.contains(protocol));
            }
        }
    }

    #[test]
    fn test_configuration_validation() {
        // Test various configuration scenarios
        let configs = vec![
            // All protocols enabled
            NasConfig {
                smb_enabled: true,
                nfs_enabled: true,
                http_enabled: true,
                bind_address: "0.0.0.0".to_string(),
                smb_port: 445,
                nfs_port: 2049,
                http_port: 8080,
                share_root: PathBuf::from("/nas/shares"),
            },
            // Only SMB
            NasConfig {
                smb_enabled: true,
                nfs_enabled: false,
                http_enabled: false,
                bind_address: "127.0.0.1".to_string(),
                smb_port: 8445,
                nfs_port: 2049,
                http_port: 8080,
                share_root: PathBuf::from("/tmp/smb_only"),
            },
            // Only HTTP
            NasConfig {
                smb_enabled: false,
                nfs_enabled: false,
                http_enabled: true,
                bind_address: "127.0.0.1".to_string(),
                smb_port: 445,
                nfs_port: 2049,
                http_port: 8081,
                share_root: PathBuf::from("/tmp/http_only"),
            },
        ];

        for config in configs {
            // Each configuration should be valid
            let _server = NasServer::new(config);
            // Server creation should succeed
        }

        Ok(())
    }
}
