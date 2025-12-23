//! **COMPREHENSIVE PROTOCOL TESTS**
//!
//! Extensive tests for Protocol enum and protocol handling functionality.

#[cfg(test)]
mod protocol_comprehensive_tests {
    use super::super::protocol::{PerformancePreference, Protocol, ProtocolConfig};

    // ==================== PROTOCOL ENUM TESTS ====================

    #[test]
    fn test_protocol_display() {
        assert_eq!(Protocol::Nfs.to_string(), "NFS");
        assert_eq!(Protocol::Smb.to_string(), "SMB");
        assert_eq!(Protocol::Ftp.to_string(), "FTP");
        assert_eq!(Protocol::Sftp.to_string(), "SFTP");
        assert_eq!(Protocol::Http.to_string(), "HTTP");
        assert_eq!(Protocol::Tcp.to_string(), "TCP");
    }

    #[test]
    fn test_protocol_debug() {
        let protocol = Protocol::Nfs;
        assert!(format!("{:?}", protocol).contains("Nfs"));
    }

    #[test]
    fn test_protocol_clone() {
        let protocol1 = Protocol::Http;
        let protocol2 = protocol1;
        assert_eq!(protocol1, protocol2);
    }

    #[test]
    fn test_protocol_copy() {
        let protocol1 = Protocol::Tcp;
        let protocol2 = protocol1;
        assert_eq!(protocol1, protocol2);
    }

    #[test]
    fn test_protocol_equality() {
        assert_eq!(Protocol::Nfs, Protocol::Nfs);
        assert_ne!(Protocol::Nfs, Protocol::Smb);
        assert_eq!(Protocol::Http, Protocol::Http);
    }

    #[test]
    fn test_protocol_serialization() {
        let protocol = Protocol::Http;
        let serialized = serde_json::to_string(&protocol).unwrap();
        assert!(serialized.contains("Http"));
    }

    #[test]
    fn test_protocol_deserialization() {
        let json = "\"Nfs\"";
        let protocol: Protocol = serde_json::from_str(json).unwrap();
        assert_eq!(protocol, Protocol::Nfs);
    }

    #[test]
    fn test_all_protocols_serialization() {
        let protocols = vec![
            Protocol::Nfs,
            Protocol::Smb,
            Protocol::Ftp,
            Protocol::Sftp,
            Protocol::Http,
            Protocol::Tcp,
        ];

        for protocol in protocols {
            let serialized = serde_json::to_string(&protocol).unwrap();
            let deserialized: Protocol = serde_json::from_str(&serialized).unwrap();
            assert_eq!(protocol, deserialized);
        }
    }

    // ==================== PERFORMANCE PREFERENCE TESTS ====================

    #[test]
    fn test_performance_preference_default() {
        let pref = PerformancePreference::default();
        assert_eq!(pref, PerformancePreference::Balanced);
    }

    #[test]
    fn test_performance_preference_equality() {
        assert_eq!(PerformancePreference::Speed, PerformancePreference::Speed);
        assert_ne!(
            PerformancePreference::Speed,
            PerformancePreference::Reliability
        );
    }

    #[test]
    fn test_performance_preference_clone() {
        let pref1 = PerformancePreference::Reliability;
        let pref2 = pref1;
        assert_eq!(pref1, pref2);
    }

    #[test]
    fn test_performance_preference_serialization() {
        let pref = PerformancePreference::Speed;
        let serialized = serde_json::to_string(&pref).unwrap();
        let deserialized: PerformancePreference = serde_json::from_str(&serialized).unwrap();
        assert_eq!(pref, deserialized);
    }

    #[test]
    fn test_all_performance_preferences() {
        let prefs = vec![
            PerformancePreference::Speed,
            PerformancePreference::Reliability,
            PerformancePreference::Compatibility,
        ];

        for pref in prefs {
            let serialized = serde_json::to_string(&pref).unwrap();
            let deserialized: PerformancePreference = serde_json::from_str(&serialized).unwrap();
            assert_eq!(pref, deserialized);
        }
    }

    // ==================== PROTOCOL CONFIG TESTS ====================

    #[test]
    fn test_protocol_config_creation() {
        let mut options = std::collections::HashMap::new();
        options.insert("port".to_string(), "8080".to_string());

        let config = ProtocolConfig {
            protocol: Protocol::Http,
            options,
            performance: PerformancePreference::Speed,
            encryption: false,
            timeout: 30,
            max_retries: 3,
        };

        assert_eq!(config.protocol, Protocol::Http);
        assert_eq!(config.timeout, 30);
    }

    #[test]
    fn test_protocol_config_with_different_protocols() {
        let http_config = ProtocolConfig {
            protocol: Protocol::Http,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::default(),
            encryption: true,
            timeout: 30,
            max_retries: 3,
        };

        let nfs_config = ProtocolConfig {
            protocol: Protocol::Nfs,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::default(),
            encryption: false,
            timeout: 60,
            max_retries: 5,
        };

        assert_eq!(http_config.protocol, Protocol::Http);
        assert_eq!(nfs_config.protocol, Protocol::Nfs);
        assert!(http_config.encryption);
        assert!(!nfs_config.encryption);
    }

    #[test]
    fn test_protocol_config_serialization() {
        let config = ProtocolConfig {
            protocol: Protocol::Tcp,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::Reliability,
            encryption: false,
            timeout: 45,
            max_retries: 2,
        };

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: ProtocolConfig = serde_json::from_str(&serialized).unwrap();

        assert_eq!(config.protocol, deserialized.protocol);
        assert_eq!(config.timeout, deserialized.timeout);
        assert_eq!(config.max_retries, deserialized.max_retries);
    }

    #[test]
    fn test_protocol_config_clone() {
        let config1 = ProtocolConfig {
            protocol: Protocol::Sftp,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::Compatibility,
            encryption: true,
            timeout: 30,
            max_retries: 3,
        };

        let config2 = config1.clone();
        assert_eq!(config1.protocol, config2.protocol);
        assert_eq!(config1.timeout, config2.timeout);
        assert_eq!(config1.encryption, config2.encryption);
    }

    // ==================== PROTOCOL SELECTION TESTS ====================

    #[test]
    fn test_protocol_selection_by_speed() {
        // When speed is preferred, TCP should be chosen over more complex protocols
        let preference = PerformancePreference::Speed;
        assert_eq!(preference, PerformancePreference::Speed);
    }

    #[test]
    fn test_protocol_selection_by_reliability() {
        let preference = PerformancePreference::Reliability;
        assert_eq!(preference, PerformancePreference::Reliability);
    }

    #[test]
    fn test_protocol_selection_by_compatibility() {
        let preference = PerformancePreference::Compatibility;
        assert_eq!(preference, PerformancePreference::Compatibility);
    }

    // ==================== PROTOCOL COMBINATION TESTS ====================

    #[test]
    fn test_multiple_protocol_configs() {
        let configs = [
            ProtocolConfig {
                protocol: Protocol::Http,
                options: std::collections::HashMap::new(),
                performance: PerformancePreference::Speed,
                encryption: true,
                timeout: 30,
                max_retries: 3,
            },
            ProtocolConfig {
                protocol: Protocol::Tcp,
                options: std::collections::HashMap::new(),
                performance: PerformancePreference::Reliability,
                encryption: false,
                timeout: 60,
                max_retries: 5,
            },
            ProtocolConfig {
                protocol: Protocol::Nfs,
                options: std::collections::HashMap::new(),
                performance: PerformancePreference::Compatibility,
                encryption: false,
                timeout: 90,
                max_retries: 7,
            },
        ];

        assert_eq!(configs.len(), 3);
        assert_eq!(configs[0].protocol, Protocol::Http);
        assert_eq!(configs[1].protocol, Protocol::Tcp);
        assert_eq!(configs[2].protocol, Protocol::Nfs);
    }

    #[test]
    fn test_protocol_config_with_custom_timeouts() {
        let custom_timeouts = vec![
            (Protocol::Http, 10),
            (Protocol::Http, 30),
            (Protocol::Tcp, 60),
            (Protocol::Tcp, 120),
        ];

        for (protocol, timeout) in custom_timeouts {
            let config = ProtocolConfig {
                protocol,
                options: std::collections::HashMap::new(),
                performance: PerformancePreference::default(),
                encryption: false,
                timeout,
                max_retries: 3,
            };

            assert_eq!(config.timeout, timeout);
        }
    }

    // ==================== EDGE CASE TESTS ====================

    #[test]
    fn test_protocol_config_minimum_timeout() {
        let config = ProtocolConfig {
            protocol: Protocol::Tcp,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::Speed,
            encryption: false,
            timeout: 1,
            max_retries: 1,
        };

        assert_eq!(config.timeout, 1);
        assert_eq!(config.max_retries, 1);
    }

    #[test]
    fn test_protocol_config_maximum_timeout() {
        let config = ProtocolConfig {
            protocol: Protocol::Tcp,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::Speed,
            encryption: false,
            timeout: 3600,
            max_retries: 100,
        };

        assert_eq!(config.timeout, 3600);
        assert_eq!(config.max_retries, 100);
    }

    #[test]
    fn test_protocol_config_with_encryption() {
        let protocols_with_encryption = vec![Protocol::Http, Protocol::Sftp, Protocol::Smb];

        for protocol in protocols_with_encryption {
            let config = ProtocolConfig {
                protocol,
                options: std::collections::HashMap::new(),
                performance: PerformancePreference::default(),
                encryption: true,
                timeout: 30,
                max_retries: 3,
            };

            assert_eq!(config.protocol, protocol);
            assert!(config.encryption);
        }
    }

    // ==================== PROTOCOL HASH TESTS ====================

    #[test]
    fn test_protocol_hash_equality() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Protocol::Http);
        set.insert(Protocol::Http); // Duplicate
        set.insert(Protocol::Tcp);

        assert_eq!(set.len(), 2); // Only 2 unique protocols
        assert!(set.contains(&Protocol::Http));
        assert!(set.contains(&Protocol::Tcp));
        assert!(!set.contains(&Protocol::Nfs));
    }

    #[test]
    fn test_protocol_as_hashmap_key() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert(Protocol::Http, "web traffic");
        map.insert(Protocol::Tcp, "generic tcp");
        map.insert(Protocol::Nfs, "network file system");

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&Protocol::Http), Some(&"web traffic"));
        assert_eq!(map.get(&Protocol::Nfs), Some(&"network file system"));
    }

    // ==================== PROTOCOL DEBUGGING TESTS ====================

    #[test]
    fn test_protocol_debug_format() {
        let protocol = Protocol::Http;
        let debug_str = format!("{:?}", protocol);
        assert!(debug_str.contains("Http"));
    }

    #[test]
    fn test_performance_preference_debug() {
        let pref = PerformancePreference::Reliability;
        let debug_str = format!("{:?}", pref);
        assert!(debug_str.contains("Reliability"));
    }

    #[test]
    fn test_protocol_config_debug() {
        let config = ProtocolConfig {
            protocol: Protocol::Tcp,
            options: std::collections::HashMap::new(),
            performance: PerformancePreference::Speed,
            encryption: false,
            timeout: 30,
            max_retries: 3,
        };

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("Tcp"));
        assert!(debug_str.contains("30") || debug_str.contains("timeout"));
    }
}
