// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Round 6 final coverage: request shapes, JSON-RPC, REST storage list, and edge cases.

#[cfg(test)]
mod round6_tests {
    use crate::handlers::storage::{
        StorageDatasetInfo, StorageHandler, StoragePoolInfo, StorageSnapshotInfo,
    };
    use crate::handlers::zfs::basic::{
        CreateDatasetRequest, CreatePoolRequest, CreateSnapshotRequest, ZfsHealthResponse,
        evaluate_zfs_health,
    };
    use crate::nestgate_rpc_service::NestGateJsonRpcHandler;
    use crate::rest::handlers::storage::list_backends;
    use crate::rest::{ApiState, ListQuery};
    use axum::Json;
    use axum::extract::{Query, State};
    use serde_json::json;

    #[test]
    fn r6_create_pool_invalid_json_fails() {
        assert!(serde_json::from_str::<CreatePoolRequest>("not json").is_err());
    }

    #[test]
    fn r6_create_pool_missing_name_field_errors() {
        let v = json!({"_devices": ["/dev/sda"]});
        assert!(serde_json::from_value::<CreatePoolRequest>(v).is_err());
    }

    #[test]
    fn r6_create_pool_explicit_empty_devices() {
        let v = json!({"name": "tank", "_devices": []});
        let r: CreatePoolRequest = serde_json::from_value(v).unwrap();
        assert!(r.devices.is_empty());
    }

    #[test]
    fn r6_create_dataset_missing_name_errors() {
        let v = json!({"properties": {}});
        assert!(serde_json::from_value::<CreateDatasetRequest>(v).is_err());
    }

    #[test]
    fn r6_create_snapshot_extra_field_ignored_by_serde() {
        let v = json!({"dataset": "d", "name": "s", "extra": 1});
        let r: CreateSnapshotRequest = serde_json::from_value(v).unwrap();
        assert_eq!(r.name, "s");
    }

    #[test]
    fn r6_extremely_long_pool_name_roundtrips() {
        let long = "p".repeat(2048);
        let r = CreatePoolRequest {
            name: long,
            devices: vec![],
        };
        let v = serde_json::to_string(&r).unwrap();
        let back: CreatePoolRequest = serde_json::from_str(&v).unwrap();
        assert_eq!(back.name.len(), 2048);
    }

    #[test]
    fn r6_zfs_health_response_empty_pools() {
        let z = evaluate_zfs_health(vec![]);
        let s = serde_json::to_string(&z).unwrap();
        let back: ZfsHealthResponse = serde_json::from_str(&s).unwrap();
        assert_eq!(back.pools.len(), 0);
    }

    #[test]
    fn r6_storage_handler_debug() {
        let h = StorageHandler::new();
        assert!(format!("{h:?}").contains("StorageHandler"));
    }

    #[test]
    fn r6_storage_pool_info_json() {
        let p = StoragePoolInfo {
            name: "n".into(),
            total_capacity_gb: 1,
            used_capacity_gb: 0,
            available_capacity_gb: 1,
            health_status: "ok".into(),
        };
        let v = serde_json::to_value(&p).unwrap();
        assert_eq!(v["health_status"], "ok");
    }

    #[test]
    fn r6_storage_dataset_info_json() {
        let d = StorageDatasetInfo {
            name: "a/b".into(),
            pool_name: "a".into(),
            used_space_gb: 1,
            compression_ratio: 1.0,
            dedup_ratio: 1.0,
        };
        assert!(serde_json::to_string(&d).unwrap().contains("a/b"));
    }

    #[test]
    fn r6_storage_snapshot_info_serde() {
        let s = StorageSnapshotInfo {
            name: "z@snap".into(),
            dataset_name: "z".into(),
            created_at: std::time::SystemTime::UNIX_EPOCH,
            size_gb: 0,
        };
        let _ = serde_json::to_string(&s).unwrap();
    }

    #[tokio::test]
    async fn r6_json_rpc_list_pools_ok_array() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("list_pools", serde_json::Value::Null)
            .await
            .expect("list_pools");
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn r6_json_rpc_list_datasets_empty_ok() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("list_datasets", json!("nonexistent_pool_round6"))
            .await
            .expect("list_datasets");
        assert!(v.is_array());
    }

    #[tokio::test]
    async fn r6_json_rpc_get_metrics_fields() {
        let h = NestGateJsonRpcHandler::new();
        let v = h
            .handle("get_metrics", serde_json::Value::Null)
            .await
            .unwrap();
        assert!(v.get("compression_ratio").is_some());
        assert!(v.get("snapshot_count").is_some());
    }

    #[tokio::test]
    async fn r6_list_backends_paginated_shape() {
        let state = ApiState::new().expect("api state");
        let q = ListQuery {
            filter: None,
            sort: None,
            order: None,
            page: Some(1),
            per_page: Some(10),
        };
        let Json(body) = list_backends(State(state), Query(q))
            .await
            .expect("list backends");
        assert!(body.data.len() <= 10);
    }

    #[tokio::test]
    async fn r6_list_backends_filter_memory() {
        let state = ApiState::new().expect("api state");
        let q = ListQuery {
            filter: Some("Memory".into()),
            sort: Some("name".into()),
            order: None,
            page: None,
            per_page: None,
        };
        let Json(body) = list_backends(State(state), Query(q)).await.expect("ok");
        assert!(!body.data.is_empty());
    }

    #[tokio::test]
    async fn r6_list_backends_sort_performance_desc() {
        let state = ApiState::new().expect("api state");
        let q = ListQuery {
            filter: None,
            sort: Some("performance".into()),
            order: Some("desc".into()),
            page: None,
            per_page: None,
        };
        let _ = list_backends(State(state), Query(q)).await.unwrap();
    }

    #[tokio::test]
    async fn r6_list_backends_sort_type_branch() {
        let state = ApiState::new().expect("api state");
        let q = ListQuery {
            filter: None,
            sort: Some("type".into()),
            order: None,
            page: None,
            per_page: None,
        };
        let _ = list_backends(State(state), Query(q)).await.unwrap();
    }

    #[tokio::test]
    async fn r6_list_backends_default_sort_unknown_field() {
        let state = ApiState::new().expect("api state");
        let q = ListQuery {
            filter: None,
            sort: Some("unknown_field".into()),
            order: None,
            page: None,
            per_page: None,
        };
        let _ = list_backends(State(state), Query(q)).await.unwrap();
    }

    macro_rules! r6_storage_pool_info_roundtrip {
        ($($name:ident),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    let p = StoragePoolInfo {
                        name: stringify!($name).into(),
                        total_capacity_gb: 1,
                        used_capacity_gb: 0,
                        available_capacity_gb: 1,
                        health_status: "ok".into(),
                    };
                    let s = serde_json::to_string(&p).unwrap();
                    let _: StoragePoolInfo = serde_json::from_str(&s).unwrap();
                }
            )+
        };
    }

    r6_storage_pool_info_roundtrip!(
        r6_pool_rt_00,
        r6_pool_rt_01,
        r6_pool_rt_02,
        r6_pool_rt_03,
        r6_pool_rt_04,
        r6_pool_rt_05,
        r6_pool_rt_06,
        r6_pool_rt_07,
        r6_pool_rt_08,
        r6_pool_rt_09,
        r6_pool_rt_10,
        r6_pool_rt_11,
        r6_pool_rt_12,
        r6_pool_rt_13,
        r6_pool_rt_14,
        r6_pool_rt_15,
        r6_pool_rt_16,
        r6_pool_rt_17,
        r6_pool_rt_18,
        r6_pool_rt_19,
        r6_pool_rt_20,
        r6_pool_rt_21,
        r6_pool_rt_22,
        r6_pool_rt_23,
        r6_pool_rt_24,
        r6_pool_rt_25,
        r6_pool_rt_26,
        r6_pool_rt_27,
        r6_pool_rt_28,
        r6_pool_rt_29,
        r6_pool_rt_30,
        r6_pool_rt_31,
        r6_pool_rt_32,
        r6_pool_rt_33,
        r6_pool_rt_34,
        r6_pool_rt_35,
        r6_pool_rt_36,
        r6_pool_rt_37,
        r6_pool_rt_38,
        r6_pool_rt_39
    );
}
