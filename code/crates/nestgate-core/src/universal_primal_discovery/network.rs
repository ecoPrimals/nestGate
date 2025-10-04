/// Network Discovery Module
/// Handles all network-related discovery operations including:
/// - Bind address detection and optimal interface selection
/// - Dynamic port discovery and availability scanning
/// - Network interface introspection
/// - Service endpoint resolution
use crate::{NestGateError, Result};
// **MIGRATED**: Using canonical config system instead of deprecated unified_types
use crate::config::canonical_master::NestGateCanonicalConfig;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;
/// Network discovery configuration
#[derive(Debug, Clone)]
pub struct NetworkDiscoveryConfig {
    pub scan_timeout: Duration,
    pub preferred_interfaces: Vec<String>,
    pub port_scan_range: (u16, u16),
    pub interface_priority: Vec<String>,
}
impl Default for NetworkDiscoveryConfig {
    fn default() -> Self {
        Self {
            scan_timeout: Duration::from_secs(5),
            preferred_interfaces: vec!["eth0".to_string(), "wlan0".to_string()],
            port_scan_range: (8000, 9000),
            interface_priority: vec!["lo".to_string(), "eth0".to_string(), "wlan0".to_string()],
        }
    }
}

/// Network interface information
#[derive(Debug, Clone)]
pub struct InterfaceInfo {
    pub name: String,
    pub ip_endpoint: IpAddr,
    pub is_up: bool,
    pub is_loopback: bool,
    pub priority_score: i32,
}
/// Network discovery subsystem
#[derive(Debug)]
#[allow(dead_code)] // Framework infrastructure
pub struct NetworkDiscovery {
    #[allow(dead_code)] // Framework field - intentionally unused
    config: NestGateCanonicalConfig,
    discovery_config: NetworkDiscoveryConfig,
}
impl Default for NetworkDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkDiscovery {
    /// Create new network discovery subsystem
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
            discovery_config: NetworkDiscoveryConfig::default(),
        }
    }

    /// Create with custom configuration
    #[must_use]
    pub fn with_config(config: NestGateCanonicalConfig) -> Self {
        Self {
            config,
            discovery_config: NetworkDiscoveryConfig::default(),
        }
    }

    /// **PRIMAL DISCOVERY**: Find available bind address through network discovery
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub async fn discover_bind_address(&self, service_name: &str) -> Result<IpAddr> {
        // Try environment first (external configuration)
        if let Ok(addr) = std::env::var(format!(
            "NESTGATE_{}_BIND_ADDRESS",
            service_name.to_uppercase()
        )) {
            if let Ok(ip) = addr.parse::<IpAddr>() {
                return Ok(ip);
            }
        }

        // Network introspection - detect best interface
        self.detect_optimal_bind_interface().await
    }

    /// **PRIMAL DISCOVERY**: Find available port through port scanning
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn discover_available_port(
        &self,
        service_name: &str,
        start_range: u16,
    ) -> Result<u16> {
        // Check environment configuration
        if let Ok(port_str) =
            std::env::var(format!("NESTGATE_{}_PORT", service_name.to_uppercase()))
        {
            if let Ok(port) = port_str.parse::<u16>() {
                if self.port_is_available(port).await? {
                    return Ok(port);
                }
            }
        }

        // Dynamic port discovery - scan for available port
        for port in start_range..=65535 {
            if self.port_is_available(port).await? {
                return Ok(port);
            }
        }

        Err(NestGateError::System(Box::new(
            crate::error::variants::core_errors::SystemErrorDetails {
                message: "No available ports found".to_string(),
                component: "network_discovery".to_string(),
                operation: Some("find_available_port".to_string()),
                context: None,
            },
        )))
    }

    /// **NETWORK INTROSPECTION**: Detect optimal bind interface
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn detect_optimal_bind_interface(&self) -> Result<IpAddr> {
        // Get all available interfaces
        let interfaces = self.get_available_interfaces()?;

        if interfaces.is_empty() {
            return Ok(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
        }

        // Find highest priority interface that's up
        let optimal_interface = interfaces
            .iter()
            .filter(|iface| iface.is_up)
            .max_by_key(|iface| iface.priority_score)
            .ok_or_else(|| {
                NestGateError::System(Box::new(
                    crate::error::variants::core_errors::SystemErrorDetails {
                        message: "No suitable network interface found".to_string(),
                        component: "network_discovery".to_string(),
                        operation: Some("find_optimal_interface".to_string()),
                        context: None,
                    },
                ))
            })?;

        Ok(optimal_interface.ip_endpoint)
    }

    /// **PORT AVAILABILITY**: Check if port is available
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub async fn port_is_available(&self, port: u16) -> Result<bool> {
        use tokio::net::TcpListener;

        // Try to bind to the port
        match TcpListener::bind(format!("127.0.0.1:{port}")).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// **INTERFACE ENUMERATION**: Get all available network interfaces
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub fn get_available_interfaces(&self) -> Result<Vec<InterfaceInfo>> {
        // Simplified implementation - in a real system this would use proper network interface APIs
        let mut interfaces = Vec::new();

        // Localhost (highest priority for development)
        interfaces.push(InterfaceInfo {
            name: "lo".to_string(),
            ip_endpoint: IpAddr::V4(std::net::Ipv4Addr::LOCALHOST),
            is_up: true,
            is_loopback: true,
            priority_score: 100,
        });

        // Add common interface patterns with reasonable defaults
        for (idx, interface_name) in self
            .discovery_config
            .preferred_interfaces
            .iter()
            .enumerate()
        {
            interfaces.push(InterfaceInfo {
                name: interface_name.clone(),
                ip_endpoint: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)),
                is_up: true,
                is_loopback: false,
                priority_score: 50 - idx as i32,
            });
        }

        Ok(interfaces)
    }

    /// **SERVICE ENDPOINT DISCOVERY**: Discover service endpoint through network scanning
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub async fn discover_service_endpoint(&self, service_name: &str) -> Result<String> {
        // Environment override
        if let Ok(endpoint) =
            std::env::var(&["NESTGATE_", &service_name.to_uppercase(), "_ENDPOINT"].concat())
        {
            return Ok(endpoint);
        }

        // Network discovery fallback
        self.scan_network_for_service(service_name).await
    }

    /// **NETWORK SCANNING**: Scan network for service
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    pub async fn scan_network_for_service(&self, service_name: &str) -> Result<String> {
        // Simplified implementation - in a real system this would do actual network discovery
        let bind_address = self.detect_optimal_bind_interface().await?;
        // Use the port range from discovery config
        let (start_port, _end_port) = self.discovery_config.port_scan_range; // PEDANTIC: Fixed unused variable
                                                                             // start_port is already available from the tuple above
        let port = self
            .discover_available_port(service_name, start_port)
            .await?;

        Ok(format!("http://{bind_address}:{port}"))
    }

    /// **CAPABILITY ENDPOINT DISCOVERY**: Discover capability endpoint through adapter
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub async fn discover_capability_endpoint(&self, capability: &str) -> Result<String> {
        // Environment-based discovery
        if let Ok(endpoint) =
            std::env::var(&["NESTGATE_", &capability.to_uppercase(), "_ENDPOINT"].concat())
        {
            return Ok(endpoint);
        }

        // Default capability endpoint generation
        let bind_address = self.detect_optimal_bind_interface().await?;
        let base_port = 9000;
        let capability_port = base_port + (capability.len() % 100) as u16;

        Ok(format!("http://{bind_address}:{capability_port}"))
    }

    /// **NETWORK CONFIGURATION**: Get network configuration summary
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - The operation fails due to invalid input
    /// - System resources are unavailable
    /// - Network or I/O errors occur
    #[must_use]
    pub async fn get_network_config(&self) -> Result<HashMap<String, String>> {
        let mut config = HashMap::new();

        config.insert(
            "scan_timeout".to_string(),
            format!("{:?}", self.discovery_config.scan_timeout),
        );
        config.insert(
            "port_range".to_string(),
            format!("{:?}", self.discovery_config.port_scan_range),
        );
        config.insert(
            "preferred_interfaces".to_string(),
            self.discovery_config.preferred_interfaces.join(","),
        );

        Ok(config)
    }
}
