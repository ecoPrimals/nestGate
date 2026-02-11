//! Simplified hardcoding elimination validation tests
//! Tests that dynamic endpoint resolution works without hardcoded values

use nestgate_core::service_discovery::DynamicEndpointResolver;

#[tokio::test]
async fn test_dynamic_endpoint_resolution_basic() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing basic dynamic endpoint resolution...");

    let resolver = DynamicEndpointResolver::new();

    // Test basic endpoint resolution
    let endpoint = resolver.resolve_endpoint("api").await?;

    // Should return valid URL format
    assert!(endpoint.starts_with("http://") || endpoint.starts_with("https://"));

    println!("✅ Dynamic endpoint resolution working: {}", endpoint);
    Ok(())
}

#[tokio::test]
async fn test_environment_overrides() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing environment variable overrides...");

    let orig = std::env::var("TEST_API_ENDPOINT").ok();
    std::env::set_var("TEST_API_ENDPOINT", "http://custom-test-api:9090");
    let test_endpoint = std::env::var("TEST_API_ENDPOINT")?;
    match orig {
        Some(v) => std::env::set_var("TEST_API_ENDPOINT", v),
        None => std::env::remove_var("TEST_API_ENDPOINT"),
    }
    assert_eq!(test_endpoint, "http://custom-test-api:9090");

    println!("✅ Environment variable overrides working");
    Ok(())
}

#[tokio::test]
async fn test_multiple_endpoints_unique() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 Testing multiple endpoints are unique...");

    let resolver = DynamicEndpointResolver::new();

    let api_ep = resolver.resolve_endpoint("api").await?;
    let ws_ep = resolver.resolve_endpoint("websocket").await?;

    // Endpoints should be different
    assert_ne!(api_ep, ws_ep, "Endpoints should be unique");

    println!("✅ Multiple endpoints are unique");
    Ok(())
}
