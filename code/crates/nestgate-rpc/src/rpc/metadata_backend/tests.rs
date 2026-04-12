// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::file_backend::service_record_path;
use super::*;
use nestgate_types::error::NestGateError;
use tempfile::tempdir;
use tokio::fs;
use tokio::task::JoinSet;

#[tokio::test]
async fn in_memory_metadata_roundtrip() {
    let backend = InMemoryMetadataBackend::new();
    let record = ServiceRecord {
        name: "test-primal".into(),
        capabilities: vec!["storage".into(), "compute".into()],
        endpoint: Some("http://localhost:8080".into()),
        metadata: HashMap::new(),
    };
    backend.store_service(record).await.expect("store");
    let fetched = backend.get_service("test-primal").await.expect("get");
    assert_eq!(fetched.capabilities.len(), 2);

    let by_cap = backend.find_by_capability("storage").await.expect("find");
    assert_eq!(by_cap.len(), 1);
    assert_eq!(by_cap[0].name, "test-primal");

    let empty = backend.find_by_capability("quantum").await.expect("find");
    assert!(empty.is_empty());
}

#[tokio::test]
async fn file_metadata_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let record = ServiceRecord {
        name: "test-primal".into(),
        capabilities: vec!["storage".into(), "compute".into()],
        endpoint: Some("http://localhost:8080".into()),
        metadata: HashMap::new(),
    };
    backend.store_service(record).await.expect("store");
    let fetched = backend.get_service("test-primal").await.expect("get");
    assert_eq!(fetched.capabilities.len(), 2);

    let by_cap = backend.find_by_capability("storage").await.expect("find");
    assert_eq!(by_cap.len(), 1);
    assert_eq!(by_cap[0].name, "test-primal");

    let listed = backend
        .list_services_by_name_prefix("test-")
        .await
        .expect("list");
    assert_eq!(listed.len(), 1);
}

#[tokio::test]
async fn file_metadata_persists_across_instances() {
    let dir = tempdir().expect("tempdir");
    let base = dir.path().to_path_buf();
    {
        let backend = FileMetadataBackend::new(base.clone()).expect("new");
        backend
            .store_service(ServiceRecord {
                name: "persisted".into(),
                capabilities: vec!["x".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
    }
    let backend2 = FileMetadataBackend::new(base).expect("reopen");
    let got = backend2.get_service("persisted").await.expect("get");
    assert_eq!(got.name, "persisted");
}

#[tokio::test]
async fn file_metadata_delete_service() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    backend
        .store_service(ServiceRecord {
            name: "gone".into(),
            capabilities: vec![],
            endpoint: None,
            metadata: HashMap::new(),
        })
        .await
        .expect("store");
    backend.delete_service("gone").await.expect("delete");
    let err = backend.get_service("gone").await.expect_err("gone");
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn file_metadata_creates_nonexistent_base_directory() {
    let root = tempdir().expect("tempdir");
    let deep = root.path().join("nested/new/metadata_base");
    assert!(!deep.exists());
    let backend = FileMetadataBackend::new(deep.clone()).expect("new");
    assert!(deep.join(METADATA_SERVICES_NAMESPACE).is_dir());
    backend
        .store_service(ServiceRecord {
            name: "probe".into(),
            capabilities: vec!["x".into()],
            endpoint: None,
            metadata: HashMap::new(),
        })
        .await
        .expect("store");
    assert!(backend.get_service("probe").await.is_ok());
}

#[tokio::test]
async fn file_metadata_skips_non_utf8_json_files_in_services_dir() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let junk_path = dir
        .path()
        .join(METADATA_SERVICES_NAMESPACE)
        .join("corrupt.json");
    fs::write(&junk_path, &[0xFF, 0xFE, 0x80])
        .await
        .expect("write junk");
    let all = backend
        .list_services_by_name_prefix("")
        .await
        .expect("list");
    assert!(
        all.is_empty(),
        "invalid UTF-8 / JSON should be skipped, not crash"
    );
}

#[tokio::test]
async fn file_metadata_service_name_utf8_edge_cases_roundtrip() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let name = "primal-\u{1F980}-\u{0301}A";
    backend
        .store_service(ServiceRecord {
            name: name.into(),
            capabilities: vec!["c".into()],
            endpoint: None,
            metadata: HashMap::new(),
        })
        .await
        .expect("store");
    let got = backend.get_service(name).await.expect("get");
    assert_eq!(got.name, name);
}

#[tokio::test]
async fn file_metadata_find_by_capability_multiple_services_sorted() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    for n in ["zebra", "alpha", "mango"] {
        backend
            .store_service(ServiceRecord {
                name: n.into(),
                capabilities: vec!["shared-cap".into(), "other".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
    }
    let found = backend
        .find_by_capability("shared-cap")
        .await
        .expect("find");
    assert_eq!(found.len(), 3);
    assert_eq!(
        found.iter().map(|r| r.name.as_str()).collect::<Vec<_>>(),
        vec!["alpha", "mango", "zebra"]
    );
}

#[tokio::test]
async fn file_metadata_concurrent_stores() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let tasks: Vec<_> = (0u32..16)
        .map(|i| {
            let b = backend.clone();
            tokio::spawn(async move {
                b.store_service(ServiceRecord {
                    name: format!("concurrent-{i}"),
                    capabilities: vec!["cc".into()],
                    endpoint: None,
                    metadata: HashMap::new(),
                })
                .await
            })
        })
        .collect();
    for t in tasks {
        t.await.expect("join").expect("store");
    }
    let listed = backend
        .list_services_by_name_prefix("concurrent-")
        .await
        .expect("list");
    assert_eq!(listed.len(), 16);
    let caps = backend.find_by_capability("cc").await.expect("find");
    assert_eq!(caps.len(), 16);
}

#[tokio::test]
async fn in_memory_get_service_not_found() {
    let backend = InMemoryMetadataBackend::new();
    let err = backend
        .get_service("missing")
        .await
        .expect_err("missing service");
    assert!(matches!(err, NestGateError::Api(_)));
}

#[tokio::test]
async fn in_memory_delete_service_not_found() {
    let backend = InMemoryMetadataBackend::new();
    let err = backend
        .delete_service("nope")
        .await
        .expect_err("delete missing");
    assert!(matches!(err, NestGateError::Api(_)));
}

#[tokio::test]
async fn file_metadata_get_service_not_found() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let err = backend.get_service("absent").await.expect_err("not found");
    assert!(matches!(err, NestGateError::Api(_)));
}

#[tokio::test]
async fn file_metadata_get_service_invalid_json() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let path = service_record_path(backend.base_dir(), "bad-json");
    fs::write(&path, b"{\"name\":")
        .await
        .expect("write truncated json");
    let err = backend.get_service("bad-json").await.expect_err("serde");
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn file_metadata_delete_service_not_found() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let err = backend
        .delete_service("never-stored")
        .await
        .expect_err("not found");
    assert!(matches!(err, NestGateError::Api(_)));
}

#[tokio::test]
async fn file_metadata_iter_skips_subdirectories_and_non_json_files() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let svc = dir.path().join(METADATA_SERVICES_NAMESPACE);
    fs::create_dir(svc.join("nested"))
        .await
        .expect("mkdir nested");
    fs::write(svc.join("note.txt"), b"{}").await.expect("txt");
    backend
        .store_service(ServiceRecord {
            name: "only-valid".into(),
            capabilities: vec!["x".into()],
            endpoint: None,
            metadata: HashMap::new(),
        })
        .await
        .expect("store");
    let listed = backend
        .list_services_by_name_prefix("")
        .await
        .expect("list");
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0].name, "only-valid");
}

#[tokio::test]
async fn file_metadata_read_dir_error_when_services_path_is_file() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let services_path = dir.path().join(METADATA_SERVICES_NAMESPACE);
    fs::remove_dir_all(&services_path)
        .await
        .expect("remove services dir");
    fs::write(&services_path, b"not-a-dir")
        .await
        .expect("replace with file");
    let err = backend
        .find_by_capability("any")
        .await
        .expect_err("read_dir should fail");
    assert!(!err.to_string().is_empty());
}

#[tokio::test]
async fn default_metadata_backend_enum_dispatches_file_and_memory() {
    let dir = tempdir().expect("tempdir");
    let file = DefaultMetadataBackend::File(
        FileMetadataBackend::new(dir.path().to_path_buf()).expect("new"),
    );
    let mem = DefaultMetadataBackend::InMemory(InMemoryMetadataBackend::new());

    for (label, backend) in [("file", file), ("mem", mem)] {
        let name = format!("svc-{label}");
        backend
            .store_service(ServiceRecord {
                name: name.clone(),
                capabilities: vec!["c".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await
            .expect("store");
        let got = backend.get_service(&name).await.expect("get");
        assert_eq!(got.name, name);
        backend.delete_service(&name).await.expect("delete");
    }
}

#[tokio::test]
async fn file_metadata_concurrent_mixed_ops() {
    let dir = tempdir().expect("tempdir");
    let backend = FileMetadataBackend::new(dir.path().to_path_buf()).expect("new");
    let mut set = JoinSet::new();
    for i in 0u32..32 {
        let b = backend.clone();
        set.spawn(async move {
            let name = format!("mix-{i}");
            b.store_service(ServiceRecord {
                name: name.clone(),
                capabilities: vec!["m".into()],
                endpoint: None,
                metadata: HashMap::new(),
            })
            .await?;
            let _ = b.get_service(&name).await?;
            if i % 4 == 0 {
                b.delete_service(&name).await?;
            }
            let _ = b.list_services_by_name_prefix("mix-").await?;
            let _ = b.find_by_capability("m").await?;
            Ok::<(), nestgate_types::error::NestGateError>(())
        });
    }
    while let Some(res) = set.join_next().await {
        res.expect("join").expect("task");
    }
    let remaining = backend
        .list_services_by_name_prefix("mix-")
        .await
        .expect("list");
    assert_eq!(remaining.len(), 24);
}

#[test]
fn default_metadata_base_dir_paths_contain_nestgate_metadata() {
    let p = default_metadata_base_dir();
    let s = p.to_string_lossy();
    assert!(s.contains("nestgate"), "{s}");
    assert!(s.contains("metadata"), "{s}");
}

#[test]
fn default_metadata_base_dir_from_env_source_ends_with_nestgate_metadata_segment() {
    // Resolution may use etcetera or env fallbacks; path always ends with …/nestgate/metadata.
    let p = default_metadata_base_dir_from_env_source(&nestgate_types::MapEnv::new());
    assert!(p.ends_with("nestgate/metadata"), "{}", p.display());
}
