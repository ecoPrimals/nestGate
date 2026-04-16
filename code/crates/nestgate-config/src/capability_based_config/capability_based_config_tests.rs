// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use nestgate_types::{EnvSource, MapEnv};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn test_initialize_with_map_env() {
    let env = Arc::new(MapEnv::from([("NESTGATE_DISCOVERY_ENABLED", "false")]));
    let config = CapabilityConfig::initialize_with_env(env).await.unwrap();
    assert!(!config.discovery_config.enabled);
}

#[tokio::test]
async fn test_capability_config_initialization() {
    let config = CapabilityConfig::initialize().await;
    assert!(config.is_ok());
}

#[tokio::test]
async fn test_self_knowledge_has_identity() {
    let config = CapabilityConfig::initialize().await.unwrap();
    let knowledge = config.self_knowledge();

    assert_eq!(knowledge.identity.primal_type, "nestgate");
    assert!(!knowledge.identity.id.is_empty());
}

#[tokio::test]
async fn test_discovery_config_default() {
    let config = DiscoveryConfig::default();
    assert!(!config.methods.is_empty());
}

#[test]
fn test_service_endpoint_url() {
    let endpoint = ServiceEndpoint {
        protocol: "http".to_string(),
        address: "localhost".to_string(),
        port: 8080,
        path: Some("/api/v1".to_string()),
    };

    assert_eq!(endpoint.url(), "http://localhost:8080/api/v1");
}

#[test]
fn test_service_endpoint_url_no_path() {
    let endpoint = ServiceEndpoint {
        protocol: "https".to_string(),
        address: "example.com".to_string(),
        port: 443,
        path: None,
    };

    assert_eq!(endpoint.url(), "https://example.com:443");
}

#[tokio::test]
async fn test_announce_when_discovery_disabled() {
    let mut config = CapabilityConfig::initialize().await.unwrap();
    config.discovery_config.enabled = false;

    // Should succeed but do nothing
    let result = config.announce();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_discover_capability_not_found() {
    let config = CapabilityConfig::initialize().await.unwrap();

    // With discovery disabled and no environment, should fail clearly
    let result = config.discover_capability("nonexistent").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn initialize_with_env_rejects_invalid_api_port() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_API_PORT", "not-a-u16"),
    ]));
    let err = CapabilityConfig::initialize_with_env(env).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn initialize_with_env_rejects_empty_api_host() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_API_HOST", ""),
    ]));
    let err = CapabilityConfig::initialize_with_env(env).await;
    assert!(err.is_err());
}

#[tokio::test]
async fn nestgate_capabilities_env_splits_and_trims() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_CAPABILITIES", " alpha , beta "),
    ]));
    let config = CapabilityConfig::initialize_with_env(env).await.unwrap();
    let caps = &config.self_knowledge().capabilities;
    assert_eq!(caps, &vec!["alpha".to_string(), "beta".to_string()]);
}

#[tokio::test]
async fn discover_from_environment_then_cache_and_second_call_hits_cache() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_API_HOST", "127.0.0.1"),
        ("NESTGATE_API_PORT", "9090"),
    ]));
    let config = CapabilityConfig::initialize_with_env(env).await.unwrap();

    let first = config.discover_capability("api").await.unwrap();
    assert_eq!(first.address, "127.0.0.1");
    assert_eq!(first.port, 9090);

    let second = config.discover_capability("api").await.unwrap();
    assert_eq!(second.address, first.address);

    let map = config.discovered_capabilities().await;
    assert!(map.contains_key("api"));
}

#[tokio::test]
async fn discover_capability_invalid_peer_port_in_env_errors() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_FOO_HOST", "localhost"),
        ("NESTGATE_FOO_PORT", "bad"),
    ]));
    let config = CapabilityConfig::initialize_with_env(env).await.unwrap();
    let err = config.discover_capability("foo").await;
    assert!(err.is_err());
}

#[tokio::test]
async fn discover_capability_runs_stub_discovery_methods_when_enabled() {
    let env = Arc::new(MapEnv::new());
    let mut config = CapabilityConfig::initialize_with_env(Arc::new(env))
        .await
        .unwrap();
    config.discovery_config.enabled = true;
    config.discovery_config.methods = vec![
        DiscoveryMethod::Environment,
        DiscoveryMethod::MDns,
        DiscoveryMethod::DnsSd,
        DiscoveryMethod::Consul,
        DiscoveryMethod::Kubernetes,
    ];

    let err = config.discover_capability("missing").await;
    assert!(err.is_err());
}

#[tokio::test]
async fn get_port_reads_literal_env_var() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_METRICS_PORT", "12345"),
    ]));
    let config = CapabilityConfig::initialize_with_env(Arc::new(env))
        .await
        .unwrap();
    let port = config.get_port("NESTGATE_METRICS_PORT").await.unwrap();
    assert_eq!(port, 12345);
}

#[tokio::test]
async fn get_port_invalid_env_value_errors() {
    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_METRICS_PORT", "oops"),
    ]));
    let config = CapabilityConfig::initialize_with_env(Arc::new(env))
        .await
        .unwrap();
    let err = config.get_port("NESTGATE_METRICS_PORT").await;
    assert!(err.is_err());
}

/// Hides `NESTGATE_API_PORT` after discovery so `get_port` exercises the
/// `discover_capability` path (cache hit) instead of parsing the env var directly.
struct HideApiPortAfterDiscover {
    map: Arc<Mutex<MapEnv>>,
    hide_api_port: Arc<AtomicBool>,
}

impl EnvSource for HideApiPortAfterDiscover {
    fn get(&self, key: &str) -> Option<String> {
        if self.hide_api_port.load(Ordering::SeqCst) && key == "NESTGATE_API_PORT" {
            return None;
        }
        self.map.lock().expect("lock map").get(key)
    }

    fn vars(&self) -> Vec<(String, String)> {
        self.map.lock().expect("lock map").vars()
    }
}

#[tokio::test]
async fn get_port_uses_cached_discovery_when_port_env_hidden() {
    let map = MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_API_HOST", "10.0.0.1"),
        ("NESTGATE_API_PORT", "6000"),
    ]);
    let hide = Arc::new(AtomicBool::new(false));
    let env = Arc::new(HideApiPortAfterDiscover {
        map: Arc::new(Mutex::new(map)),
        hide_api_port: hide.clone(),
    });
    let config = CapabilityConfig::initialize_with_env(env.clone() as Arc<dyn EnvSource>)
        .await
        .unwrap();
    config.discover_capability("api").await.unwrap();
    hide.store(true, Ordering::SeqCst);
    let port = config.get_port("NESTGATE_API_PORT").await.unwrap();
    assert_eq!(port, 6000);
}

#[tokio::test]
async fn get_port_bails_when_unconfigured() {
    let env = Arc::new(MapEnv::from([("NESTGATE_DISCOVERY_ENABLED", "false")]));
    let config = CapabilityConfig::initialize_with_env(Arc::new(env))
        .await
        .unwrap();
    let err = config.get_port("NESTGATE_STORAGE_PORT").await;
    assert!(err.is_err());
}

#[tokio::test]
async fn get_bind_address_parses_or_uses_default() {
    let env = Arc::new(MapEnv::from([("NESTGATE_DISCOVERY_ENABLED", "false")]));
    let config = CapabilityConfig::initialize_with_env(env).await.unwrap();
    let addr = config
        .get_bind_address("NESTGATE_BIND_ADDR", "127.0.0.1:0")
        .unwrap();
    assert_eq!(addr.port(), 0);

    let env = Arc::new(MapEnv::from([
        ("NESTGATE_DISCOVERY_ENABLED", "false"),
        ("NESTGATE_BIND_ADDR", "not-a-socket"),
    ]));
    let config = CapabilityConfig::initialize_with_env(env).await.unwrap();
    assert!(
        config
            .get_bind_address("NESTGATE_BIND_ADDR", "127.0.0.1:0")
            .is_err()
    );
}

#[tokio::test]
async fn announce_with_discovery_enabled_hits_all_methods() {
    let env = Arc::new(MapEnv::from([("NESTGATE_DISCOVERY_ENABLED", "true")]));
    let mut config = CapabilityConfig::initialize_with_env(Arc::new(env))
        .await
        .unwrap();
    config.discovery_config.methods = vec![
        DiscoveryMethod::Environment,
        DiscoveryMethod::MDns,
        DiscoveryMethod::DnsSd,
        DiscoveryMethod::Consul,
        DiscoveryMethod::Kubernetes,
    ];
    assert!(config.announce().is_ok());
}

#[test]
fn discovery_config_from_env_source_respects_false() {
    let env = MapEnv::from([("NESTGATE_DISCOVERY_ENABLED", "false")]);
    let cfg = DiscoveryConfig::from_env_source(&env);
    assert!(!cfg.enabled);
}
