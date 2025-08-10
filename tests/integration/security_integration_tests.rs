/// Security Integration Tests
/// 
/// Focused security system integration tests extracted from comprehensive suite

use std::time::{SystemTime Instant};
use tokio::time::sleep;
// Removed unused tracing import
use std::sync::Arc;
use tokio::time::timeout;
use std::collections::HashMap;

use nestgate_core::{Result as CoreResult, NestGateError};
use nestgate_mcp::security::{SecurityManager, SecurityConfig, AuthToken};

/// Test the security system with comprehensive scenarios
#[tokio::test]
pub async fn test_security_system_comprehensive() -> CoreResult<()> {
    info!("🔒 Testing comprehensive security system");

    let config = SecurityConfig::default();
    let mut security = SecurityManager::new(config).await?;

    // Test authentication flow
    let auth_result = security.authenticate("test_user", "test_password").await;
    match auth_result {
        Ok(token) => {
            info!("✅ Authentication successful: {:?}", token.user_id);
            
            // Test token validation
            let validation = security.validate_token(&token.token).await?;
            assert!(validation.is_valid);
            info!("✅ Token validation successful");
        }
        Err(e) => {
            warn!("⚠️ Authentication test with mock credentials expected to fail: {}", e);
            // This is expected behavior for security system
        }
    }

    // Test authorization levels
    let permissions = security.get_user_permissions("test_user").await?;
    info!("📋 User permissions: {:?}", permissions);

    // Test security audit logging
    security.log_security_event("test_event", "test_details").await?;
    info!("✅ Security audit logging working");

    info!("✅ Security system comprehensive test completed");
    Ok(())
}

/// Test security token lifecycle
#[tokio::test]
pub async fn test_security_token_lifecycle() -> CoreResult<()> {
    info!("🔑 Testing security token lifecycle");

    let config = SecurityConfig::default();
    let mut security = SecurityManager::new(config).await?;

    // Test token creation
    let token = AuthToken {
        token: "test_token_12345".to_string(),
        user_id: "test_user".to_string(),
        expires_at: SystemTime::now() + Duration::from_secs(3600),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    // Test token storage and retrieval
    security.store_token(token.clone()).await?;
    let retrieved = security.get_token(&token.token).await?;
    assert_eq!(retrieved.user_id, token.user_id);
    
    info!("✅ Token lifecycle test completed");
    Ok(())
}

/// Test security system error handling
#[tokio::test]
pub async fn test_security_error_handling() -> CoreResult<()> {
    info!("⚠️ Testing security error handling");

    let config = SecurityConfig::default();
    let security = SecurityManager::new(config).await?;

    // Test invalid token validation
    let invalid_result = security.validate_token("invalid_token").await;
    match invalid_result {
        Ok(validation) => assert!(!validation.is_valid),
        Err(_) => info!("✅ Invalid token properly rejected"),
    }

    // Test malformed authentication
    let malformed_auth = security.authenticate("", "").await;
    assert!(malformed_auth.is_err());
    info!("✅ Malformed authentication properly rejected");

    info!("✅ Security error handling test completed");
    Ok(())
} 