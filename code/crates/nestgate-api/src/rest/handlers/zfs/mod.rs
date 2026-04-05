// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! ZFS module - Pure data layer handlers for ZFS dataset and snapshot operations.
//!
//! These handlers focus solely on data operations without authentication,
//! providing clean data access for management and other management systems.

mod dataset_handlers;
pub(crate) mod helpers;
mod snapshot_handlers;

// Re-export all public handlers for backward compatibility
pub use dataset_handlers::{
    create_dataset, delete_dataset, get_dataset, get_dataset_properties, get_dataset_stats,
    list_datasets, set_dataset_properties, update_dataset,
};
pub use snapshot_handlers::{
    clone_snapshot, create_snapshot, delete_snapshot, get_snapshot, list_snapshots,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::ListQuery;
    use crate::rest::models::{CloneSnapshotRequest, CreateSnapshotRequest};
    use crate::rest::models::{CreateDatasetRequest, DatasetProperties, DatasetType};
    use axum::extract::{Path, Query, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use std::collections::HashMap;

    async fn create_test_state_with_dataset(name: &str) -> crate::rest::ApiState {
        let state = crate::rest::ApiState::new().expect("Failed to create test state");
        state
            .zfs_engines
            .insert(name.to_string(), "placeholder_engine".to_string());
        state
    }

    fn create_test_state() -> crate::rest::ApiState {
        crate::rest::ApiState::new().expect("Failed to create test state")
    }

    #[tokio::test]
    async fn test_list_datasets_empty() {
        let state = create_test_state();
        let query = ListQuery {
            page: None,
            per_page: None,
            sort: None,
            order: None,
            filter: None,
        };
        let result = list_datasets(State(state), Query(query)).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.0.data.is_empty());
    }

    #[tokio::test]
    async fn test_list_datasets_with_data() {
        let state = create_test_state_with_dataset("tank/data").await;
        let query = ListQuery {
            page: Some(1),
            per_page: Some(10),
            sort: None,
            order: None,
            filter: None,
        };
        let result = list_datasets(State(state), Query(query)).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.0.data.is_empty());
        assert_eq!(response.0.data[0].name, "tank/data");
    }

    #[tokio::test]
    async fn test_list_datasets_with_filter_and_sort() {
        let state = create_test_state_with_dataset("tank/data").await;
        let query = ListQuery {
            page: Some(1),
            per_page: Some(50),
            sort: Some("name".to_string()),
            order: Some("asc".to_string()),
            filter: Some("tank".to_string()),
        };
        let result = list_datasets(State(state), Query(query)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_create_dataset_empty_name_fails() {
        let state = create_test_state();
        let request = CreateDatasetRequest {
            name: String::new(),
            dataset_type: DatasetType::Filesystem,
            backend: crate::rest::models::StorageBackendType::Filesystem,
            properties: None,
            quota: None,
            description: None,
        };
        let result = create_dataset(State(state), axum::Json(request)).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0.code, "INVALID_NAME");
    }

    #[tokio::test]
    async fn test_get_dataset_not_found() {
        let state = create_test_state();
        let result = get_dataset(State(state), Path("nonexistent".to_string())).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.0.code, "DATASET_NOT_FOUND");
    }

    #[tokio::test]
    async fn test_get_dataset_found() {
        let state = create_test_state_with_dataset("tank/data").await;
        let result = get_dataset(State(state), Path("tank/data".to_string())).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.0.data.name, "tank/data");
    }

    #[tokio::test]
    async fn test_delete_dataset_not_found() {
        let state = create_test_state();
        let result = delete_dataset(State(state), Path("nonexistent".to_string())).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_dataset_success() {
        let state = create_test_state_with_dataset("tank/to_delete").await;
        let result = delete_dataset(State(state), Path("tank/to_delete".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_dataset_properties_not_found() {
        let state = create_test_state();
        let result = get_dataset_properties(State(state), Path("nonexistent".to_string())).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_dataset_properties_found() {
        let state = create_test_state_with_dataset("tank/data").await;
        let result = get_dataset_properties(State(state), Path("tank/data".to_string())).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_set_dataset_properties_not_found() {
        let state = create_test_state();
        let props = DatasetProperties {
            name: "nonexistent".to_string(),
            mountpoint: None,
            quota: None,
            reservation: None,
            compression: false,
            compression_type: None,
            checksum: false,
            checksum_type: None,
            deduplication: false,
            encryption: false,
            readonly: false,
            custom: HashMap::new(),
        };
        let result = set_dataset_properties(
            State(state),
            Path("nonexistent".to_string()),
            axum::Json(props),
        )
        .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_dataset_stats_not_found() {
        let state = create_test_state();
        let result = get_dataset_stats(State(state), Path("nonexistent".to_string())).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_dataset_stats_found() {
        let state = create_test_state_with_dataset("tank/data").await;
        let result = get_dataset_stats(State(state), Path("tank/data".to_string())).await;
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.0.data.name, "tank/data");
    }

    #[tokio::test]
    async fn test_list_snapshots() {
        let state = create_test_state_with_dataset("tank/data").await;
        let query = ListQuery {
            page: None,
            per_page: None,
            sort: None,
            order: None,
            filter: None,
        };
        let response = list_snapshots(State(state), Path("tank/data".to_string()), Query(query))
            .await
            .into_response();
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_create_snapshot() {
        let state = create_test_state_with_dataset("tank/data").await;
        let request = CreateSnapshotRequest {
            name: "snap1".to_string(),
            description: None,
            recursive: false,
            properties: HashMap::new(),
            tags: None,
        };
        let response = create_snapshot(
            State(state),
            Path("tank/data".to_string()),
            axum::Json(request),
        )
        .await
        .into_response();
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_delete_snapshot() {
        let state = create_test_state_with_dataset("tank/data").await;
        let response = delete_snapshot(
            State(state),
            Path(("tank/data".to_string(), "snap1".to_string())),
        )
        .await
        .into_response();
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }

    #[tokio::test]
    async fn test_clone_snapshot_empty_name_fails() {
        let state = create_test_state();
        let request = CloneSnapshotRequest {
            target_dataset_name: "tank/data".to_string(),
            clone_name: String::new(),
            properties: None,
            description: None,
        };
        let response = clone_snapshot(
            State(state),
            Path(("tank/data".to_string(), "snap1".to_string())),
            axum::Json(request),
        )
        .await
        .into_response();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_clone_snapshot_success() {
        let state = create_test_state();
        let request = CloneSnapshotRequest {
            target_dataset_name: "tank/data".to_string(),
            clone_name: "tank/clone".to_string(),
            properties: None,
            description: None,
        };
        let response = clone_snapshot(
            State(state),
            Path(("tank/data".to_string(), "snap1".to_string())),
            axum::Json(request),
        )
        .await
        .into_response();
        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }

    #[test]
    fn test_storage_backend_type_to_string() {
        assert_eq!(
            crate::rest::models::StorageBackendType::Filesystem.to_string(),
            "zfs"
        );
        assert_eq!(
            crate::rest::models::StorageBackendType::Memory.to_string(),
            "memory"
        );
        assert_eq!(
            crate::rest::models::StorageBackendType::Cloud.to_string(),
            "cloud"
        );
        assert_eq!(
            crate::rest::models::StorageBackendType::Remote.to_string(),
            "remote"
        );
    }
}
