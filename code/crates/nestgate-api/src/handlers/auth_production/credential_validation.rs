// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Credential-based authentication and user provisioning handlers.

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
#[expect(
    clippy::unused_async,
    reason = "Stub API; async needed when capability is wired"
)]
pub async fn authenticate(
    State(_handler): State<ProductionAuthHandler>,
    Json(credentials): Json<AuthCredentials>,
) -> std::result::Result<Json<AuthResponse>, StatusCode> {
    info!(
        "Authentication rejected for user: {} — identity provider not yet wired",
        credentials.username
    );

    Err(StatusCode::UNAUTHORIZED)
}

/// **CREATE USER HANDLER**
///
/// Create a new user with specified role and permissions.
///
/// User provisioning requires an identity-provider capability (e.g.
/// BTSP-backed user store). Until wired, returns `501 NOT IMPLEMENTED`.
#[expect(
    clippy::unused_async,
    reason = "Stub API; async needed when capability is wired"
)]
pub async fn create_user(
    State(_handler): State<ProductionAuthHandler>,
    Json(request): Json<CreateUserRequest>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    info!(
        "User creation rejected for {} — identity provider not yet wired",
        request.username
    );
    Err(StatusCode::NOT_IMPLEMENTED)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn authenticate_returns_unauthorized() {
        let handler = ProductionAuthHandler::new();
        let credentials = AuthCredentials {
            username: "any_user".into(),
            password: "password123".into(),
        };

        let result = authenticate(State(handler), Json(credentials)).await;
        assert_eq!(
            result.unwrap_err(),
            StatusCode::UNAUTHORIZED,
            "no IdP wired — must return 401"
        );
    }

    #[tokio::test]
    async fn create_user_returns_not_implemented() {
        let handler = ProductionAuthHandler::new();
        let request = CreateUserRequest {
            user_id: "test_user".into(),
            username: "testuser".into(),
            role: "user".into(),
            permissions: vec!["read".into()],
        };

        let result = create_user(State(handler), Json(request)).await;
        assert_eq!(
            result.unwrap_err(),
            StatusCode::NOT_IMPLEMENTED,
            "no IdP wired — must return 501"
        );
    }

    #[tokio::test]
    async fn role_display_covers_all_variants() {
        use super::super::auth_manager::Role;
        let variants = [
            Role::Admin,
            Role::Operator,
            Role::Service,
            Role::ReadOnly,
            Role::User,
        ];
        for role in &variants {
            assert!(!role.to_string().is_empty());
        }
    }
}
