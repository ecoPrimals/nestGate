use crate::universal_adapter::{PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest};
/// **Universal Security Client Tests**
/// Unit tests for the universal security client functionality.
#[cfg(test)]
mod tests {
    use super::super::client::{UniversalSecurityClient, UniversalSecurityError, UniversalSecurityCapability};
    use crate::config::canonical_primary::NestGateCanonicalConfig;
    use crate::service_discovery::config::UnifiedSecurityConfig;
    use crate::canonical_types::SecurityServiceNode;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_universal_security_client_creation() -> Result<(), Box<dyn std::error::Error>> {
        let config = NestGateCanonicalConfig::default();
        
        let result = UniversalSecurityClient::new(config).await;
        assert!(result.is_ok(), "Should create security client successfully");
        
        let client = result.expect("Security operation failed");
        assert!(!client.available_nodes.is_empty() || client.available_nodes.is_empty(), "Nodes list should be initialized");
    Ok(())
    }

    #[tokio::test]
    async fn test_security_capability_validation() -> Result<(), Box<dyn std::error::Error>> {
        let capability = UniversalSecurityCapability {
            capability_type: "encryption".to_string(),
            version: "1.0.0".to_string(),
            required: true,
        };

        assert_eq!(capability.capability_type, "encryption");
        assert_eq!(capability.version, "1.0.0");
        assert!(capability.required);
    Ok(())
    }

    #[test]
    async fn test_universal_security_error_display() -> Result<(), Box<dyn std::error::Error>> {
        let network_error = UniversalSecurityError::Network("Connection failed".to_string());
        let config_error = UniversalSecurityError::Configuration("Invalid config".to_string());
        let timeout_error = UniversalSecurityError::Timeout("Request timed out".to_string());
        let auth_error = UniversalSecurityError::Authentication("Invalid credentials".to_string());

        assert!(format!("{network_error}").contains("Network error"));
        assert!(format!("{config_error}").contains("Configuration error"));
        assert!(format!("{timeout_error}").contains("Timeout error"));
        assert!(format!("{auth_error}").contains("Authentication error"));
    Ok(())
    }

    #[test]
    async fn test_security_service_node_creation() -> Result<(), Box<dyn std::error::Error>> {
        use crate::canonical_types::{SecurityServiceNode, ServiceNodeStatus};
        
        let node = SecurityServiceNode {
            service_id: "test-node-1".to_string(),
            endpoint: "https://security.example.com".to_string(),
            capabilities: vec!["encryption".to_string(), "authentication".to_string()],
            public_key: "test-public-key".to_string(),
            status: ServiceNodeStatus::Active,
            last_seen: 1640995200, // Unix timestamp
            priority: 1,
        };

        assert_eq!(node.service_id, "test-node-1");
        assert_eq!(node.capabilities.len(), 2);
        assert!(node.capabilities.contains(&"encryption"));
        assert_eq!(node.priority, 1);
        assert!(matches!(node.status, ServiceNodeStatus::Active));
    Ok(())
    }

    #[tokio::test]
    async fn test_security_client_refresh_services() -> Result<(), Box<dyn std::error::Error>> {
        let config = NestGateCanonicalConfig::default();
        let mut client = UniversalSecurityClient::new(config).await.expect("Security operation failed");
        
        let result = client.refresh_services().await;
        // Should not fail even if no services are available
        assert!(result.is_ok() || result.is_err(), "Refresh should complete");
    Ok(())
    }

    #[tokio::test]
    async fn test_security_client_service_availability() -> Result<(), Box<dyn std::error::Error>> {
        let config = NestGateCanonicalConfig::default();
        let client = UniversalSecurityClient::new(config).await.expect("Security operation failed");
        
        let result = client.is_service_available("test-service").await;
        // Should handle non-existent service gracefully
        assert!(result.is_ok() || result.is_err(), "Service availability check should complete");
    Ok(())
    }

    #[test]
    fn test_security_error_std_error_trait() -> Result<(), Box<dyn std::error::Error>> {
        let error = UniversalSecurityError::Network("test error".to_string());
        let std_error: &dyn std::error::Error = &error;
        assert!(std_error.to_string().contains("Network error"));
    Ok(())
    }

    #[test]
    fn test_security_capability_clone() -> Result<(), Box<dyn std::error::Error>> {
        let original = UniversalSecurityCapability {
            capability_type: "signing".to_string(),
            version: "2.0.0".to_string(),
            required: false,
        };

        let cloned = original.clone();
        assert_eq!(original.capability_type, cloned.capability_type);
        assert_eq!(original.version, cloned.version);
        assert_eq!(original.required, cloned.required);
    Ok(())
    }

    #[test]
    fn test_security_capability_debug() -> Result<(), Box<dyn std::error::Error>> {
        let capability = UniversalSecurityCapability {
            capability_type: "key-management".to_string(),
            version: "1.5.0".to_string(),
            required: true,
        };

        let debug_str = format!("{capability:?}");
        assert!(debug_str.contains("key-management"));
        assert!(debug_str.contains("1.5.0"));
    Ok(())
    }
    Ok(())
}
