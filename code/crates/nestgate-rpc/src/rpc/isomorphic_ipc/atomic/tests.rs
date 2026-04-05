// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

use super::discovery;
use super::*;

#[test]
fn test_atomic_type_required_capabilities() {
    assert_eq!(AtomicType::Tower.required_capabilities().len(), 2);
    assert_eq!(AtomicType::Node.required_capabilities().len(), 3);
    assert_eq!(AtomicType::Nest.required_capabilities().len(), 4);
}

#[test]
fn test_atomic_capabilities_are_roles_not_primal_names() {
    for cap in AtomicType::Nest.required_capabilities() {
        assert!(
            !["beardog", "songbird", "nestgate", "squirrel", "toadstool"].contains(&cap),
            "Capability '{}' should be a role, not a primal name",
            cap
        );
    }
}

#[test]
fn test_atomic_type_requires_storage() {
    assert!(!AtomicType::Tower.requires_storage());
    assert!(!AtomicType::Node.requires_storage());
    assert!(AtomicType::Nest.requires_storage());
}

#[test]
fn test_custom_atomic_type() {
    let custom = AtomicType::Custom {
        name: "EDGE".to_string(),
        required_capabilities: vec![
            capabilities::DEVICE.to_string(),
            capabilities::STORAGE.to_string(),
        ],
    };
    assert_eq!(custom.name(), "EDGE");
    assert_eq!(custom.required_capabilities().len(), 2);
    assert!(custom.requires_storage());
}

#[test]
fn test_atomic_status_is_operational() {
    let status = AtomicStatus {
        atomic_type: AtomicType::Nest,
        overall_health: HealthStatus::Healthy,
        component_statuses: vec![
            ("nestgate (self)".to_string(), HealthStatus::Healthy),
            ("device-provider".to_string(), HealthStatus::Healthy),
        ],
    };
    assert!(status.is_operational());

    let degraded_status = AtomicStatus {
        atomic_type: AtomicType::Nest,
        overall_health: HealthStatus::Degraded,
        component_statuses: vec![
            ("nestgate (self)".to_string(), HealthStatus::Degraded),
            ("device-provider".to_string(), HealthStatus::Healthy),
        ],
    };
    assert!(degraded_status.is_operational());
    assert!(degraded_status.overall_health.needs_attention());

    let unhealthy_status = AtomicStatus {
        atomic_type: AtomicType::Nest,
        overall_health: HealthStatus::Unhealthy,
        component_statuses: vec![
            ("nestgate (self)".to_string(), HealthStatus::Unhealthy),
            ("device-provider".to_string(), HealthStatus::Healthy),
        ],
    };
    assert!(!unhealthy_status.is_operational());
}

#[tokio::test]
async fn test_verify_nest_health() {
    let result = verify_nest_health().await;
    let _ = result;
}

#[tokio::test]
async fn test_discover_available_primals_does_not_panic() {
    let primals = discovery::discover_available_primals();
    let _ = primals;
}

#[test]
fn atomic_type_names() {
    assert_eq!(AtomicType::Tower.name(), "TOWER");
    assert_eq!(AtomicType::Node.name(), "NODE");
    assert_eq!(AtomicType::Nest.name(), "NEST");
}

#[test]
fn atomic_status_components_needing_attention_filters() {
    let s = AtomicStatus {
        atomic_type: AtomicType::Nest,
        overall_health: HealthStatus::Healthy,
        component_statuses: vec![
            ("ok".to_string(), HealthStatus::Healthy),
            ("bad".to_string(), HealthStatus::Unhealthy),
        ],
    };
    let names = s.components_needing_attention();
    assert_eq!(names, vec!["bad"]);
}

#[test]
fn gather_socket_search_dirs_includes_uid_run_path() {
    let dirs = discovery::gather_socket_search_dirs();
    let uid = uzers::get_current_uid();
    assert!(
        dirs.iter().any(|d| d.contains(&format!("/run/user/{uid}"))),
        "{dirs:?}"
    );
    assert!(dirs.iter().any(|d| d == "/tmp"));
}

#[test]
fn discover_primal_socket_biomeos_dir() {
    let dir = tempfile::tempdir().expect("tempdir");
    let sock = dir.path().join("tower.sock");
    std::fs::write(&sock, b"x").unwrap();
    nestgate_platform::env_process::set_var(
        "BIOMEOS_SOCKET_DIR",
        dir.path().to_string_lossy().as_ref(),
    );
    let p = discovery::discover_primal_socket("tower");
    nestgate_platform::env_process::remove_var("BIOMEOS_SOCKET_DIR");
    assert_eq!(p, Some(sock));
}

#[test]
fn discover_primal_socket_family_scoped_under_xdg() {
    let dir = tempfile::tempdir().expect("tempdir");
    let bio = dir.path().join("biomeos");
    std::fs::create_dir_all(&bio).unwrap();
    let sock = bio.join("beacon-fam9.sock");
    std::fs::write(&sock, b"x").unwrap();
    nestgate_platform::env_process::set_var(
        "XDG_RUNTIME_DIR",
        dir.path().to_string_lossy().as_ref(),
    );
    nestgate_platform::env_process::set_var("NESTGATE_FAMILY_ID", "fam9");
    let p = discovery::discover_primal_socket("beacon");
    nestgate_platform::env_process::remove_var("XDG_RUNTIME_DIR");
    nestgate_platform::env_process::remove_var("NESTGATE_FAMILY_ID");
    assert_eq!(p, Some(sock));
}

#[test]
fn discover_primal_socket_tmp_fallback() {
    let tmp_sock = std::path::PathBuf::from("/tmp").join("ng_atomic_cov_primal.sock");
    let _ = std::fs::remove_file(&tmp_sock);
    std::fs::write(&tmp_sock, b"x").unwrap();
    let p = discovery::discover_primal_socket("ng_atomic_cov_primal");
    let _ = std::fs::remove_file(&tmp_sock);
    assert_eq!(p, Some(tmp_sock));
}

#[tokio::test]
async fn get_nestgate_endpoint_for_atomic_formats_result_or_errors() {
    let r = get_nestgate_endpoint_for_atomic().await;
    if let Ok(s) = r {
        assert!(!s.is_empty());
    }
}
