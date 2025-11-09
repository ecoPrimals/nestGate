/// Network Configuration Module
/// Configuration structures for native async network implementation
use serde::{Deserialize, Serialize};
use std::time::Duration;
/// Network configuration for native async implementation
/// Network configuration for native async networking
/// Defines connection parameters, timeouts, and performance settings
/// **⚠️ DEPRECATED**: Use `CanonicalNetworkConfig` from `canonical_primary::domains::network`
#[deprecated(
    since = "0.9.0",
    note = "Use canonical_primary::domains::network::CanonicalNetworkConfig instead"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Server host
    pub host: String,
    /// Server port
    pub port: u16,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Enable TLS
    pub enable_tls: bool,
}
#[allow(deprecated)] // Keeping for backwards compatibility during migration
impl Default for NetworkConfig {
    fn default() -> Self {
        use crate::constants::hardcoding::ports;
        Self {
            host: crate::constants::canonical_defaults::network::LOCALHOST.to_string(),
            port: ports::HTTP_DEFAULT,
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            enable_tls: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_network_config_default() {
        let config = NetworkConfig::default();

        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        assert_eq!(config.request_timeout, Duration::from_secs(60));
        assert!(!config.enable_tls);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_custom() {
        let config = NetworkConfig {
            host: "192.168.1.100".to_string(),
            port: 9090,
            connection_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(120),
            enable_tls: true,
        };

        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 9090);
        assert_eq!(config.connection_timeout, Duration::from_secs(10));
        assert_eq!(config.request_timeout, Duration::from_secs(120));
        assert!(config.enable_tls);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_clone() {
        let config1 = NetworkConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.host, config2.host);
        assert_eq!(config1.port, config2.port);
        assert_eq!(config1.connection_timeout, config2.connection_timeout);
        assert_eq!(config1.request_timeout, config2.request_timeout);
        assert_eq!(config1.enable_tls, config2.enable_tls);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_serialization() {
        let config = NetworkConfig {
            host: "example.com".to_string(),
            port: 443,
            connection_timeout: Duration::from_secs(15),
            request_timeout: Duration::from_secs(90),
            enable_tls: true,
        };

        let json = serde_json::to_string(&config).expect("Serialization failed");
        assert!(json.contains("example.com"));
        assert!(json.contains("443"));

        let deserialized: NetworkConfig =
            serde_json::from_str(&json).expect("Deserialization failed");
        assert_eq!(deserialized.host, "example.com");
        assert_eq!(deserialized.port, 443);
        assert!(deserialized.enable_tls);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_tls_enabled() {
        let mut config = NetworkConfig::default();
        assert!(!config.enable_tls);

        config.enable_tls = true;
        assert!(config.enable_tls);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_timeout_values() {
        let config = NetworkConfig {
            host: "localhost".to_string(),
            port: 8080,
            connection_timeout: Duration::from_millis(500),
            request_timeout: Duration::from_millis(2000),
            enable_tls: false,
        };

        assert_eq!(config.connection_timeout.as_millis(), 500);
        assert_eq!(config.request_timeout.as_millis(), 2000);
        assert!(config.connection_timeout < config.request_timeout);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_port_ranges() {
        let config_low = NetworkConfig {
            host: "test".to_string(),
            port: 80,
            connection_timeout: Duration::from_secs(1),
            request_timeout: Duration::from_secs(1),
            enable_tls: false,
        };
        assert_eq!(config_low.port, 80);

        let config_high = NetworkConfig {
            host: "test".to_string(),
            port: 65535,
            connection_timeout: Duration::from_secs(1),
            request_timeout: Duration::from_secs(1),
            enable_tls: false,
        };
        assert_eq!(config_high.port, 65535);
    }

    #[test]
    #[allow(deprecated)]
    fn test_network_config_debug_format() {
        let config = NetworkConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("NetworkConfig"));
        assert!(debug_str.contains("host"));
        assert!(debug_str.contains("port"));
    }
}
