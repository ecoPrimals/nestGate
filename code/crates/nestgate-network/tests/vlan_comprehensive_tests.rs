//! Comprehensive tests for unified_network_extensions::vlan module
//!
//! Tests cover:
//! - NetworkVlanSettings struct creation
//! - Default implementation
//! - Serialization/deserialization
//! - Field validation
//! - Debug implementation

use nestgate_network::unified_network_extensions::vlan::*;

// ==================== STRUCT CREATION TESTS ====================

#[test]
fn test_network_vlan_settings_creation() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 100,
    };

    assert!(settings.enabled);
    assert_eq!(settings.default_vlan_id, 100);
}

#[test]
fn test_network_vlan_settings_disabled() {
    let settings = NetworkVlanSettings {
        enabled: false,
        default_vlan_id: 1,
    };

    assert!(!settings.enabled);
    assert_eq!(settings.default_vlan_id, 1);
}

#[test]
fn test_network_vlan_settings_max_vlan_id() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 4094, // Max standard VLAN ID
    };

    assert_eq!(settings.default_vlan_id, 4094);
}

#[test]
fn test_network_vlan_settings_min_vlan_id() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 1, // Min VLAN ID
    };

    assert_eq!(settings.default_vlan_id, 1);
}

// ==================== DEFAULT TESTS ====================

#[test]
fn test_network_vlan_settings_default() {
    let settings = NetworkVlanSettings::default();

    assert!(!settings.enabled);
    assert_eq!(settings.default_vlan_id, 1);
}

#[test]
fn test_default_vlan_is_standard() {
    let settings = NetworkVlanSettings::default();

    // Default VLAN ID should be 1 (standard default)
    assert_eq!(settings.default_vlan_id, 1);
}

#[test]
fn test_default_disabled() {
    let settings = NetworkVlanSettings::default();

    // VLAN should be disabled by default for safety
    assert!(!settings.enabled);
}

// ==================== CLONE TESTS ====================

#[test]
fn test_network_vlan_settings_clone() {
    let original = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 42,
    };

    let cloned = original.clone();

    assert_eq!(original.enabled, cloned.enabled);
    assert_eq!(original.default_vlan_id, cloned.default_vlan_id);
}

#[test]
fn test_clone_independence() {
    let original = NetworkVlanSettings::default();
    let mut cloned = original.clone();

    cloned.enabled = true;
    cloned.default_vlan_id = 100;

    // Original should be unchanged
    assert!(!original.enabled);
    assert_eq!(original.default_vlan_id, 1);
}

// ==================== DEBUG TESTS ====================

#[test]
fn test_network_vlan_settings_debug() {
    let settings = NetworkVlanSettings::default();
    let debug_str = format!("{:?}", settings);

    assert!(debug_str.contains("NetworkVlanSettings"));
    assert!(debug_str.contains("enabled"));
    assert!(debug_str.contains("default_vlan_id"));
}

#[test]
fn test_debug_shows_values() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 123,
    };
    let debug_str = format!("{:?}", settings);

    assert!(debug_str.contains("true"));
    assert!(debug_str.contains("123"));
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_network_vlan_settings_serialization() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 50,
    };

    let json = serde_json::to_string(&settings).expect("Serialization failed");

    assert!(json.contains("enabled"));
    assert!(json.contains("default_vlan_id"));
    assert!(json.contains("50"));
}

#[test]
fn test_network_vlan_settings_deserialization() {
    let json = r#"{"enabled":true,"default_vlan_id":75}"#;

    let settings: NetworkVlanSettings = serde_json::from_str(json).expect("Deserialization failed");

    assert!(settings.enabled);
    assert_eq!(settings.default_vlan_id, 75);
}

#[test]
fn test_serialization_roundtrip() {
    let original = NetworkVlanSettings {
        enabled: false,
        default_vlan_id: 200,
    };

    let json = serde_json::to_string(&original).expect("Serialization failed");
    let recovered: NetworkVlanSettings =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(original.enabled, recovered.enabled);
    assert_eq!(original.default_vlan_id, recovered.default_vlan_id);
}

#[test]
fn test_serialization_default() {
    let settings = NetworkVlanSettings::default();
    let json = serde_json::to_string(&settings).expect("Serialization failed");

    assert!(json.contains("false"));
    assert!(json.contains("1"));
}

// ==================== VLAN ID RANGE TESTS ====================

#[test]
fn test_vlan_id_zero() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 0,
    };

    // Zero is technically reserved but type allows it
    assert_eq!(settings.default_vlan_id, 0);
}

#[test]
fn test_vlan_id_standard_range() {
    let valid_ids = vec![1, 100, 500, 1000, 2000, 3000, 4094];

    for id in valid_ids {
        let settings = NetworkVlanSettings {
            enabled: true,
            default_vlan_id: id,
        };
        assert_eq!(settings.default_vlan_id, id);
    }
}

#[test]
fn test_vlan_id_max_u16() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: u16::MAX,
    };

    // Type allows full u16 range even if not standard
    assert_eq!(settings.default_vlan_id, u16::MAX);
}

// ==================== ENABLED FLAG TESTS ====================

#[test]
fn test_enabled_true() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 1,
    };

    assert!(settings.enabled);
}

#[test]
fn test_enabled_false() {
    let settings = NetworkVlanSettings {
        enabled: false,
        default_vlan_id: 1,
    };

    assert!(!settings.enabled);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_create_modify_serialize() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 42,
    };

    let json = serde_json::to_string(&settings).expect("Serialization failed");
    let recovered: NetworkVlanSettings =
        serde_json::from_str(&json).expect("Deserialization failed");

    assert!(recovered.enabled);
    assert_eq!(recovered.default_vlan_id, 42);
}

#[test]
fn test_multiple_instances() {
    let settings1 = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 10,
    };

    let settings2 = NetworkVlanSettings {
        enabled: false,
        default_vlan_id: 20,
    };

    let settings3 = NetworkVlanSettings::default();

    assert!(settings1.enabled);
    assert!(!settings2.enabled);
    assert!(!settings3.enabled);

    assert_eq!(settings1.default_vlan_id, 10);
    assert_eq!(settings2.default_vlan_id, 20);
    assert_eq!(settings3.default_vlan_id, 1);
}

#[test]
fn test_json_with_whitespace() {
    let json = r#"
        {
            "enabled": true,
            "default_vlan_id": 999
        }
    "#;

    let settings: NetworkVlanSettings = serde_json::from_str(json).expect("Deserialization failed");

    assert!(settings.enabled);
    assert_eq!(settings.default_vlan_id, 999);
}

#[test]
fn test_pretty_json_serialization() {
    let settings = NetworkVlanSettings {
        enabled: true,
        default_vlan_id: 123,
    };

    let json = serde_json::to_string_pretty(&settings).expect("Serialization failed");

    assert!(json.contains("enabled"));
    assert!(json.contains("default_vlan_id"));
}
