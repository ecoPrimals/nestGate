//! NestGate Network Layer
//!
//! This crate provides network protocol implementations and API services
//! for the NestGate system. Supports both standalone operation and optional
//! ecosystem integration.

pub mod api;
pub mod connection_manager;
pub mod nfs;
pub mod protocol;
pub mod smb;
pub mod songbird;
pub mod vlan;

// Re-export main types
pub use api::{ApiResponse, NetworkApi, ServiceInstance, ServiceStatus, SongbirdClient};
pub use connection_manager::{
    ActiveConnection, ConnectionRequest, ConnectionResponse, ConnectionType,
    SongbirdConnectionManager,
};
pub use nfs::{
    MountRequest as NfsMountRequest, MountResponse as NfsMountResponse, NfsExport, NfsServer,
};
pub use protocol::{PerformancePreference, Protocol, ProtocolConfig, ProtocolManager};
pub use smb::{SmbMountRequest, SmbMountResponse, SmbServer, SmbShare};
pub use songbird::{ServiceRegistration, SongbirdConfig, SongbirdIntegration};
pub use vlan::{VlanConfig, VlanManager};

/// Network layer result type
pub type Result<T> = nestgate_core::Result<T>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::net::IpAddr;

    #[test]
    fn test_protocol_enum_variants() {
        // Test all protocol variants
        let protocols = [
            Protocol::Nfs,
            Protocol::Smb,
            Protocol::Ftp,
            Protocol::Sftp,
            Protocol::Http,
        ];

        assert_eq!(protocols.len(), 5);

        // Test Display implementation
        assert_eq!(Protocol::Nfs.to_string(), "NFS");
        assert_eq!(Protocol::Smb.to_string(), "SMB");
        assert_eq!(Protocol::Ftp.to_string(), "FTP");
        assert_eq!(Protocol::Sftp.to_string(), "SFTP");
        assert_eq!(Protocol::Http.to_string(), "HTTP");

        // Test equality and hash (should work due to derives)
        assert_eq!(Protocol::Nfs, Protocol::Nfs);
        assert_ne!(Protocol::Nfs, Protocol::Smb);
    }

    #[test]
    fn test_performance_preference_variants() {
        // Test all performance preference variants
        let preferences = [
            PerformancePreference::Speed,
            PerformancePreference::Reliability,
            PerformancePreference::Compatibility,
            PerformancePreference::Balanced,
        ];

        assert_eq!(preferences.len(), 4);

        // Test default
        assert_eq!(
            PerformancePreference::default(),
            PerformancePreference::Balanced
        );

        // Test equality
        assert_eq!(PerformancePreference::Speed, PerformancePreference::Speed);
        assert_ne!(
            PerformancePreference::Speed,
            PerformancePreference::Reliability
        );
    }

    #[test]
    fn test_protocol_config_creation() {
        let config = ProtocolConfig {
            protocol: Protocol::Nfs,
            options: HashMap::new(),
            performance: PerformancePreference::Speed,
            encryption: true,
            timeout: 60,
            max_retries: 5,
        };

        assert_eq!(config.protocol, Protocol::Nfs);
        assert_eq!(config.performance, PerformancePreference::Speed);
        assert!(config.encryption);
        assert_eq!(config.timeout, 60);
        assert_eq!(config.max_retries, 5);
        assert!(config.options.is_empty());
    }

    #[test]
    fn test_protocol_config_default() {
        let config = ProtocolConfig::default();

        assert_eq!(config.protocol, Protocol::Nfs);
        assert_eq!(config.performance, PerformancePreference::Balanced);
        assert!(config.encryption);
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_retries, 3);
        assert!(config.options.is_empty());
    }

    #[test]
    fn test_protocol_config_with_options() {
        let mut options = HashMap::new();
        options.insert("version".to_string(), "4.1".to_string());
        options.insert("rsize".to_string(), "1048576".to_string());

        let config = ProtocolConfig {
            protocol: Protocol::Nfs,
            options: options.clone(),
            performance: PerformancePreference::Speed,
            encryption: false,
            timeout: 45,
            max_retries: 2,
        };

        assert_eq!(config.options.len(), 2);
        assert_eq!(config.options.get("version"), Some(&"4.1".to_string()));
        assert_eq!(config.options.get("rsize"), Some(&"1048576".to_string()));
        assert!(!config.encryption);
    }

    #[test]
    fn test_protocol_manager_creation() {
        let manager = ProtocolManager::new();

        // Should start with no handlers
        assert!(manager.supported_protocols().is_empty());

        // Test default creation
        let default_manager = ProtocolManager::default();
        assert!(default_manager.supported_protocols().is_empty());
    }

    #[test]
    fn test_vlan_config_creation() {
        let vlan = VlanConfig {
            vlan_id: 100,
            name: "Production".to_string(),
            description: "Production network".to_string(),
            ip_range: Some(
                std::env::var("NESTGATE_TEST_VLAN_IP_RANGE")
                    .unwrap_or_else(|_| "192.168.100.0/24".to_string()),
            ),
            gateway: Some(IpAddr::V4(
                std::env::var("NESTGATE_TEST_VLAN_GATEWAY")
                    .unwrap_or_else(|_| "192.168.100.1".to_string())
                    .parse()
                    .unwrap(),
            )),
            enabled: true,
        };

        assert_eq!(vlan.vlan_id, 100);
        assert_eq!(vlan.name, "Production");
        assert_eq!(vlan.description, "Production network");
        assert_eq!(
            vlan.ip_range,
            Some(
                std::env::var("NESTGATE_TEST_VLAN_IP_RANGE")
                    .unwrap_or_else(|_| "192.168.100.0/24".to_string())
            ),
        );
        assert!(vlan.gateway.is_some());
        assert!(vlan.enabled);
    }

    #[tokio::test]
    async fn test_vlan_manager_creation() {
        let manager = VlanManager::new();

        // Should start with no VLANs
        let vlans = manager.list_vlans().await.unwrap();
        assert!(vlans.is_empty());

        // Test default creation
        let default_manager = VlanManager::default();
        let default_vlans = default_manager.list_vlans().await.unwrap();
        assert!(default_vlans.is_empty());
    }

    #[tokio::test]
    async fn test_vlan_manager_add_vlan() {
        let manager = VlanManager::new();

        let vlan = VlanConfig {
            vlan_id: 200,
            name: "Test VLAN".to_string(),
            description: "Test description".to_string(),
            ip_range: Some(
                std::env::var("NESTGATE_DEV_VLAN_IP_RANGE")
                    .unwrap_or_else(|_| "10.0.200.0/24".to_string()),
            ),
            gateway: Some(IpAddr::V4(
                std::env::var("NESTGATE_DEV_VLAN_GATEWAY")
                    .unwrap_or_else(|_| "10.0.200.1".to_string())
                    .parse()
                    .unwrap(),
            )),
            enabled: true,
        };

        // Add VLAN should succeed
        assert!(manager.add_vlan(vlan.clone()).await.is_ok());

        // Should be able to retrieve it
        let retrieved = manager.get_vlan(200).await.unwrap();
        assert_eq!(retrieved.vlan_id, 200);
        assert_eq!(retrieved.name, "Test VLAN");

        // List should contain one VLAN
        let vlans = manager.list_vlans().await.unwrap();
        assert_eq!(vlans.len(), 1);
    }

    #[tokio::test]
    async fn test_vlan_manager_invalid_vlan_id() {
        let manager = VlanManager::new();

        // Test invalid VLAN ID (0)
        let invalid_vlan_0 = VlanConfig {
            vlan_id: 0,
            name: "Invalid".to_string(),
            description: "Invalid VLAN ID".to_string(),
            ip_range: None,
            gateway: None,
            enabled: false,
        };

        assert!(manager.add_vlan(invalid_vlan_0).await.is_err());

        // Test invalid VLAN ID (too high)
        let invalid_vlan_high = VlanConfig {
            vlan_id: 5000,
            name: "Invalid".to_string(),
            description: "Invalid VLAN ID".to_string(),
            ip_range: None,
            gateway: None,
            enabled: false,
        };

        assert!(manager.add_vlan(invalid_vlan_high).await.is_err());
    }

    #[tokio::test]
    async fn test_vlan_manager_duplicate_vlan() {
        let manager = VlanManager::new();

        let vlan = VlanConfig {
            vlan_id: 300,
            name: "Duplicate Test".to_string(),
            description: "Test duplicate".to_string(),
            ip_range: None,
            gateway: None,
            enabled: true,
        };

        // First add should succeed
        assert!(manager.add_vlan(vlan.clone()).await.is_ok());

        // Second add should fail
        assert!(manager.add_vlan(vlan).await.is_err());
    }

    #[tokio::test]
    async fn test_vlan_manager_remove_vlan() {
        let manager = VlanManager::new();

        let vlan = VlanConfig {
            vlan_id: 400,
            name: "Remove Test".to_string(),
            description: "Test removal".to_string(),
            ip_range: None,
            gateway: None,
            enabled: true,
        };

        // Add VLAN
        assert!(manager.add_vlan(vlan).await.is_ok());

        // Remove should succeed
        assert!(manager.remove_vlan(400).await.is_ok());

        // Should not be found anymore
        assert!(manager.get_vlan(400).await.is_err());

        // Remove non-existent should fail
        assert!(manager.remove_vlan(999).await.is_err());
    }

    #[tokio::test]
    async fn test_vlan_manager_enable_disable() {
        let manager = VlanManager::new();

        let vlan = VlanConfig {
            vlan_id: 500,
            name: "Enable/Disable Test".to_string(),
            description: "Test enable/disable".to_string(),
            ip_range: None,
            gateway: None,
            enabled: false,
        };

        // Add disabled VLAN
        assert!(manager.add_vlan(vlan).await.is_ok());

        // Enable should work
        assert!(manager.enable_vlan(500).await.is_ok());
        let enabled_vlan = manager.get_vlan(500).await.unwrap();
        assert!(enabled_vlan.enabled);

        // Disable should work
        assert!(manager.disable_vlan(500).await.is_ok());
        let disabled_vlan = manager.get_vlan(500).await.unwrap();
        assert!(!disabled_vlan.enabled);

        // Enable/disable non-existent should fail
        assert!(manager.enable_vlan(999).await.is_err());
        assert!(manager.disable_vlan(999).await.is_err());
    }

    #[tokio::test]
    async fn test_vlan_manager_get_enabled_vlans() {
        let manager = VlanManager::new();

        // Add enabled VLAN
        let enabled_vlan = VlanConfig {
            vlan_id: 600,
            name: "Enabled".to_string(),
            description: "Enabled VLAN".to_string(),
            ip_range: None,
            gateway: None,
            enabled: true,
        };

        // Add disabled VLAN
        let disabled_vlan = VlanConfig {
            vlan_id: 601,
            name: "Disabled".to_string(),
            description: "Disabled VLAN".to_string(),
            ip_range: None,
            gateway: None,
            enabled: false,
        };

        assert!(manager.add_vlan(enabled_vlan).await.is_ok());
        assert!(manager.add_vlan(disabled_vlan).await.is_ok());

        // Should only get enabled VLANs
        let enabled_vlans = manager.get_enabled_vlans().await.unwrap();
        assert_eq!(enabled_vlans.len(), 1);
        assert_eq!(enabled_vlans[0].vlan_id, 600);
        assert!(enabled_vlans[0].enabled);
    }

    #[test]
    fn test_connection_type_variants() {
        // Test all connection type variants
        let connection_types = [
            ConnectionType::Api,
            ConnectionType::Nfs,
            ConnectionType::Smb,
            ConnectionType::Iscsi,
            ConnectionType::S3,
            ConnectionType::Internal("test-service".to_string()),
            ConnectionType::Health,
            ConnectionType::Metrics,
        ];

        assert_eq!(connection_types.len(), 8);

        // Test equality
        assert_eq!(ConnectionType::Api, ConnectionType::Api);
        assert_ne!(ConnectionType::Api, ConnectionType::Nfs);

        // Test Internal variant
        if let ConnectionType::Internal(service) = &connection_types[5] {
            assert_eq!(service, "test-service");
        } else {
            assert!(false, "Expected Internal connection type");
        }
    }

    #[test]
    fn test_connection_request_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("priority".to_string(), "high".to_string());

        let request = ConnectionRequest {
            source_service: "nestgate-api".to_string(),
            target_service: "nestgate-storage".to_string(),
            connection_type: ConnectionType::Api,
            required_capabilities: vec!["read".to_string(), "write".to_string()],
            metadata: metadata.clone(),
        };

        assert_eq!(request.source_service, "nestgate-api");
        assert_eq!(request.target_service, "nestgate-storage");
        assert!(matches!(request.connection_type, ConnectionType::Api));
        assert_eq!(request.required_capabilities.len(), 2);
        assert_eq!(request.metadata.get("priority"), Some(&"high".to_string()));
    }

    #[test]
    fn test_connection_response_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("region".to_string(), "us-east-1".to_string());

        let response = ConnectionResponse {
            connection_id: "conn-12345".to_string(),
            endpoint: std::env::var("NESTGATE_TEST_ENDPOINT")
                .unwrap_or_else(|_| "192.168.1.100:8080".to_string()),
            token: Some("auth-token-xyz".to_string()),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            metadata: metadata.clone(),
        };

        assert_eq!(response.connection_id, "conn-12345");
        assert_eq!(
            response.endpoint,
            std::env::var("NESTGATE_TEST_ENDPOINT")
                .unwrap_or_else(|_| "192.168.1.100:8080".to_string())
        );
        assert!(response.token.is_some());
        assert!(response.expires_at.is_some());
        assert_eq!(
            response.metadata.get("region"),
            Some(&"us-east-1".to_string())
        );
    }

    #[test]
    fn test_songbird_connection_manager_creation() {
        let manager = SongbirdConnectionManager::new(
            std::env::var("NESTGATE_API_URL").unwrap_or_else(|_| {
                format!(
                    "http://localhost:{}",
                    nestgate_core::constants::network::api_port()
                )
            }),
            "test-service".to_string(),
        );

        // Manager should be created successfully
        // We can't inspect internal state but creation shouldn't panic
        assert_eq!(
            std::mem::size_of_val(&manager),
            std::mem::size_of::<SongbirdConnectionManager>()
        );
    }

    #[test]
    fn test_service_status_variants() {
        // Test all service status variants
        let statuses = [
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ];

        assert_eq!(statuses.len(), 5);

        // Test equality
        assert_eq!(ServiceStatus::Running, ServiceStatus::Running);
        assert_ne!(ServiceStatus::Running, ServiceStatus::Stopped);
    }

    #[test]
    fn test_api_response_success() {
        let data = vec!["item1".to_string(), "item2".to_string()];
        let response = ApiResponse::success(data.clone());

        assert!(response.success);
        assert!(response.error.is_none());
        assert_eq!(response.data, Some(data));
    }

    #[test]
    fn test_api_response_error() {
        let error_message = "Something went wrong".to_string();
        let response: ApiResponse<String> = ApiResponse::error(error_message.clone());

        assert!(!response.success);
        assert_eq!(response.error, Some(error_message));
        assert!(response.data.is_none());
    }

    #[test]
    fn test_network_api_creation() {
        let api = NetworkApi::new();

        // API should be created successfully
        // We can't inspect internal state but creation shouldn't panic
        assert_eq!(
            std::mem::size_of_val(&api),
            std::mem::size_of::<NetworkApi>()
        );
    }

    #[test]
    fn test_songbird_client_creation() {
        let client = SongbirdClient::new(
            std::env::var("NESTGATE_SONGBIRD_CLIENT_URL")
                .unwrap_or_else(|_| "http://localhost:9000".to_string()),
        );

        // Client should be created successfully
        // We can't inspect internal state but creation shouldn't panic
        assert_eq!(
            std::mem::size_of_val(&client),
            std::mem::size_of::<SongbirdClient>()
        );
    }

    #[test]
    fn test_nfs_server_creation() {
        let server = NfsServer::new();

        // Server should be created successfully
        // We can't inspect internal state but creation shouldn't panic
        assert_eq!(
            std::mem::size_of_val(&server),
            std::mem::size_of::<NfsServer>()
        );
    }

    #[test]
    fn test_smb_server_creation() {
        let server = SmbServer::new();

        // Server should be created successfully
        // We can't inspect internal state but creation shouldn't panic
        assert_eq!(
            std::mem::size_of_val(&server),
            std::mem::size_of::<SmbServer>()
        );
    }
}
