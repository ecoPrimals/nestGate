//! Network Integration Tests for NestGate
//!
//! Lightweight integration tests that avoid system operations

use chrono::Utc;
use nestgate_network::{
    ApiResponse, ConnectionRequest, ConnectionResponse, ConnectionType, NetworkApi, NfsExport,
    NfsServer, PerformancePreference, Protocol, ProtocolConfig, ProtocolManager, ServiceInstance,
    ServiceRegistration, ServiceStatus, SmbServer, SmbShare, SongbirdConfig,
    SongbirdConnectionManager, VlanConfig, VlanManager,
};
use std::collections::HashMap;
use std::net::IpAddr;
use std::path::PathBuf;

#[cfg(test)]
mod basic_network_tests {
    use super::*;

    #[test]
    fn test_protocol_variants() {
        let protocols = [
            Protocol::Nfs,
            Protocol::Smb,
            Protocol::Ftp,
            Protocol::Sftp,
            Protocol::Http,
        ];

        assert_eq!(protocols.len(), 5);
        assert_eq!(Protocol::Nfs.to_string(), "NFS");
        assert_eq!(Protocol::Smb.to_string(), "SMB");
        assert_eq!(Protocol::Http.to_string(), "HTTP");
    }

    #[test]
    fn test_performance_preference_variants() {
        let preferences = [
            PerformancePreference::Speed,
            PerformancePreference::Reliability,
            PerformancePreference::Compatibility,
            PerformancePreference::Balanced,
        ];

        assert_eq!(preferences.len(), 4);
        assert_eq!(
            PerformancePreference::default(),
            PerformancePreference::Balanced
        );
    }

    #[test]
    fn test_service_status_variants() {
        let statuses = vec![
            ServiceStatus::Starting,
            ServiceStatus::Running,
            ServiceStatus::Stopping,
            ServiceStatus::Stopped,
            ServiceStatus::Failed,
        ];

        assert_eq!(statuses.len(), 5);

        for status in statuses {
            let cloned = status.clone();
            assert_eq!(status, cloned);
        }
    }

    #[test]
    fn test_connection_type_variants() {
        let types = [
            ConnectionType::Api,
            ConnectionType::Nfs,
            ConnectionType::Smb,
            ConnectionType::Iscsi,
            ConnectionType::S3,
            ConnectionType::Internal("test".to_string()),
            ConnectionType::Health,
            ConnectionType::Metrics,
        ];

        assert_eq!(types.len(), 8);
        assert_eq!(ConnectionType::Api, ConnectionType::Api);
        assert_ne!(ConnectionType::Api, ConnectionType::Nfs);
    }
}

#[cfg(test)]
mod protocol_config_tests {
    use super::*;

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
    }

    #[test]
    fn test_protocol_config_default() {
        let config = ProtocolConfig::default();

        assert_eq!(config.protocol, Protocol::Nfs);
        assert_eq!(config.performance, PerformancePreference::Balanced);
        assert!(config.encryption);
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_protocol_manager_creation() {
        let _manager = ProtocolManager::new();
        let _default_manager = ProtocolManager::default();
        // Just test creation without hanging operations
    }
}

#[cfg(test)]
mod vlan_tests {
    use super::*;

    #[test]
    fn test_vlan_config_creation() {
        let vlan = VlanConfig {
            vlan_id: 100,
            name: "Production".to_string(),
            description: "Production network".to_string(),
            ip_range: Some("192.168.100.0/24".to_string()),
            gateway: Some(IpAddr::V4("192.168.100.1".parse().unwrap())),
            enabled: true,
        };

        assert_eq!(vlan.vlan_id, 100);
        assert_eq!(vlan.name, "Production");
        assert_eq!(vlan.description, "Production network");
        assert_eq!(vlan.ip_range, Some("192.168.100.0/24".to_string()));
        assert!(vlan.gateway.is_some());
        assert!(vlan.enabled);
    }

    #[tokio::test]
    async fn test_vlan_manager_creation() {
        let manager = VlanManager::new();

        // Should start with no VLANs
        let vlans = manager.list_vlans().await.unwrap();
        assert!(vlans.is_empty());
    }
}

#[cfg(test)]
mod service_instance_tests {
    use super::*;

    #[test]
    fn test_service_instance_creation() {
        let instance = ServiceInstance {
            id: "test-service-1".to_string(),
            name: "test-service".to_string(),
            host: "localhost".to_string(),
            port: 8080,
            status: ServiceStatus::Running,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(instance.id, "test-service-1");
        assert_eq!(instance.name, "test-service");
        assert_eq!(instance.host, "localhost");
        assert_eq!(instance.port, 8080);
        assert_eq!(instance.status, ServiceStatus::Running);
    }

    #[test]
    fn test_api_response_success() {
        let response = ApiResponse::success("Operation completed".to_string());

        assert!(response.success);
        assert_eq!(response.data, Some("Operation completed".to_string()));
        assert!(response.error.is_none());
    }

    #[test]
    fn test_api_response_error() {
        let response: ApiResponse<String> = ApiResponse::error("Something went wrong".to_string());

        assert!(!response.success);
        assert!(response.data.is_none());
        assert_eq!(response.error, Some("Something went wrong".to_string()));
    }
}

#[cfg(test)]
mod network_api_tests {
    use super::*;

    #[test]
    fn test_network_api_creation() {
        let _api = NetworkApi::new();
        // Just test creation
    }

    #[tokio::test]
    async fn test_network_api_service_registration() {
        let api = NetworkApi::new();

        let service = ServiceInstance {
            id: "api-test-1".to_string(),
            name: "api-test".to_string(),
            host: "127.0.0.1".to_string(),
            port: 3000,
            status: ServiceStatus::Running,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Register service should succeed
        assert!(api.register_service(service.clone()).await.is_ok());

        // Should be able to retrieve service status
        let status = api.get_service_status(&service.name).await.unwrap();
        assert_eq!(status, ServiceStatus::Running);

        // List services should contain our service
        let services = api.list_services().await.unwrap();
        assert_eq!(services.len(), 1);
        assert_eq!(services[0].name, "api-test");
    }
}

#[cfg(test)]
mod nfs_server_tests {
    use super::*;

    #[test]
    fn test_nfs_server_creation() {
        let _server = NfsServer::new();
        // Just test creation
    }

    #[test]
    fn test_nfs_export_creation() {
        let export = NfsExport {
            path: PathBuf::from("/export/home"),
            options: Default::default(),
            client_access: vec!["192.168.1.0/24".to_string()],
        };

        assert_eq!(export.path, PathBuf::from("/export/home"));
        assert_eq!(export.client_access.len(), 1);
        assert_eq!(export.client_access[0], "192.168.1.0/24");
    }

    #[tokio::test]
    async fn test_nfs_server_list_exports() {
        let server = NfsServer::new();

        // Should start with no exports
        let exports = server.list_exports().await.unwrap();
        assert!(exports.is_empty());
    }
}

#[cfg(test)]
mod smb_server_tests {
    use super::*;

    #[test]
    fn test_smb_server_creation() {
        let _server = SmbServer::new();
        // Just test creation
    }

    #[test]
    fn test_smb_share_creation() {
        let share = SmbShare {
            name: "public".to_string(),
            path: PathBuf::from("/srv/samba/public"),
            comment: "Public share".to_string(),
            read_only: false,
            guest_ok: true,
            browseable: true,
        };

        assert_eq!(share.name, "public");
        assert_eq!(share.path, PathBuf::from("/srv/samba/public"));
        assert_eq!(share.comment, "Public share");
        assert!(!share.read_only);
        assert!(share.guest_ok);
        assert!(share.browseable);
    }

    #[tokio::test]
    async fn test_smb_server_list_shares() {
        let server = SmbServer::new();

        // Should start with no shares
        let shares = server.list_shares().await.unwrap();
        assert!(shares.is_empty());
    }
}

#[cfg(test)]
mod songbird_config_tests {
    use super::*;

    #[test]
    fn test_songbird_config_default() {
        let config = SongbirdConfig::default();

        assert!(!config.orchestrator_url.is_empty());
        assert!(config.registration_interval > 0);
        assert!(config.health_check_interval > 0);
        assert!(config.discovery_interval > 0);
        assert!(config.auto_port_allocation);
        assert!(!config.service_metadata.is_empty());
    }

    #[test]
    fn test_service_registration_default() {
        let registration = ServiceRegistration::default();

        assert!(!registration.name.is_empty());
        assert!(!registration.service_type.is_empty());
        assert!(!registration.version.is_empty());
        assert!(!registration.address.is_empty());
        assert!(!registration.endpoints.is_empty());
        assert!(!registration.capabilities.is_empty());
        assert!(!registration.metadata.is_empty());
        assert!(!registration.health_endpoint.is_empty());
    }
}

#[cfg(test)]
mod connection_manager_tests {
    use super::*;

    #[test]
    fn test_connection_request_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("priority".to_string(), "high".to_string());

        let request = ConnectionRequest {
            source_service: "nestgate-api".to_string(),
            target_service: "nestgate-storage".to_string(),
            connection_type: ConnectionType::Api,
            required_capabilities: vec!["storage".to_string(), "zfs".to_string()],
            metadata,
        };

        assert_eq!(request.source_service, "nestgate-api".to_string());
        assert_eq!(request.target_service, "nestgate-storage".to_string());
        assert_eq!(request.connection_type, ConnectionType::Api);
        assert_eq!(request.required_capabilities.len(), 2);
        assert_eq!(request.metadata.len(), 1);
    }

    #[test]
    fn test_connection_response_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("protocol_version".to_string(), "1.0".to_string());

        let response = ConnectionResponse {
            connection_id: "conn-123".to_string(),
            endpoint: "storage-node:8080".to_string(),
            token: Some("auth-token-456".to_string()),
            expires_at: Some(Utc::now()),
            metadata,
        };

        assert_eq!(response.connection_id, "conn-123");
        assert_eq!(response.endpoint, "storage-node:8080".to_string());
        assert!(response.token.is_some());
        assert!(response.expires_at.is_some());
        assert_eq!(response.metadata.len(), 1);
    }

    #[test]
    fn test_songbird_connection_manager_creation() {
        let _manager = SongbirdConnectionManager::new(
            "http://songbird:8000".to_string(),
            "nestgate-test".to_string(),
        );
        // Just test creation
    }
}

#[cfg(test)]
mod integration_validation_tests {
    use super::*;

    #[tokio::test]
    async fn test_complete_network_integration() {
        // Create all major components
        let _api = NetworkApi::new();
        let _nfs_server = NfsServer::new();
        let _smb_server = SmbServer::new();
        let _protocol_manager = ProtocolManager::new();
        let _vlan_manager = VlanManager::new();

        // All components should be created successfully
        // No hanging operations
    }

    #[tokio::test]
    async fn test_protocol_integration() {
        let _manager = ProtocolManager::new();

        // Create protocol configurations
        let nfs_config = ProtocolConfig {
            protocol: Protocol::Nfs,
            options: HashMap::new(),
            performance: PerformancePreference::Speed,
            encryption: true,
            timeout: 30,
            max_retries: 3,
        };

        let smb_config = ProtocolConfig {
            protocol: Protocol::Smb,
            options: HashMap::new(),
            performance: PerformancePreference::Reliability,
            encryption: true,
            timeout: 45,
            max_retries: 5,
        };

        // Configurations should be valid
        assert_eq!(nfs_config.protocol, Protocol::Nfs);
        assert_eq!(smb_config.protocol, Protocol::Smb);
        assert_ne!(nfs_config.performance, smb_config.performance);
    }

    #[tokio::test]
    async fn test_service_lifecycle() {
        let api = NetworkApi::new();

        // Create service instance
        let service = ServiceInstance {
            id: "lifecycle-test".to_string(),
            name: "lifecycle-service".to_string(),
            host: "test-host".to_string(),
            port: 9000,
            status: ServiceStatus::Starting,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // Register service
        assert!(api.register_service(service.clone()).await.is_ok());

        // Check service status
        let status = api.get_service_status("lifecycle-service").await.unwrap();
        assert_eq!(status, ServiceStatus::Starting);

        // List services
        let services = api.list_services().await.unwrap();
        assert!(!services.is_empty());

        let found_service = services.iter().find(|s| s.name == "lifecycle-service");
        assert!(found_service.is_some());
        assert_eq!(found_service.unwrap().port, 9000);
    }

    #[tokio::test]
    async fn test_lightweight_storage_protocol_integration() {
        let nfs_server = NfsServer::new();
        let smb_server = SmbServer::new();

        // Just test that servers can be created and list operations work
        let nfs_exports = nfs_server.list_exports().await.unwrap();
        let smb_shares = smb_server.list_shares().await.unwrap();

        assert!(nfs_exports.is_empty());
        assert!(smb_shares.is_empty());
    }
}
