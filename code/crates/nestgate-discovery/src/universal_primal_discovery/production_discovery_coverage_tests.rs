// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Coverage tests for [`super::production_discovery`]: config construction, defaults, discovery
//! methods, and standalone helpers. Process environment is scoped with `temp_env::with_vars`.

use super::production_discovery::*;
use nestgate_config::config::canonical_primary::NestGateCanonicalConfig;
use nestgate_config::constants::canonical_defaults::network;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

fn canonical_default() -> NestGateCanonicalConfig {
    NestGateCanonicalConfig::default()
}

// ==================== default_port_for_service / default_bind_for_service ====================

#[test]
fn default_port_for_service_metrics_uses_metrics_default() {
    assert_eq!(
        ServiceDiscoveryConfig::default_port_for_service("metrics"),
        network::DEFAULT_METRICS_PORT
    );
}

#[test]
fn default_port_for_service_internal_family_uses_internal_default() {
    for name in [
        "health",
        "admin",
        "websocket",
        "network",
        "storage",
        "zfs",
        "mcp",
        "automation",
    ] {
        assert_eq!(
            ServiceDiscoveryConfig::default_port_for_service(name),
            network::DEFAULT_INTERNAL_PORT,
            "name={name}"
        );
    }
}

#[test]
fn default_port_for_service_unknown_uses_api_default() {
    assert_eq!(
        ServiceDiscoveryConfig::default_port_for_service("unknown_service"),
        network::DEFAULT_API_PORT
    );
}

#[test]
fn default_bind_for_service_api_and_web_are_unspecified() {
    assert_eq!(
        ServiceDiscoveryConfig::default_bind_for_service("api"),
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    );
    assert_eq!(
        ServiceDiscoveryConfig::default_bind_for_service("web"),
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    );
}

#[test]
fn default_bind_for_service_internal_uses_loopback() {
    let b = ServiceDiscoveryConfig::default_bind_for_service("metrics");
    assert_eq!(b, IpAddr::V4(Ipv4Addr::LOCALHOST));
}

// ==================== ServiceDiscoveryConfig::from_environment ====================

#[test]
fn from_environment_applies_canonical_network_api_to_api_service() {
    let config = canonical_default();
    let expected_port = config.network.api.port;

    let discovered = match ServiceDiscoveryConfig::from_environment(&config) {
        Ok(c) => c,
        Err(e) => panic!("from_environment: {e:?}"),
    };

    let Some(api) = discovered.services.get("api") else {
        panic!("api service missing");
    };
    assert_eq!(api.port, expected_port);
    assert_eq!(api.bind_address, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
}

#[test]
fn from_environment_reads_web_host_port_bind_from_process_env() {
    // `discover_services_from_config` overwrites only `api` from canonical config; env vars on
    // `WEB_*` (and other non-api services) are preserved.
    temp_env::with_vars(
        [
            ("WEB_HOST", Some("10.0.0.1")),
            ("WEB_PORT", Some("9443")),
            ("WEB_BIND", Some("0.0.0.0")),
        ],
        || {
            let config = canonical_default();
            let discovered = match ServiceDiscoveryConfig::from_environment(&config) {
                Ok(c) => c,
                Err(e) => panic!("from_environment: {e:?}"),
            };
            let web = discovered.services.get("web").expect("web present");
            assert_eq!(web.host, "10.0.0.1");
            assert_eq!(web.port, 9443);
            assert_eq!(web.bind_address, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
        },
    );
}

#[test]
fn from_environment_invalid_bind_falls_back_to_default_bind() {
    temp_env::with_vars([("METRICS_BIND", Some("not-a-valid-ip"))], || {
        let config = canonical_default();
        let discovered = match ServiceDiscoveryConfig::from_environment(&config) {
            Ok(c) => c,
            Err(e) => panic!("from_environment: {e:?}"),
        };
        let metrics = discovered.services.get("metrics").expect("metrics present");
        assert_eq!(
            metrics.bind_address,
            ServiceDiscoveryConfig::default_bind_for_service("metrics")
        );
    });
}

#[test]
fn from_environment_merges_nestgate_limit_and_timeout_env() {
    temp_env::with_vars(
        [
            ("NESTGATE_MAX_CONNECTIONS", Some("2048")),
            ("NESTGATE_CONNECT_TIMEOUT", Some("7")),
        ],
        || {
            let config = canonical_default();
            let discovered = match ServiceDiscoveryConfig::from_environment(&config) {
                Ok(c) => c,
                Err(e) => panic!("from_environment: {e:?}"),
            };
            assert_eq!(
                discovered.resource_limits.get("max_connections"),
                Some(&2048_usize)
            );
            assert_eq!(
                discovered.operation_timeouts.get("connect"),
                Some(&Duration::from_secs(7))
            );
        },
    );
}

// ==================== ProductionServiceDiscovery ====================

#[test]
fn production_discovery_discover_port_bind_endpoint_for_configured_web_service() {
    temp_env::with_vars(
        [
            ("WEB_HOST", Some("192.168.1.10")),
            ("WEB_PORT", Some("4000")),
            ("WEB_BIND", Some("127.0.0.1")),
        ],
        || {
            let config = canonical_default();
            let discovery = match ProductionServiceDiscovery::new(&config) {
                Ok(d) => d,
                Err(e) => panic!("new: {e:?}"),
            };

            let port = match discovery.discover_port("web") {
                Ok(p) => p,
                Err(e) => panic!("discover_port: {e:?}"),
            };
            assert_eq!(port, 4000);

            let bind = match discovery.discover_bind_address("web") {
                Ok(b) => b,
                Err(e) => panic!("discover_bind_address: {e:?}"),
            };
            assert_eq!(bind, IpAddr::V4(Ipv4Addr::LOCALHOST));

            let ep = match discovery.discover_endpoint("web") {
                Ok(e) => e,
                Err(e) => panic!("discover_endpoint: {e:?}"),
            };
            assert_eq!(ep, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 4000));
        },
    );
}

#[test]
fn production_discovery_unknown_service_falls_back_to_defaults() {
    let config = canonical_default();
    let discovery = match ProductionServiceDiscovery::new(&config) {
        Ok(d) => d,
        Err(e) => panic!("new: {e:?}"),
    };

    let port = match discovery.discover_port("no_such_service") {
        Ok(p) => p,
        Err(e) => panic!("discover_port: {e:?}"),
    };
    assert_eq!(port, discovery.config().defaults.default_port);

    let bind = match discovery.discover_bind_address("no_such_service") {
        Ok(b) => b,
        Err(e) => panic!("discover_bind_address: {e:?}"),
    };
    assert_eq!(bind, discovery.config().defaults.default_bind);

    let ep = match discovery.discover_endpoint("no_such_service") {
        Ok(e) => e,
        Err(e) => panic!("discover_endpoint: {e:?}"),
    };
    assert_eq!(
        ep,
        SocketAddr::new(
            discovery.config().defaults.default_bind,
            discovery.config().defaults.default_port
        )
    );
}

#[test]
fn production_discovery_discover_limit_branches() {
    let config = canonical_default();
    let discovery = match ProductionServiceDiscovery::new(&config) {
        Ok(d) => d,
        Err(e) => panic!("new: {e:?}"),
    };

    assert_eq!(
        match discovery.discover_limit("connections") {
            Ok(v) => v,
            Err(e) => panic!("{e:?}"),
        },
        1000
    );
    assert_eq!(
        match discovery.discover_limit("requests_per_second") {
            Ok(v) => v,
            Err(e) => panic!("{e:?}"),
        },
        100
    );
    assert_eq!(
        match discovery.discover_limit("totally_unknown_limit") {
            Ok(v) => v,
            Err(e) => panic!("{e:?}"),
        },
        discovery.config().defaults.default_limit
    );
}

#[test]
fn production_discovery_discover_timeout_branches() {
    let config = canonical_default();
    let discovery = match ProductionServiceDiscovery::new(&config) {
        Ok(d) => d,
        Err(e) => panic!("new: {e:?}"),
    };

    assert_eq!(
        match discovery.discover_timeout("connect") {
            Ok(v) => v,
            Err(e) => panic!("{e:?}"),
        },
        Duration::from_secs(10)
    );
    assert_eq!(
        match discovery.discover_timeout("health_check") {
            Ok(v) => v,
            Err(e) => panic!("{e:?}"),
        },
        Duration::from_secs(5)
    );
    assert_eq!(
        match discovery.discover_timeout("unknown_op") {
            Ok(v) => v,
            Err(e) => panic!("{e:?}"),
        },
        discovery.config().defaults.default_timeout
    );
}

// ==================== Standalone functions ====================

#[test]
fn create_production_discovery_and_standalone_discover_functions_agree() {
    temp_env::with_vars(
        [("WEB_PORT", Some("7777")), ("WEB_BIND", Some("127.0.0.1"))],
        || {
            let config = canonical_default();

            let _pd = match create_production_discovery(&config) {
                Ok(d) => d,
                Err(e) => panic!("create_production_discovery: {e:?}"),
            };

            let bind = match discover_bind_address_standalone(&config, "web") {
                Ok(b) => b,
                Err(e) => panic!("{e:?}"),
            };
            assert_eq!(bind, IpAddr::V4(Ipv4Addr::LOCALHOST));

            let port = match discover_port_standalone(&config, "web") {
                Ok(p) => p,
                Err(e) => panic!("{e:?}"),
            };
            assert_eq!(port, 7777);

            let ep = match discover_endpoint_standalone(&config, "web") {
                Ok(e) => e,
                Err(e) => panic!("{e:?}"),
            };
            assert_eq!(ep, SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 7777));

            let lim = match discover_limit_standalone(&config, "memory_mb") {
                Ok(v) => v,
                Err(e) => panic!("{e:?}"),
            };
            assert_eq!(lim, 512);

            let to = match discover_timeout_standalone(&config, "discovery") {
                Ok(t) => t,
                Err(e) => panic!("{e:?}"),
            };
            assert_eq!(to, Duration::from_secs(15));
        },
    );
}
