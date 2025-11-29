/// Network Discovery Module
/// Handles all network-related discovery operations including:
/// - Bind address detection and optimal interface selection
/// - Dynamic port discovery and availability scanning
/// - Network interface introspection
/// - Service endpoint resolution
use crate::{NestGateError, Result};
// **MIGRATED**: Using canonical config system instead of deprecated unified_types
use crate::config::canonical_primary::NestGateCanonicalConfig;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

// Import runtime config
use super::network_discovery_config::{NetworkRuntimeConfig, SharedNetworkRuntimeConfig};
/// Network discovery configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
///
/// **Migration Path**:
/// ```rust,ignore
/// // OLD (deprecated):
/// use crate::network::config::NetworkDiscoveryConfig;
///
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::NetworkDiscoveryConfig; // Now aliases to CanonicalNetworkConfig
/// ```
///
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(
    since = "0.11.0",
    note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
/// Configuration for NetworkDiscovery
pub struct NetworkDiscoveryConfig {
    /// Scan Timeout
    pub scan_timeout: Duration,
    /// Preferred Interfaces
    pub preferred_interfaces: Vec<String>,
    /// Port Scan Range
    pub port_scan_range: (u16, u16),
    /// Interface Priority
    pub interface_priority: Vec<String>,
}
#[allow(deprecated)]
impl Default for NetworkDiscoveryConfig {
    /// Returns the default instance
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
/// Interfaceinfo
pub struct InterfaceInfo {
    /// Name
    pub name: String,
    /// Ip Endpoint
    pub ip_endpoint: IpAddr,
    /// Whether up
    pub is_up: bool,
    /// Whether loopback
    pub is_loopback: bool,
    /// Priority Score
    pub priority_score: i32,
}
/// Network discovery subsystem
#[derive(Debug)]
#[allow(dead_code)] // Framework infrastructure
/// Networkdiscovery
pub struct NetworkDiscovery {
    #[allow(dead_code)] // Framework field - intentionally unused
    config: NestGateCanonicalConfig,
    #[allow(deprecated)]
    discovery_config: NetworkDiscoveryConfig,
    /// Runtime configuration (immutable, thread-safe)
    runtime_config: SharedNetworkRuntimeConfig,
}
impl Default for NetworkDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkDiscovery {
    /// Create new network discovery subsystem
    ///
    /// This constructor loads runtime configuration from environment variables.
    /// For testing or custom configurations, use `with_runtime_config()`.
    #[must_use]
    #[allow(deprecated)]
    pub fn new() -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
            discovery_config: NetworkDiscoveryConfig::default(),
            runtime_config: Arc::new(NetworkRuntimeConfig::from_env()),
        }
    }

    /// Create with custom configuration
    #[must_use]
    #[allow(deprecated)]
    pub fn with_config(config: NestGateCanonicalConfig) -> Self {
        Self {
            config,
            discovery_config: NetworkDiscoveryConfig::default(),
            runtime_config: Arc::new(NetworkRuntimeConfig::from_env()),
        }
    }

    /// Create with custom runtime configuration
    ///
    /// This is the recommended constructor for testing and when you need
    /// explicit control over runtime values (bind addresses, ports, endpoints).
    #[must_use]
    #[allow(deprecated)]
    pub fn with_runtime_config(runtime_config: SharedNetworkRuntimeConfig) -> Self {
        Self {
            config: NestGateCanonicalConfig::default(),
            discovery_config: NetworkDiscoveryConfig::default(),
            runtime_config,
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
    pub async fn discover_bind_address(&self, service_name: &str) -> Result<IpAddr> {
        // Try runtime config first (immutable, thread-safe)
        if let Some(addr) = self.runtime_config.get_bind_address(service_name) {
            return Ok(addr);
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
        // Check runtime configuration (immutable, thread-safe)
        if let Some(port) = self.runtime_config.get_bind_port(service_name) {
            if self.port_is_available(port).await? {
                return Ok(port);
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
    pub async fn discover_service_endpoint(&self, service_name: &str) -> Result<String> {
        // Runtime config override (immutable, thread-safe)
        if let Some(endpoint) = self.runtime_config.get_service_endpoint(service_name) {
            return Ok(endpoint.to_string());
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
    pub async fn discover_capability_endpoint(&self, capability: &str) -> Result<String> {
        // Runtime config-based discovery (immutable, thread-safe)
        if let Some(endpoint) = self.runtime_config.get_capability_endpoint(capability) {
            return Ok(endpoint.to_string());
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

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
///
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Networkdiscoveryconfigcanonical
pub type NetworkDiscoveryConfigCanonical =
    crate::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using NetworkDiscoveryConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.
