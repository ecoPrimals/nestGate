//! Comprehensive tests for primal self-knowledge and discovery

use anyhow::Result;

#[tokio::test]
async fn test_primal_initialization() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;
    let identity = primal.identity();

    assert_eq!(identity.primal_type, "nestgate");
    assert!(!identity.id.is_empty());
    assert!(!identity.version.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_primal_has_storage_capability() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;
    let capabilities = primal.capabilities();

    assert!(!capabilities.is_empty());
    assert!(
        capabilities.iter().any(|c| c.name == "storage"),
        "Should have storage capability"
    );

    Ok(())
}

#[tokio::test]
async fn test_primal_endpoints_configured() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;
    let endpoints = primal.endpoints();

    assert!(!endpoints.is_empty(), "Should have at least one endpoint");

    for endpoint in endpoints {
        assert!(!endpoint.protocol.is_empty());
        assert!(!endpoint.address.is_empty());
        assert!(endpoint.port > 0);
    }

    Ok(())
}

#[tokio::test]
async fn test_primal_endpoint_url_generation() {
    use nestgate_core::primal_self_knowledge::Endpoint;

    let endpoint = Endpoint {
        protocol: "http".to_string(),
        address: "0.0.0.0".to_string(),
        port: 3000,
        path: Some("/api/v1".to_string()),
        health_path: Some("/health".to_string()),
    };

    assert_eq!(endpoint.url(), "http://0.0.0.0:3000/api/v1");
    assert_eq!(
        endpoint.health_url(),
        Some("http://0.0.0.0:3000/health".to_string())
    );
}

#[tokio::test]
async fn test_primal_endpoint_without_health() {
    use nestgate_core::primal_self_knowledge::Endpoint;

    let endpoint = Endpoint {
        protocol: "grpc".to_string(),
        address: "localhost".to_string(),
        port: 50051,
        path: None,
        health_path: None,
    };

    assert_eq!(endpoint.url(), "grpc://localhost:50051");
    assert_eq!(endpoint.health_url(), None);
}

#[tokio::test]
async fn test_primal_announce_self() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;

    // Should succeed even if no mechanisms are active
    let result = primal.announce_self();
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_primal_discover_from_environment() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let orig_host = std::env::var("BEARDOG_HOST").ok();
    let orig_port = std::env::var("BEARDOG_PORT").ok();
    nestgate_core::env_process::set_var("BEARDOG_HOST", "beardog.local");
    nestgate_core::env_process::set_var("BEARDOG_PORT", "4000");

    let mut primal = PrimalSelfKnowledge::initialize().await?;
    let discovered = primal.discover_primal("beardog").await?;

    match orig_host {
        Some(v) => nestgate_core::env_process::set_var("BEARDOG_HOST", v),
        None => nestgate_core::env_process::remove_var("BEARDOG_HOST"),
    }
    match orig_port {
        Some(v) => nestgate_core::env_process::set_var("BEARDOG_PORT", v),
        None => nestgate_core::env_process::remove_var("BEARDOG_PORT"),
    }
    assert_eq!(discovered.identity.primal_type, "beardog");
    assert_eq!(discovered.primary_endpoint.address, "beardog.local");
    assert_eq!(discovered.primary_endpoint.port, 4000);

    Ok(())
}

#[tokio::test]
async fn test_primal_discovery_caching() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let orig_host = std::env::var("SONGBIRD_HOST").ok();
    let orig_port = std::env::var("SONGBIRD_PORT").ok();
    nestgate_core::env_process::set_var("SONGBIRD_HOST", "songbird.local");
    nestgate_core::env_process::set_var("SONGBIRD_PORT", "5000");

    let mut primal = PrimalSelfKnowledge::initialize().await?;

    let _ = primal.discover_primal("songbird").await?;

    match orig_host {
        Some(v) => nestgate_core::env_process::set_var("SONGBIRD_HOST", v),
        None => nestgate_core::env_process::remove_var("SONGBIRD_HOST"),
    }
    match orig_port {
        Some(v) => nestgate_core::env_process::set_var("SONGBIRD_PORT", v),
        None => nestgate_core::env_process::remove_var("SONGBIRD_PORT"),
    }
    let discovered = primal.discovered_primals();
    assert!(discovered.contains_key("songbird"));
    Ok(())
}

#[tokio::test]
async fn test_primal_discovery_failure_no_config() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let mut primal = PrimalSelfKnowledge::initialize().await?;

    // Try to discover non-existent primal
    let result = primal.discover_primal("nonexistent_primal_xyz").await;

    assert!(result.is_err(), "Should fail without configuration");

    Ok(())
}

#[tokio::test]
async fn test_discovered_primal_has_capability() {
    use nestgate_core::primal_self_knowledge::{
        Capability, DiscoveredPrimal, DiscoveryMechanism, Endpoint, PrimalIdentity,
    };
    use std::collections::HashMap;

    let discovered = DiscoveredPrimal {
        identity: PrimalIdentity {
            id: "test-123".to_string(),
            primal_type: "testprimal".to_string(),
            version: "1.0.0".to_string(),
            started_at: std::time::SystemTime::now(),
        },
        capabilities: vec![Capability {
            name: "zfs".to_string(),
            description: "ZFS management".to_string(),
            endpoint: "/zfs".to_string(),
            metadata: HashMap::new(),
        }],
        primary_endpoint: Endpoint {
            protocol: "http".to_string(),
            address: "test.local".to_string(),
            port: 3000,
            path: None,
            health_path: None,
        },
        discovered_at: std::time::SystemTime::now(),
        discovery_method: DiscoveryMechanism::Environment,
    };

    assert!(discovered.has_capability("zfs"));
    assert!(discovered.has_capability("ZFS")); // Case insensitive
    assert!(!discovered.has_capability("nonexistent"));
}

#[tokio::test]
async fn test_discovered_primal_primary_endpoint() {
    use nestgate_core::primal_self_knowledge::{
        DiscoveredPrimal, DiscoveryMechanism, Endpoint, PrimalIdentity,
    };

    let discovered = DiscoveredPrimal {
        identity: PrimalIdentity {
            id: "test-456".to_string(),
            primal_type: "testprimal".to_string(),
            version: "2.0.0".to_string(),
            started_at: std::time::SystemTime::now(),
        },
        capabilities: vec![],
        primary_endpoint: Endpoint {
            protocol: "https".to_string(),
            address: "secure.local".to_string(),
            port: 443,
            path: Some("/api".to_string()),
            health_path: None,
        },
        discovered_at: std::time::SystemTime::now(),
        discovery_method: DiscoveryMechanism::Environment,
    };

    assert_eq!(
        discovered.primary_endpoint(),
        "https://secure.local:443/api"
    );
}

#[tokio::test]
async fn test_primal_identity_generation() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal1 = PrimalSelfKnowledge::initialize().await?;
    let primal2 = PrimalSelfKnowledge::initialize().await?;

    let id1 = &primal1.identity().id;
    let id2 = &primal2.identity().id;

    // IDs should be unique (UUID v4)
    assert_ne!(id1, id2);

    Ok(())
}

#[tokio::test]
async fn test_capability_metadata() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;
    let capabilities = primal.capabilities();

    for capability in capabilities {
        assert!(!capability.name.is_empty());
        assert!(!capability.description.is_empty());
        assert!(!capability.endpoint.is_empty());
        // metadata may be empty
    }

    Ok(())
}

#[tokio::test]
async fn test_discovered_primals_initially_empty() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;
    let discovered = primal.discovered_primals();

    assert!(discovered.is_empty(), "No primals discovered yet");

    Ok(())
}

#[tokio::test]
async fn test_primal_version_from_cargo() -> Result<()> {
    use nestgate_core::primal_self_knowledge::PrimalSelfKnowledge;

    let primal = PrimalSelfKnowledge::initialize().await?;
    let version = &primal.identity().version;

    // Should match Cargo.toml version
    assert!(!version.is_empty());
    assert!(version.contains('.'), "Version should have dots");

    Ok(())
}
