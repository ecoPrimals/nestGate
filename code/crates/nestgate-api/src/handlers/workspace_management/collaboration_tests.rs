// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Workspace collaboration handlers return HTTP 501 with a JSON body describing required
//! external capabilities (identity, ACLs, UI).

use super::collaboration::*;
use axum::{Json, extract::Path, http::StatusCode};

fn assert_share_unavailable(status: StatusCode, body: &serde_json::Value) {
    assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body["error"].as_str(),
        Some("workspace_sharing_not_available")
    );
    assert!(body["message"].as_str().is_some_and(|m| !m.is_empty()));
}

fn assert_unshare_unavailable(status: StatusCode, body: &serde_json::Value) {
    assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    assert_eq!(
        body["error"].as_str(),
        Some("workspace_unsharing_not_available")
    );
}

#[cfg(test)]
mod share_tests {
    use super::*;

    #[test]
    fn test_share_workspace_not_implemented() {
        let (status, Json(body)) = share_workspace(Path("test-ws".to_string()));
        assert_share_unavailable(status, &body);
    }

    #[test]
    fn test_share_with_various_ids() {
        for id in [
            "simple",
            "my-workspace-123",
            "my_workspace_456",
            "org-789-workspace",
            "",
            &(String::from("workspace-") + &"a".repeat(200)),
        ] {
            let (status, Json(body)) = share_workspace(Path(id.to_string()));
            assert_share_unavailable(status, &body);
        }
    }
}

#[cfg(test)]
mod unshare_tests {
    use super::*;

    #[test]
    fn test_unshare_workspace_not_implemented() {
        let (status, Json(body)) = unshare_workspace(Path("test-ws".to_string()));
        assert_unshare_unavailable(status, &body);
    }

    #[test]
    fn test_unshare_with_various_ids() {
        for id in [
            "simple",
            "my-workspace-789",
            "my_workspace_abc",
            "org-456-workspace",
            "",
            &(String::from("workspace-") + &"b".repeat(200)),
        ] {
            let (status, Json(body)) = unshare_workspace(Path(id.to_string()));
            assert_unshare_unavailable(status, &body);
        }
    }
}

#[cfg(test)]
mod consistency_tests {
    use super::*;

    #[test]
    fn test_both_functions_return_same_status() {
        let workspace_id = "test-ws-123".to_string();
        let (s1, Json(b1)) = share_workspace(Path(workspace_id.clone()));
        let (s2, Json(b2)) = unshare_workspace(Path(workspace_id));
        assert_eq!(s1, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(s2, StatusCode::NOT_IMPLEMENTED);
        assert!(b1["message"].is_string());
        assert!(b2["message"].is_string());
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_no_panics_on_share_with_special_chars() {
        for special_id in ["ws@123", "ws#456", "ws$789"] {
            let (status, Json(body)) = share_workspace(Path(special_id.to_string()));
            assert_share_unavailable(status, &body);
        }
    }

    #[test]
    fn test_no_panics_on_unshare_with_special_chars() {
        for special_id in ["ws@123", "ws#456", "ws$789"] {
            let (status, Json(body)) = unshare_workspace(Path(special_id.to_string()));
            assert_unshare_unavailable(status, &body);
        }
    }
}
