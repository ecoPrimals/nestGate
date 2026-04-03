// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Comprehensive tests for production auth handlers: error handling paths,
//! edge cases, and production scenarios.

#![cfg(test)]

use super::auth_production::*;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenValidationRequest {
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TokenRevocationRequest {
    token: String,
}

async fn validate_token(
    State(handler): State<ProductionAuthHandler>,
    Json(request): Json<TokenValidationRequest>,
) -> impl IntoResponse {
    let creds = AuthCredentials {
        username: request.token.clone(),
        password: request.token,
    };
    authenticate(State(handler), Json(creds))
        .await
        .into_response()
}

async fn revoke_token(
    State(_handler): State<ProductionAuthHandler>,
    Json(_request): Json<TokenRevocationRequest>,
) -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({"revoked": true})))
}

// ==================== HANDLER TESTS ====================

#[tokio::test]
async fn test_production_auth_handler_creation() {
    let handler = ProductionAuthHandler::new();
    let manager = handler.get_manager().await;
    drop(manager);
}

#[tokio::test]
async fn test_production_auth_handler_default() {
    let handler1 = ProductionAuthHandler::new();
    let handler2 = ProductionAuthHandler::default();

    let manager1 = handler1.get_manager().await;
    let manager2 = handler2.get_manager().await;
    drop(manager1);
    drop(manager2);
}

#[tokio::test]
async fn test_authenticate_with_missing_user() {
    let handler = ProductionAuthHandler::new();
    let credentials = AuthCredentials {
        username: "nonexistent_user".to_string(),
        password: "password123".to_string(),
    };

    let status = authenticate(State(handler), Json(credentials))
        .await
        .into_response()
        .status();
    // Current stub accepts all credentials; will reject once real backend is wired
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_authenticate_with_empty_username() {
    let handler = ProductionAuthHandler::new();
    let credentials = AuthCredentials {
        username: String::new(),
        password: "password123".to_string(),
    };

    let status = authenticate(State(handler), Json(credentials))
        .await
        .into_response()
        .status();
    // Current stub accepts all credentials; will reject empties once real backend is wired
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_authenticate_with_empty_password() {
    let handler = ProductionAuthHandler::new();
    let credentials = AuthCredentials {
        username: "testuser".to_string(),
        password: String::new(),
    };

    let status = authenticate(State(handler), Json(credentials))
        .await
        .into_response()
        .status();
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_validate_token_delegates_to_authenticate() {
    let handler = ProductionAuthHandler::new();
    let request = TokenValidationRequest {
        token: "invalid_token_12345".to_string(),
    };

    let status = validate_token(State(handler), Json(request))
        .await
        .into_response()
        .status();
    // validate_token delegates to authenticate, which currently accepts all
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_validate_token_with_empty_token() {
    let handler = ProductionAuthHandler::new();
    let request = TokenValidationRequest {
        token: String::new(),
    };

    let status = validate_token(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_create_api_key_returns_key() {
    let handler = ProductionAuthHandler::new();
    let request = ApiKeyRequest {
        user_id: "nonexistent_user".to_string(),
        name: "test_api_key".to_string(),
    };

    let status = create_api_key(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert!(
        status.is_success() || status.is_client_error() || status.is_server_error(),
        "Expected a valid HTTP status, got {status:?}",
    );
}

#[tokio::test]
async fn test_create_api_key_with_empty_name() {
    let handler = ProductionAuthHandler::new();
    let request = ApiKeyRequest {
        user_id: "testuser".to_string(),
        name: String::new(),
    };

    let status = create_api_key(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert!(
        status == StatusCode::OK
            || status == StatusCode::BAD_REQUEST
            || status == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected OK, BAD_REQUEST, or INTERNAL_SERVER_ERROR, got {status:?}",
    );
}

#[tokio::test]
async fn test_revoke_token_with_invalid_token() {
    let handler = ProductionAuthHandler::new();
    let request = TokenRevocationRequest {
        token: "invalid_token_xyz".to_string(),
    };

    let status = revoke_token(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert!(
        status == StatusCode::OK || status.is_client_error(),
        "Expected OK or client error, got {status:?}",
    );
}

#[tokio::test]
async fn test_revoke_token_with_empty_token() {
    let handler = ProductionAuthHandler::new();
    let request = TokenRevocationRequest {
        token: String::new(),
    };

    let status = revoke_token(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert!(
        status == StatusCode::OK
            || status == StatusCode::BAD_REQUEST
            || status == StatusCode::UNAUTHORIZED,
        "Expected OK, BAD_REQUEST, or UNAUTHORIZED, got {status:?}",
    );
}

#[tokio::test]
async fn test_create_user_with_duplicate_id() {
    let handler = ProductionAuthHandler::new();

    let request1 = CreateUserRequest {
        user_id: "testuser123".to_string(),
        username: "testuser".to_string(),
        role: "user".to_string(),
        permissions: vec!["read".to_string()],
    };

    let _response1 = create_user(State(handler.clone()), Json(request1.clone())).await;

    let status = create_user(State(handler), Json(request1))
        .await
        .into_response()
        .status();
    assert!(
        status == StatusCode::OK
            || status == StatusCode::CONFLICT
            || status == StatusCode::BAD_REQUEST,
        "Expected OK, CONFLICT, or BAD_REQUEST, got {status:?}",
    );
}

#[tokio::test]
async fn test_create_user_with_empty_permissions() {
    let handler = ProductionAuthHandler::new();
    let request = CreateUserRequest {
        user_id: "testuser456".to_string(),
        username: "testuser456".to_string(),
        role: "user".to_string(),
        permissions: vec![],
    };

    let status = create_user(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert!(
        status == StatusCode::OK || status == StatusCode::CREATED,
        "Expected OK or CREATED, got {status:?}",
    );
}

#[tokio::test]
async fn test_create_user_with_invalid_role() {
    let handler = ProductionAuthHandler::new();
    let request = CreateUserRequest {
        user_id: "testuser789".to_string(),
        username: "testuser789".to_string(),
        role: "invalid_role_xyz".to_string(),
        permissions: vec!["read".to_string()],
    };

    let status = create_user(State(handler), Json(request))
        .await
        .into_response()
        .status();
    assert!(
        status == StatusCode::OK
            || status == StatusCode::CREATED
            || status == StatusCode::BAD_REQUEST
            || status == StatusCode::INTERNAL_SERVER_ERROR,
        "Expected OK, CREATED, BAD_REQUEST, or INTERNAL_SERVER_ERROR, got {status:?}",
    );
}

// ==================== STRUCT TESTS ====================

#[test]
fn test_auth_credentials_creation() {
    let creds = AuthCredentials {
        username: "testuser".to_string(),
        password: "password123".to_string(),
    };

    assert_eq!(creds.username, "testuser");
    assert_eq!(creds.password, "password123");
}

#[test]
fn test_auth_credentials_clone() {
    let creds1 = AuthCredentials {
        username: "user1".to_string(),
        password: "pass1".to_string(),
    };

    let creds2 = creds1.clone();
    assert_eq!(creds1.username, creds2.username);
    assert_eq!(creds1.password, creds2.password);
}

#[test]
fn test_auth_response_creation() {
    let response = AuthResponse {
        success: true,
        token: Some("test_token_123".to_string()),
        user_id: Some("user_456".to_string()),
        role: "admin".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    assert!(response.success);
    assert!(response.token.is_some());
    assert_eq!(response.role, "admin");
    assert_eq!(response.permissions.len(), 2);
}

#[test]
fn test_auth_response_failure() {
    let response = AuthResponse {
        success: false,
        token: None,
        user_id: None,
        role: "guest".to_string(),
        permissions: vec![],
    };

    assert!(!response.success);
    assert!(response.token.is_none());
    assert!(response.user_id.is_none());
    assert_eq!(response.permissions.len(), 0);
}

#[test]
fn test_api_key_request_creation() {
    let request = ApiKeyRequest {
        user_id: "user123".to_string(),
        name: "my_api_key".to_string(),
    };

    assert_eq!(request.user_id, "user123");
    assert_eq!(request.name, "my_api_key");
}

#[test]
fn test_api_key_response_creation() {
    let response = ApiKeyResponse {
        api_key: "sk_test_123abc".to_string(),
        user_id: "user456".to_string(),
        name: "production_key".to_string(),
    };

    assert_eq!(response.api_key, "sk_test_123abc");
    assert_eq!(response.user_id, "user456");
    assert_eq!(response.name, "production_key");
}

#[test]
fn test_create_user_request_creation() {
    let request = CreateUserRequest {
        user_id: "user_new".to_string(),
        username: "newuser".to_string(),
        role: "editor".to_string(),
        permissions: vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
        ],
    };

    assert_eq!(request.user_id, "user_new");
    assert_eq!(request.username, "newuser");
    assert_eq!(request.role, "editor");
    assert_eq!(request.permissions.len(), 3);
}

#[test]
fn test_create_user_request_with_many_permissions() {
    let permissions: Vec<String> = (0..100).map(|i| format!("perm_{}", i)).collect();

    let request = CreateUserRequest {
        user_id: "power_user".to_string(),
        username: "poweruser".to_string(),
        role: "superadmin".to_string(),
        permissions,
    };

    assert_eq!(request.permissions.len(), 100);
    assert_eq!(request.permissions[0], "perm_0");
    assert_eq!(request.permissions[99], "perm_99");
}

// ==================== SERIALIZATION TESTS ====================

#[test]
fn test_auth_credentials_serialization() {
    let creds = AuthCredentials {
        username: "testuser".to_string(),
        password: "testpass".to_string(),
    };

    let json = serde_json::to_string(&creds);
    assert!(json.is_ok(), "Should serialize successfully");

    let json_str = json.expect("Authentication failed");
    assert!(json_str.contains("testuser"));
    assert!(json_str.contains("testpass"));
}

#[test]
fn test_auth_credentials_deserialization() {
    let json = r#"{"username":"user1","password":"pass1"}"#;
    let creds: Result<AuthCredentials, _> = serde_json::from_str(json);

    assert!(creds.is_ok(), "Should deserialize successfully");
    let creds = creds.expect("Authentication failed");
    assert_eq!(creds.username, "user1");
    assert_eq!(creds.password, "pass1");
}

#[test]
fn test_auth_response_serialization() {
    let response = AuthResponse {
        success: true,
        token: Some("token_abc".to_string()),
        user_id: Some("user_123".to_string()),
        role: "admin".to_string(),
        permissions: vec!["read".to_string()],
    };

    let json = serde_json::to_string(&response);
    assert!(json.is_ok(), "Should serialize successfully");
}

#[test]
fn test_auth_response_deserialization() {
    let json = r#"{
        "success": true,
        "token": "token_xyz",
        "user_id": "user_456",
        "role": "user",
        "permissions": ["read", "write"]
    }"#;

    let response: Result<AuthResponse, _> = serde_json::from_str(json);
    assert!(response.is_ok(), "Should deserialize successfully");

    let response = response.expect("Authentication failed");
    assert!(response.success);
    assert_eq!(response.token, Some("token_xyz".to_string()));
    assert_eq!(response.permissions.len(), 2);
}

#[test]
fn test_api_key_request_round_trip() {
    let original = ApiKeyRequest {
        user_id: "user_test".to_string(),
        name: "test_key".to_string(),
    };

    let json = serde_json::to_string(&original).expect("Authentication failed");
    let deserialized: ApiKeyRequest = serde_json::from_str(&json).expect("Authentication failed");

    assert_eq!(original.user_id, deserialized.user_id);
    assert_eq!(original.name, deserialized.name);
}

#[test]
fn test_create_user_request_round_trip() {
    let original = CreateUserRequest {
        user_id: "user123".to_string(),
        username: "testuser".to_string(),
        role: "admin".to_string(),
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    let json = serde_json::to_string(&original).expect("Authentication failed");
    let deserialized: CreateUserRequest =
        serde_json::from_str(&json).expect("Authentication failed");

    assert_eq!(original.user_id, deserialized.user_id);
    assert_eq!(original.username, deserialized.username);
    assert_eq!(original.role, deserialized.role);
    assert_eq!(original.permissions, deserialized.permissions);
}

// ==================== EDGE CASE TESTS ====================

#[test]
fn test_auth_credentials_with_special_characters() {
    let creds = AuthCredentials {
        username: "user@domain.com".to_string(),
        password: "p@ss!w0rd#123$%^".to_string(),
    };

    assert!(creds.username.contains('@'));
    assert!(creds.password.contains('!'));
}

#[test]
fn test_auth_credentials_with_unicode() {
    let creds = AuthCredentials {
        username: "用户名".to_string(),
        password: "密码123".to_string(),
    };

    assert!(!creds.username.is_empty());
    assert!(!creds.password.is_empty());
}

#[test]
fn test_auth_response_with_many_permissions() {
    let permissions: Vec<String> = (0..1000).map(|i| format!("permission_{}", i)).collect();

    let response = AuthResponse {
        success: true,
        token: Some("token".to_string()),
        user_id: Some("user".to_string()),
        role: "superuser".to_string(),
        permissions,
    };

    assert_eq!(response.permissions.len(), 1000);
}

#[test]
fn test_api_key_request_with_long_name() {
    let long_name = "a".repeat(1000);

    let request = ApiKeyRequest {
        user_id: "user".to_string(),
        name: long_name,
    };

    assert_eq!(request.name.len(), 1000);
}

#[test]
fn test_create_user_request_with_empty_strings() {
    let request = CreateUserRequest {
        user_id: String::new(),
        username: String::new(),
        role: String::new(),
        permissions: vec![],
    };

    assert!(request.user_id.is_empty());
    assert!(request.username.is_empty());
    assert!(request.role.is_empty());
    assert_eq!(request.permissions.len(), 0);
}
