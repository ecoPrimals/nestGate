//! Ecosystem Discovery
//!
//! Dynamic ecosystem service discovery for finding Songbirds and coordinating service discovery

use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};
use std::process::Command;
use std::time::{Duration, SystemTime};
use tracing::error;

use crate::types::*;
use crate::Result;

/// Configuration for ecosystem discovery
#[derive(Debug, Clone)]
pub struct DiscoveryConfig {
    /// Discovery timeout in seconds
    pub timeout_seconds: u64,
    /// Ports to scan for services
    pub discovery_ports: Option<Vec<u16>>,
    /// Known discovery endpoints
    pub discovery_endpoints: Option<Vec<String>>,
    /// Enable mDNS discovery
    pub enable_mdns: bool,
    /// Enable network scanning
    pub enable_network_scan: bool,
    /// Cache expiration time in seconds
    pub cache_expiration_seconds: u64,
    /// Network ranges to scan
    pub network_ranges: Vec<String>,
    /// Default scan networks
    pub default_scan_networks: Vec<String>,
}

impl DiscoveryConfig {
    pub fn from_automation_config(_config: &AutomationConfig) -> Self {
        Self {
            timeout_seconds: 10,
            discovery_ports: None,     // Will use defaults
            discovery_endpoints: None, // Will use defaults
            enable_mdns: true,
            enable_network_scan: true,
            cache_expiration_seconds: std::env::var("NESTGATE_DISCOVERY_CACHE_EXPIRATION_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300), // 5 minutes
            network_ranges: std::env::var("NESTGATE_DISCOVERY_NETWORK_RANGES")
                .map(|ranges| ranges.split(',').map(String::from).collect())
                .unwrap_or_else(|_| {
                    vec![
                        "192.168.1.0/24".to_string(),
                        "192.168.0.0/24".to_string(),
                        "10.0.0.0/24".to_string(),
                    ]
                }),
            default_scan_networks: std::env::var("NESTGATE_DEFAULT_SCAN_NETWORKS")
                .map(|networks| networks.split(',').map(String::from).collect())
                .unwrap_or_else(|_| {
                    vec![
                        "192.168.1.0/24".to_string(),
                        "192.168.0.0/24".to_string(),
                        "10.0.0.0/24".to_string(),
                    ]
                }),
        }
    }
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 10,
            discovery_ports: Some(
                std::env::var("NESTGATE_DISCOVERY_PORTS")
                    .ok()
                    .and_then(|s| {
                        s.split(',')
                            .filter_map(|p| p.trim().parse().ok())
                            .collect::<Vec<u16>>()
                            .into()
                    })
                    .filter(|v: &Vec<u16>| !v.is_empty())
                    .unwrap_or_else(|| vec![8080, 3000, 3001, 8000, 9000]),
            ),
            discovery_endpoints: Some(vec![
                format!(
                    "http://{}:{}/api/v1/discovery/songbirds",
                    nestgate_core::constants::addresses::localhost(),
                    nestgate_core::constants::network::api_port()
                ),
                format!(
                    "http://{}:{}/api/v1/discovery/songbirds",
                    nestgate_core::constants::addresses::localhost(),
                    nestgate_core::constants::network::discovery_port()
                ),
            ]),
            enable_mdns: true,
            enable_network_scan: true,
            cache_expiration_seconds: std::env::var("NESTGATE_DISCOVERY_CACHE_EXPIRATION_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300),
            network_ranges: std::env::var("NESTGATE_DISCOVERY_NETWORK_RANGES")
                .map(|ranges| ranges.split(',').map(String::from).collect())
                .unwrap_or_else(|_| {
                    vec![
                        "192.168.1.0/24".to_string(),
                        "192.168.0.0/24".to_string(),
                        "10.0.0.0/24".to_string(),
                    ]
                }),
            default_scan_networks: std::env::var("NESTGATE_DEFAULT_SCAN_NETWORKS")
                .map(|networks| networks.split(',').map(String::from).collect())
                .unwrap_or_else(|_| {
                    vec![
                        "192.168.1.0/24".to_string(),
                        "192.168.0.0/24".to_string(),
                        "10.0.0.0/24".to_string(),
                    ]
                }),
        }
    }
}

/// Dynamic ecosystem discovery service
#[cfg(feature = "network-integration")]
#[derive(Debug)]
pub struct EcosystemDiscovery {
    config: DiscoveryConfig,
    client: reqwest::Client,
    discovered_instances: std::sync::Arc<std::sync::RwLock<HashMap<String, SongbirdInstance>>>,
    last_discovery: std::sync::Arc<std::sync::RwLock<Option<SystemTime>>>,
}

#[cfg(feature = "network-integration")]
impl EcosystemDiscovery {
    pub fn new(config: &AutomationConfig) -> Result<Self> {
        Ok(Self {
            config: DiscoveryConfig::from_automation_config(config),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(
                    std::env::var("NESTGATE_DISCOVERY_REQUEST_TIMEOUT_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(10), // 10 seconds default
                ))
                .build()
                .map_err(|e| crate::types::AutomationError::NetworkError(e.to_string()))?,
            discovered_instances: std::sync::Arc::new(std::sync::RwLock::new(HashMap::new())),
            last_discovery: std::sync::Arc::new(std::sync::RwLock::new(None)),
        })
    }

    /// Discover Songbird instances on the network
    pub async fn discover_songbirds(&self) -> Result<Vec<SongbirdInstance>> {
        tracing::info!("Starting Songbird discovery");

        let mut discovered = Vec::new();

        // 1. Try mDNS/Bonjour discovery
        if let Ok(mdns_instances) = self.discover_via_mdns().await {
            discovered.extend(mdns_instances);
        }

        // 2. Try network scanning on common subnets
        if let Ok(scan_instances) = self.discover_via_network_scan().await {
            discovered.extend(scan_instances);
        }

        // 3. Try known discovery endpoints
        if let Ok(endpoint_instances) = self.discover_via_endpoints().await {
            discovered.extend(endpoint_instances);
        }

        // Update our cache
        {
            let mut cache = match self.discovered_instances.write() {
                Ok(cache) => cache,
                Err(e) => {
                    error!("Failed to acquire discovery cache lock: {}", e);
                    return Err(AutomationError::Cache(
                        "Failed to acquire discovery cache lock".to_string(),
                    ));
                }
            };
            cache.clear();
            for instance in &discovered {
                cache.insert(instance.instance_id.clone(), instance.clone());
            }
        }

        // Update last discovery time
        {
            let mut last = match self.last_discovery.write() {
                Ok(last) => last,
                Err(e) => {
                    error!("Failed to acquire last discovery lock: {}", e);
                    return Err(AutomationError::Cache(
                        "Failed to acquire last discovery lock".to_string(),
                    ));
                }
            };
            *last = Some(SystemTime::now());
        }

        tracing::info!("Discovered {} Songbird instances", discovered.len());
        Ok(discovered)
    }

    /// Discover via mDNS/Bonjour (local network)
    async fn discover_via_mdns(&self) -> Result<Vec<SongbirdInstance>> {
        let mut instances = Vec::new();

        // Try to discover NestGate services via mDNS
        // This would typically use a library like `mdns` or `async-mdns`
        // For now, we'll implement a basic UDP multicast approach

        match self.try_mdns_discovery().await {
            Ok(mdns_instances) => {
                instances.extend(mdns_instances);
                tracing::debug!("Found {} instances via mDNS", instances.len());
            }
            Err(e) => {
                tracing::warn!("mDNS discovery failed: {}", e);
            }
        }

        Ok(instances)
    }

    /// Try mDNS discovery using UDP multicast
    async fn try_mdns_discovery(&self) -> Result<Vec<SongbirdInstance>> {
        use tokio::net::UdpSocket;

        let instances = Vec::new();

        // Create UDP socket for mDNS
        let socket = UdpSocket::bind("0.0.0.0:0").await.map_err(|e| {
            crate::types::AutomationError::NetworkError(format!("Failed to bind UDP socket: {}", e))
        })?;

        // Join mDNS multicast group
        let multicast_addr = std::env::var("NESTGATE_MULTICAST_ADDRESS")
            .unwrap_or_else(|_| {
                format!(
                    "{}:{}",
                    std::env::var("NESTGATE_MULTICAST_IP")
                        .unwrap_or_else(|_| "224.0.0.251".to_string()),
                    std::env::var("NESTGATE_MULTICAST_PORT").unwrap_or_else(|_| "5353".to_string())
                )
            })
            .parse::<SocketAddr>()
            .map_err(|e| {
                crate::types::AutomationError::NetworkError(format!("Invalid mDNS address: {}", e))
            })?;

        // Send mDNS query for _nestgate._tcp.local
        let query = self.build_mdns_query();

        match socket.send_to(&query, multicast_addr).await {
            Ok(_) => {
                tracing::debug!("Sent mDNS query for NestGate services");
                // mDNS response parsing requires full DNS packet implementation
                // For now, return empty list as this requires more complex mDNS parsing
            }
            Err(e) => {
                tracing::warn!("Failed to send mDNS query: {}", e);
            }
        }

        Ok(instances)
    }

    /// Build mDNS query packet for NestGate services
    fn build_mdns_query(&self) -> Vec<u8> {
        // Simplified mDNS query for _nestgate._tcp.local
        // In a real implementation, this would be a properly formatted DNS packet
        vec![
            0x00, 0x00, // Transaction ID
            0x01, 0x00, // Flags (standard query)
            0x00, 0x01, // Questions: 1
            0x00, 0x00, // Answer RRs: 0
            0x00, 0x00, // Authority RRs: 0
            0x00,
            0x00, // Additional RRs: 0
                  // Query for _nestgate._tcp.local would go here
        ]
    }

    /// Discover via network scanning
    async fn discover_via_network_scan(&self) -> Result<Vec<SongbirdInstance>> {
        let mut instances = Vec::new();

        // Get local network subnets to scan
        let subnets = self.get_local_subnets().await?;

        for subnet in subnets {
            tracing::debug!("Scanning subnet: {}", subnet);

            // Scan common NestGate ports on the subnet
            let subnet_instances = self.scan_subnet(&subnet).await?;
            instances.extend(subnet_instances);
        }

        Ok(instances)
    }

    /// Get local network subnets to scan
    async fn get_local_subnets(&self) -> Result<Vec<String>> {
        let mut subnets = Vec::new();

        // Try to get network information using `ip route` on Linux
        match Command::new("ip").args(["route", "show"]).output() {
            Ok(output) => {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("192.168.") || line.contains("10.") || line.contains("172.") {
                        // Extract subnet from route line
                        if let Some(subnet) = self.extract_subnet_from_route(line) {
                            subnets.push(subnet);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to get network routes: {}", e);
                // Fallback to configurable subnets
                subnets.extend(self.config.network_ranges.clone());
            }
        }

        Ok(subnets)
    }

    /// Extract subnet from ip route output line
    fn extract_subnet_from_route(&self, line: &str) -> Option<String> {
        // Parse lines like "192.168.1.0/24 dev eth0 proto kernel scope link src 192.168.1.100"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if !parts.is_empty() && parts[0].contains('/') {
            Some(parts[0].to_string())
        } else {
            None
        }
    }

    /// Scan a subnet for NestGate instances
    async fn scan_subnet(&self, subnet: &str) -> Result<Vec<SongbirdInstance>> {
        let mut tasks = Vec::new();

        // Parse subnet and generate IP addresses to scan
        let subnet_ips = self.parse_subnet_to_ips(subnet)?;

        // Use configurable ports instead of hardcoded values
        let discovery_ports = self
            .config
            .discovery_ports
            .clone()
            .unwrap_or_else(|| vec![8080, 3000, 3001, 8000, 9000]);

        for ip in &subnet_ips[..std::cmp::min(subnet_ips.len(), 20)] {
            // Limit to 20 IPs to avoid overload
            let ip_string = ip.to_string();
            let ports = discovery_ports.clone();

            let task = tokio::spawn(async move {
                for port in ports {
                    let url = format!("http://{}:{}/api/v1/status", ip_string, port);

                    // Try to connect to see if there's a NestGate instance
                    match reqwest::Client::new()
                        .get(&url)
                        .timeout(Duration::from_secs(
                            std::env::var("NESTGATE_DISCOVERY_TIMEOUT_SECS")
                                .ok()
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(2), // 2 seconds default
                        ))
                        .send()
                        .await
                    {
                        Ok(response) if response.status().is_success() => {
                            // Found a potential NestGate instance
                            return Some(SongbirdInstance {
                                instance_id: format!("discovered-{}-{}", ip_string, port),
                                endpoint: format!("http://{}:{}", ip_string, port),
                                version: "unknown".to_string(),
                                capabilities: Vec::new(),
                                last_seen: SystemTime::now(),
                                is_ephemeral: true,
                            });
                        }
                        _ => continue,
                    }
                }
                None
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        let mut instances = Vec::new();
        for task in tasks {
            if let Ok(Some(instance)) = task.await {
                instances.push(instance);
            }
        }

        Ok(instances)
    }

    /// Discover via known discovery endpoints
    async fn discover_via_endpoints(&self) -> Result<Vec<SongbirdInstance>> {
        let mut instances = Vec::new();

        // Use configurable discovery endpoints instead of hardcoded values
        let discovery_endpoints = self.config.discovery_endpoints.clone().unwrap_or_else(|| {
            vec![
                format!(
                    "http://{}:{}/api/v1/discovery/songbirds",
                    nestgate_core::constants::addresses::localhost(),
                    nestgate_core::constants::network::api_port()
                ),
                format!(
                    "http://{}:{}/api/v1/discovery/songbirds",
                    nestgate_core::constants::addresses::localhost(),
                    nestgate_core::constants::network::discovery_port()
                ),
            ]
        });

        for endpoint in discovery_endpoints {
            match self
                .client
                .get(&endpoint)
                .timeout(Duration::from_secs(
                    std::env::var("NESTGATE_DISCOVERY_HEALTH_TIMEOUT_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(5), // 5 seconds default
                ))
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(instances_json) = response.json::<Vec<SongbirdInstance>>().await {
                            instances.extend(instances_json);
                            tracing::debug!(
                                "Found {} instances from endpoint {}",
                                instances.len(),
                                endpoint
                            );
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to query discovery endpoint {}: {}", endpoint, e);
                }
            }
        }

        Ok(instances)
    }

    /// Get cached discovered instances
    pub fn get_cached_instances(&self) -> Vec<SongbirdInstance> {
        let cache = match self.discovered_instances.read() {
            Ok(cache) => cache,
            Err(e) => {
                error!("Failed to acquire discovery cache read lock: {}", e);
                return Vec::new();
            }
        };
        cache.values().cloned().collect()
    }

    /// Check if discovery cache is stale
    pub fn is_cache_stale(&self) -> bool {
        let last = match self.last_discovery.read() {
            Ok(last) => last,
            Err(e) => {
                error!("Failed to acquire last discovery read lock: {}", e);
                return true;
            }
        };
        match *last {
            Some(last_time) => {
                last_time.elapsed().unwrap_or(Duration::from_secs(0))
                    > Duration::from_secs(
                        std::env::var("NESTGATE_DISCOVERY_CACHE_STALENESS_SECS")
                            .ok()
                            .and_then(|s| s.parse().ok())
                            .unwrap_or(300), // 5 minutes default
                    )
            }
            None => true,
        }
    }

    /// Parse subnet string into list of IP addresses to scan
    fn parse_subnet_to_ips(&self, subnet: &str) -> Result<Vec<Ipv4Addr>> {
        let mut ips = Vec::new();

        // Parse subnet (e.g., "192.168.1.0/24")
        let (base_ip, _mask) = subnet.split_once('/').unwrap_or((subnet, "24"));
        let base_parts: Vec<&str> = base_ip.split('.').collect();

        if base_parts.len() != 4 {
            return Ok(ips);
        }

        let base_a: u8 = base_parts[0].parse().unwrap_or(0);
        let base_b: u8 = base_parts[1].parse().unwrap_or(0);
        let base_c: u8 = base_parts[2].parse().unwrap_or(0);

        // Generate IPs in the subnet (1-254)
        for host in 1..=254 {
            let ip = Ipv4Addr::new(base_a, base_b, base_c, host);
            ips.push(ip);
        }

        Ok(ips)
    }
}

#[cfg(not(feature = "network-integration"))]
#[derive(Debug)]
pub struct EcosystemDiscovery;

#[cfg(not(feature = "network-integration"))]
impl EcosystemDiscovery {
    pub fn new(_config: &AutomationConfig) -> Result<Self> {
        Ok(Self)
    }

    pub async fn discover_songbirds(&self) -> Result<Vec<SongbirdInstance>> {
        // Return empty list when network integration is disabled
        Ok(vec![])
    }

    pub fn get_cached_instances(&self) -> Vec<SongbirdInstance> {
        vec![]
    }

    pub fn is_cache_stale(&self) -> bool {
        false
    }
}
