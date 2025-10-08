//
// Secrets management functionality that delegates to available security modules.
// This is outside NestGate's core storage focus and should be handled by
// dedicated security infrastructure.

use axum::{extract::Json, extract::Path, http::StatusCode};
use nestgate_core::zero_cost_security_provider::AuthTokenManager;
use serde_json::{json, Value};
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
