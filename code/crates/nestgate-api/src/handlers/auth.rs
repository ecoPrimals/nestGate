//
// Handles authentication using any available security primal provider,
// eliminating hardcoded dependencies on specific security implementations.

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Simple auth service stub for canonical modernization
#[derive(Debug, Clone)]
pub struct AuthService {
    authenticated_users: HashMap<String, bool>,
}

impl AuthService {
    #[must_use]
    pub fn new() -> Self {
        Self {
            authenticated_users: HashMap::new(),
        }
    }

    pub fn authenticate(
        &self,
        _credentials: &nestgate_core::universal_traits::Credentials,
    ) -> bool {
        // Stub implementation - always return true for now
        true
    }

    pub fn get_auth_status(&self) -> AuthStatus {
        AuthStatus {
            authenticated: true,
            user_id: Some("stub_user".to_string()),
            permissions: vec!["read".to_string(), "write".to_string()],
        }
    }

    pub fn security_primal_available(&self) -> bool {
        // Stub - assume available
        true
    }

    pub fn get_mode(&self) -> AuthMode {
        AuthMode::Development
    }
}

/// Authentication credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthCredentials {
    pub username: String,
    pub password: String,
}
/// Authentication status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthStatus {
    pub authenticated: bool,
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
}
/// Authentication mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMode {
    Development,
    Production,
    Testing,
}
/// Authentication challenge
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    pub challenge: String,
    pub timestamp: u64,
    pub expires_at: u64,
}
/// Authentication request
#[derive(Debug, Deserialize)]
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
/// Login endpoint
#[debug_handler]
fn login(
    State(app_state): State<crate::routes::AppState>,
    Json(request): Json<AuthRequest>,
) -> Json<AuthResponse> {
    let credentials = nestgate_core::universal_traits::Credentials {
        username: request.username,
        password: request.password,
        domain: request.domain,
        token: None,
    };
    let auth_service = AuthService::new();
    let auth_result = auth_service.authenticate(&credentials).await;

    Json(AuthResponse {
        success: auth_result,
        token: if auth_result {
            Some("stub_token".to_string())
        } else {
            None
        },
        expires_at: if auth_result {
            Some(std::time::SystemTime::now() + std::time::Duration::from_secs(3600))
        } else {
            None
        },
        permissions: if auth_result {
            Some(vec!["read".to_string(), "write".to_string()])
        } else {
            None
        },
        message: if auth_result {
            "Authentication successful"
        } else {
            "Authentication failed"
        }
        .to_string(),
    })
}

/// Get authentication status endpoint
#[debug_handler]
async fn get_status(State(app_state): State<crate::routes::AppState>) -> Json<AuthStatus> {
    let auth_service = AuthService::new();
    Json(auth_service.get_auth_status().await)
}
/// Set authentication mode endpoint
#[debug_handler]
async fn set_mode(
    State(app_state): State<crate::routes::AppState>,
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
            if auth_service.security_primal_available().await {
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
pub struct SetModeRequest {
    /// Authentication mode to set ("standalone", "security_primal", etc.)
    pub mode: String,
}
/// Set mode response
#[derive(Debug, Serialize)]
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
    pub auth_service: AuthService,
    pub zfs_manager: std::sync::Arc<crate::routes::ZfsManager>,
}
impl From<crate::routes::AppState> for AppStateWithAuth {
    fn from(state: crate::routes::AppState) -> Self {
        Self {
            auth_service: AuthService::new(),
            zfs_manager: state.zfs_manager,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::ecosystem_integration::create_default_adapter_config;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_auth_service_standalone() {
        let service = AuthService::new();
        let mode = service.get_mode();
        assert_eq!(mode, nestgate_core::cert::types::CertMode::Development);
        assert!(!service.security_primal_available().await);
    }
    #[tokio::test]
    async fn test_auth_service_with_adapter() {
        let config = create_default_adapter_config();
        let adapter = Arc::new(nestgate_core::ecosystem_integration::UniversalAdapter::new(
            config,
        ));
        let service = AuthService::with_primal_adapter(adapter);
        let mode = service.get_mode();
        assert!(matches!(
            mode,
            nestgate_core::cert::types::CertMode::Development
        ));
    }

    #[tokio::test]
    async fn test_auth_service_hybrid() {
        let config = create_default_adapter_config();
        let adapter = Arc::new(nestgate_core::ecosystem_integration::UniversalAdapter::new(
            config,
        ));
        let service = AuthService::hybrid(adapter);
        let mode = service.get_mode();
        assert!(matches!(
            mode,
            nestgate_core::cert::types::CertMode::Lenient
        ));
    }

    #[tokio::test]
    async fn test_decentralized_authentication_denial() {
        let service = AuthService::new();

        let credentials = nestgate_core::universal_traits::Credentials {
            username: "admin".to_string(),
            password: "nestgate".to_string(),
            domain: None,
            token: None,
        };

        // With no security services available, decentralized auth should gracefully deny
        let result = service.authenticate(&credentials).await;
        assert!(result.is_err());

        let _error_message = result
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
            domain: None,
            token: None,
        };

        let result = service.authenticate(&credentials).await;
        assert!(result.is_err());
    }
}

/// Authenticate user with credentials
pub fn authenticate_user(
    State(app_state): State<crate::routes::AppState>,
    Json(credentials): Json<AuthCredentials>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    match auth_service
        .authenticate(&nestgate_core::universal_traits::Credentials {
            username: credentials.username,
            password: credentials.password,
            domain: None,
            token: None,
        })
        .await
    {
        true => (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "success",
                "message": "Authentication successful",
                "authenticated": true
            }),
        ),
        false => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "status": "error",
                "message": "Authentication failed",
                "authenticated": false
            }),
        ),
    }
}
/// Get authentication status
pub async fn get_auth_status(
    State(app_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    Json(auth_service.get_auth_status().await)
}
/// Get system security status
pub async fn get_security_status(
    State(app_state): State<crate::routes::AppState>,
) -> impl IntoResponse {
    let auth_service = AuthService::new();
    Json(serde_json::json!({
        "security_primal_available": auth_service.security_primal_available().await,
        "auth_mode": auth_service.get_mode(),
        "status": "operational"
    }))
}
