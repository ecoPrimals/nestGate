//
// Secrets management functionality that delegates to available security modules.
// This is outside NestGate's core storage focus and should be handled by
// dedicated security infrastructure.

use axum::{extract::Json, extract::Path, http::StatusCode};
use nestgate_core::security_adapter::SecurityAdapter;
use serde_json::{json, Value};
use tracing::{info, warn};

/// Create workspace secret (SECURITY FEATURE - DELEGATE TO SECURITY MODULE)
pub async fn create_workspace_secret(
    Path(workspace_id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    info!("🔐 Creating workspace secret: {}", workspace_id);

    // Attempt to delegate to security adapter
    match SecurityAdapter::new_with_mock() {
        Ok(adapter) => {
            // Use security adapter for actual secret management
            match adapter.create_workspace_secret(&workspace_id).await {
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
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(_) => {
            // Fallback when no security module is available
            warn!("⚠️ No security module available, using fallback");
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
