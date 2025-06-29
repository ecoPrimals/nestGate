//! Authentication handler supporting dual-mode operation
//!
//! Handles authentication in both standalone and BearDog integrated modes

use crate::models::ErrorResponse;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json as ResponseJson},
};
use nestgate_core::cert::{BearDogConfig, CertMode, CertValidator};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Authentication request payload
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    /// Client certificate in PEM format
    pub certificate: String,
    /// Optional service identifier
    pub service_id: Option<String>,
    /// Authentication mode preference (standalone, beardog, hybrid)
    pub mode: Option<String>,
}

/// Authentication response payload
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    /// Whether authentication was successful
    pub authenticated: bool,
    /// Authentication mode used
    pub mode: String,
    /// Session token (if applicable)
    pub token: Option<String>,
    /// Expiration time in seconds
    pub expires_in: Option<u64>,
    /// Additional metadata
    pub metadata: Option<serde_json::Value>,
}

/// Authentication service state
#[derive(Debug, Clone)]
pub struct AuthService {
    /// Certificate validator
    pub validator: Arc<RwLock<CertValidator>>,
    /// Default authentication mode
    pub default_mode: CertMode,
    /// BearDog configuration (if available)
    pub beardog_config: Option<BearDogConfig>,
}

impl AuthService {
    /// Create new authentication service in standalone mode
    pub fn standalone() -> Self {
        Self {
            validator: Arc::new(RwLock::new(CertValidator::standalone())),
            default_mode: CertMode::Standalone,
            beardog_config: None,
        }
    }

    /// Create new authentication service with BearDog integration
    pub fn with_beardog(config: BearDogConfig) -> Self {
        Self {
            validator: Arc::new(RwLock::new(CertValidator::with_beardog(config.clone()))),
            default_mode: CertMode::BearDog,
            beardog_config: Some(config),
        }
    }

    /// Create hybrid authentication service
    pub fn hybrid(config: BearDogConfig) -> Self {
        Self {
            validator: Arc::new(RwLock::new(CertValidator::hybrid(config.clone()))),
            default_mode: CertMode::Hybrid,
            beardog_config: Some(config),
        }
    }

    /// Get current authentication mode
    pub async fn current_mode(&self) -> CertMode {
        let validator = self.validator.read().await;
        validator.mode().clone()
    }

    /// Check if BearDog integration is available
    pub async fn beardog_available(&self) -> bool {
        let validator = self.validator.read().await;
        validator.beardog_available().await
    }
}

/// Authenticate client certificate
pub async fn authenticate(
    State(auth_service): State<AuthService>,
    headers: HeaderMap,
    Json(request): Json<AuthRequest>,
) -> impl IntoResponse {
    // Validate request
    if request.certificate.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                message: "Certificate required".to_string(),
                code: None,
                details: Some(serde_json::Value::String(
                    "Client certificate must be provided".to_string(),
                )),
            }),
        )
            .into_response();
    }

    // Determine authentication mode
    let requested_mode = request.mode.as_deref().unwrap_or("default");
    let current_mode = auth_service.current_mode().await;

    // Validate certificate
    let mut validator = auth_service.validator.write().await;
    let auth_result = match validator.validate_cert(&request.certificate).await {
        Ok(is_valid) => is_valid,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                ResponseJson(ErrorResponse {
                    message: "Authentication failed".to_string(),
                    code: None,
                    details: Some(serde_json::Value::String(format!(
                        "Validation error: {}",
                        e
                    ))),
                }),
            )
                .into_response();
        }
    };

    // Generate response
    let response = if auth_result {
        // Successful authentication
        let mode_str = match current_mode {
            CertMode::Standalone => "standalone",
            CertMode::BearDog => "beardog",
            CertMode::Hybrid => "hybrid",
        };

        // Generate session token (simplified)
        let token = format!("nestgate_{}_{}", mode_str, chrono::Utc::now().timestamp());

        AuthResponse {
            authenticated: true,
            mode: mode_str.to_string(),
            token: Some(token),
            expires_in: Some(3600), // 1 hour
            metadata: Some(serde_json::json!({
                "service_id": request.service_id,
                "auth_time": chrono::Utc::now().to_rfc3339(),
                "beardog_available": auth_service.beardog_available().await,
            })),
        }
    } else {
        // Failed authentication
        AuthResponse {
            authenticated: false,
            mode: match current_mode {
                CertMode::Standalone => "standalone",
                CertMode::BearDog => "beardog",
                CertMode::Hybrid => "hybrid",
            }
            .to_string(),
            token: None,
            expires_in: None,
            metadata: None,
        }
    };

    let status_code = if auth_result {
        StatusCode::OK
    } else {
        StatusCode::UNAUTHORIZED
    };

    (status_code, ResponseJson(response)).into_response()
}

/// Get authentication service status
pub async fn auth_status(State(auth_service): State<AuthService>) -> impl IntoResponse {
    let current_mode = auth_service.current_mode().await;
    let beardog_available = auth_service.beardog_available().await;

    let response = serde_json::json!({
        "mode": match current_mode {
            CertMode::Standalone => "standalone",
            CertMode::BearDog => "beardog",
            CertMode::Hybrid => "hybrid",
        },
        "beardog_available": beardog_available,
        "capabilities": {
            "standalone": true,
            "beardog": auth_service.beardog_config.is_some(),
            "hybrid": auth_service.beardog_config.is_some(),
        },
        "status": "operational",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    (StatusCode::OK, ResponseJson(response)).into_response()
}

/// Switch authentication mode (if supported)
pub async fn switch_mode(
    State(auth_service): State<AuthService>,
    Json(request): Json<serde_json::Value>,
) -> impl IntoResponse {
    let requested_mode = request
        .get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("hybrid");

    match requested_mode {
        "standalone" => {
            // Switch to standalone mode
            let new_validator = CertValidator::standalone();
            let mut validator = auth_service.validator.write().await;
            *validator = new_validator;

            (
                StatusCode::OK,
                ResponseJson(serde_json::json!({
                    "mode": "standalone",
                    "status": "switched",
                    "message": "Authentication mode switched to standalone"
                })),
            )
                .into_response()
        }
        "beardog" => {
            if let Some(config) = &auth_service.beardog_config {
                let new_validator = CertValidator::with_beardog(config.clone());
                let mut validator = auth_service.validator.write().await;
                *validator = new_validator;

                (
                    StatusCode::OK,
                    ResponseJson(serde_json::json!({
                        "mode": "beardog",
                        "status": "switched",
                        "message": "Authentication mode switched to BearDog"
                    })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    ResponseJson(ErrorResponse {
                        message: "BearDog not configured".to_string(),
                        code: None,
                        details: Some(serde_json::Value::String(
                            "BearDog configuration required for this mode".to_string(),
                        )),
                    }),
                )
                    .into_response()
            }
        }
        "hybrid" => {
            if let Some(config) = &auth_service.beardog_config {
                let new_validator = CertValidator::hybrid(config.clone());
                let mut validator = auth_service.validator.write().await;
                *validator = new_validator;

                (
                    StatusCode::OK,
                    ResponseJson(serde_json::json!({
                        "mode": "hybrid",
                        "status": "switched",
                        "message": "Authentication mode switched to hybrid"
                    })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    ResponseJson(ErrorResponse {
                        message: "BearDog not configured".to_string(),
                        code: None,
                        details: Some(serde_json::Value::String(
                            "BearDog configuration required for hybrid mode".to_string(),
                        )),
                    }),
                )
                    .into_response()
            }
        }
        _ => (
            StatusCode::BAD_REQUEST,
            ResponseJson(ErrorResponse {
                message: "Invalid mode".to_string(),
                code: None,
                details: Some(serde_json::Value::String(
                    "Supported modes: standalone, beardog, hybrid".to_string(),
                )),
            }),
        )
            .into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use nestgate_core::cert::CertUtils;

    #[tokio::test]
    async fn test_auth_service_standalone() {
        let service = AuthService::standalone();
        let mode = service.current_mode().await;
        assert_eq!(mode, CertMode::Standalone);
        assert!(!service.beardog_available().await);
    }

    #[tokio::test]
    async fn test_auth_service_beardog() {
        let config = BearDogConfig::default();
        let service = AuthService::with_beardog(config);
        let mode = service.current_mode().await;
        assert_eq!(mode, CertMode::BearDog);
    }

    #[tokio::test]
    async fn test_auth_service_hybrid() {
        let config = BearDogConfig::default();
        let service = AuthService::hybrid(config);
        let mode = service.current_mode().await;
        assert_eq!(mode, CertMode::Hybrid);
    }

    #[tokio::test]
    async fn test_auth_request_validation() {
        let service = AuthService::standalone();

        // Test valid certificate
        let cert = CertUtils::generate_self_signed().unwrap();
        let request = AuthRequest {
            certificate: cert,
            service_id: Some("test-service".to_string()),
            mode: Some("standalone".to_string()),
        };

        // Test certificate validation
        let mut validator = service.validator.write().await;
        let result = validator.validate_cert(&request.certificate).await;
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_auth_response_serialization() {
        let response = AuthResponse {
            authenticated: true,
            mode: "standalone".to_string(),
            token: Some("test-token".to_string()),
            expires_in: Some(3600),
            metadata: Some(serde_json::json!({"test": "data"})),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(!json.is_empty());
        assert!(json.contains("authenticated"));
        assert!(json.contains("standalone"));
    }

    #[test]
    fn test_auth_request_deserialization() {
        let json = r#"{
            "certificate": "-----BEGIN CERTIFICATE-----\ntest\n-----END CERTIFICATE-----",
            "service_id": "test-service",
            "mode": "hybrid"
        }"#;

        let request: AuthRequest = serde_json::from_str(json).unwrap();
        assert!(!request.certificate.is_empty());
        assert_eq!(request.service_id, Some("test-service".to_string()));
        assert_eq!(request.mode, Some("hybrid".to_string()));
    }
}
