//! Security Integration Tests
//! 
//! Tests the complete security implementation including authentication,
//! authorization, token management, and password security

use std::time::Duration;
use tokio::time::sleep;

// We'll use the core security implementation for testing
use nestgate_core::security::{SecurityManager, SecurityConfig, Role, Permission};

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
        "Test User".to_string(),
        weak_password,
        Some(Role::User),
    ).await;
    
    assert!(result.is_err(), "Should reject weak password");
    println!("✅ Weak password correctly rejected");
    
    // Should succeed with strong password
    let strong_password = "StrongPassword123!";
    security_manager.register_user(
        "testuser".to_string(),
        "Test User".to_string(),
        strong_password,
        Some(Role::User),
    ).await.expect("Failed to register user with strong password");
    
    println!("✅ User registered with strong password");
    
    // Test 2: Authentication and Token Management
    println!("\n🔑 Testing authentication and token management...");
    
    // Valid authentication
    let token = security_manager.authenticate("testuser", strong_password).await
        .expect("Failed to authenticate user");
    
    assert_eq!(token.user_id, "testuser");
    assert_eq!(token.role, Role::User);
    assert!(token.is_valid());
    
    println!("✅ User authenticated successfully");
    println!("   Token: {}", &token.token[..20]);
    println!("   Role: {:?}", token.role);
    println!("   Permissions: {:?}", token.permissions);
    
    // Test 3: Token Validation
    println!("\n🎫 Testing token validation...");
    
    let validated_token = security_manager.validate_token(&token.token).await
        .expect("Failed to validate token");
    
    assert_eq!(validated_token.user_id, "testuser");
    println!("✅ Token validated successfully");
    
    // Test 4: Authorization Testing
    println!("\n🛡️ Testing authorization system...");
    
    // User should have read permissions
    let has_read = security_manager.check_authorization("testuser", "system:read").await
        .expect("Failed to check read authorization");
    assert!(has_read, "User should have read permissions");
    println!("✅ User has correct read permissions");
    
    // User should NOT have admin permissions
    let has_admin = security_manager.check_authorization("testuser", "admin:operations").await
        .expect("Failed to check admin authorization");
    assert!(!has_admin, "User should not have admin permissions");
    println!("✅ User correctly denied admin permissions");
    
    // Test 5: Admin User Creation and Permissions
    println!("\n👑 Testing admin user creation and permissions...");
    
    security_manager.register_user(
        "admin".to_string(),
        "Administrator".to_string(),
        "AdminPassword123!",
        Some(Role::Admin),
    ).await.expect("Failed to register admin user");
    
    let admin_token = security_manager.authenticate("admin", "AdminPassword123!").await
        .expect("Failed to authenticate admin");
    
    let has_admin_perms = security_manager.check_authorization("admin", "admin:operations").await
        .expect("Failed to check admin authorization");
    assert!(has_admin_perms, "Admin should have admin permissions");
    println!("✅ Admin user has correct permissions");
    
    // Test 6: Token Expiration
    println!("\n⏰ Testing token expiration...");
    
    // Token should be valid initially
    assert!(security_manager.validate_token(&token.token).await.is_ok());
    println!("✅ Token valid before expiration");
    
    // Wait for token to expire
    sleep(Duration::from_secs(3)).await;
    
    // Token should be expired now
    let expired_result = security_manager.validate_token(&token.token).await;
    assert!(expired_result.is_err(), "Token should be expired");
    println!("✅ Token correctly expired");
    
    // Test 7: Token Limit Enforcement
    println!("\n🚫 Testing token limit enforcement...");
    
    // Create multiple tokens for the same user
    let mut tokens = Vec::new();
    for i in 0..3 {
        let token = security_manager.authenticate("testuser", strong_password).await
            .expect(&format!("Failed to create token {}", i));
        tokens.push(token);
    }
    
    // Should fail on 4th token (limit is 3)
    let fourth_token = security_manager.authenticate("testuser", strong_password).await;
    assert!(fourth_token.is_err(), "Should reject 4th token due to limit");
    println!("✅ Token limit correctly enforced");
    
    // Test 8: Token Revocation
    println!("\n🔒 Testing token revocation...");
    
    // Revoke a specific token
    security_manager.revoke_token(&tokens[0].token).await
        .expect("Failed to revoke token");
    
    let revoked_result = security_manager.validate_token(&tokens[0].token).await;
    assert!(revoked_result.is_err(), "Revoked token should be invalid");
    println!("✅ Token revocation working correctly");
    
    // Test 9: Security Statistics
    println!("\n📊 Testing security statistics...");
    
    let stats = security_manager.get_security_stats().await
        .expect("Failed to get security stats");
    
    assert!(stats.total_users >= 2); // testuser + admin
    assert!(stats.active_tokens > 0);
    assert!(stats.authentication_required);
    
    println!("✅ Security statistics:");
    println!("   Total users: {}", stats.total_users);
    println!("   Active users: {}", stats.active_users);
    println!("   Active tokens: {}", stats.active_tokens);
    println!("   Expired tokens: {}", stats.expired_tokens);
    
    // Test 10: Cleanup and Final Validation
    println!("\n🧹 Testing cleanup operations...");
    
    // Clean up expired tokens
    security_manager.cleanup_expired_tokens().await
        .expect("Failed to cleanup expired tokens");
    
    // Revoke all tokens for a user
    security_manager.revoke_user_tokens("testuser").await
        .expect("Failed to revoke user tokens");
    
    let user_token_count = security_manager.get_user_token_count("testuser").await
        .expect("Failed to get user token count");
    assert_eq!(user_token_count, 0, "All user tokens should be revoked");
    
    println!("✅ Cleanup operations completed successfully");
    
    println!("\n🎉 All security tests passed! Security implementation is working correctly.");
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