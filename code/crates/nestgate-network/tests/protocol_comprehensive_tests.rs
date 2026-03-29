// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

#![allow(
    dead_code,
    missing_docs,
    unused_imports,
    unused_variables,
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]

//! Comprehensive tests for protocol module
//!
//! Tests cover:
//! - Protocol enum variants and Display implementation
//! - PerformancePreference enum and Default
//! - ProtocolConfig creation and defaults
//! - MountRequest/MountResponse types
//! - Credentials handling
//! - MountStatus types
//! - ProtocolManager lifecycle and operations

#![allow(deprecated)]

use nestgate_network::protocol::*;
use std::collections::HashMap;
use std::path::PathBuf;

// ==================== PROTOCOL ENUM TESTS ====================

#[test]
fn test_protocol_variants() {
    // Test all protocol variants can be created
    let nfs = Protocol::Nfs;
    let smb = Protocol::Smb;
    let _ftp = Protocol::Ftp;
    let _sftp = Protocol::Sftp;
    let _http = Protocol::Http;
    let _tcp = Protocol::Tcp;

    // Verify they can be cloned
    let _nfs_clone = nfs;
    let _smb_clone = smb;

    // Verify Debug works
    let debug_str = format!("{:?}", nfs);
    assert!(debug_str.contains("Nfs"));
}

#[test]
fn test_protocol_equality() {
    assert_eq!(Protocol::Nfs, Protocol::Nfs);
    assert_ne!(Protocol::Nfs, Protocol::Smb);

    // Test copy semantics
    let proto1 = Protocol::Http;
    let proto2 = proto1;
    assert_eq!(proto1, proto2);
}

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
fn test_protocol_serialization() {
    let proto = Protocol::Nfs;
    let json = serde_json::to_string(&proto).expect("Serialization failed");
    let deserialized: Protocol = serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(proto, deserialized);
}

#[test]
fn test_protocol_hash() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Protocol::Nfs);
    set.insert(Protocol::Smb);
    set.insert(Protocol::Nfs); // Duplicate

    assert_eq!(set.len(), 2); // Only 2 unique protocols
    assert!(set.contains(&Protocol::Nfs));
    assert!(set.contains(&Protocol::Smb));
}

// ==================== PERFORMANCE PREFERENCE TESTS ====================

#[test]
fn test_performance_preference_variants() {
    let speed = PerformancePreference::Speed;
    let _reliability = PerformancePreference::Reliability;
    let _compatibility = PerformancePreference::Compatibility;
    let balanced = PerformancePreference::Balanced;

    // Verify cloning
    let _speed_clone = speed;

    // Verify Debug
    let debug_str = format!("{:?}", balanced);
    assert!(debug_str.contains("Balanced"));
}

#[test]
fn test_performance_preference_default() {
    let default_pref = PerformancePreference::default();
    assert_eq!(default_pref, PerformancePreference::Balanced);
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
fn test_performance_preference_serialization() {
    let pref = PerformancePreference::Speed;
    let json = serde_json::to_string(&pref).expect("Serialization failed");
    let deserialized: PerformancePreference =
        serde_json::from_str(&json).expect("Deserialization failed");
    assert_eq!(pref, deserialized);
}

// ==================== PROTOCOL CONFIG TESTS ====================

#[test]
fn test_protocol_config_default() {
    let config = ProtocolConfig::default();

    assert_eq!(config.protocol, Protocol::Nfs);
    assert!(config.options.is_empty());
    assert_eq!(config.performance, PerformancePreference::Balanced);
    assert!(config.encryption);
    assert_eq!(config.timeout, 30);
    assert_eq!(config.max_retries, 3);
}

#[test]
fn test_protocol_config_custom() {
    let mut options = HashMap::new();
    options.insert("version".to_string(), "4".to_string());

    let config = ProtocolConfig {
        protocol: Protocol::Smb,
        options,
        performance: PerformancePreference::Speed,
        encryption: false,
        timeout: 60,
        max_retries: 5,
    };

    assert_eq!(config.protocol, Protocol::Smb);
    assert_eq!(config.options.get("version"), Some(&"4".to_string()));
    assert_eq!(config.performance, PerformancePreference::Speed);
    assert!(!config.encryption);
    assert_eq!(config.timeout, 60);
    assert_eq!(config.max_retries, 5);
}

#[test]
fn test_protocol_config_clone() {
    let config = ProtocolConfig::default();
    let cloned = config.clone();

    assert_eq!(config.protocol, cloned.protocol);
    assert_eq!(config.timeout, cloned.timeout);
}

#[test]
fn test_protocol_config_serialization() {
    let config = ProtocolConfig::default();
    let json = serde_json::to_string(&config).expect("Serialization failed");
    let deserialized: ProtocolConfig = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(config.protocol, deserialized.protocol);
    assert_eq!(config.timeout, deserialized.timeout);
}

#[test]
fn test_protocol_config_with_options() {
    let mut config = ProtocolConfig::default();
    config
        .options
        .insert("key1".to_string(), "value1".to_string());
    config
        .options
        .insert("key2".to_string(), "value2".to_string());

    assert_eq!(config.options.len(), 2);
    assert_eq!(config.options.get("key1"), Some(&"value1".to_string()));
}

// ==================== CREDENTIALS TESTS ====================

#[test]
fn test_credentials_creation() {
    let creds = Credentials {
        username: "testuser".to_string(),
        password: "testpass".to_string(),
        domain: Some("TESTDOMAIN".to_string()),
    };

    assert_eq!(creds.username, "testuser");
    assert_eq!(creds.password, "testpass");
    assert_eq!(creds.domain, Some("TESTDOMAIN".to_string()));
}

#[test]
fn test_credentials_without_domain() {
    let creds = Credentials {
        username: "user".to_string(),
        password: "pass".to_string(),
        domain: None,
    };

    assert!(creds.domain.is_none());
}

#[test]
fn test_credentials_clone() {
    let creds = Credentials {
        username: "user".to_string(),
        password: "pass".to_string(),
        domain: Some("domain".to_string()),
    };

    let cloned = creds.clone();
    assert_eq!(creds.username, cloned.username);
    assert_eq!(creds.password, cloned.password);
}

#[test]
fn test_credentials_serialization() {
    let creds = Credentials {
        username: "user".to_string(),
        password: "pass".to_string(),
        domain: None,
    };

    let json = serde_json::to_string(&creds).expect("Serialization failed");
    let deserialized: Credentials = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(creds.username, deserialized.username);
    assert_eq!(creds.password, deserialized.password);
}

// ==================== MOUNT REQUEST TESTS ====================

#[test]
fn test_mount_request_creation() {
    let request = MountRequest {
        protocol: Protocol::Nfs,
        server: "192.168.1.100".to_string(),
        remote_path: "/export/data".to_string(),
        mount_point: PathBuf::from("/mnt/nfs"),
        credentials: None,
        options: HashMap::new(),
    };

    assert_eq!(request.protocol, Protocol::Nfs);
    assert_eq!(request.server, "192.168.1.100");
    assert_eq!(request.remote_path, "/export/data");
    assert_eq!(request.mount_point, PathBuf::from("/mnt/nfs"));
    assert!(request.credentials.is_none());
}

#[test]
fn test_mount_request_with_credentials() {
    let creds = Credentials {
        username: "admin".to_string(),
        password: "secret".to_string(),
        domain: Some("CORP".to_string()),
    };

    let request = MountRequest {
        protocol: Protocol::Smb,
        server: "fileserver".to_string(),
        remote_path: "share".to_string(),
        mount_point: PathBuf::from("/mnt/smb"),
        credentials: Some(creds),
        options: HashMap::new(),
    };

    assert!(request.credentials.is_some());
    let creds = request.credentials.unwrap();
    assert_eq!(creds.username, "admin");
}

#[test]
fn test_mount_request_with_options() {
    let mut options = HashMap::new();
    options.insert("ro".to_string(), "true".to_string());
    options.insert("nolock".to_string(), "true".to_string());

    let request = MountRequest {
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        mount_point: PathBuf::from("/mnt"),
        credentials: None,
        options,
    };

    assert_eq!(request.options.len(), 2);
    assert_eq!(request.options.get("ro"), Some(&"true".to_string()));
}

#[test]
fn test_mount_request_serialization() {
    let request = MountRequest {
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        mount_point: PathBuf::from("/mnt"),
        credentials: None,
        options: HashMap::new(),
    };

    let json = serde_json::to_string(&request).expect("Serialization failed");
    let deserialized: MountRequest = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(request.protocol, deserialized.protocol);
    assert_eq!(request.server, deserialized.server);
}

// ==================== MOUNT RESPONSE TESTS ====================

#[test]
fn test_mount_response_success() {
    let response = MountResponse {
        mount_id: "mount-123".to_string(),
        success: true,
        message: "Mount successful".to_string(),
        mount_point: PathBuf::from("/mnt/nfs"),
    };

    assert_eq!(response.mount_id, "mount-123");
    assert!(response.success);
    assert_eq!(response.message, "Mount successful");
}

#[test]
fn test_mount_response_failure() {
    let response = MountResponse {
        mount_id: "".to_string(),
        success: false,
        message: "Connection refused".to_string(),
        mount_point: PathBuf::from("/mnt/nfs"),
    };

    assert!(!response.success);
    assert!(response.message.contains("refused"));
}

#[test]
fn test_mount_response_clone() {
    let response = MountResponse {
        mount_id: "id".to_string(),
        success: true,
        message: "ok".to_string(),
        mount_point: PathBuf::from("/mnt"),
    };

    let cloned = response.clone();
    assert_eq!(response.mount_id, cloned.mount_id);
}

#[test]
fn test_mount_response_serialization() {
    let response = MountResponse {
        mount_id: "id".to_string(),
        success: true,
        message: "ok".to_string(),
        mount_point: PathBuf::from("/mnt"),
    };

    let json = serde_json::to_string(&response).expect("Serialization failed");
    let deserialized: MountResponse = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(response.mount_id, deserialized.mount_id);
    assert_eq!(response.success, deserialized.success);
}

// ==================== MOUNT STATUS TESTS ====================

#[test]
fn test_mount_status_creation() {
    let status = MountStatus {
        mount_id: "mount-123".to_string(),
        mounted: true,
        mount_point: PathBuf::from("/mnt/nfs"),
        protocol: Protocol::Nfs,
        server: "192.168.1.100".to_string(),
        remote_path: "/export".to_string(),
        last_access: None,
        error: None,
    };

    assert_eq!(status.mount_id, "mount-123");
    assert!(status.mounted);
    assert_eq!(status.protocol, Protocol::Nfs);
    assert!(status.error.is_none());
}

#[test]
fn test_mount_status_with_error() {
    let status = MountStatus {
        mount_id: "mount-failed".to_string(),
        mounted: false,
        mount_point: PathBuf::from("/mnt/nfs"),
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        last_access: None,
        error: Some("Connection timeout".to_string()),
    };

    assert!(!status.mounted);
    assert!(status.error.is_some());
    assert_eq!(status.error.unwrap(), "Connection timeout");
}

#[test]
fn test_mount_status_with_last_access() {
    let now = chrono::Utc::now();
    let status = MountStatus {
        mount_id: "mount-123".to_string(),
        mounted: true,
        mount_point: PathBuf::from("/mnt"),
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        last_access: Some(now),
        error: None,
    };

    assert!(status.last_access.is_some());
}

#[test]
fn test_mount_status_serialization() {
    let status = MountStatus {
        mount_id: "mount-123".to_string(),
        mounted: true,
        mount_point: PathBuf::from("/mnt"),
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        last_access: None,
        error: None,
    };

    let json = serde_json::to_string(&status).expect("Serialization failed");
    let deserialized: MountStatus = serde_json::from_str(&json).expect("Deserialization failed");

    assert_eq!(status.mount_id, deserialized.mount_id);
    assert_eq!(status.mounted, deserialized.mounted);
}

// ==================== PROTOCOL MANAGER TESTS ====================

#[test]
fn test_protocol_manager_new() {
    let manager = ProtocolManager::new();
    assert!(manager.supported_protocols().is_empty());
}

#[test]
fn test_protocol_manager_default() {
    let manager = ProtocolManager::default();
    assert!(manager.supported_protocols().is_empty());
}

#[test]
fn test_protocol_manager_register_protocol() {
    let mut manager = ProtocolManager::new();

    manager.register_protocol(Protocol::Nfs);
    manager.register_protocol(Protocol::Smb);

    let protocols = manager.supported_protocols();
    assert_eq!(protocols.len(), 2);
    assert!(protocols.contains(&Protocol::Nfs));
    assert!(protocols.contains(&Protocol::Smb));
}

#[test]
fn test_protocol_manager_register_duplicate() {
    let mut manager = ProtocolManager::new();

    manager.register_protocol(Protocol::Nfs);
    manager.register_protocol(Protocol::Nfs); // Duplicate

    let protocols = manager.supported_protocols();
    assert_eq!(protocols.len(), 1); // Only one unique protocol
}

#[test]
fn test_protocol_manager_mount_unsupported_protocol() {
    let manager = ProtocolManager::new();
    // Don't register any protocols

    let request = MountRequest {
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        mount_point: PathBuf::from("/mnt"),
        credentials: None,
        options: HashMap::new(),
    };

    let result = manager.mount(request);
    assert!(result.is_err());
}

#[test]
fn test_protocol_manager_mount_supported_protocol() {
    let mut manager = ProtocolManager::new();
    manager.register_protocol(Protocol::Nfs);

    let request = MountRequest {
        protocol: Protocol::Nfs,
        server: "server".to_string(),
        remote_path: "/data".to_string(),
        mount_point: PathBuf::from("/mnt/nfs"),
        credentials: None,
        options: HashMap::new(),
    };

    let result = manager.mount(request);
    assert!(result.is_ok());

    let response = result.unwrap();
    assert!(response.success);
    assert!(!response.mount_id.is_empty());
    assert_eq!(response.mount_point, PathBuf::from("/mnt/nfs"));
}

#[test]
fn test_protocol_manager_unmount_unsupported() {
    let manager = ProtocolManager::new();

    let result = manager.unmount(Protocol::Nfs, "mount-123");
    assert!(result.is_err());
}

#[test]
fn test_protocol_manager_unmount_supported() {
    let mut manager = ProtocolManager::new();
    manager.register_protocol(Protocol::Nfs);

    let result = manager.unmount(Protocol::Nfs, "mount-123");
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_protocol_manager_get_status_unsupported() {
    let manager = ProtocolManager::new();

    let result = manager.get_status(Protocol::Nfs, "mount-123");
    assert!(result.is_err());
}

#[test]
fn test_protocol_manager_get_status_supported() {
    let mut manager = ProtocolManager::new();
    manager.register_protocol(Protocol::Nfs);

    let result = manager.get_status(Protocol::Nfs, "mount-123");
    assert!(result.is_ok());

    let status = result.unwrap();
    assert_eq!(status.mount_id, "mount-123");
    assert!(status.mounted);
    assert_eq!(status.protocol, Protocol::Nfs);
}

#[test]
fn test_protocol_manager_debug() {
    let manager = ProtocolManager::new();
    let debug_str = format!("{:?}", manager);
    assert!(debug_str.contains("ProtocolManager"));
}

#[test]
fn test_protocol_manager_multiple_protocols() {
    let mut manager = ProtocolManager::new();

    manager.register_protocol(Protocol::Nfs);
    manager.register_protocol(Protocol::Smb);
    manager.register_protocol(Protocol::Http);

    let protocols = manager.supported_protocols();
    assert_eq!(protocols.len(), 3);
}

// ==================== INTEGRATION TESTS ====================

#[test]
fn test_full_mount_workflow() {
    let mut manager = ProtocolManager::new();
    manager.register_protocol(Protocol::Nfs);

    // Create mount request
    let request = MountRequest {
        protocol: Protocol::Nfs,
        server: "192.168.1.100".to_string(),
        remote_path: "/export/data".to_string(),
        mount_point: PathBuf::from("/mnt/nfs"),
        credentials: None,
        options: HashMap::new(),
    };

    // Mount
    let mount_result = manager.mount(request);
    assert!(mount_result.is_ok());
    let response = mount_result.unwrap();
    let mount_id = response.mount_id.clone();

    // Get status
    let status_result = manager.get_status(Protocol::Nfs, &mount_id);
    assert!(status_result.is_ok());
    let status = status_result.unwrap();
    assert!(status.mounted);

    // Unmount
    let unmount_result = manager.unmount(Protocol::Nfs, &mount_id);
    assert!(unmount_result.is_ok());
}

#[test]
fn test_protocol_config_with_all_options() {
    let mut options = HashMap::new();
    options.insert("version".to_string(), "4.1".to_string());
    options.insert("ro".to_string(), "true".to_string());
    options.insert("nolock".to_string(), "true".to_string());

    let config = ProtocolConfig {
        protocol: Protocol::Nfs,
        options,
        performance: PerformancePreference::Speed,
        encryption: true,
        timeout: 120,
        max_retries: 10,
    };

    assert_eq!(config.options.len(), 3);
    assert_eq!(config.timeout, 120);
    assert_eq!(config.max_retries, 10);
}
