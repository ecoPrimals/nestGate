// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! `primal.announce` — self-registration with the ecosystem coordinator.
//!
//! On startup (after the Unix socket is bound), `NestGate` announces its
//! capabilities to the ecosystem coordinator so that `capability.call` routing
//! can discover and score `NestGate` as a storage/content provider.
//!
//! The announcement is **best-effort**: if the coordinator is unreachable the
//! server continues normally. Re-announcement happens on reconnect or version change.

use nestgate_types::EnvSource;
use nestgate_types::error::Result;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn};

use super::model_cache_handlers::UNIX_SOCKET_SUPPORTED_METHODS;

/// Capability domains `NestGate` provides to the ecosystem.
const ANNOUNCED_CAPABILITIES: &[&str] = &["storage", "content"];

/// Signal tier — `NestGate` participates in the Nest Atomic composition.
const SIGNAL_TIERS: &[&str] = &["nest"];

/// Federation-capable methods advertised for cross-gate routing.
const FEDERATION_METHODS: &[&str] = &[
    "content.replicate",
    "content.replicate.pull",
    "content.fetch_heads",
    "content.push",
    "content.sync",
];

/// Build the `primal.announce` JSON-RPC params payload.
///
/// `own_socket` is the path to `NestGate`'s bound Unix socket.
/// The payload includes gate identity and federation endpoints for mesh
/// routing (cross-gate CAS access).
#[must_use]
pub fn build_announce_payload(own_socket: &Path) -> Value {
    let methods: Vec<&str> = UNIX_SOCKET_SUPPORTED_METHODS
        .iter()
        .filter(|m| m.starts_with("storage.") || m.starts_with("content."))
        .copied()
        .collect();

    let gate_id = std::env::var("NESTGATE_GATE_ID")
        .or_else(|_| std::env::var("NESTGATE_FAMILY_ID"))
        .unwrap_or_else(|_| String::from("standalone"));

    let mut endpoints = serde_json::Map::new();
    endpoints.insert(
        String::from("uds"),
        Value::String(own_socket.to_string_lossy().into_owned()),
    );
    if let Ok(port) = std::env::var("NESTGATE_API_PORT") {
        endpoints.insert(
            String::from("tcp"),
            Value::String(format!("tcp://127.0.0.1:{port}")),
        );
    }

    let mut storage_backend = serde_json::Map::new();
    if let Ok(dataset) = std::env::var("NESTGATE_ZFS_CAS_DATASET") {
        storage_backend.insert(String::from("type"), Value::String(String::from("zfs")));
        storage_backend.insert(String::from("dataset"), Value::String(dataset));
    } else {
        storage_backend.insert(
            String::from("type"),
            Value::String(String::from("filesystem")),
        );
    }

    json!({
        "primal": "nestgate",
        "socket": own_socket.to_string_lossy(),
        "pid": std::process::id(),
        "gate_id": gate_id,
        "capabilities": ANNOUNCED_CAPABILITIES,
        "methods": methods,
        "federation_methods": FEDERATION_METHODS,
        "endpoints": endpoints,
        "storage_backend": storage_backend,
        "signal_tiers": SIGNAL_TIERS,
        "cost_hints": {
            "storage": 10.0,
            "content": 15.0
        },
        "latency_estimates": {
            "storage": 50,
            "content": 20
        },
        "version": env!("CARGO_PKG_VERSION")
    })
}

/// Discover the ecosystem coordinator socket via standard locations.
///
/// Socket names are capability-based, not primal-specific. The ecosystem
/// directory segment is resolved via `ECOSYSTEM_NAME` / `BIOMEOS_SERVICE_NAME`
/// env, defaulting to `"biomeos"`.
///
/// Search order:
/// 1. `BIOMEOS_IPC_SOCKET` (explicit override; protocol-level name, kept for compat)
/// 2. `BIOMEOS_SOCKET_DIR/{ecosystem}.sock` (protocol-level name, kept for compat)
/// 3. `$XDG_RUNTIME_DIR/{ecosystem}/{ecosystem}.sock` or `neural-api.sock`
/// 4. `temp_dir()/{ecosystem}.sock`
fn discover_coordinator_socket(env: &(impl EnvSource + ?Sized)) -> Option<PathBuf> {
    let eco = nestgate_config::constants::system::ecosystem_name(env);

    // Protocol-level env var name; kept for backward compatibility.
    if let Some(explicit) = env.get("BIOMEOS_IPC_SOCKET") {
        let p = PathBuf::from(explicit);
        if p.exists() {
            return Some(p);
        }
    }

    // Protocol-level env var name; kept for backward compatibility.
    if let Some(dir) = env.get("BIOMEOS_SOCKET_DIR") {
        let p = PathBuf::from(dir).join(format!("{eco}.sock"));
        if p.exists() {
            return Some(p);
        }
    }

    if let Some(xdg) = env.get("XDG_RUNTIME_DIR") {
        let sock_name = format!("{eco}.sock");
        for name in &[sock_name.as_str(), "neural-api.sock"] {
            let p = PathBuf::from(&xdg).join(&eco).join(name);
            if p.exists() {
                return Some(p);
            }
        }
    }

    let tmp = std::env::temp_dir().join(format!("{eco}.sock"));
    if tmp.exists() {
        return Some(tmp);
    }

    None
}

/// Send `primal.announce` to the ecosystem coordinator. Best-effort — logs warnings on failure.
///
/// # Errors
///
/// Returns `Ok(())` even when the coordinator is unreachable (server must not block).
/// Returns `Err` only on internal payload construction failures (should not happen).
pub async fn announce_to_coordinator(own_socket: &Path) -> Result<()> {
    let env = nestgate_types::ProcessEnv;
    let Some(coordinator_path) = discover_coordinator_socket(&env) else {
        debug!(
            "ecosystem coordinator socket not found — skipping primal.announce (will retry on reconnect)"
        );
        return Ok(());
    };

    let payload = build_announce_payload(own_socket);
    let coordinator_str = coordinator_path.to_string_lossy().to_string();

    info!(
        "Announcing to ecosystem coordinator at {} (capabilities: {:?})",
        coordinator_str, ANNOUNCED_CAPABILITIES
    );

    let endpoint = nestgate_types::TransportEndpoint::uds(&coordinator_path);
    match super::JsonRpcClient::connect_transport(&endpoint).await {
        Ok(mut client) => match client.call("primal.announce", payload).await {
            Ok(resp) => {
                info!("primal.announce accepted: {resp}");
                Ok(())
            }
            Err(e) => {
                warn!("primal.announce call failed: {e} — routing will use defaults");
                Ok(())
            }
        },
        Err(e) => {
            warn!(
                "Could not connect to ecosystem coordinator at {}: {e} — skipping announce",
                coordinator_path.display()
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn payload_has_required_fields() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));

        assert_eq!(payload["primal"], "nestgate");
        assert_eq!(payload["socket"], "/tmp/nestgate.sock");
        assert!(payload["pid"].is_number());
        assert!(payload["capabilities"].is_array());
        assert!(payload["methods"].is_array());
        assert!(payload["signal_tiers"].is_array());
        assert!(payload["cost_hints"].is_object());
        assert!(payload["latency_estimates"].is_object());
        assert!(payload["version"].is_string());
    }

    #[test]
    fn payload_capabilities_match_expected() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        let caps: Vec<&str> = payload["capabilities"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(caps, &["storage", "content"]);
    }

    #[test]
    fn payload_methods_are_filtered_correctly() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        let methods: Vec<&str> = payload["methods"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();

        assert!(
            methods
                .iter()
                .all(|m| m.starts_with("storage.") || m.starts_with("content.")),
            "all methods should be storage.* or content.*"
        );
        assert!(methods.contains(&"storage.store"));
        assert!(methods.contains(&"content.put"));
        assert!(methods.contains(&"content.resolve"));
        assert!(
            !methods.contains(&"health.liveness"),
            "health methods should be excluded"
        );
    }

    #[test]
    fn payload_signal_tiers_are_nest() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        let tiers: Vec<&str> = payload["signal_tiers"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert_eq!(tiers, &["nest"]);
    }

    #[test]
    fn payload_cost_hints_present() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        assert!(payload["cost_hints"]["storage"].as_f64().unwrap() > 0.0);
        assert!(payload["cost_hints"]["content"].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn discovery_returns_none_without_sockets() {
        use nestgate_types::MapEnv;
        let env = MapEnv::new();
        assert!(discover_coordinator_socket(&env).is_none());
    }

    #[test]
    fn payload_includes_gate_id() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        assert!(payload["gate_id"].is_string());
    }

    #[test]
    fn payload_includes_endpoints() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        assert!(payload["endpoints"].is_object());
        assert_eq!(payload["endpoints"]["uds"], "/tmp/nestgate.sock");
    }

    #[test]
    fn payload_includes_federation_methods() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        let fed: Vec<&str> = payload["federation_methods"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(fed.contains(&"content.replicate"));
        assert!(fed.contains(&"content.replicate.pull"));
        assert!(fed.contains(&"content.sync"));
    }

    #[test]
    fn payload_includes_storage_backend() {
        let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
        assert!(payload["storage_backend"].is_object());
        assert!(payload["storage_backend"]["type"].is_string());
    }

    #[test]
    fn gate_id_defaults_to_standalone() {
        temp_env::with_vars(
            [
                ("NESTGATE_GATE_ID", None::<&str>),
                ("NESTGATE_FAMILY_ID", None::<&str>),
            ],
            || {
                let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
                assert_eq!(payload["gate_id"], "standalone");
            },
        );
    }

    #[test]
    fn gate_id_from_env() {
        temp_env::with_var("NESTGATE_GATE_ID", Some("eastGate"), || {
            let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
            assert_eq!(payload["gate_id"], "eastGate");
        });
    }

    #[test]
    fn zfs_backend_from_env() {
        temp_env::with_var(
            "NESTGATE_ZFS_CAS_DATASET",
            Some("tank/nestgate-cas"),
            || {
                let payload = build_announce_payload(Path::new("/tmp/nestgate.sock"));
                assert_eq!(payload["storage_backend"]["type"], "zfs");
                assert_eq!(payload["storage_backend"]["dataset"], "tank/nestgate-cas");
            },
        );
    }
}
