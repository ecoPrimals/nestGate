//! **NETWORK CONFIG MIGRATION**
//!
//! Migration implementation for nestgate-network's NetworkConfig
//! to the canonical configuration system.

use nestgate_core::config::migration_traits::IntoCanonicalNetworkConfig;
use nestgate_core::config::canonical_master::{NetworkConfig as CanonicalNetworkConfig, LoadBalancerConfig, ServiceDiscoveryConfig, ExternalNetworkConfig};
use crate::types::NetworkConfig;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;
use std::collections::HashMap;

// ==================== SECTION ====================

/// Migration implementation for nestgate-network NetworkConfig
impl IntoCanonicalNetworkConfig for NetworkConfig {
    fn into_canonical(self) -> CanonicalNetworkConfig<8080, 30000> {
        CanonicalNetworkConfig {
            bind_address: self.host.parse::<IpAddr>()
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: self.port,
            request_timeout: Duration::from_secs(self.connection_timeout_seconds),
            connection_timeout: Duration::from_secs(self.connection_timeout_seconds),
            max_connections: self.max_connections as usize,
            keep_alive_timeout: Duration::from_secs(self.keep_alive_timeout_seconds),
            tls_enabled: false, // Default - not in original config
            tls_cert_path: None,
            tls_key_path: None,
            load_balancer: LoadBalancerConfig::default(),
            service_discovery: ServiceDiscoveryConfig::default(),
            external: ExternalNetworkConfig::default(),
            network_settings: HashMap::new(),
        }
    }
    
    fn into_canonical_with_params<const API_PORT: u16, const TIMEOUT_MS: u64>(
        self
    ) -> CanonicalNetworkConfig<API_PORT, TIMEOUT_MS> {
        CanonicalNetworkConfig {
            bind_address: self.host.parse::<IpAddr>()
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),
            port: self.port,
            request_timeout: Duration::from_millis(TIMEOUT_MS),
            connection_timeout: Duration::from_secs(self.connection_timeout_seconds),
            max_connections: self.max_connections as usize,
            keep_alive_timeout: Duration::from_secs(self.keep_alive_timeout_seconds),
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            load_balancer: LoadBalancerConfig::default(),
            service_discovery: ServiceDiscoveryConfig::default(),
            external: ExternalNetworkConfig::default(),
            network_settings: HashMap::new(),
        }
    }
}

// ==================== SECTION ====================

/// Migration helper functions for NetworkConfig
impl NetworkConfig {
    /// Create a canonical config with preserved port range info
    pub fn into_canonical_with_port_range(self) -> CanonicalNetworkConfig<8080, 30000> {
        let mut canonical = self.into_canonical();
        
        // Preserve port range information in network_settings
        canonical.network_settings.insert(
            "port_range_start".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.port_range_start))
        );
        canonical.network_settings.insert(
            "port_range_end".to_string(),
            serde_json::Value::Number(serde_json::Number::from(self.port_range_end))
        );
        canonical.network_settings.insert(
            "keep_alive".to_string(),
            serde_json::Value::Bool(self.keep_alive)
        );
        
        canonical
    }
}

// ==================== SECTION ====================

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_network_config_migration() {
        let original = NetworkConfig {
            host: "0.0.0.0".to_string(),
            port: 9000,
            max_connections: 500,
            connection_timeout_seconds: 45,
            port_range_start: 8000,
            port_range_end: 9000,
            keep_alive: true,
            keep_alive_timeout_seconds: 120,
        };
        
        let canonical = original.into_canonical();
        
        // Verify migration preserved key values
        assert_eq!(canonical.port, 9000);
        assert_eq!(canonical.max_connections, 500);
        assert_eq!(canonical.connection_timeout, Duration::from_secs(45));
        assert_eq!(canonical.keep_alive_timeout, Duration::from_secs(120));
    }
    
    #[test]
    fn test_network_config_with_port_range() {
        let original = NetworkConfig {
            host: "localhost".to_string(),
            port: 8080,
            max_connections: 1000,
            connection_timeout_seconds: 30,
            port_range_start: 9000,
            port_range_end: 9999,
            keep_alive: true,
            keep_alive_timeout_seconds: 60,
        };
        
        let canonical = original.into_canonical_with_port_range();
        
        // Verify port range preserved in settings
        assert!(canonical.network_settings.contains_key("port_range_start"));
        assert!(canonical.network_settings.contains_key("port_range_end"));
        assert_eq!(
            canonical.network_settings.get("port_range_start").unwrap(),
            &serde_json::Value::Number(serde_json::Number::from(9000))
        );
    }
} 