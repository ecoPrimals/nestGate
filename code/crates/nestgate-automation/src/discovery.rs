//! Ecosystem Discovery
//! 
//! Dynamic ecosystem service discovery for finding Songbirds and coordinating service discovery

use crate::types::*;
use crate::Result;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
                .timeout(Duration::from_secs(10))
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
            let mut cache = self.discovered_instances.write().unwrap();
            cache.clear();
            for instance in &discovered {
                cache.insert(instance.instance_id.clone(), instance.clone());
            }
        }
        
        // Update last discovery time
        {
            let mut last = self.last_discovery.write().unwrap();
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
        let socket = UdpSocket::bind("0.0.0.0:0").await
            .map_err(|e| crate::types::AutomationError::NetworkError(format!("Failed to bind UDP socket: {}", e)))?;
        
        // Join mDNS multicast group
        let multicast_addr = "224.0.0.251:5353".parse::<SocketAddr>()
            .map_err(|e| crate::types::AutomationError::NetworkError(format!("Invalid mDNS address: {}", e)))?;
        
        // Send mDNS query for _nestgate._tcp.local
        let query = self.build_mdns_query();
        
        match socket.send_to(&query, multicast_addr).await {
            Ok(_) => {
                tracing::debug!("Sent mDNS query for NestGate services");
                // TODO: Listen for responses and parse them
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
            0x00, 0x00, // Additional RRs: 0
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
        use std::process::Command;
        
        let mut subnets = Vec::new();
        
        // Try to get network information using `ip route` on Linux
        match Command::new("ip").args(&["route", "show"]).output() {
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
                // Fallback to common subnets
                subnets.extend(vec![
                    "192.168.1.0/24".to_string(),
                    "192.168.0.0/24".to_string(),
                    "10.0.0.0/24".to_string(),
                ]);
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
        let mut instances = Vec::new();
        
        // Parse subnet (e.g., "192.168.1.0/24")
        let (base_ip, _mask) = subnet.split_once('/').unwrap_or((subnet, "24"));
        let base_parts: Vec<&str> = base_ip.split('.').collect();
        
        if base_parts.len() != 4 {
            return Ok(instances);
        }
        
        let base_a: u8 = base_parts[0].parse().unwrap_or(0);
        let base_b: u8 = base_parts[1].parse().unwrap_or(0);
        let base_c: u8 = base_parts[2].parse().unwrap_or(0);
        
        // Scan common host IPs in the subnet (1-254)
        let mut scan_tasks = Vec::new();
        
        for host in 1..=254 {
            let ip = Ipv4Addr::new(base_a, base_b, base_c, host);
            let client = self.client.clone();
            
            let task = tokio::spawn(async move {
                // Try common NestGate ports
                for port in [8080, 3000, 3001, 8000, 9000] {
                    let url = format!("http://{}:{}/api/v1/status", ip, port);
                    
                    match client.get(&url).timeout(Duration::from_millis(500)).send().await {
                        Ok(response) => {
                            if response.status().is_success() {
                                if let Ok(text) = response.text().await {
                                    if text.contains("nestgate") || text.contains("songbird") {
                                        return Some(SongbirdInstance {
                                            instance_id: format!("{}:{}", ip, port),
                                            endpoint: format!("http://{}:{}", ip, port),
                                            version: "unknown".to_string(),
                                            capabilities: vec!["storage".to_string()],
                                            last_seen: SystemTime::now(),
                                            is_ephemeral: true, // Local network discovery
                                        });
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Host not responding on this port, continue
                        }
                    }
                }
                None
            });
            
            scan_tasks.push(task);
        }
        
        // Wait for all scan tasks with a reasonable timeout
        let scan_results = futures::future::join_all(scan_tasks).await;
        
        for result in scan_results {
            if let Ok(Some(instance)) = result {
                instances.push(instance);
            }
        }
        
        Ok(instances)
    }
    
    /// Discover via known discovery endpoints
    async fn discover_via_endpoints(&self) -> Result<Vec<SongbirdInstance>> {
        let mut instances = Vec::new();
        
        // Try known discovery endpoints or registries
        let discovery_endpoints = vec![
            "http://localhost:8080/api/v1/discovery/songbirds",
            "http://127.0.0.1:3001/api/v1/discovery/songbirds",
        ];
        
        for endpoint in discovery_endpoints {
            match self.client.get(endpoint).timeout(Duration::from_secs(5)).send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        if let Ok(body) = response.text().await {
                            // Try to parse as JSON array of SongbirdInstance
                            if let Ok(endpoint_instances) = serde_json::from_str::<Vec<SongbirdInstance>>(&body) {
                                instances.extend(endpoint_instances);
                                tracing::debug!("Found {} instances from endpoint {}", instances.len(), endpoint);
                            }
                        }
                    }
                }
                Err(e) => {
                    tracing::debug!("Discovery endpoint {} not available: {}", endpoint, e);
                }
            }
        }
        
        Ok(instances)
    }
    
    /// Get cached discovered instances
    pub fn get_cached_instances(&self) -> Vec<SongbirdInstance> {
        let cache = self.discovered_instances.read().unwrap();
        cache.values().cloned().collect()
    }
    
    /// Check if discovery cache is stale
    pub fn is_cache_stale(&self) -> bool {
        let last = self.last_discovery.read().unwrap();
        match *last {
            Some(last_time) => {
                last_time.elapsed().unwrap_or(Duration::from_secs(0)) > Duration::from_secs(300) // 5 minutes
            }
            None => true,
        }
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