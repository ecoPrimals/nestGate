// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

#![allow(
    unused,
    dead_code,
    deprecated,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    clippy::cargo
)]

//! Comprehensive tests for the new capability-based discovery system

use anyhow::Result;

#[tokio::test]
async fn test_capability_config_initialization_success() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config = CapabilityConfig::initialize().await?;
    let knowledge = config.self_knowledge();

    assert_eq!(knowledge.identity.primal_type, "nestgate");
    assert!(!knowledge.identity.id.is_empty());
    assert!(!knowledge.identity.version.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_capability_config_has_endpoints() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config = CapabilityConfig::initialize().await?;
    let knowledge = config.self_knowledge();

    assert!(
        !knowledge.endpoints.is_empty(),
        "Should have at least one endpoint"
    );

    for endpoint in &knowledge.endpoints {
        assert!(!endpoint.protocol.is_empty());
        assert!(!endpoint.address.is_empty());
        // Port 0 is valid — means ephemeral (OS-assigned at bind time)
    }

    Ok(())
}

#[tokio::test]
async fn test_capability_config_has_capabilities() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config = CapabilityConfig::initialize().await?;
    let knowledge = config.self_knowledge();

    assert!(
        !knowledge.capabilities.is_empty(),
        "Should have at least one capability"
    );
    assert!(
        knowledge.capabilities.contains(&"storage".to_string()),
        "Should include storage capability"
    );

    Ok(())
}

#[tokio::test]
async fn test_discovery_from_environment_variables() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let orig_host = std::env::var("NESTGATE_TESTSERVICE_HOST").ok();
    let orig_port = std::env::var("NESTGATE_TESTSERVICE_PORT").ok();
    nestgate_core::env_process::set_var("NESTGATE_TESTSERVICE_HOST", "localhost");
    nestgate_core::env_process::set_var("NESTGATE_TESTSERVICE_PORT", "9999");

    let config = CapabilityConfig::initialize().await?;

    let result = config.discover_capability("testservice").await;

    match orig_host {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_TESTSERVICE_HOST", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_TESTSERVICE_HOST"),
    }
    match orig_port {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_TESTSERVICE_PORT", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_TESTSERVICE_PORT"),
    }
    assert!(result.is_ok(), "Should discover service from environment");
    let endpoint = result?;
    assert_eq!(endpoint.address, "localhost");
    assert_eq!(endpoint.port, 9999);

    Ok(())
}

#[tokio::test]
async fn test_discovery_caching() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let orig_host = std::env::var("NESTGATE_CACHE_TEST_HOST").ok();
    let orig_port = std::env::var("NESTGATE_CACHE_TEST_PORT").ok();
    nestgate_core::env_process::set_var("NESTGATE_CACHE_TEST_HOST", "cachehost");
    nestgate_core::env_process::set_var("NESTGATE_CACHE_TEST_PORT", "8888");

    let config = CapabilityConfig::initialize().await?;

    let endpoint1 = config.discover_capability("cache_test").await?;
    let endpoint2 = config.discover_capability("cache_test").await?;

    match orig_host {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_CACHE_TEST_HOST", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_CACHE_TEST_HOST"),
    }
    match orig_port {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_CACHE_TEST_PORT", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_CACHE_TEST_PORT"),
    }
    assert_eq!(endpoint1.address, endpoint2.address);
    assert_eq!(endpoint1.port, endpoint2.port);

    Ok(())
}

#[tokio::test]
async fn test_discovery_failure_without_config() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config = CapabilityConfig::initialize().await?;

    // Try to discover a non-existent service without environment config
    let result = config.discover_capability("nonexistent_service_xyz").await;

    assert!(
        result.is_err(),
        "Should fail to discover non-configured service"
    );

    Ok(())
}

#[tokio::test]
async fn test_service_endpoint_url_construction() {
    use nestgate_core::capability_based_config::ServiceEndpoint;

    let endpoint = ServiceEndpoint {
        protocol: "https".to_string(),
        address: "example.com".to_string(),
        port: 443,
        path: Some("/api/v2".to_string()),
    };

    assert_eq!(endpoint.url(), "https://example.com:443/api/v2");
}

#[tokio::test]
async fn test_service_endpoint_url_without_path() {
    use nestgate_core::capability_based_config::ServiceEndpoint;

    let endpoint = ServiceEndpoint {
        protocol: "http".to_string(),
        address: "localhost".to_string(),
        port: 3000,
        path: None,
    };

    assert_eq!(endpoint.url(), "http://localhost:3000");
}

#[tokio::test]
async fn test_announce_capabilities() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config = CapabilityConfig::initialize().await?;

    // Announce should succeed (even if it does nothing without discovery mechanisms)
    let result = config.announce();
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_get_port_from_environment() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let orig = std::env::var("NESTGATE_CUSTOM_PORT").ok();
    nestgate_core::env_process::set_var("NESTGATE_CUSTOM_PORT", "7777");

    let config = CapabilityConfig::initialize().await?;
    let port = config.get_port("NESTGATE_CUSTOM_PORT").await?;

    match orig {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_CUSTOM_PORT", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_CUSTOM_PORT"),
    }
    assert_eq!(port, 7777);

    Ok(())
}

#[tokio::test]
async fn test_get_port_invalid_value() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let orig = std::env::var("NESTGATE_INVALID_PORT").ok();
    nestgate_core::env_process::set_var("NESTGATE_INVALID_PORT", "not_a_number");

    let config = CapabilityConfig::initialize().await?;
    let result = config.get_port("NESTGATE_INVALID_PORT").await;

    match orig {
        Some(v) => nestgate_core::env_process::set_var("NESTGATE_INVALID_PORT", v),
        None => nestgate_core::env_process::remove_var("NESTGATE_INVALID_PORT"),
    }
    assert!(result.is_err(), "Should fail with invalid port");

    Ok(())
}

#[tokio::test]
async fn test_discovered_capabilities_initially_empty() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config = CapabilityConfig::initialize().await?;
    let discovered = config.discovered_capabilities().await;

    assert!(discovered.is_empty(), "No capabilities discovered yet");

    Ok(())
}

#[tokio::test]
async fn test_primal_identity_uniqueness() -> Result<()> {
    use nestgate_core::capability_based_config::CapabilityConfig;

    let config1 = CapabilityConfig::initialize().await?;
    let config2 = CapabilityConfig::initialize().await?;

    let id1 = &config1.self_knowledge().identity.id;
    let id2 = &config2.self_knowledge().identity.id;

    assert_ne!(id1, id2, "Each primal should have unique ID");

    Ok(())
}
