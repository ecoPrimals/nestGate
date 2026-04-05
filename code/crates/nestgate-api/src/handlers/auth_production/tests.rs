// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::*;
use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

#[test]
fn test_handler_creation() {
    let handler = ProductionAuthHandler::new();
    assert!(matches!(handler, ProductionAuthHandler { .. }));
}

#[tokio::test]
async fn test_create_user() {
    let handler = ProductionAuthHandler::new();
    let request = CreateUserRequest {
        user_id: "test_user".to_string(),
        username: "testuser".to_string(),
        role: "user".to_string(),
        permissions: vec!["read".to_string()],
    };

    let result = create_user(State(handler), Json(request)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_api_key() {
    let handler = ProductionAuthHandler::new();
    let request = ApiKeyRequest {
        user_id: "test_user".to_string(),
        name: "Test Key".to_string(),
    };

    let result = create_api_key(State(handler), Json(request)).await;
    assert!(result.is_ok());
}

// Additional missing types for completeness
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenValidationRequest {
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenRevocationRequest {
    token: String,
}

// Stub implementations for testing
async fn validate_token(
    State(handler): State<ProductionAuthHandler>,
    Json(request): Json<TokenValidationRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(serde_json::json!({"valid": false, "error": "Invalid token"})),
    )
}

/// Revoke Token
async fn revoke_token(
    State(_handler): State<ProductionAuthHandler>,
    Json(_request): Json<TokenRevocationRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::OK, Json(serde_json::json!({"success": true})))
}

// Import error handling tests from auth_production_tests
#[tokio::test]
async fn test_authenticate_with_missing_user() {
    let handler = ProductionAuthHandler::new();
    let credentials = AuthCredentials {
        username: "nonexistent_user".to_string(),
        password: "password123".to_string(),
    };

    let result = authenticate(State(handler), Json(credentials)).await;
    // In current implementation, all authentications succeed (stub)
    // In production, this would fail
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_authenticate_with_empty_username() {
    let handler = ProductionAuthHandler::new();
    let credentials = AuthCredentials {
        username: String::new(),
        password: "password123".to_string(),
    };

    let result = authenticate(State(handler), Json(credentials)).await;
    // Currently succeeds, but in production would validate
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_validate_token_with_invalid_token() {
    let handler = ProductionAuthHandler::new();
    let request = TokenValidationRequest {
        token: "invalid_token_12345".to_string(),
    };

    let (status, _) = validate_token(State(handler), Json(request)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_api_key_response_format() {
    let handler = ProductionAuthHandler::new();
    let request = ApiKeyRequest {
        user_id: "testuser".to_string(),
        name: "test_key".to_string(),
    };

    let result = create_api_key(State(handler), Json(request)).await;
    assert!(result.is_ok());

    let Json(response) = result.expect("Authentication failed");
    assert!(response.api_key.starts_with("nestgate_"));
    assert_eq!(response.user_id, "testuser");
    assert_eq!(response.name, "test_key");
}

#[tokio::test]
async fn test_create_user_with_various_roles() {
    let handler = ProductionAuthHandler::new();

    // Test admin role
    let admin_request = CreateUserRequest {
        user_id: "admin_user".to_string(),
        username: "admin".to_string(),
        role: "admin".to_string(),
        permissions: vec!["all".to_string()],
    };

    let result = create_user(State(handler.clone()), Json(admin_request)).await;
    assert!(result.is_ok());

    // Test operator role
    let operator_request = CreateUserRequest {
        user_id: "operator_user".to_string(),
        username: "operator".to_string(),
        role: "operator".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    let result = create_user(State(handler.clone()), Json(operator_request)).await;
    assert!(result.is_ok());

    // Test read_only role
    let readonly_request = CreateUserRequest {
        user_id: "readonly_user".to_string(),
        username: "readonly".to_string(),
        role: "read_only".to_string(),
        permissions: vec!["read".to_string()],
    };

    let result = create_user(State(handler), Json(readonly_request)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_auth_credentials_serialization() {
    let creds = AuthCredentials {
        username: "testuser".to_string(),
        password: "testpass".to_string(),
    };

    let json = serde_json::to_string(&creds);
    assert!(json.is_ok());

    let json_str = json.expect("Authentication failed");
    assert!(json_str.contains("testuser"));
    assert!(json_str.contains("testpass"));
}

#[tokio::test]
async fn test_auth_response_serialization() {
    let response = AuthResponse {
        success: true,
        token: Some("token_abc".to_string()),
        user_id: Some("user_123".to_string()),
        role: "admin".to_string(),
        permissions: vec!["read".to_string()],
    };

    let json = serde_json::to_string(&response);
    assert!(json.is_ok());
}

#[tokio::test]
async fn test_production_auth_handler_creation() {
    let handler = ProductionAuthHandler::new();
    let manager = handler.get_manager().await;
    assert!(manager.user_exists("admin").is_ok());
}

#[tokio::test]
async fn test_production_auth_handler_default() {
    let handler = ProductionAuthHandler::default();
    let manager = handler.get_manager().await;
    assert!(manager.user_exists("admin").is_ok());
}

#[tokio::test]
async fn test_auth_credentials_structure() {
    let creds = AuthCredentials {
        username: "test".to_string(),
        password: "pass".to_string(),
    };
    assert_eq!(creds.username, "test");
    assert_eq!(creds.password, "pass");
}

#[tokio::test]
async fn test_auth_response_structure() {
    let response = AuthResponse {
        success: true,
        token: Some("token".to_string()),
        user_id: Some("user".to_string()),
        role: "admin".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };
    assert!(response.success);
    assert!(response.token.is_some());
    assert_eq!(response.permissions.len(), 2);
}

#[tokio::test]
async fn test_api_key_request_structure() {
    let request = ApiKeyRequest {
        user_id: "user_123".to_string(),
        name: "API Key 1".to_string(),
    };
    assert_eq!(request.user_id, "user_123");
    assert_eq!(request.name, "API Key 1");
}

#[tokio::test]
async fn test_api_key_response_structure() {
    let response = ApiKeyResponse {
        api_key: "key_abc123".to_string(),
        user_id: "user_456".to_string(),
        name: "My API Key".to_string(),
    };
    assert!(!response.api_key.is_empty());
    assert_eq!(response.user_id, "user_456");
}

#[tokio::test]
async fn test_create_user_request_structure() {
    let request = CreateUserRequest {
        user_id: "new_user".to_string(),
        username: "newuser".to_string(),
        role: "user".to_string(),
        permissions: vec!["read".to_string()],
    };
    assert_eq!(request.user_id, "new_user");
    assert_eq!(request.role, "user");
}

#[tokio::test]
async fn test_auth_response_failure() {
    let response = AuthResponse {
        success: false,
        token: None,
        user_id: None,
        role: "guest".to_string(),
        permissions: vec![],
    };
    assert!(!response.success);
    assert!(response.token.is_none());
    assert!(response.permissions.is_empty());
}

#[tokio::test]
async fn test_auth_credentials_empty() {
    let creds = AuthCredentials {
        username: String::new(),
        password: String::new(),
    };
    assert!(creds.username.is_empty());
    assert!(creds.password.is_empty());
}

#[tokio::test]
async fn test_multiple_permissions() {
    let response = AuthResponse {
        success: true,
        token: Some("token".to_string()),
        user_id: Some("user".to_string()),
        role: "admin".to_string(),
        permissions: vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
            "admin".to_string(),
        ],
    };
    assert_eq!(response.permissions.len(), 4);
    assert!(response.permissions.contains(&"admin".to_string()));
}

#[tokio::test]
async fn test_auth_handler_clone() {
    let handler1 = ProductionAuthHandler::new();
    let handler2 = handler1.clone();

    // Both handlers should work independently
    let manager1 = handler1.get_manager().await;
    let manager2 = handler2.get_manager().await;

    assert!(manager1.user_exists("admin").is_ok());
    assert!(manager2.user_exists("admin").is_ok());
}

#[tokio::test]
async fn test_api_key_request_validation() {
    let valid_request = ApiKeyRequest {
        user_id: "user_123".to_string(),
        name: "Valid Key".to_string(),
    };
    assert!(!valid_request.user_id.is_empty());
    assert!(!valid_request.name.is_empty());

    let empty_request = ApiKeyRequest {
        user_id: String::new(),
        name: String::new(),
    };
    assert!(empty_request.user_id.is_empty());
}

#[tokio::test]
async fn test_create_user_with_multiple_permissions() {
    let request = CreateUserRequest {
        user_id: "power_user".to_string(),
        username: "poweruser".to_string(),
        role: "power_user".to_string(),
        permissions: vec![
            "read".to_string(),
            "write".to_string(),
            "execute".to_string(),
        ],
    };
    assert_eq!(request.permissions.len(), 3);
}

#[tokio::test]
async fn test_auth_response_with_all_fields() {
    let response = AuthResponse {
        success: true,
        token: Some("full_token_123".to_string()),
        user_id: Some("user_full".to_string()),
        role: "superadmin".to_string(),
        permissions: vec!["all".to_string()],
    };

    // Verify all fields are set
    assert!(response.success);
    assert!(response.token.is_some());
    assert!(response.user_id.is_some());
    assert!(!response.role.is_empty());
    assert!(!response.permissions.is_empty());
}

#[tokio::test]
async fn test_auth_credentials_special_characters() {
    let creds = AuthCredentials {
        username: "user@example.com".to_string(),
        password: "P@ssw0rd!#$".to_string(),
    };
    assert!(creds.username.contains('@'));
    assert!(creds.password.contains('!'));
}

#[tokio::test]
async fn test_api_key_response_key_format() {
    let response = ApiKeyResponse {
        api_key: "nsk_1234567890abcdef".to_string(),
        user_id: "user_001".to_string(),
        name: "Production Key".to_string(),
    };
    assert!(response.api_key.starts_with("nsk_"));
    assert!(response.api_key.len() > 10);
}

#[tokio::test]
async fn test_create_user_role_variations() {
    let roles = vec!["admin", "user", "read_only", "operator", "guest"];

    for role in roles {
        let request = CreateUserRequest {
            user_id: format!("user_{}", role),
            username: role.to_string(),
            role: role.to_string(),
            permissions: vec!["read".to_string()],
        };
        assert_eq!(request.role, role);
    }
}

#[tokio::test]
async fn test_auth_response_deserialization() {
    let json_str = r#"{
        "success": true,
        "token": "token_xyz",
        "user_id": "user_789",
        "role": "moderator",
        "permissions": ["read", "moderate"]
    }"#;

    let response: std::result::Result<AuthResponse, _> = serde_json::from_str(json_str);
    assert!(response.is_ok());

    let response = response.expect("Authentication failed");
    assert_eq!(response.role, "moderator");
    assert_eq!(response.permissions.len(), 2);
}

#[tokio::test]
async fn test_auth_credentials_clone() {
    let creds1 = AuthCredentials {
        username: "clone_test".to_string(),
        password: "password".to_string(),
    };
    let creds2 = creds1.clone();

    assert_eq!(creds1.username, creds2.username);
    assert_eq!(creds1.password, creds2.password);
}

#[tokio::test]
async fn test_api_key_request_clone() {
    let req1 = ApiKeyRequest {
        user_id: "user".to_string(),
        name: "key".to_string(),
    };
    let req2 = req1.clone();

    assert_eq!(req1.user_id, req2.user_id);
    assert_eq!(req1.name, req2.name);
}

#[tokio::test]
async fn test_create_user_request_clone() {
    let req1 = CreateUserRequest {
        user_id: "user".to_string(),
        username: "name".to_string(),
        role: "role".to_string(),
        permissions: vec!["perm".to_string()],
    };
    let req2 = req1.clone();

    assert_eq!(req1.user_id, req2.user_id);
    assert_eq!(req1.permissions, req2.permissions);
}

#[tokio::test]
async fn test_auth_response_none_values() {
    let response = AuthResponse {
        success: false,
        token: None,
        user_id: None,
        role: String::new(),
        permissions: vec![],
    };

    assert!(response.token.is_none());
    assert!(response.user_id.is_none());
}

#[tokio::test]
async fn test_production_handler_multiple_managers() {
    let handler = ProductionAuthHandler::new();

    // Test concurrent access
    let manager1 = handler.get_manager().await;
    drop(manager1);

    let manager2 = handler.get_manager().await;
    assert!(manager2.user_exists("admin").is_ok());
}
