// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Filesystem-backed NAT / beacon handler coverage (`nat_handlers.rs`).

use super::super::StorageState;
use super::super::nat_handlers;

#[tokio::test]
async fn nat_store_retrieve_roundtrip() {
    let state = StorageState::new().expect("storage state");
    let peer_id = format!("nat-peer-{}", uuid::Uuid::new_v4());
    let params = serde_json::json!({
        "peer_id": &peer_id,
        "traversal_info": {"upnp": true, "port": 443}
    });
    nat_handlers::nat_store_traversal_info(Some(&params), &state)
        .await
        .expect("store");

    let got = nat_handlers::nat_retrieve_traversal_info(
        Some(&serde_json::json!({ "peer_id": &peer_id })),
        &state,
    )
    .await
    .expect("retrieve");
    assert_eq!(got["peer_id"], peer_id);
    assert_eq!(got["traversal_info"]["upnp"], true);

    let path = nestgate_config::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join("_nat_traversal")
        .join(format!("{peer_id}.json"));
    let _ = tokio::fs::remove_file(&path).await;
}

#[tokio::test]
async fn nat_retrieve_not_found() {
    let state = StorageState::new().expect("storage state");
    let peer_id = format!("nat-missing-{}", uuid::Uuid::new_v4());
    let err = nat_handlers::nat_retrieve_traversal_info(
        Some(&serde_json::json!({ "peer_id": &peer_id })),
        &state,
    )
    .await
    .expect_err("missing file");
    assert!(
        err.to_string().contains("No NAT traversal") || err.to_string().contains("not found"),
        "{err}"
    );
}

#[tokio::test]
async fn nat_retrieve_corrupted_file() {
    let state = StorageState::new().expect("storage state");
    let peer_id = format!("nat-bad-json-{}", uuid::Uuid::new_v4());
    let dir = nestgate_config::config::storage_paths::get_storage_base_path()
        .join("datasets")
        .join("_nat_traversal");
    tokio::fs::create_dir_all(&dir).await.expect("mkdir");
    let path = dir.join(format!("{peer_id}.json"));
    tokio::fs::write(&path, b"{ not json")
        .await
        .expect("write corrupt");

    let err = nat_handlers::nat_retrieve_traversal_info(
        Some(&serde_json::json!({ "peer_id": &peer_id })),
        &state,
    )
    .await
    .expect_err("corrupt");
    assert!(
        err.to_string().contains("Corrupted NAT") || err.to_string().contains("corrupt"),
        "{err}"
    );
    let _ = tokio::fs::remove_file(&path).await;
}

#[tokio::test]
async fn beacon_store_retrieve_list_delete_roundtrip() {
    let state = StorageState::new().expect("storage state");
    let peer_id = format!("beacon-{}", uuid::Uuid::new_v4());
    let store_params = serde_json::json!({
        "peer_id": &peer_id,
        "beacon_data": {"sig": "abc"},
        "endpoint": "http://10.0.0.1:1"
    });
    nat_handlers::beacon_store(Some(&store_params), &state)
        .await
        .expect("beacon store");

    let list = nat_handlers::beacon_list(Some(&serde_json::json!({})), &state)
        .await
        .expect("list");
    let ids = list["peer_ids"].as_array().expect("peer_ids");
    assert!(ids.iter().any(|v| v.as_str() == Some(peer_id.as_str())));

    let got =
        nat_handlers::beacon_retrieve(Some(&serde_json::json!({ "peer_id": &peer_id })), &state)
            .await
            .expect("retrieve");
    assert_eq!(got["peer_id"], peer_id);

    let del =
        nat_handlers::beacon_delete(Some(&serde_json::json!({ "peer_id": &peer_id })), &state)
            .await
            .expect("delete");
    assert_eq!(del["deleted"], true);

    let err =
        nat_handlers::beacon_delete(Some(&serde_json::json!({ "peer_id": &peer_id })), &state)
            .await
            .expect_err("already deleted");
    assert!(err.to_string().contains("No beacon"), "{err}");
}
