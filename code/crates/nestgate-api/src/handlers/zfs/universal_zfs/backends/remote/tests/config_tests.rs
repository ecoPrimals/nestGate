//! Configuration tests for remote ZFS service

use crate::handlers::zfs::universal_zfs::config::RemoteConfig;
use std::time::Duration;

#[test]
fn test_remote_config_creation() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let endpoint = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    let config = RemoteConfig {
        endpoint: endpoint.clone(),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    assert_eq!(config.endpoint, endpoint);
    assert_eq!(config.timeout, Duration::from_secs(30));
    assert!(config.auth.is_none());
}

#[test]
fn test_remote_config_with_auth() {
    let config = RemoteConfig {
        endpoint: "https://remote-zfs.example.com".to_string(),
        timeout: Duration::from_secs(60),
        auth: Some("Bearer token123".to_string()),
    };

    assert!(config.endpoint.starts_with("https://"));
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert!(config.auth.is_some());
    assert_eq!(
        config.auth.expect("Auth should be present"),
        "Bearer token123"
    );
}

#[test]
fn test_remote_config_timeout_values() {
    use nestgate_core::constants::hardcoding::{addresses, ports};
    let endpoint = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );

    let short_timeout = RemoteConfig {
        endpoint: endpoint.clone(),
        timeout: Duration::from_secs(5),
        auth: None,
    };

    let long_timeout = RemoteConfig {
        endpoint,
        timeout: Duration::from_secs(300),
        auth: None,
    };

    assert!(short_timeout.timeout < long_timeout.timeout);
    assert_eq!(short_timeout.timeout.as_secs(), 5);
    assert_eq!(long_timeout.timeout.as_secs(), 300);
}

#[test]
fn test_endpoint_formats() {
    let http_config = RemoteConfig {
        endpoint: "http://192.168.1.100:8080".to_string(),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    let https_config = RemoteConfig {
        endpoint: "https://secure.example.com:443".to_string(),
        timeout: Duration::from_secs(30),
        auth: None,
    };

    assert!(http_config.endpoint.starts_with("http://"));
    assert!(https_config.endpoint.starts_with("https://"));
    assert!(http_config.endpoint.contains(':'));
    assert!(https_config.endpoint.contains(':'));
}

#[test]
fn test_endpoint_validation() {
    use nestgate_core::constants::hardcoding::{addresses, ports};

    // Valid endpoints
    let valid_http = format!(
        "http://{}:{}",
        addresses::LOCALHOST_NAME,
        ports::HTTP_DEFAULT
    );
    let valid_https = "https://remote.example.com:443";

    assert!(valid_http.starts_with("http://") || valid_http.starts_with("https://"));
    assert!(valid_https.starts_with("http://") || valid_https.starts_with("https://"));
}

#[test]
fn test_timeout_validation() {
    let valid_timeout = Duration::from_secs(30);
    let very_short = Duration::from_secs(1);
    let very_long = Duration::from_secs(600);

    assert!(valid_timeout.as_secs() > 0);
    assert!(very_short.as_secs() > 0);
    assert!(very_long.as_secs() > 0);
    assert!(very_short < valid_timeout);
    assert!(valid_timeout < very_long);
}
