// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Authentication handlers delegate to `nestgate-security` (`HybridAuthenticationManager`).
// When no security capability or local auth material is configured, endpoints return
// explicit HTTP errors rather than hardcoded success or placeholder credentials.
use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
};
use nestgate_security::zero_cost_security_provider::{
    AuthenticationConfig, HybridAuthenticationManager, ZeroCostAuthToken, ZeroCostCredentials,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Human-readable message when authentication cannot run because nothing is configured
const AUTH_UNCONFIGURED: &str = "Authentication is not configured: set NESTGATE_LOCAL_AUTH_HASH for standalone password \
     auth, or configure AUTH_CAPABILITY_ENDPOINT / NESTGATE_SECURITY_AUTH_ENDPOINT / \
     AUTH_PROVIDER_ENDPOINT for capability-based auth.";

fn authentication_config_snapshot() -> AuthenticationConfig {
    AuthenticationConfig::default()
}

fn auth_http_status(err: &nestgate_core::NestGateError) -> StatusCode {
    let msg = err.to_string();
    if msg.contains("Invalid credentials") {
        StatusCode::UNAUTHORIZED
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    }
}

fn auth_response_from_error(err: nestgate_core::NestGateError) -> AuthResponse {
    AuthResponse {
        success: false,
        token: None,
        expires_at: None,
        permissions: None,
        message: err.to_string(),
    }
}

/// Auth service: delegates to `nestgate-security` hybrid authentication (no hardcoded users).
#[derive(Debug, Clone)]
/// Service implementation for Auth
pub struct AuthService {
    hybrid: Arc<HybridAuthenticationManager>,
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthService {
    /// Creates a new auth service backed by [`HybridAuthenticationManager`].
    #[must_use]
    pub fn new() -> Self {
        Self {
            hybrid: Arc::new(HybridAuthenticationManager::new(
                authentication_config_snapshot(),
            )),
        }
    }

    /// Authenticate via `nestgate-security` (local hash or discovered security capability).
    ///
    /// # Errors
    ///
    /// Returns [`nestgate_core::NestGateError`] when credentials are invalid or no auth path is configured.
    pub async fn authenticate(
        &self,
        credentials: &nestgate_core::universal_traits::Credentials,
    ) -> Result<ZeroCostAuthToken, nestgate_core::NestGateError> {
        let zc = ZeroCostCredentials::new_password(
            credentials.username.clone(),
            credentials.password.clone(),
        );
        self.hybrid.authenticate(&zc).await
    }

    /// Gets Auth Status — stateless HTTP layer has no session; reports unauthenticated.
    #[must_use]
    pub const fn get_auth_status(&self) -> AuthStatus {
        AuthStatus {
            authenticated: false,
            user_id: None,
            permissions: vec![],
        }
    }

    /// Whether a capability-style external auth endpoint is configured (env-based discovery).
    #[must_use]
    pub fn security_primal_available(&self) -> bool {
        authentication_config_snapshot().use_external_auth
    }

    /// Gets Mode — production when external auth endpoint env is set, else development.
    #[must_use]
    pub fn get_mode(&self) -> AuthMode {
        if authentication_config_snapshot().use_external_auth {
            AuthMode::Production
        } else {
            AuthMode::Development
        }
    }
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authcredentials
pub struct AuthCredentials {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}
/// Authentication status response
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authstatus
pub struct AuthStatus {
    /// Authenticated
    pub authenticated: bool,
    /// User identifier
    pub user_id: Option<String>,
    /// Permissions
    pub permissions: Vec<String>,
}
/// Authentication mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
/// Authmode
pub enum AuthMode {
    /// Development
    Development,
    /// Production
    Production,
    /// Testing
    Testing,
}
/// Authentication challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Authchallenge
pub struct AuthChallenge {
    /// Challenge
    pub challenge: String,
    /// Timestamp
    pub timestamp: u64,
    /// Expires At
    pub expires_at: u64,
}
/// Authentication request
#[derive(Debug, Deserialize)]
/// Request parameters for Auth operation
pub struct AuthRequest {
    /// Username for authentication
    pub username: String,
    /// Password for authentication
    pub password: String,
    /// Optional domain for domain-based authentication
    pub domain: Option<String>,
}
/// Authentication response
#[derive(Debug, Serialize)]
/// Response data for Auth operation
pub struct AuthResponse {
    /// Whether the authentication operation was successful
    pub success: bool,
    /// Authentication token if login was successful
    pub token: Option<String>,
    /// Token expiration time if applicable
    pub expires_at: Option<std::time::SystemTime>,
    /// List of user permissions if authentication was successful
    pub permissions: Option<Vec<String>>,
    /// Human-readable message describing the result
    pub message: String,
}
/// Authentication router
pub fn auth_router() -> Router<crate::routes::AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/status", get(get_status))
        .route("/mode", post(set_mode))
}
/// Login endpoint — delegates to `nestgate-security` or returns a service / auth error.
async fn login(
    State(_app_state): State<crate::routes::AppState>,
    Json(request): Json<AuthRequest>,
) -> impl IntoResponse {
    let credentials = nestgate_core::universal_traits::Credentials {
        username: request.username,
        password: request.password,
        mfa_token: None,
        client_info: request.domain,
    };
    let auth_service = AuthService::new();
    match auth_service.authenticate(&credentials).await {
        Ok(token) => Json(AuthResponse {
            success: true,
            token: Some(token.token),
            expires_at: Some(token.expires_at),
            permissions: Some(token.permissions),
            message: "Authentication successful".to_string(),
        })
        .into_response(),
        Err(e) => {
            let status = auth_http_status(&e);
            let body = auth_response_from_error(e);
            (status, Json(body)).into_response()
        }
    }
}

/// Get authentication status endpoint
async fn get_status(State(_app_state): State<crate::routes::AppState>) -> Json<AuthStatus> {
    let auth_service = AuthService::new();
    Json(auth_service.get_auth_status())
}
/// Set authentication mode endpoint
async fn set_mode(
    State(_app_state): State<crate::routes::AppState>,
    Json(request): Json<SetModeRequest>,
) -> Json<SetModeResponse> {
    match request.mode.as_str() {
        "standalone" => Json(SetModeResponse {
            success: true,
            mode: "standalone",
            message: "Authentication mode switched to standalone".to_string(),
        }),
        "security_primal" => {
            let auth_service = AuthService::new();
            if auth_service.security_primal_available() {
                Json(SetModeResponse {
                    success: true,
                    mode: "security_primal",
                    message: "Authentication mode switched to security primal".to_string(),
                })
            } else {
                Json(SetModeResponse {
                    success: false,
                    mode: "standalone",
                    message: "No security capability endpoint configured (set AUTH_CAPABILITY_ENDPOINT, NESTGATE_SECURITY_AUTH_ENDPOINT, or AUTH_PROVIDER_ENDPOINT)".to_string(),
                })
            }
        }
        "hybrid" => Json(SetModeResponse {
            success: true,
            mode: "hybrid",
            message: "Authentication mode switched to hybrid".to_string(),
        }),
        _ => {
            let auth_service = AuthService::new();
            Json(SetModeResponse {
                success: false,
                mode: match auth_service.get_mode() {
                    AuthMode::Development => "development",
                    AuthMode::Production => "production",
                    AuthMode::Testing => "testing",
                },
                message: "Supported modes: standalone, security_primal, hybrid".to_string(),
            })
        }
    }
}
/// Set mode request
#[derive(Debug, Deserialize)]
/// Request parameters for `SetMode` operation
pub struct SetModeRequest {
    /// Authentication mode to set ("standalone", "`security_primal`", etc.)
    pub mode: String,
}
/// Set mode response
#[derive(Debug, Serialize)]
/// Response data for `SetMode` operation
pub struct SetModeResponse {
    /// Whether the mode change was successful
    pub success: bool,
    /// The authentication mode that was set
    pub mode: &'static str,
    /// Human-readable message describing the result
    pub message: String,
}
/// `AppState` with auth service
pub struct AppStateWithAuth {
    /// Auth Service
    pub auth_service: AuthService,
    /// Zfs Manager
    pub zfs_manager: std::sync::Arc<crate::routes::ZfsManager>,
}
impl From<crate::routes::AppState> for AppStateWithAuth {
    /// From
    fn from(state: crate::routes::AppState) -> Self {
        Self {
            auth_service: AuthService::new(),
            zfs_manager: state.zfs_manager,
        }
    }
}

/// Authenticate user with credentials
#[must_use]
pub async fn authenticate_user(
    State(_app_state): State<crate::routes::AppState>,
    Json(credentials): Json<AuthCredentials>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    match auth_service
        .authenticate(&nestgate_core::universal_traits::Credentials {
            username: credentials.username,
            password: credentials.password,
            mfa_token: None,
            client_info: None,
        })
        .await
    {
        Ok(token) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "success",
                "message": "Authentication successful",
                "authenticated": true,
                "token": token.token,
            })),
        )
            .into_response(),
        Err(e) => {
            let status = auth_http_status(&e);
            (
                status,
                Json(serde_json::json!({
                    "status": "error",
                    "message": e.to_string(),
                    "authenticated": false
                })),
            )
                .into_response()
        }
    }
}
/// Get authentication status
#[must_use]
pub fn get_auth_status(State(_app_state): State<crate::routes::AppState>) -> impl IntoResponse {
    let auth_service = AuthService::new();
    Json(auth_service.get_auth_status())
}
/// Get system security status
#[must_use]
pub fn get_security_status(State(_app_state): State<crate::routes::AppState>) -> impl IntoResponse {
    let auth_service = AuthService::new();
    let cfg = authentication_config_snapshot();
    let local_hash = std::env::var("NESTGATE_LOCAL_AUTH_HASH")
        .ok()
        .is_some_and(|s| !s.is_empty());
    let configured = cfg.use_external_auth || local_hash;
    Json(serde_json::json!({
        "security_auth_endpoint_configured": cfg.use_external_auth,
        "local_password_auth_configured": local_hash,
        "auth_mode": auth_service.get_mode(),
        "status": if configured { "configured" } else { "unconfigured" },
        "message": if configured { "Authentication backends available" } else { AUTH_UNCONFIGURED }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_types_serde_roundtrip() {
        let creds = AuthCredentials {
            username: "u".into(),
            password: "p".into(),
        };
        let j = serde_json::to_string(&creds).unwrap();
        let back: AuthCredentials = serde_json::from_str(&j).unwrap();
        assert_eq!(back.username, "u");

        let status = AuthStatus {
            authenticated: false,
            user_id: None,
            permissions: vec!["read".into()],
        };
        let j = serde_json::to_string(&status).unwrap();
        let back: AuthStatus = serde_json::from_str(&j).unwrap();
        assert!(!back.authenticated);

        let mode = AuthMode::Production;
        let j = serde_json::to_string(&mode).unwrap();
        let back: AuthMode = serde_json::from_str(&j).unwrap();
        assert_eq!(back, AuthMode::Production);

        let ch = AuthChallenge {
            challenge: "c".into(),
            timestamp: 1,
            expires_at: 2,
        };
        let j = serde_json::to_string(&ch).unwrap();
        let back: AuthChallenge = serde_json::from_str(&j).unwrap();
        assert_eq!(back.challenge, "c");
    }

    #[test]
    fn auth_response_serializes() {
        let r = AuthResponse {
            success: false,
            token: None,
            expires_at: None,
            permissions: None,
            message: "m".into(),
        };
        let v = serde_json::to_value(&r).unwrap();
        assert_eq!(v["message"], "m");
    }

    #[test]
    fn set_mode_request_deserializes() {
        let req: SetModeRequest = serde_json::from_str(r#"{"mode":"standalone"}"#).unwrap();
        assert_eq!(req.mode, "standalone");
    }

    #[tokio::test]
    async fn test_auth_service_standalone() {
        let service = AuthService::new();
        let mode = service.get_mode();
        assert_eq!(mode, AuthMode::Development);
        assert!(!service.security_primal_available());
    }

    #[tokio::test]
    async fn test_authentication_without_local_hash_returns_error() {
        let service = AuthService::new();

        let credentials = nestgate_core::universal_traits::Credentials {
            username: "admin".to_string(),
            password: "nestgate".to_string(),
            mfa_token: None,
            client_info: None,
        };

        let result = service.authenticate(&credentials).await;
        assert!(result.is_err());

        let error_message = result
            .expect_err("Expected authentication to fail")
            .to_string();
        assert!(
            error_message.contains("NESTGATE_LOCAL_AUTH_HASH")
                || error_message.contains("Security error"),
            "{error_message}"
        );
    }

    #[tokio::test]
    async fn test_invalid_credentials() {
        let service = AuthService::new();

        let credentials = nestgate_core::universal_traits::Credentials {
            username: "invalid".to_string(),
            password: "wrong".to_string(),
            mfa_token: None,
            client_info: None,
        };

        let result = service.authenticate(&credentials).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn auth_router_hits_login_status_and_mode_branches() {
        use axum_test::TestServer;

        let app = auth_router().with_state(crate::routes::AppState::new());
        let server = TestServer::new(app).expect("server");

        let login = server
            .post("/login")
            .json(&serde_json::json!({
                "username": "u",
                "password": "p",
                "domain": "example.test"
            }))
            .await;
        assert_eq!(
            login.status_code(),
            axum::http::StatusCode::SERVICE_UNAVAILABLE
        );

        server.get("/status").await.assert_status_ok();

        for mode in ["standalone", "security_primal", "hybrid", "unknown_mode"] {
            let r = server
                .post("/mode")
                .json(&serde_json::json!({ "mode": mode }))
                .await;
            r.assert_status_ok();
        }
    }
}
