// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//! **NETWORK DISCOVERY METHODS**
//!
//! Network-based capability discovery methods for the Infant Discovery Architecture.

use super::capability_scanner::{CapabilityInfo, DiscoveryMethod};
use nestgate_types::error::NestGateError;
use std::collections::HashMap;
// Removed unused import: use std::future::Future;
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time::timeout;
use tracing::{debug, info, warn};

/// DNS-SRV record discovery method
#[derive(Debug)]
/// Dnsservicediscovery
pub struct DnsServiceDiscovery {
    /// Domain to search for SRV records
    domain: String,
    /// Service types to look for
    service_types: Vec<String>,
    /// DNS timeout
    timeout_duration: Duration,
}

impl DnsServiceDiscovery {
    /// Create a new DNS service discovery
    #[must_use]
    pub fn new(domain: String) -> Self {
        Self {
            domain,
            service_types: vec![
                "_nestgate-orchestration._tcp".to_string(),
                "_nestgate-security._tcp".to_string(),
                "_nestgate-ai._tcp".to_string(),
                "_nestgate-storage._tcp".to_string(),
                "_nestgate-compute._tcp".to_string(),
            ],
            timeout_duration: Duration::from_secs(5),
        }
    }

    /// Add a custom service type to discover
    pub fn add_service_type(&mut self, service_type: String) {
        self.service_types.push(service_type);
    }

    /// Set DNS timeout
    pub const fn set_timeout(&mut self, timeout: Duration) {
        self.timeout_duration = timeout;
    }
}

impl DiscoveryMethod for DnsServiceDiscovery {
    /// Discover
    async fn discover(&self) -> Result<Vec<CapabilityInfo>, NestGateError> {
        let mut capabilities = Vec::new();

        debug!("Starting DNS-SRV discovery for domain: {}", self.domain);

        for service_type in &self.service_types {
            let query = format!("{}.{}", service_type, self.domain);
            debug!("Querying DNS SRV record: {}", query);

            // In a real implementation, this would use a DNS library like trust-dns
            // For now, we'll simulate the discovery
            match self.query_srv_record(&query).await {
                Ok(records) => {
                    for record in records {
                        let capability_type = self.extract_capability_type(service_type);
                        let mut metadata = HashMap::new();
                        metadata.insert("source".to_string(), "dns-srv".to_string());
                        metadata.insert("service_type".to_string(), service_type.clone());
                        metadata.insert("priority".to_string(), record.priority.to_string());
                        metadata.insert("weight".to_string(), record.weight.to_string());

                        capabilities.push(CapabilityInfo {
                            capability_type,
                            endpoint: format!("http://{}:{}", record.target, record.port),
                            confidence: 0.85, // High confidence for DNS records
                            metadata,
                        });
                    }
                }
                Err(e) => {
                    warn!("Failed to query SRV record {}: {}", query, e);
                }
            }
        }

        if !capabilities.is_empty() {
            info!(
                "DNS-SRV discovery found {} capabilities",
                capabilities.len()
            );
        }

        Ok(capabilities)
    }

    /// Method Name
    fn method_name(&self) -> &'static str {
        "dns-srv"
    }
}

impl DnsServiceDiscovery {
    /// Extract capability type from service type
    fn extract_capability_type(&self, service_type: &str) -> String {
        // Extract from "_nestgate-orchestration._tcp" -> "orchestration"
        if let Some(start) = service_type.find("nestgate-") {
            let start_pos = start + 9; // Length of "nestgate-"
            if let Some(end) = service_type[start_pos..].find('.') {
                return service_type[start_pos..start_pos + end].to_string();
            }
        }
        "unknown".to_string()
    }

    /// Query SRV record (placeholder implementation)
    async fn query_srv_record(&self, _query: &str) -> Result<Vec<SrvRecord>, NestGateError> {
        // In a real implementation, this would use a DNS resolver
        // For demonstration, return empty results
        Ok(Vec::new())
    }
}

/// SRV record structure
#[derive(Debug, Clone)]
/// Srvrecord
pub struct SrvRecord {
    /// Priority
    pub priority: u16,
    /// Weight
    pub weight: u16,
    /// Port
    pub port: u16,
    /// Target
    pub target: String,
}

/// Multicast discovery method
#[derive(Debug)]
/// Multicastdiscovery
pub struct MulticastDiscovery {
    /// Multicast groups to listen on
    multicast_groups: Vec<SocketAddr>,
    /// Discovery timeout
    timeout_duration: Duration,
    /// Discovery message format
    #[allow(dead_code)]
    discovery_message: String,
}

impl MulticastDiscovery {
    /// Create a new multicast discovery
    #[must_use]
    pub fn new() -> Self {
        // SAFETY: These are well-known, standard multicast addresses that will never fail to parse
        // mDNS: 224.0.0.251:5353 (RFC 6762)
        // SSDP: 239.255.255.250:1900 (UPnP specification)
        #[allow(clippy::expect_used)]
        Self {
            multicast_groups: vec![
                "224.0.0.251:5353"
                    .parse()
                    .expect("BUG: mDNS multicast address is standard and must parse"), // mDNS
                "239.255.255.250:1900"
                    .parse()
                    .expect("BUG: SSDP multicast address is standard and must parse"), // SSDP
            ],
            timeout_duration: Duration::from_secs(3),
            discovery_message: "NESTGATE-DISCOVERY".to_string(),
        }
    }

    /// Add a multicast group to listen on
    pub fn add_multicast_group(&mut self, addr: SocketAddr) {
        self.multicast_groups.push(addr);
    }

    /// Set discovery timeout
    pub const fn set_timeout(&mut self, timeout: Duration) {
        self.timeout_duration = timeout;
    }
}

impl Default for MulticastDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

impl DiscoveryMethod for MulticastDiscovery {
    /// Discover
    async fn discover(&self) -> Result<Vec<CapabilityInfo>, NestGateError> {
        let mut capabilities = Vec::new();

        debug!(
            "Starting multicast discovery on {} groups",
            self.multicast_groups.len()
        );

        for group in &self.multicast_groups {
            debug!("Listening for announcements on {}", group);

            match self.listen_for_announcements(group).await {
                Ok(announcements) => {
                    for announcement in announcements {
                        if let Ok(capability) = self.parse_announcement(&announcement) {
                            capabilities.push(capability);
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to listen on multicast group {}: {}", group, e);
                }
            }
        }

        if !capabilities.is_empty() {
            info!(
                "Multicast discovery found {} capabilities",
                capabilities.len()
            );
        }

        Ok(capabilities)
    }

    /// Method Name
    fn method_name(&self) -> &'static str {
        "multicast"
    }
}

impl MulticastDiscovery {
    /// Listen for capability announcements
    async fn listen_for_announcements(
        &self,
        group: &SocketAddr,
    ) -> Result<Vec<String>, NestGateError> {
        let bind_addr = format!(
            "{}:0",
            nestgate_config::constants::network_defaults::get_bind_address()
        );
        let socket = UdpSocket::bind(&bind_addr).await.map_err(|e| {
            NestGateError::Internal(Box::new(
                nestgate_types::error::variants::core_errors::InternalErrorDetails {
                    message: format!("Failed to bind UDP socket: {e}"),
                    component: "multicast_discovery".to_string(),
                    location: Some(format!("{}:{}", file!(), line!())),
                    is_bug: false,
                    context: None,
                },
            ))
        })?;

        // Join multicast group (simplified for demonstration)
        debug!("Joined multicast group: {}", group);

        let mut announcements = Vec::new();
        let mut buf = [0u8; 1024];

        // Listen for announcements with timeout
        match timeout(self.timeout_duration, socket.recv_from(&mut buf)).await {
            Ok(Ok((len, addr))) => {
                let message = String::from_utf8_lossy(&buf[..len]);
                debug!("Received announcement from {}: {}", addr, message);
                announcements.push(message.to_string());
            }
            Ok(Err(e)) => {
                warn!("Error receiving multicast data: {}", e);
            }
            Err(_) => {
                debug!("Multicast discovery timeout for group {}", group);
            }
        }

        Ok(announcements)
    }

    /// Parse capability announcement
    fn parse_announcement(&self, announcement: &str) -> Result<CapabilityInfo, NestGateError> {
        // Parse announcement format: "NESTGATE-DISCOVERY:capability_type:endpoint:metadata"
        let parts: Vec<&str> = announcement.split(':').collect();

        if parts.len() >= 3 && parts[0] == "NESTGATE-DISCOVERY" {
            let capability_type = parts[1].to_string();

            // Handle URLs with colons by reconstructing the endpoint
            let endpoint = if parts.len() >= 4 && (parts[2] == "http" || parts[2] == "https") {
                // Reconstruct URL: http://host:port
                if parts.len() >= 5 {
                    format!("{}:{}:{}", parts[2], parts[3], parts[4])
                } else {
                    format!("{}:{}", parts[2], parts[3])
                }
            } else {
                parts[2].to_string()
            };

            let mut metadata = HashMap::new();
            metadata.insert("source".to_string(), "multicast".to_string());

            // Parse additional metadata if present
            let metadata_start = if parts.len() >= 4 && (parts[2] == "http" || parts[2] == "https")
            {
                // Skip URL parts when looking for metadata
                if parts.len() >= 5 { 5 } else { 4 }
            } else {
                3
            };

            if parts.len() > metadata_start {
                for metadata_part in &parts[metadata_start..] {
                    if let Some((key, value)) = metadata_part.split_once('=') {
                        metadata.insert(key.to_string(), value.to_string());
                    }
                }
            }

            Ok(CapabilityInfo {
                capability_type,
                endpoint,
                confidence: 0.75, // Medium confidence for multicast
                metadata,
            })
        } else {
            Err(NestGateError::Internal(Box::new(
                nestgate_types::error::variants::core_errors::InternalErrorDetails {
                    message: format!("Invalid announcement format: {announcement}"),
                    component: "multicast_discovery".to_string(),
                    location: Some(format!("{}:{}", file!(), line!())),
                    is_bug: false,
                    context: None,
                },
            )))
        }
    }
}

/// Network port scanning discovery
#[derive(Debug)]
/// Portscandiscovery
pub struct PortScanDiscovery {
    /// IP ranges to scan
    ip_ranges: Vec<IpRange>,
    /// Port ranges to scan for each capability
    capability_ports: HashMap<String, Vec<u16>>,
    /// Scan timeout per port
    #[allow(dead_code)]
    timeout_duration: Duration,
}

/// IP range for scanning
#[derive(Debug, Clone)]
/// Iprange
pub struct IpRange {
    /// Start
    pub start: IpAddr,
    /// End
    pub end: IpAddr,
}

impl PortScanDiscovery {
    /// Create a new port scan discovery
    ///
    /// Loads port configuration from environment:
    /// - `NESTGATE_API_PORT`: Default HTTP port (default: 8080)
    /// - `NESTGATE_METRICS_PORT`: Metrics port (default: 9090)
    /// - `NESTGATE_HEALTH_PORT`: Health check port (default: 8081)
    /// - `NESTGATE_WEBSOCKET_PORT`: WebSocket port (default: 9001)
    #[must_use]
    pub fn new() -> Self {
        use nestgate_config::config::environment::EnvironmentConfig;

        let env_config =
            EnvironmentConfig::from_env().unwrap_or_else(|_| EnvironmentConfig::default());

        let api_port = env_config.network.port.get();
        let metrics_port = env_config.monitoring.metrics_port.get();

        // ✅ Environment-driven ports with smart defaults (not hardcoded!)
        // Override with env vars: NESTGATE_HEALTH_PORT, NESTGATE_WEBSOCKET_PORT, etc.
        let health_port = std::env::var("NESTGATE_HEALTH_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8081);

        let websocket_port = std::env::var("NESTGATE_WEBSOCKET_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(9001);

        let https_port = std::env::var("NESTGATE_HTTPS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8443);

        let security_port = std::env::var("NESTGATE_SECURITY_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(9000);

        let security_https_port = std::env::var("NESTGATE_SECURITY_HTTPS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(9443);

        let ai_port = std::env::var("NESTGATE_AI_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7000);

        let ai_https_port = std::env::var("NESTGATE_AI_HTTPS_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(7443);

        let ai_alt_port = std::env::var("NESTGATE_AI_ALT_PORT")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(8000);

        let mut capability_ports = HashMap::new();
        capability_ports.insert(
            "orchestration".to_string(),
            vec![api_port, https_port, metrics_port],
        );
        capability_ports.insert(
            "security".to_string(),
            vec![security_port, security_https_port],
        );
        capability_ports.insert("ai".to_string(), vec![ai_port, ai_https_port, ai_alt_port]);
        capability_ports.insert("storage".to_string(), vec![health_port, websocket_port]);

        Self {
            ip_ranges: Vec::new(),
            capability_ports,
            timeout_duration: Duration::from_millis(500),
        }
    }

    /// Add an IP range to scan
    pub fn add_ip_range(&mut self, start: IpAddr, end: IpAddr) {
        self.ip_ranges.push(IpRange { start, end });
    }

    /// Add local network ranges (192.168.x.x, 10.x.x.x, 172.16-31.x.x)
    pub fn add_local_networks(&mut self) {
        // SAFETY: These are hardcoded RFC 1918 private IP addresses that will never fail to parse
        #[allow(clippy::expect_used)]
        // Add common local network ranges
        {
            self.add_ip_range(
                "192.168.1.1"
                    .parse()
                    .expect("BUG: RFC 1918 private IP address must parse"),
                "192.168.1.254"
                    .parse()
                    .expect("BUG: RFC 1918 private IP address must parse"),
            );
            self.add_ip_range(
                "10.0.0.1"
                    .parse()
                    .expect("BUG: RFC 1918 private IP address must parse"),
                "10.0.0.254"
                    .parse()
                    .expect("BUG: RFC 1918 private IP address must parse"),
            );
        }
    }
}

impl Default for PortScanDiscovery {
    /// Returns the default instance
    fn default() -> Self {
        let mut discovery = Self::new();
        discovery.add_local_networks();
        discovery
    }
}

impl DiscoveryMethod for PortScanDiscovery {
    /// Discover
    async fn discover(&self) -> Result<Vec<CapabilityInfo>, NestGateError> {
        let mut capabilities = Vec::new();

        info!(
            "Starting port scan discovery across {} IP ranges",
            self.ip_ranges.len()
        );

        for ip_range in &self.ip_ranges {
            debug!(
                "Scanning IP range: {:?} - {:?}",
                ip_range.start, ip_range.end
            );

            for (capability_type, ports) in &self.capability_ports {
                for &port in ports {
                    // Scan a subset of IPs in the range (for demonstration)
                    if let Some(found_capabilities) =
                        self.scan_port(ip_range, capability_type, port).await
                    {
                        capabilities.extend(found_capabilities);
                    }
                }
            }
        }

        if !capabilities.is_empty() {
            info!(
                "Port scan discovery found {} capabilities",
                capabilities.len()
            );
        }

        Ok(capabilities)
    }

    /// Method Name
    fn method_name(&self) -> &'static str {
        "port-scan"
    }
}

impl PortScanDiscovery {
    /// Scan a specific port on IP range
    async fn scan_port(
        &self,
        _ip_range: &IpRange,
        capability_type: &str,
        port: u16,
    ) -> Option<Vec<CapabilityInfo>> {
        // In a real implementation, this would scan the IP range
        // For demonstration, we'll return empty results
        debug!("Scanning for {} services on port {}", capability_type, port);
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dns_service_discovery() {
        let discovery = DnsServiceDiscovery::new("local".to_string());

        // Test capability type extraction
        let capability_type = discovery.extract_capability_type("_nestgate-orchestration._tcp");
        assert_eq!(capability_type, "orchestration");

        let capability_type = discovery.extract_capability_type("_nestgate-security._tcp");
        assert_eq!(capability_type, "security");
    }

    #[tokio::test]
    async fn test_dns_extract_capability_type_edge_cases() {
        let discovery = DnsServiceDiscovery::new("local".to_string());

        assert_eq!(discovery.extract_capability_type("_nestgate-ai._tcp"), "ai");
        assert_eq!(
            discovery.extract_capability_type("_nestgate-storage._tcp"),
            "storage"
        );
        assert_eq!(
            discovery.extract_capability_type("_nestgate-compute._tcp"),
            "compute"
        );
        assert_eq!(
            discovery.extract_capability_type("unknown-service._tcp"),
            "unknown"
        );
    }

    #[tokio::test]
    async fn test_dns_discovery_empty_result() {
        let discovery = DnsServiceDiscovery::new("local".to_string());
        let capabilities = discovery.discover().await.unwrap();
        // query_srv_record returns empty Vec, so we get empty capabilities
        assert!(capabilities.is_empty());
    }

    #[tokio::test]
    async fn test_dns_add_service_type_and_set_timeout() {
        let mut discovery = DnsServiceDiscovery::new("test".to_string());
        discovery.add_service_type("_custom._tcp".to_string());
        discovery.set_timeout(Duration::from_secs(10));
        assert_eq!(discovery.method_name(), "dns-srv");
    }

    #[tokio::test]
    async fn test_multicast_discovery() {
        let discovery = MulticastDiscovery::new();

        // Test announcement parsing (generic orchestration capability)
        let announcement =
            "NESTGATE-DISCOVERY:orchestration:http://orchestration-service:8080:priority=100";
        let capability = discovery
            .parse_announcement(announcement)
            .expect("Network operation failed");

        assert_eq!(capability.capability_type, "orchestration");
        assert_eq!(capability.endpoint, "http://orchestration-service:8080");
        assert_eq!(
            capability.metadata.get("priority"),
            Some(&"100".to_string())
        );
    }

    #[tokio::test]
    async fn test_multicast_parse_announcement_invalid_format() {
        let discovery = MulticastDiscovery::new();
        let result = discovery.parse_announcement("invalid:announcement");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_multicast_parse_announcement_https_url() {
        let discovery = MulticastDiscovery::new();
        let announcement = "NESTGATE-DISCOVERY:storage:https:127.0.0.1:9443:region=us";
        let capability = discovery.parse_announcement(announcement).unwrap();
        assert_eq!(capability.capability_type, "storage");
        assert_eq!(capability.endpoint, "https:127.0.0.1:9443");
    }

    #[tokio::test]
    async fn test_multicast_parse_announcement_minimal() {
        let discovery = MulticastDiscovery::new();
        let announcement = "NESTGATE-DISCOVERY:compute:host:5000";
        let capability = discovery.parse_announcement(announcement).unwrap();
        assert_eq!(capability.capability_type, "compute");
        assert_eq!(capability.endpoint, "host");
    }

    #[tokio::test]
    async fn test_multicast_add_group_and_set_timeout() {
        let mut discovery = MulticastDiscovery::new();
        discovery.add_multicast_group("224.0.0.1:5354".parse().unwrap());
        discovery.set_timeout(Duration::from_secs(5));
        assert_eq!(discovery.method_name(), "multicast");
    }

    #[tokio::test]
    async fn test_multicast_default() {
        let discovery = MulticastDiscovery::default();
        assert_eq!(discovery.method_name(), "multicast");
    }

    #[tokio::test]
    async fn test_srv_record_construction() {
        let record = SrvRecord {
            priority: 10,
            weight: 5,
            port: 8080,
            target: "host.example.com".to_string(),
        };
        assert_eq!(record.priority, 10);
        assert_eq!(record.port, 8080);
    }

    #[tokio::test]
    async fn test_port_scan_discovery() {
        let mut discovery = PortScanDiscovery::new();
        discovery.add_ip_range(
            "127.0.0.1".parse().expect("Network operation failed"),
            "127.0.0.1".parse().expect("Network operation failed"),
        );

        assert_eq!(discovery.method_name(), "port-scan");
        let capabilities = discovery.discover().await.unwrap();
        assert!(capabilities.is_empty());
    }

    #[tokio::test]
    async fn test_port_scan_add_local_networks() {
        let mut discovery = PortScanDiscovery::new();
        discovery.add_local_networks();
        let capabilities = discovery.discover().await.unwrap();
        assert!(capabilities.is_empty());
    }

    #[tokio::test]
    async fn test_port_scan_default() {
        let discovery = PortScanDiscovery::default();
        assert_eq!(discovery.method_name(), "port-scan");
    }

    #[tokio::test]
    async fn test_ip_range_construction() {
        let range = IpRange {
            start: "192.168.1.1".parse().unwrap(),
            end: "192.168.1.254".parse().unwrap(),
        };
        assert_eq!(range.start.to_string(), "192.168.1.1");
    }
}
