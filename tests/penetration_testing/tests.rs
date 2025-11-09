/// Penetration Testing Tests
use nestgate_core::config::NestGateCanonicalConfig;

#[tokio::test]
async fn test_penetration_basic() -> Result<(), Box<dyn std::error::Error>> {
    let config = CanonicalTestConfig::penetration_tests();
    // Test implementation continues with canonical config
    assert!(config.test_domain.integration.penetration_testing.enabled);
    Ok(())
}
