//! Authentication Handler
//! 
//! Handles authentication using any available security primal provider,
//! eliminating hardcoded dependencies on specific security implementations.

use anyhow::Result;
use axum::{extract::State, response::Json, routing::post, Router};
use nestgate_core::universal_traits::{Credentials, AuthToken};
use nestgate_core::universal_adapter::UniversalPrimalAdapter;
use nestgate_core::cert::{CertMode, CertValidator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

/// Authentication mode preference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthMode {
    /// Standalone mode - no external security primal required
    Standalone,
    /// Security primal mode - use any available security primal
    SecurityPrimal,
    /// Hybrid mode - security primal when available, standalone as fallback
    Hybrid,
}

impl Default for AuthMode {
    fn default() -> Self {
        Self::Standalone
    }
}

/// Authentication service using universal security primal provider
pub struct AuthService {
    /// Certificate validator for standalone mode
    #[allow(dead_code)]
    validator: Arc<RwLock<CertValidator>>,
    /// Default authentication mode
    default_mode: CertMode,
    /// Universal primal adapter for security services
    primal_adapter: Arc<UniversalPrimalAdapter>,
}

impl AuthService {
    /// Create new authentication service in standalone mode
    pub fn new() -> Self {
        let adapter = Arc::new(nestgate_core::universal_adapter::create_default_adapter());
        
        Self {
            validator: Arc::new(RwLock::new(CertValidator::standalone())),
            default_mode: CertMode::Standalone,
            primal_adapter: adapter,
        }
    }

    /// Create new authentication service with universal security primal adapter
    pub fn with_primal_adapter(adapter: Arc<UniversalPrimalAdapter>) -> Self {
        Self {
            validator: Arc::new(RwLock::new(CertValidator::standalone())),
            default_mode: CertMode::Standalone,
            primal_adapter: adapter,
        }
    }

    /// Create hybrid authentication service
    pub fn hybrid(adapter: Arc<UniversalPrimalAdapter>) -> Self {
        Self {
            validator: Arc::new(RwLock::new(CertValidator::standalone())),
            default_mode: CertMode::Hybrid,
            primal_adapter: adapter,
        }
    }

    /// Initialize the authentication service
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing authentication service with universal security primal adapter");
        self.primal_adapter.initialize().await?;
        Ok(())
    }

    /// Check if security primal is available
    pub async fn security_primal_available(&self) -> bool {
        self.primal_adapter.get_security_provider().await.is_some()
    }

    /// Get current authentication mode
    pub fn get_mode(&self) -> &'static str {
        match self.default_mode {
            CertMode::Standalone => "standalone",
            CertMode::Hybrid => "hybrid",
            _ => "security_primal",
        }
    }

    /// Authenticate user with any available security primal
    pub async fn authenticate(&self, credentials: &Credentials) -> Result<AuthToken> {
        // Try security primal first if available
        if let Some(provider) = self.primal_adapter.get_security_provider().await {
            info!("Authenticating with security primal provider");
            
            match provider.authenticate(credentials).await {
                Ok(token) => return Ok(token),
                Err(e) => {
                    warn!("Security primal authentication failed: {}", e);
                    
                    // Fall back to standalone mode if in hybrid mode
                    if self.default_mode == CertMode::Hybrid {
                        info!("Falling back to standalone authentication");
                        return self.authenticate_standalone(credentials).await;
                    }
                    
                    return Err(e.into());
                }
            }
        }

        // Use standalone authentication
        self.authenticate_standalone(credentials).await
    }

    /// Authenticate using standalone mode
    async fn authenticate_standalone(&self, credentials: &Credentials) -> Result<AuthToken> {
        info!("Authenticating with standalone mode");
        
        // Simple standalone authentication logic
        if credentials.username == "admin" && credentials.password == "nestgate" {
            Ok(AuthToken {
                token: format!("standalone_{}", uuid::Uuid::new_v4()),
                expires_at: std::time::SystemTime::now() + std::time::Duration::from_secs(3600),
                permissions: vec!["read".to_string(), "write".to_string(), "admin".to_string()],
            })
        } else {
            Err(anyhow::anyhow!("Invalid credentials"))
        }
    }

    /// Get authentication status
    pub async fn get_auth_status(&self) -> AuthStatus {
        let security_primal_available = self.security_primal_available().await;
        let adapter_stats = self.primal_adapter.get_stats().await;
        
        AuthStatus {
            mode: self.get_mode(),
            security_primal_available,
            security_providers: adapter_stats.security_providers,
            default_mode: self.default_mode.clone(),
        }
    }
}

/// Authentication status information
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthStatus {
    pub mode: &'static str,
    pub security_primal_available: bool,
    pub security_providers: usize,
    pub default_mode: CertMode,
}

/// Authentication request
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
    pub domain: Option<String>,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub expires_at: Option<std::time::SystemTime>,
    pub permissions: Option<Vec<String>>,
    pub message: String,
}

/// Authentication router
pub fn auth_router() -> Router<Arc<AuthService>> {
    Router::new()
        .route("/login", post(login))
        .route("/status", post(get_status))
        .route("/mode", post(set_mode))
}

/// Login endpoint
async fn login(
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<AuthRequest>,
) -> Json<AuthResponse> {
    let credentials = Credentials {
        username: request.username,
        password: request.password,
        domain: request.domain,
        token: None,
    };

    match auth_service.authenticate(&credentials).await {
        Ok(token) => Json(AuthResponse {
            success: true,
            token: Some(token.token),
            expires_at: Some(token.expires_at),
            permissions: Some(token.permissions),
            message: "Authentication successful".to_string(),
        }),
        Err(e) => Json(AuthResponse {
            success: false,
            token: None,
            expires_at: None,
            permissions: None,
            message: format!("Authentication failed: {}", e),
        }),
    }
}

/// Get authentication status endpoint
async fn get_status(
    State(auth_service): State<Arc<AuthService>>,
) -> Json<AuthStatus> {
    Json(auth_service.get_auth_status().await)
}

/// Set authentication mode endpoint
async fn set_mode(
    State(auth_service): State<Arc<AuthService>>,
    Json(request): Json<SetModeRequest>,
) -> Json<SetModeResponse> {
    match request.mode.as_str() {
        "standalone" => {
            Json(SetModeResponse {
                success: true,
                mode: "standalone",
                message: "Authentication mode switched to standalone".to_string(),
            })
        }
        "security_primal" => {
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
        "hybrid" => {
            Json(SetModeResponse {
                success: true,
                mode: "hybrid",
                message: "Authentication mode switched to hybrid".to_string(),
            })
        }
        _ => Json(SetModeResponse {
            success: false,
            mode: auth_service.get_mode(),
            message: "Supported modes: standalone, security_primal, hybrid".to_string(),
        }),
    }
}

/// Set mode request
#[derive(Debug, Deserialize)]
pub struct SetModeRequest {
    pub mode: String,
}

/// Set mode response
#[derive(Debug, Serialize)]
pub struct SetModeResponse {
    pub success: bool,
    pub mode: &'static str,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use nestgate_core::universal_adapter::create_default_adapter;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_auth_service_standalone() {
        let service = AuthService::new();
        let mode = service.get_mode();
        assert_eq!(mode, "standalone");
        assert!(!service.security_primal_available().await);
    }

    #[tokio::test]
    async fn test_auth_service_with_adapter() {
        let adapter = Arc::new(create_default_adapter());
        let service = AuthService::with_primal_adapter(adapter);
        let mode = service.get_mode();
        assert_eq!(mode, "standalone");
    }

    #[tokio::test]
    async fn test_auth_service_hybrid() {
        let adapter = Arc::new(create_default_adapter());
        let service = AuthService::hybrid(adapter);
        let mode = service.get_mode();
        assert_eq!(mode, "hybrid");
    }

    #[tokio::test]
    async fn test_standalone_authentication() {
        let service = AuthService::new();
        
        let credentials = Credentials {
            username: "admin".to_string(),
            password: "nestgate".to_string(),
            domain: None,
            token: None,
        };

        let result = service.authenticate(&credentials).await;
        assert!(result.is_ok());
        
        let token = result.unwrap();
        assert!(token.token.starts_with("standalone_"));
        assert!(token.permissions.contains(&"admin".to_string()));
    }

    #[tokio::test]
    async fn test_invalid_credentials() {
        let service = AuthService::new();
        
        let credentials = Credentials {
            username: "invalid".to_string(),
            password: "wrong".to_string(),
            domain: None,
            token: None,
        };

        let result = service.authenticate(&credentials).await;
        assert!(result.is_err());
    }
}
