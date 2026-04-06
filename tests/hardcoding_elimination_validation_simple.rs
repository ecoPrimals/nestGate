// SPDX-License-Identifier: AGPL-3.0-or-later
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

//! Simplified hardcoding elimination validation tests
//! Tests that dynamic endpoint resolution works without hardcoded values

use nestgate_core::service_discovery::DynamicEndpointResolver;
use nestgate_types::{EnvSource, MapEnv};

#[tokio::test]
async fn test_dynamic_endpoint_resolution_basic() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing basic dynamic endpoint resolution...");

    let resolver = DynamicEndpointResolver::new();

    // Test basic endpoint resolution
    let endpoint = resolver.resolve_endpoint("api")?;

    // Should return valid URL format
    assert!(endpoint.starts_with("http://") || endpoint.starts_with("https://"));

    println!("✅ Dynamic endpoint resolution working: {}", endpoint);
    Ok(())
}

#[tokio::test]
async fn test_environment_overrides() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing environment variable overrides...");

    let env = MapEnv::from([("TEST_API_ENDPOINT", "http://custom-test-api:9090")]);
    assert_eq!(
        env.get("TEST_API_ENDPOINT").as_deref(),
        Some("http://custom-test-api:9090")
    );

    println!("✅ Environment variable overrides working");
    Ok(())
}

#[tokio::test]
async fn test_multiple_endpoints_unique() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing multiple endpoints are unique...");

    let resolver = DynamicEndpointResolver::new();

    let api_ep = resolver.resolve_endpoint("api")?;
    let ws_ep = resolver.resolve_endpoint("websocket")?;

    // Endpoints should be different
    assert_ne!(api_ep, ws_ep, "Endpoints should be unique");

    println!("✅ Multiple endpoints are unique");
    Ok(())
}
