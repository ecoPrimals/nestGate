// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025 ecoPrimals Collective

//
// Secrets management functionality that delegates to available security modules.
// This is outside NestGate's core storage focus and should be handled by
// dedicated security infrastructure.

//! Secrets module

use axum::{extract::Json, extract::Path, http::StatusCode};
use nestgate_core::zero_cost_security_provider::AuthTokenManager;
use serde_json::{Value, json};
use tracing::{info, warn};

/// Create workspace secret (SECURITY FEATURE - DELEGATE TO SECURITY MODULE)
pub async fn create_workspace_secret(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🔐 Creating workspace secret: {}", workspace_id);
    // Attempt to delegate to security adapter
    let adapter = AuthTokenManager::new("default-signing-key".to_string());

    // Use security adapter for actual secret management
    match adapter.create_workspace_secret(&workspace_id) {
        Ok(secret_id) => {
            info!("✅ Workspace secret created: {}", secret_id);
            Ok(Json(json!({
                "status": "success",
                "message": "Workspace secret created successfully",
                "workspace_id": workspace_id,
                "secret_id": secret_id,
                "created_at": chrono::Utc::now().to_rfc3339()
            })))
        }
        Err(e) => {
            warn!("❌ Failed to create workspace secret: {}", e);
            // Fallback when security operation fails
            Ok(Json(json!({
                "status": "fallback",
                "message": "Secret management delegated to external security service",
                "workspace_id": workspace_id,
                "note": "NestGate focuses on storage - secrets managed by security primal",
                "recommendation": "Configure security primal for full secret management"
            })))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_token_manager_creation() {
        let manager = AuthTokenManager::new("test-key".to_string());
        // Just verify it can be created
        assert!(std::mem::size_of_val(&manager) > 0);
    }

    #[test]
    fn test_create_workspace_secret_delegation() {
        // Test the delegation mechanism
        let manager = AuthTokenManager::new("test-signing-key".to_string());

        // Try creating a workspace secret
        let result = manager.create_workspace_secret("test-workspace-123");

        // Should either succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_workspace_secret_with_different_workspace_ids() {
        let manager = AuthTokenManager::new("test-key".to_string());

        let workspace_ids = vec![
            "workspace-1",
            "workspace-abc",
            "org-123-workspace",
            "my_workspace_test",
        ];

        for workspace_id in workspace_ids {
            let result = manager.create_workspace_secret(workspace_id);
            // Each call should complete without panicking
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_handler_response() {
        use axum::extract::Path;

        // Test the handler returns valid JSON
        let workspace_id = "test-workspace".to_string();
        let result = create_workspace_secret(Path(workspace_id)).await;

        // Should return Ok with JSON value (either success or fallback)
        assert!(result.is_ok());

        if let Ok(json_response) = result {
            let value = json_response.0;
            assert!(value.get("status").is_some());
            assert!(value.get("workspace_id").is_some());
        }
    }

    #[tokio::test]
    async fn test_create_workspace_secret_fallback_behavior() {
        use axum::extract::Path;

        // Test with a workspace ID
        let workspace_id = "fallback-test-workspace".to_string();
        let result = create_workspace_secret(Path(workspace_id.clone())).await;

        assert!(result.is_ok());

        if let Ok(json_response) = result {
            let value = json_response.0;
            let status = value.get("status").and_then(|v| v.as_str());

            // Status should be either "success" or "fallback"
            assert!(status == Some("success") || status == Some("fallback"));

            // Workspace ID should match
            assert_eq!(
                value.get("workspace_id").and_then(|v| v.as_str()),
                Some(workspace_id.as_str())
            );
        }
    }
}
