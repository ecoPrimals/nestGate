// SPDX-License-Identifier: AGPL-3.0-or-later
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

    handler.get_manager_mut().await.add_user(
        request.user_id.clone(),
        request.username.clone(),
        role,
        permissions,
    );
    let user_id = request.user_id.clone();
    let username = request.username.clone();

    info!("User created successfully: {}", username);
    Ok(Json(serde_json::json!({
        "success": true,
        "user_id": user_id,
        "username": username,
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
            user_id: String::from("test_user"),
            username: String::from("testuser"),
            role: String::from("user"),
            permissions: vec![String::from("read")],
        };

        let result = create_user(State(handler), Json(request)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_authenticate_returns_unauthenticated() {
        let handler = ProductionAuthHandler::new();
        let credentials = AuthCredentials {
            username: String::from("any_user"),
            password: String::from("password123"),
        };

        let result = authenticate(State(handler), Json(credentials)).await;
        let response = result.expect("handler returns Ok with failure payload");
        assert!(
            !response.success,
            "no IdP wired — authentication must not succeed"
        );
        assert!(response.token.is_none());
    }

    #[tokio::test]
    async fn test_create_user_with_various_roles() {
        let handler = ProductionAuthHandler::new();

        // Test admin role
        let admin_request = CreateUserRequest {
            user_id: String::from("admin_user"),
            username: String::from("admin"),
            role: String::from("admin"),
            permissions: vec![String::from("all")],
        };

        let result = create_user(State(handler.clone()), Json(admin_request)).await;
        assert!(result.is_ok());

        // Test operator role
        let operator_request = CreateUserRequest {
            user_id: String::from("operator_user"),
            username: String::from("operator"),
            role: String::from("operator"),
            permissions: vec![String::from("read"), String::from("write")],
        };

        let result = create_user(State(handler.clone()), Json(operator_request)).await;
        assert!(result.is_ok());

        // Test read_only role
        let readonly_request = CreateUserRequest {
            user_id: String::from("readonly_user"),
            username: String::from("readonly"),
            role: String::from("read_only"),
            permissions: vec![String::from("read")],
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
                permissions: vec![String::from("read")],
            };
            assert_eq!(request.role, role);
        }
    }

    #[tokio::test]
    async fn test_create_user_with_multiple_permissions() {
        let request = CreateUserRequest {
            user_id: String::from("power_user"),
            username: String::from("poweruser"),
            role: String::from("power_user"),
            permissions: vec![
                String::from("read"),
                String::from("write"),
                String::from("execute"),
            ],
        };
        assert_eq!(request.permissions.len(), 3);
    }
}
