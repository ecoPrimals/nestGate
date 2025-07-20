//! Security Integration Tests
//!
//! Tests the complete security implementation including authentication,
//! authorization, token management, and password security

use std::time::Duration;
use tokio::time::sleep;

// We'll use the MCP security implementation for testing
use nestgate_mcp::security::{SecurityManager, SecurityConfig, AuthToken, Role, Permission};

#[tokio::test]
async fn test_comprehensive_security_workflow() {
    println!("🔐 Testing comprehensive security workflow");

    // Create security manager with production-like settings
    let mut config = SecurityConfig::default();
    config.token_expiration = Some(2); // 2 seconds for testing
    config.min_password_length = 12;
    config.max_tokens_per_user = 3;
    config.enforce_password_policy = true;
    config.enable_audit_logging = true;

    let security_manager = SecurityManager::new(config);
    security_manager.initialize().await.expect("Failed to initialize security manager");

    println!("✅ Security manager initialized successfully");

    // Test 1: User Registration with Password Policy
    println!("\n📝 Testing user registration with password policy...");

    // Should fail with weak password
    let weak_password = "weak123";
    let result = security_manager.register_user(
        "testuser".to_string(),
        weak_password.to_string(),
        vec!["system:read".to_string()],
    ).await;

    assert!(result.is_err(), "Should reject weak password");
    println!("✅ Weak password correctly rejected");

    // Should succeed with strong password
    let strong_password = "StrongPassword123!";
    let user_token = security_manager.register_user(
        "testuser".to_string(),
        strong_password.to_string(),
        vec!["system:read".to_string(), "system:write".to_string()],
    ).await.expect("Failed to register user with strong password");

    println!("✅ User registered with strong password");

    // Test 2: Authentication and Token Management
    println!("\n🔑 Testing authentication and token management...");

    // Login with correct credentials
    let login_token = security_manager.login(
        "testuser".to_string(),
        strong_password.to_string(),
    ).await.expect("Failed to login with correct credentials");

    println!("✅ Login successful with correct credentials");

    // Validate token
    let validation_result = security_manager.validate_token(&login_token.token).await;
    assert!(validation_result.is_ok(), "Token should be valid");
    println!("✅ Token validation successful");

    // Test 3: Authorization
    println!("\n🛡️ Testing authorization...");

    let authorized_read = security_manager.check_authorization(&login_token, "system:read").await;
    assert!(authorized_read.is_ok(), "User should have read permission");
    println!("✅ Read authorization successful");

    let authorized_write = security_manager.check_authorization(&login_token, "system:write").await;
    assert!(authorized_write.is_ok(), "User should have write permission");
    println!("✅ Write authorization successful");

    let unauthorized_admin = security_manager.check_authorization(&login_token, "system:admin").await;
    assert!(unauthorized_admin.is_err(), "User should not have admin permission");
    println!("✅ Admin authorization correctly denied");

    // Test 4: Token Expiration
    println!("\n⏰ Testing token expiration...");

    // Wait for token to expire (2 seconds)
    sleep(Duration::from_secs(3)).await;

    let expired_validation = security_manager.validate_token(&login_token.token).await;
    assert!(expired_validation.is_err(), "Expired token should be invalid");
    println!("✅ Token expiration working correctly");

    // Test 5: Multiple Token Management
    println!("\n📋 Testing multiple token management...");

    // Create multiple tokens for the same user
    let token1 = security_manager.login("testuser".to_string(), strong_password.to_string()).await?;
    let token2 = security_manager.login("testuser".to_string(), strong_password.to_string()).await?;
    let token3 = security_manager.login("testuser".to_string(), strong_password.to_string()).await?;

    // All tokens should be valid initially
    assert!(security_manager.validate_token(&token1.token).await.is_ok());
    assert!(security_manager.validate_token(&token2.token).await.is_ok());
    assert!(security_manager.validate_token(&token3.token).await.is_ok());

    println!("✅ Multiple token creation successful");

    // Test 6: Security Events and Audit
    println!("\n📊 Testing security audit...");

    let security_events = security_manager.get_audit_events().await.expect("Failed to get audit events");
    assert!(!security_events.is_empty(), "Should have security events logged");
    println!("✅ Security audit events logged: {} events", security_events.len());

    println!("\n🎉 All security integration tests completed successfully!");
    Ok(())
}

#[tokio::test]
async fn test_security_development_mode() {
    println!("🔧 Testing development mode security (authentication disabled)");

    let mut config = SecurityConfig::default();
    config.require_authentication = false;

    let security_manager = SecurityManager::new(config);
    security_manager.initialize().await.expect("Failed to initialize security manager");

    // In development mode, any token should be valid
    let token = security_manager.validate_token("any-token").await
        .expect("Failed to validate token in dev mode");

    assert_eq!(token.role, Role::Admin);
    assert!(token.permissions.contains(&Permission::AdminOperations));

    // Authorization should always return true
    let has_any_permission = security_manager.check_authorization("anyone", "admin:operations").await
        .expect("Failed to check authorization in dev mode");
    assert!(has_any_permission, "Dev mode should allow all operations");

    println!("✅ Development mode security working correctly");
}

#[tokio::test]
async fn test_password_security() {
    println!("🔐 Testing password security and hashing");

    let security_manager = SecurityManager::with_defaults();
    security_manager.initialize().await.expect("Failed to initialize security manager");

    // Register user with known password
    let password = "TestPassword123!";
    security_manager.register_user(
        "hashtest".to_string(),
        "Hash Test User".to_string(),
        password,
        Some(Role::User),
    ).await.expect("Failed to register user for hash test");

    // Valid password should authenticate
    let result = security_manager.authenticate("hashtest", password).await;
    assert!(result.is_ok(), "Correct password should authenticate");

    // Invalid password should fail
    let wrong_result = security_manager.authenticate("hashtest", "WrongPassword123!").await;
    assert!(wrong_result.is_err(), "Wrong password should fail");

    println!("✅ Password hashing and verification working correctly");
}

#[tokio::test]
async fn test_role_based_permissions() {
    println!("👥 Testing role-based permission system");

    let security_manager = SecurityManager::with_defaults();
    security_manager.initialize().await.expect("Failed to initialize security manager");

    // Create users with different roles
    let roles_and_users = vec![
        (Role::ReadOnly, "readonly_user"),
        (Role::User, "regular_user"),
        (Role::Service, "service_user"),
        (Role::Admin, "admin_user"),
    ];

    for (role, user_id) in &roles_and_users {
        security_manager.register_user(
            user_id.to_string(),
            format!("{:?} User", role),
            "StrongPassword123!",
            Some(role.clone()),
        ).await.expect(&format!("Failed to register {} user", user_id));

        let token = security_manager.authenticate(user_id, "StrongPassword123!").await
            .expect(&format!("Failed to authenticate {} user", user_id));

        assert_eq!(token.role, *role);
        assert_eq!(token.permissions, role.default_permissions());
    }

    // Test specific permission checks
    let readonly_has_write = security_manager.check_authorization("readonly_user", "system:write").await
        .expect("Failed to check readonly write permission");
    assert!(!readonly_has_write, "ReadOnly user should not have write permissions");

    let admin_has_write = security_manager.check_authorization("admin_user", "system:write").await
        .expect("Failed to check admin write permission");
    assert!(admin_has_write, "Admin user should have write permissions");

    println!("✅ Role-based permissions working correctly");
}