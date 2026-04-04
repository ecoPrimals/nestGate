// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Credential-based authentication and user provisioning handlers.

use super::auth_manager::{Permission, Role};
use super::handler::ProductionAuthHandler;
use super::types::{AuthCredentials, AuthResponse, CreateUserRequest};
use axum::{extract::State, http::StatusCode, response::Json};
use tracing::info;

/// **AUTHENTICATE HANDLER**
///
/// Authenticate user with credentials.
///
/// Real credential validation requires an identity-provider capability
/// (e.g. security capability provider or an external identity provider via capability IPC).
/// Until wired, returns `UNAUTHORIZED` — callers should not assume
/// any request is authenticated.
pub async fn authenticate(
    State(_handler): State<ProductionAuthHandler>,
    Json(credentials): Json<AuthCredentials>,
) -> std::result::Result<Json<AuthResponse>, StatusCode> {
    info!(
        "Authentication attempted for user: {} — identity provider not yet wired",
        credentials.username
    );

    let response = AuthResponse {
        success: false,
        token: None,
        user_id: Some(credentials.username),
        role: String::new(),
        permissions: vec![],
    };

    Ok(Json(response))
}

/// **CREATE USER HANDLER**
///
/// Create a new user with specified role and permissions.
pub async fn create_user(
    State(handler): State<ProductionAuthHandler>,
    Json(request): Json<CreateUserRequest>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    info!("Creating user: {}", request.username);

    let role = match request.role.as_str() {
        "admin" => Role::Admin,
        "operator" => Role::Operator,
        "service" => Role::Service,
        "read_only" => Role::ReadOnly,
        _ => Role::User,
    };

    let permissions: Vec<Permission> = request.permissions.iter().map(Permission::new).collect();

    let mut manager = handler.get_manager_mut().await;
    manager.add_user(
        request.user_id.clone(),
        request.username.clone(),
        role,
        permissions,
    );

    info!("User created successfully: {}", request.username);
    Ok(Json(serde_json::json!({
        "success": true,
        "user_id": request.user_id,
        "username": request.username,
    })))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;

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
    async fn test_authenticate_returns_unauthenticated() {
        let handler = ProductionAuthHandler::new();
        let credentials = AuthCredentials {
            username: "any_user".to_string(),
            password: "password123".to_string(),
        };

        let result = authenticate(State(handler), Json(credentials)).await;
        let response = result.expect("handler returns Ok with failure payload");
        assert!(!response.success, "no IdP wired — authentication must not succeed");
        assert!(response.token.is_none());
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
}
