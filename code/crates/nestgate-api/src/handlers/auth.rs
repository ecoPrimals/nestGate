//
// Handles authentication using any available security primal provider,
// eliminating hardcoded dependencies on specific security implementations.

#[cfg(test)]
#[path = "auth_comprehensive_tests.rs"]
mod auth_comprehensive_tests;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Feature-not-available error for authentication
const AUTH_NOT_AVAILABLE_MSG: &str =
    "Authentication service is not available. Use nestgate-core security module for production auth.";

// Simple auth service stub for canonical modernization
#[derive(Debug, Clone, Default)]
/// Service implementation for Auth
pub struct AuthService {
    _authenticated_users: HashMap<String, bool>,
}

impl AuthService {
    /// Creates a new auth service (empty user map; production auth uses nestgate-core).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Authenticate - returns Err with feature-not-available (no fake success)
    pub fn authenticate(
        &self,
        _credentials: &nestgate_core::universal_traits::Credentials,
    ) -> Result<bool, nestgate_core::NestGateError> {
        Err(nestgate_core::NestGateError::validation(
            "security.authentication.decentralized: Decentralized authentication required - use nestgate-core security module",
        ))
    }

    /// Gets Auth Status - returns unauthenticated (no fake stub_user)
    pub const fn get_auth_status(&self) -> AuthStatus {
        AuthStatus {
            authenticated: false,
            user_id: None,
            permissions: vec![],
        }
    }

    /// Security Primal Available - returns false (no fake availability)
    pub const fn security_primal_available(&self) -> bool {
        false
    }

    /// Gets Mode
    pub const fn get_mode(&self) -> AuthMode {
        AuthMode::Development
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
/// Login endpoint - returns 501 when auth is not implemented
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
    match auth_service.authenticate(&credentials) {
        Ok(true) => Json(AuthResponse {
            success: true,
            token: None,
            expires_at: None,
            permissions: None,
            message: "Authentication successful".to_string(),
        })
        .into_response(),
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(AuthResponse {
                success: false,
                token: None,
                expires_at: None,
                permissions: None,
                message: "Authentication failed".to_string(),
            }),
        )
            .into_response(),
        Err(_e) => (
            StatusCode::NOT_IMPLEMENTED,
            Json(AuthResponse {
                success: false,
                token: None,
                expires_at: None,
                permissions: None,
                message: AUTH_NOT_AVAILABLE_MSG.to_string(),
            }),
        )
            .into_response(),
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
                    message: "No security primal available".to_string(),
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
/// Request parameters for SetMode operation
pub struct SetModeRequest {
    /// Authentication mode to set ("standalone", "security_primal", etc.)
    pub mode: String,
}
/// Set mode response
#[derive(Debug, Serialize)]
/// Response data for SetMode operation
pub struct SetModeResponse {
    /// Whether the mode change was successful
    pub success: bool,
    /// The authentication mode that was set
    pub mode: &'static str,
    /// Human-readable message describing the result
    pub message: String,
}
/// AppState with auth service
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
pub fn authenticate_user(
    State(_app_state): State<crate::routes::AppState>,
    Json(credentials): Json<AuthCredentials>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    match auth_service.authenticate(&nestgate_core::universal_traits::Credentials {
        username: credentials.username,
        password: credentials.password,
        mfa_token: None,
        client_info: None,
    }) {
        Ok(true) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "success",
                "message": "Authentication successful",
                "authenticated": true
            })),
        )
            .into_response(),
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "status": "error",
                "message": "Authentication failed",
                "authenticated": false
            })),
        )
            .into_response(),
        Err(_) => (
            StatusCode::NOT_IMPLEMENTED,
            Json(serde_json::json!({
                "status": "error",
                "message": AUTH_NOT_AVAILABLE_MSG,
                "authenticated": false
            })),
        )
            .into_response(),
    }
}
/// Get authentication status
pub async fn get_auth_status(
    State(_app_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    Json(auth_service.get_auth_status())
}
/// Get system security status
pub async fn get_security_status(
    State(_app_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    Json(serde_json::json!({
        "security_primal_available": auth_service.security_primal_available(),
        "auth_mode": auth_service.get_mode(),
        "status": "not_implemented",
        "message": AUTH_NOT_AVAILABLE_MSG
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_auth_service_standalone() {
        let service = AuthService::new();
        let mode = service.get_mode();
        assert_eq!(mode, AuthMode::Development);
        assert!(!service.security_primal_available());
    }

    #[tokio::test]
    async fn test_decentralized_authentication_denial() {
        let service = AuthService::new();

        let credentials = nestgate_core::universal_traits::Credentials {
            username: "admin".to_string(),
            password: "nestgate".to_string(),
            mfa_token: None,
            client_info: None,
        };

        // With no security services available, decentralized auth should gracefully deny
        let result = service.authenticate(&credentials);
        assert!(result.is_err());

        let error_message = result
            .expect_err("Expected authentication to fail")
            .to_string();
        assert!(error_message.contains("Decentralized authentication required"));
        assert!(error_message.contains("security.authentication.decentralized"));
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

        let result = service.authenticate(&credentials);
        assert!(result.is_err());
    }
}
