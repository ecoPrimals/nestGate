#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::{PortManager, PortManagerConfig, PortRange, Result, ServiceType};

    #[tokio::test]
    async fn test_port_manager_initialization() -> Result<()> {
        // Create a basic configuration
        let config = PortManagerConfig {
            server: crate::config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0, // Use port 0 to get a random available port
                cors_enabled: true,
                api_key: None,
            },
            port_ranges: std::collections::HashMap::new(),
            services: std::collections::HashMap::new(),
            persistence: crate::config::PersistenceConfig {
                enabled: false,
                path: PathBuf::from(".test-state.json"),
                auto_save_interval_secs: 60,
            },
            logging: crate::config::LoggingConfig {
                level: "debug".to_string(),
                file_logging_enabled: false,
                log_file: None,
            },
        };

        // Create a port manager instance
        let mut port_manager = PortManager::new(config);

        // Initialize the port manager
        port_manager.initialize().await?;

        // Verify that the port manager initialized correctly
        assert!(port_manager.is_initialized());

        Ok(())
    }

    #[tokio::test]
    async fn test_port_allocation() -> Result<()> {
        // Create a basic configuration with port ranges
        let mut port_ranges = std::collections::HashMap::new();
        port_ranges.insert(
            ServiceType::Other("Test".to_string()),
            PortRange { start: 8000, end: 8100 }
        );

        let config = PortManagerConfig {
            server: crate::config::ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0,
                cors_enabled: true,
                api_key: None,
            },
            port_ranges,
            services: std::collections::HashMap::new(),
            persistence: crate::config::PersistenceConfig {
                enabled: false,
                path: PathBuf::from(".test-state.json"),
                auto_save_interval_secs: 60,
            },
            logging: crate::config::LoggingConfig {
                level: "debug".to_string(),
                file_logging_enabled: false,
                log_file: None,
            },
        };

        // Create a port manager instance
        let mut port_manager = PortManager::new(config);

        // Initialize the port manager
        port_manager.initialize().await?;

        // Allocate a port
        let service_type = ServiceType::Other("Test".to_string());
        let port = port_manager.allocate_port("test-service", service_type.clone(), None).await?;

        // Verify that the port is within the range
        assert!(port >= 8000 && port <= 8100);

        // Allocate a second port and make sure it's different
        let port2 = port_manager.allocate_port("test-service-2", service_type, None).await?;
        assert!(port2 >= 8000 && port2 <= 8100);
        assert_ne!(port, port2);

        Ok(())
    }
} 