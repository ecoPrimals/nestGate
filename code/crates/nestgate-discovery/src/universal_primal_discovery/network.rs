// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

/// Network Discovery Module
/// Handles all network-related discovery operations including:
/// - Bind address detection and optimal interface selection
/// - Dynamic port discovery and availability scanning
/// - Network interface introspection
/// - Service endpoint resolution
use nestgate_types::error::{NestGateError, Result};
// **MIGRATED**: Using canonical config system instead of deprecated unified_types
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;

// Import runtime config
use super::network_discovery_config::{NetworkRuntimeConfig, SharedNetworkRuntimeConfig};
/// Network discovery configuration
#[derive(Debug, Clone)]
/// ⚠️ DEPRECATED: This config has been consolidated into `canonical_primary`
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
/// Configuration for `NetworkDiscovery`
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
    ///
    /// Loads port range from environment:
    /// - `NESTGATE_API_PORT`: Start of port scan range (default: 8080)
    /// - Admin port defaults to 9090 for end of range
    fn default() -> Self {
        use nestgate_config::config::environment::EnvironmentConfig;

        let env_config =
            EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());

        let start_port = env_config.network.port.get();
        let end_port = 9090; // Admin/management port range end

        Self {
            scan_timeout: Duration::from_secs(5),
            preferred_interfaces: vec!["eth0".to_string(), "wlan0".to_string()],
            port_scan_range: (start_port, end_port),
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
/// Networkdiscovery
pub struct NetworkDiscovery {
    #[allow(dead_code)]
    canonical_primary: NestGateCanonicalConfig,
    #[allow(deprecated)]
    legacy_discovery: NetworkDiscoveryConfig,
    /// Runtime configuration (immutable, thread-safe)
    network_runtime: SharedNetworkRuntimeConfig,
}
impl Default for NetworkDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

#[allow(deprecated)]
impl NetworkDiscovery {
    /// Create new network discovery subsystem
    ///
    /// This constructor loads runtime configuration from environment variables.
    /// For testing or custom configurations, use `with_runtime_config()`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            canonical_primary: NestGateCanonicalConfig::default(),
            legacy_discovery: NetworkDiscoveryConfig::default(),
            network_runtime: Arc::new(NetworkRuntimeConfig::from_env()),
        }
    }

    /// Create with custom configuration
    #[must_use]
    pub fn with_config(config: NestGateCanonicalConfig) -> Self {
        Self {
            canonical_primary: config,
            legacy_discovery: NetworkDiscoveryConfig::default(),
            network_runtime: Arc::new(NetworkRuntimeConfig::from_env()),
        }
    }

    /// Create with custom runtime configuration
    ///
    /// This is the recommended constructor for testing and when you need
    /// explicit control over runtime values (bind addresses, ports, endpoints).
    #[must_use]
    pub fn with_runtime_config(runtime_config: SharedNetworkRuntimeConfig) -> Self {
        Self {
            canonical_primary: NestGateCanonicalConfig::default(),
            legacy_discovery: NetworkDiscoveryConfig::default(),
            network_runtime: runtime_config,
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
    pub fn discover_bind_address(&self, service_name: &str) -> Result<IpAddr> {
        // Try runtime config first (immutable, thread-safe)
        if let Some(addr) = self.network_runtime.get_bind_address(service_name) {
            return Ok(addr);
        }

        // Network introspection - detect best interface
        self.detect_optimal_bind_interface()
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
        if let Some(port) = self.network_runtime.get_bind_port(service_name)
            && self.port_is_available(port).await?
        {
            return Ok(port);
        }

        // Dynamic port discovery - scan for available port
        for port in start_range..=65535 {
            if self.port_is_available(port).await? {
                return Ok(port);
            }
        }

        Err(NestGateError::System(Box::new(
            nestgate_types::error::variants::core_errors::SystemErrorDetails {
                message: "No available ports found".into(),
                component: "network_discovery".into(),
                operation: Some("find_available_port".into()),
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
    pub fn detect_optimal_bind_interface(&self) -> Result<IpAddr> {
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
                    nestgate_types::error::variants::core_errors::SystemErrorDetails {
                        message: "No suitable network interface found".into(),
                        component: "network_discovery".into(),
                        operation: Some("find_optimal_interface".into()),
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
        // ✅ MIGRATED: Use environment-configurable localhost address
        use nestgate_config::config::environment::EnvironmentConfig;
        let env_config =
            EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());
        let bind_addr = env_config.network.host;
        match TcpListener::bind(format!("{bind_addr}:{port}")).await {
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
            .legacy_discovery
            .preferred_interfaces
            .iter()
            .enumerate()
        {
            interfaces.push(InterfaceInfo {
                name: interface_name.clone(),
                ip_endpoint: IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 1, 100)),
                is_up: true,
                is_loopback: false,
                priority_score: 50 - i32::try_from(idx).unwrap_or(0),
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
        if let Some(endpoint) = self.network_runtime.get_service_endpoint(service_name) {
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
        let bind_address = self.detect_optimal_bind_interface()?;
        // Use the port range from discovery config
        let (start_port, _end_port) = self.legacy_discovery.port_scan_range; // PEDANTIC: Fixed unused variable
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
    pub fn discover_capability_endpoint(&self, capability: &str) -> Result<String> {
        // Runtime config-based discovery (immutable, thread-safe)
        if let Some(endpoint) = self.network_runtime.get_capability_endpoint(capability) {
            return Ok(endpoint.to_string());
        }

        // Default capability endpoint generation
        let bind_address = self.detect_optimal_bind_interface()?;
        let base_port: u16 = std::env::var("NESTGATE_CAPABILITY_BASE_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(9090); // Default admin/capability port
        let offset = u16::try_from(capability.len().rem_euclid(100)).unwrap_or(0);
        let capability_port = base_port.saturating_add(offset);

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
    pub fn get_network_config(&self) -> Result<HashMap<String, String>> {
        let mut config = HashMap::new();

        config.insert(
            "scan_timeout".to_string(),
            format!("{:?}", self.legacy_discovery.scan_timeout),
        );
        config.insert(
            "port_range".to_string(),
            format!("{:?}", self.legacy_discovery.port_scan_range),
        );
        config.insert(
            "preferred_interfaces".to_string(),
            self.legacy_discovery.preferred_interfaces.join(","),
        );

        Ok(config)
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// Backward-compatible alias to `CanonicalNetworkConfig` while migrating from deprecated structs.
#[allow(deprecated, missing_docs)]
mod deprecated_canonical_aliases {
    pub type NetworkDiscoveryConfigCanonical =
        nestgate_config::config::canonical_primary::domains::network::CanonicalNetworkConfig;
}
pub use deprecated_canonical_aliases::NetworkDiscoveryConfigCanonical;

#[cfg(test)]
mod tests {
    #![allow(deprecated)] // Tests construct deprecated `NetworkDiscoveryConfig` until migration completes

    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    fn create_test_runtime_config() -> SharedNetworkRuntimeConfig {
        Arc::new(NetworkRuntimeConfig::new())
    }

    #[test]
    fn test_network_discovery_new() {
        let discovery = NetworkDiscovery::new();
        assert!(format!("{:?}", discovery).contains("NetworkDiscovery"));
    }

    #[test]
    fn test_network_discovery_default() {
        let discovery = NetworkDiscovery::default();
        assert!(format!("{:?}", discovery).contains("NetworkDiscovery"));
    }

    #[test]
    fn test_network_discovery_with_config() {
        let config = NestGateCanonicalConfig::default();
        let discovery = NetworkDiscovery::with_config(config);
        assert!(format!("{:?}", discovery).contains("NetworkDiscovery"));
    }

    #[test]
    fn test_network_discovery_with_runtime_config() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);
        assert!(format!("{:?}", discovery).contains("NetworkDiscovery"));
    }

    #[test]
    fn test_network_discovery_config_default() {
        let config = NetworkDiscoveryConfig::default();
        assert_eq!(config.scan_timeout, Duration::from_secs(5));
        assert!(config.port_scan_range.0 > 0);
        assert!(config.port_scan_range.1 > config.port_scan_range.0);
    }

    #[test]
    fn test_network_discovery_config_clone() {
        let config1 = NetworkDiscoveryConfig::default();
        let config2 = config1.clone();
        assert_eq!(config1.scan_timeout, config2.scan_timeout);
    }

    #[test]
    fn test_interface_info_creation() {
        let interface = InterfaceInfo {
            name: "eth0".to_string(),
            ip_endpoint: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            is_up: true,
            is_loopback: false,
            priority_score: 100,
        };

        assert_eq!(interface.name, "eth0");
        assert!(interface.is_up);
        assert!(!interface.is_loopback);
        assert_eq!(interface.priority_score, 100);
    }

    #[test]
    fn test_interface_info_clone() {
        let interface1 = InterfaceInfo {
            name: "lo".to_string(),
            ip_endpoint: IpAddr::V4(Ipv4Addr::LOCALHOST),
            is_up: true,
            is_loopback: true,
            priority_score: 10,
        };

        let interface2 = interface1.clone();
        assert_eq!(interface1.name, interface2.name);
        assert_eq!(interface1.is_loopback, interface2.is_loopback);
    }

    #[test]
    fn test_interface_info_debug() {
        let interface = InterfaceInfo {
            name: "wlan0".to_string(),
            ip_endpoint: IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            is_up: true,
            is_loopback: false,
            priority_score: 50,
        };

        let debug_str = format!("{:?}", interface);
        assert!(debug_str.contains("InterfaceInfo"));
        assert!(debug_str.contains("wlan0"));
    }

    #[tokio::test]
    async fn test_discover_bind_address() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let result = discovery.discover_bind_address("test_service");
        // Should return an IP address (either from config or detected)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_discover_available_port() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        // Try to find an available port starting from a high number
        let result = discovery
            .discover_available_port("test_service", 50000)
            .await;
        // Should find at least one available port
        assert!(result.is_ok());
        if let Ok(port) = result {
            assert!(port >= 50000);
        }
    }

    #[tokio::test]
    async fn test_detect_optimal_bind_interface() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let result = discovery.detect_optimal_bind_interface();
        // Should return an IP (might be localhost if no interfaces)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_port_is_available() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        // Port 1 is typically privileged and should not be available
        let result1 = discovery.port_is_available(1).await;
        // Just check it returns a result
        assert!(result1.is_ok());

        // High port should likely be available
        let result2 = discovery.port_is_available(60000).await;
        assert!(result2.is_ok());
    }

    #[test]
    fn test_get_available_interfaces() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let result = discovery.get_available_interfaces();
        // Should return a result (might be empty or have interfaces)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_discover_service_endpoint() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let result = discovery.discover_service_endpoint("test_service").await;
        // Should resolve to some endpoint
        assert!(result.is_ok());
        if let Ok(endpoint) = result {
            assert!(endpoint.starts_with("http://"));
        }
    }

    #[tokio::test]
    async fn test_discover_service_endpoint_different_services() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let result1 = discovery.discover_service_endpoint("service1").await;
        let result2 = discovery.discover_service_endpoint("service2").await;

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_get_network_config() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let result = discovery.get_network_config();
        assert!(result.is_ok());

        let config = result.unwrap();
        assert!(config.contains_key("scan_timeout"));
        assert!(config.contains_key("port_range"));
        assert!(config.contains_key("preferred_interfaces"));
    }

    #[tokio::test]
    async fn test_concurrent_bind_address_discovery() {
        let runtime_config = Arc::new(NetworkRuntimeConfig::new());
        let discovery = Arc::new(NetworkDiscovery::with_runtime_config(runtime_config));

        let d1 = Arc::clone(&discovery);
        let d2 = Arc::clone(&discovery);

        let handle1 = tokio::task::spawn_blocking(move || d1.discover_bind_address("service1"));

        let handle2 = tokio::task::spawn_blocking(move || d2.discover_bind_address("service2"));

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[tokio::test]
    async fn test_concurrent_port_discovery() {
        let runtime_config = Arc::new(NetworkRuntimeConfig::new());
        let discovery = Arc::new(NetworkDiscovery::with_runtime_config(runtime_config));

        let d1 = Arc::clone(&discovery);
        let d2 = Arc::clone(&discovery);

        let handle1 =
            tokio::spawn(async move { d1.discover_available_port("service1", 55000).await });

        let handle2 =
            tokio::spawn(async move { d2.discover_available_port("service2", 56000).await });

        let result1 = handle1.await.unwrap();
        let result2 = handle2.await.unwrap();

        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_interface_info_ipv6() {
        let interface = InterfaceInfo {
            name: "eth0".to_string(),
            ip_endpoint: IpAddr::V6(Ipv6Addr::LOCALHOST),
            is_up: true,
            is_loopback: true,
            priority_score: 20,
        };

        assert!(interface.ip_endpoint.is_ipv6());
        assert!(interface.is_loopback);
    }

    #[tokio::test]
    async fn test_multiple_service_discoveries() {
        let runtime_config = create_test_runtime_config();
        let discovery = NetworkDiscovery::with_runtime_config(runtime_config);

        let services = vec!["service1", "service2", "service3"];

        for service in services {
            let result = discovery.discover_service_endpoint(service).await;
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_network_discovery_config_debug() {
        let config = NetworkDiscoveryConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("NetworkDiscoveryConfig"));
    }
}
