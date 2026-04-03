// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Workspace template tests: create persists JSON under `NESTGATE_WORKSPACE_TEMPLATES_DIR`;
//! apply remains 501 until ZFS orchestration is integrated.

use super::templates::*;
use axum::{Json, extract::Path, http::StatusCode};

fn with_temp_template_dir<F: FnOnce()>(f: F) {
    let dir = tempfile::tempdir().expect("tempdir");
    let p = dir.path().to_str().expect("utf8 path");
    temp_env::with_var("NESTGATE_WORKSPACE_TEMPLATES_DIR", Some(p), f);
}

#[cfg(test)]
mod create_tests {
    use super::*;

    #[test]
    fn test_create_workspace_template_writes_json() {
        with_temp_template_dir(|| {
            let (status, Json(body)) =
                create_workspace_template(Path("test-workspace".to_string()));
            assert_eq!(status, StatusCode::OK);
            assert_eq!(body["status"].as_str(), Some("created"));
        });
    }

    #[test]
    fn test_create_rejects_invalid_id() {
        with_temp_template_dir(|| {
            let (status, _) = create_workspace_template(Path("../bad".to_string()));
            assert_eq!(status, StatusCode::BAD_REQUEST);
        });
    }

    #[test]
    fn test_create_rejects_empty_id() {
        with_temp_template_dir(|| {
            let (status, _) = create_workspace_template(Path(String::new()));
            assert_eq!(status, StatusCode::BAD_REQUEST);
        });
    }

    #[test]
    fn test_create_rejects_too_long_id() {
        with_temp_template_dir(|| {
            let long_id = "a".repeat(300);
            let (status, _) = create_workspace_template(Path(long_id));
            assert_eq!(status, StatusCode::BAD_REQUEST);
        });
    }
}

#[cfg(test)]
mod apply_tests {
    use super::*;

    #[test]
    fn test_apply_workspace_template_not_implemented() {
        let (status, Json(body)) = apply_workspace_template(Path("test-workspace".to_string()));
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
        assert_eq!(
            body["error"].as_str(),
            Some("workspace_template_apply_not_available")
        );
    }
}

#[cfg(test)]
mod consistency_tests {
    use super::*;

    #[test]
    fn test_create_ok_and_apply_still_501() {
        with_temp_template_dir(|| {
            let workspace_id = "test-ws-123".to_string();
            let (c_status, _) = create_workspace_template(Path(workspace_id.clone()));
            assert_eq!(c_status, StatusCode::OK);
            let (a_status, Json(ab)) = apply_workspace_template(Path(workspace_id));
            assert_eq!(a_status, StatusCode::NOT_IMPLEMENTED);
            assert_eq!(
                ab["error"].as_str(),
                Some("workspace_template_apply_not_available")
            );
        });
    }
}

#[cfg(test)]
mod error_handling_tests {
    use super::*;

    #[test]
    fn test_create_rejects_special_chars() {
        with_temp_template_dir(|| {
            let (status, _) = create_workspace_template(Path("ws@123".to_string()));
            assert_eq!(status, StatusCode::BAD_REQUEST);
        });
    }

    #[test]
    fn test_apply_with_unicode_still_501() {
        let (status, _) = apply_workspace_template(Path("espace-travail-français".to_string()));
        assert_eq!(status, StatusCode::NOT_IMPLEMENTED);
    }
}
