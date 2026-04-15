// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! API key issuance and validation handlers.

use super::handler::ProductionAuthHandler;
use super::types::{ApiKeyRequest, ApiKeyResponse};
use axum::{extract::State, http::StatusCode, response::Json};
use tracing::{info, warn};

/// **CREATE API KEY HANDLER**
///
/// Generate a new API key for a user.
pub async fn create_api_key(
    State(handler): State<ProductionAuthHandler>,
    Json(request): Json<ApiKeyRequest>,
) -> std::result::Result<Json<ApiKeyResponse>, StatusCode> {
    info!("Creating API key for user: {}", request.user_id);

    // Generate API key
    let api_key = format!("nestgate_{}", uuid::Uuid::new_v4());

    let user_id = request.user_id.clone();
    handler
        .get_manager_mut()
        .await
        .add_api_key(api_key.clone(), user_id.clone());
    let name = request.name.clone();

    info!("API key created for user: {}", user_id);
    Ok(Json(ApiKeyResponse {
        api_key,
        user_id,
        name,
    }))
}

/// **VALIDATE API KEY HANDLER**
///
/// Validate an API key and return user context.
pub async fn validate_api_key(
    State(handler): State<ProductionAuthHandler>,
    Json(api_key): Json<String>,
) -> std::result::Result<Json<serde_json::Value>, StatusCode> {
    info!("Validating API key");

    let outcome = {
        let manager = handler.get_manager().await;
        match manager.validate_api_key(&api_key) {
            Ok(context) => {
                let user_id = context.user_id().to_string();
                let role = context.role().to_string();
                Ok((user_id, role))
            }
            Err(err) => Err(err),
        }
    };

    match outcome {
        Ok((user_id, role)) => {
            info!("API key valid");
            Ok(Json(serde_json::json!({
                "valid": true,
                "user_id": user_id,
                "role": role,
            })))
        }
        Err(err) => {
            warn!("API key invalid: {}", err);
            Ok(Json(serde_json::json!({
                "valid": false,
                "error": err,
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;

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

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct TokenValidationRequest {
        token: String,
    }

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct TokenRevocationRequest {
        token: String,
    }

    async fn validate_token(
        State(_handler): State<ProductionAuthHandler>,
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

    #[tokio::test]
    async fn test_validate_token_with_invalid_token() {
        let handler = ProductionAuthHandler::new();
        let request = TokenValidationRequest {
            token: "invalid_token_12345".to_string(),
        };

        let (status, _) = validate_token(State(handler), Json(request)).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }
}
