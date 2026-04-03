// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//
// Sharing and collaboration functionality for workspaces.
// Sharing is an authz concern; NestGate core storage does not implement multi-tenant ACLs here.

use axum::{Json, extract::Path, http::StatusCode};
use serde_json::{Value, json};
use tracing::info;

const SHARE_MSG: &str = "Workspace sharing requires an authenticated identity service, ACL/role \
    storage, and optional UI (BiomeOS). NestGate storage does not expose shareable links or NFS/SMB \
    exports from this API route; integrate security capability (authorization.workspace) when available.";

const UNSHARE_MSG: &str = "Revoking workspace shares requires the same identity and authorization \
    infrastructure as sharing. This endpoint is reserved until security.workspace.revoke IPC is available.";

/// Share workspace (REQUIRES AUTHENTICATION & USER MANAGEMENT)
/// This feature requires Security security module and Management UI components
pub fn share_workspace(Path(workspace_id): Path<String>) -> (StatusCode, Json<Value>) {
    info!("Workspace sharing requested for: {}", workspace_id);
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "workspace_sharing_not_available",
            "message": SHARE_MSG,
            "workspace_id": workspace_id,
        })),
    )
}

/// Unshare workspace (REQUIRES AUTHENTICATION & USER MANAGEMENT)
/// This feature requires Security security module and Management UI components
pub fn unshare_workspace(Path(workspace_id): Path<String>) -> (StatusCode, Json<Value>) {
    info!("Workspace unsharing requested for: {}", workspace_id);
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(json!({
            "error": "workspace_unsharing_not_available",
            "message": UNSHARE_MSG,
            "workspace_id": workspace_id,
        })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;
    use axum::extract::Path;

    #[test]
    fn test_share_workspace_returns_not_implemented() {
        let workspace_id = "test-workspace".to_string();
        let (status, Json(body)) = share_workspace(Path(workspace_id));

        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(
            body["error"].as_str(),
            Some("workspace_sharing_not_available")
        );
        assert!(body["message"].as_str().is_some_and(|m| !m.is_empty()));
    }

    #[test]
    fn test_unshare_workspace_returns_not_implemented() {
        let workspace_id = "test-workspace".to_string();
        let (status, Json(body)) = unshare_workspace(Path(workspace_id));

        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(
            body["error"].as_str(),
            Some("workspace_unsharing_not_available")
        );
    }

    #[test]
    fn test_share_workspace_with_various_ids() {
        let workspace_ids = vec![
            "workspace-1",
            "org-123-ws",
            "my_workspace",
            "test-ws-abc-123",
        ];

        for workspace_id in workspace_ids {
            let (status, _) = share_workspace(Path(workspace_id.to_string()));
            assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        }
    }

    #[test]
    fn test_unshare_workspace_with_various_ids() {
        let workspace_ids = vec![
            "workspace-1",
            "org-123-ws",
            "my_workspace",
            "test-ws-abc-123",
        ];

        for workspace_id in workspace_ids {
            let (status, _) = unshare_workspace(Path(workspace_id.to_string()));
            assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        }
    }
}
