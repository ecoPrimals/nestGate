// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Ecosystem-standard transport endpoint types.
//!
//! Wire-compatible with the scaffolding tool's `TransportEndpoint` canonical standard.
//! Primals accept `TRANSPORT_ENDPOINT` as a JSON-encoded env var; the launcher
//! (Tower Atomic / membrane) decides the transport — primals never self-bind.
//!
//! ## Wire Format
//!
//! ```json
//! { "transport": "uds", "path": "/run/user/1000/biomeos/security.sock" }
//! { "transport": "tcp", "host": "127.0.0.1", "port": 9100 }
//! { "transport": "mesh_relay", "peer_id": "strandgate", "capability": "security" }
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Transport endpoint for IPC connections.
///
/// Primals read this from `TRANSPORT_ENDPOINT` env var (JSON string).
/// The launcher decides transport; the primal just connects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "transport", rename_all = "snake_case")]
pub enum TransportEndpoint {
    /// Unix domain socket (preferred on Linux).
    Uds {
        /// Absolute path to the socket file.
        path: PathBuf,
    },
    /// TCP connection (standalone / cross-network).
    Tcp {
        /// Hostname or IP address.
        host: String,
        /// Port number.
        port: u16,
    },
    /// Mesh relay via relay capability (cross-gate federation).
    MeshRelay {
        /// Remote peer identifier.
        peer_id: String,
        /// Capability domain to route through.
        capability: String,
    },
}

impl TransportEndpoint {
    /// Parse from `TRANSPORT_ENDPOINT` environment variable.
    ///
    /// # Errors
    ///
    /// Returns an error when the env var is missing or contains invalid JSON.
    pub fn from_env() -> Result<Self, TransportEndpointError> {
        Self::from_env_with(&crate::ProcessEnv)
    }

    /// Parse from `TRANSPORT_ENDPOINT` using an injectable [`EnvSource`](crate::EnvSource).
    ///
    /// # Errors
    ///
    /// Returns an error when the env var is missing or contains invalid JSON.
    pub fn from_env_with(
        env: &(impl crate::EnvSource + ?Sized),
    ) -> Result<Self, TransportEndpointError> {
        let raw = env
            .get("TRANSPORT_ENDPOINT")
            .ok_or(TransportEndpointError::NotSet)?;
        serde_json::from_str(&raw).map_err(TransportEndpointError::InvalidJson)
    }

    /// Create a UDS endpoint.
    #[must_use]
    pub fn uds(path: impl Into<PathBuf>) -> Self {
        Self::Uds { path: path.into() }
    }

    /// Create a TCP endpoint.
    #[must_use]
    pub fn tcp(host: impl Into<String>, port: u16) -> Self {
        Self::Tcp {
            host: host.into(),
            port,
        }
    }

    /// Create a mesh relay endpoint.
    #[must_use]
    pub fn mesh_relay(peer_id: impl Into<String>, capability: impl Into<String>) -> Self {
        Self::MeshRelay {
            peer_id: peer_id.into(),
            capability: capability.into(),
        }
    }

    /// Whether this endpoint represents a local (same-host) transport.
    #[must_use]
    pub fn is_local(&self) -> bool {
        match self {
            Self::Uds { .. } => true,
            Self::Tcp { host, .. } => host == "127.0.0.1" || host == "::1" || host == "localhost",
            Self::MeshRelay { .. } => false,
        }
    }

    /// Whether this endpoint uses relay infrastructure.
    #[must_use]
    pub const fn is_relayed(&self) -> bool {
        matches!(self, Self::MeshRelay { .. })
    }

    /// Transport name as it appears in the wire format.
    #[must_use]
    pub const fn transport_name(&self) -> &'static str {
        match self {
            Self::Uds { .. } => "uds",
            Self::Tcp { .. } => "tcp",
            Self::MeshRelay { .. } => "mesh_relay",
        }
    }

    /// URI-style string for logging/diagnostics (not for parsing).
    #[must_use]
    pub fn display_uri(&self) -> String {
        match self {
            Self::Uds { path } => {
                let s = path.to_string_lossy();
                s.strip_prefix('@').map_or_else(
                    || format!("unix://{s}"),
                    |abstract_name| format!("unix-abstract://{abstract_name}"),
                )
            }
            Self::Tcp { host, port } => {
                if host.contains(':') {
                    format!("tcp://[{host}]:{port}")
                } else {
                    format!("tcp://{host}:{port}")
                }
            }
            Self::MeshRelay {
                peer_id,
                capability,
            } => format!("mesh://{peer_id}/{capability}"),
        }
    }

    /// Returns the socket path if this is a UDS endpoint.
    #[must_use]
    pub fn uds_path(&self) -> Option<&std::path::Path> {
        match self {
            Self::Uds { path } => Some(path),
            _ => None,
        }
    }

    /// Returns `(host, port)` if this is a TCP endpoint.
    #[must_use]
    pub fn tcp_addr(&self) -> Option<(&str, u16)> {
        match self {
            Self::Tcp { host, port } => Some((host, *port)),
            _ => None,
        }
    }

    /// Returns `(peer_id, capability)` if this is a mesh relay endpoint.
    #[must_use]
    pub fn mesh_peer(&self) -> Option<(&str, &str)> {
        match self {
            Self::MeshRelay {
                peer_id,
                capability,
            } => Some((peer_id, capability)),
            _ => None,
        }
    }

    /// Derive the tarpc UDS endpoint from a JSON-RPC UDS endpoint.
    ///
    /// Convention: `{name}.sock` -> `{name}.tarpc.sock`.
    /// Returns `None` for non-UDS endpoints or paths not ending in `.sock`.
    #[must_use]
    pub fn tarpc_endpoint(&self) -> Option<Self> {
        let path_str = self.uds_path()?.to_string_lossy();
        let base = path_str.strip_suffix(".sock")?;
        Some(Self::uds(PathBuf::from(format!("{base}.tarpc.sock"))))
    }

    /// Build from ecosystem socket path conventions.
    ///
    /// Uses [`super::resolve_socket_path`] to determine the UDS path.
    #[must_use]
    pub fn from_primal_name(primal_name: &str, family_id: Option<&str>) -> Self {
        Self::Uds {
            path: super::resolve_socket_path(primal_name, family_id),
        }
    }

    /// Platform-default endpoint (G66 transport injection).
    ///
    /// On Unix: UDS at the standard ecosystem socket path.
    /// On non-Unix: TCP localhost with `TCP_FALLBACK_PORT` or port 0.
    #[must_use]
    pub fn platform_default(primal_name: &str, family_id: Option<&str>) -> Self {
        if cfg!(unix) {
            Self::from_primal_name(primal_name, family_id)
        } else {
            let port = std::env::var("TCP_FALLBACK_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0_u16);
            Self::Tcp {
                host: "127.0.0.1".into(),
                port,
            }
        }
    }

    /// Parse from `TRANSPORT_ENDPOINT` env var, falling back to
    /// [`from_primal_name`](Self::from_primal_name).
    ///
    /// This is the canonical G66 entry point for transport injection:
    /// ```text
    /// TRANSPORT_ENDPOINT='{"transport":"uds","path":"/run/user/1000/biomeos/nestgate.sock"}'
    /// ```
    #[must_use]
    pub fn from_env_or_default(primal_name: &str, family_id: Option<&str>) -> Self {
        std::env::var("TRANSPORT_ENDPOINT")
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(|| Self::platform_default(primal_name, family_id))
    }
}

impl fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.display_uri())
    }
}

/// Errors from `TransportEndpoint` parsing.
#[derive(Debug, thiserror::Error)]
pub enum TransportEndpointError {
    /// `TRANSPORT_ENDPOINT` env var is not set.
    #[error("`TRANSPORT_ENDPOINT` environment variable is not set")]
    NotSet,
    /// `TRANSPORT_ENDPOINT` contains invalid JSON.
    #[error("invalid `TRANSPORT_ENDPOINT` JSON: {0}")]
    InvalidJson(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MapEnv;

    #[test]
    fn uds_roundtrip_json() {
        let ep = TransportEndpoint::uds("/run/user/1000/biomeos/nestgate.sock");
        let json = serde_json::to_string(&ep).unwrap();
        let parsed: TransportEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(ep, parsed);
        assert!(json.contains(r#""transport":"uds""#));
        assert!(json.contains(r#""path":"/run/user/1000/biomeos/nestgate.sock""#));
    }

    #[test]
    fn tcp_roundtrip_json() {
        let ep = TransportEndpoint::tcp("127.0.0.1", 9100);
        let json = serde_json::to_string(&ep).unwrap();
        let parsed: TransportEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(ep, parsed);
        assert!(json.contains(r#""transport":"tcp""#));
    }

    #[test]
    fn mesh_relay_roundtrip_json() {
        let ep = TransportEndpoint::mesh_relay("strandgate", "security");
        let json = serde_json::to_string(&ep).unwrap();
        let parsed: TransportEndpoint = serde_json::from_str(&json).unwrap();
        assert_eq!(ep, parsed);
        assert!(json.contains(r#""transport":"mesh_relay""#));
    }

    #[test]
    fn wire_format_compatibility_uds() {
        let json = r#"{"transport":"uds","path":"/run/user/1000/biomeos/beardog.sock"}"#;
        let ep: TransportEndpoint = serde_json::from_str(json).unwrap();
        assert!(
            matches!(ep, TransportEndpoint::Uds { ref path } if path.to_str().unwrap().ends_with("beardog.sock"))
        );
    }

    #[test]
    fn wire_format_compatibility_tcp() {
        let json = r#"{"transport":"tcp","host":"127.0.0.1","port":9100}"#;
        let ep: TransportEndpoint = serde_json::from_str(json).unwrap();
        assert!(
            matches!(ep, TransportEndpoint::Tcp { ref host, port } if host == "127.0.0.1" && port == 9100)
        );
    }

    #[test]
    fn wire_format_compatibility_mesh_relay() {
        let json = r#"{"transport":"mesh_relay","peer_id":"strandgate","capability":"security"}"#;
        let ep: TransportEndpoint = serde_json::from_str(json).unwrap();
        assert!(
            matches!(ep, TransportEndpoint::MeshRelay { ref peer_id, ref capability } if peer_id == "strandgate" && capability == "security")
        );
    }

    #[test]
    fn from_env_not_set() {
        let env = MapEnv::from([("OTHER_VAR", "value")]);
        let err = TransportEndpoint::from_env_with(&env).unwrap_err();
        assert!(matches!(err, TransportEndpointError::NotSet));
    }

    #[test]
    fn from_env_invalid_json() {
        let env = MapEnv::from([("TRANSPORT_ENDPOINT", "not json")]);
        let err = TransportEndpoint::from_env_with(&env).unwrap_err();
        assert!(matches!(err, TransportEndpointError::InvalidJson(_)));
    }

    #[test]
    fn from_env_valid_uds() {
        let env = MapEnv::from([(
            "TRANSPORT_ENDPOINT",
            r#"{"transport":"uds","path":"/run/membrane/nestgate.sock"}"#,
        )]);
        let ep = TransportEndpoint::from_env_with(&env).unwrap();
        assert_eq!(ep, TransportEndpoint::uds("/run/membrane/nestgate.sock"));
    }

    #[test]
    fn from_env_valid_tcp() {
        let env = MapEnv::from([(
            "TRANSPORT_ENDPOINT",
            r#"{"transport":"tcp","host":"192.168.1.144","port":7700}"#,
        )]);
        let ep = TransportEndpoint::from_env_with(&env).unwrap();
        assert_eq!(ep, TransportEndpoint::tcp("192.168.1.144", 7700));
    }

    #[test]
    fn is_local_classification() {
        assert!(TransportEndpoint::uds("/tmp/test.sock").is_local());
        assert!(TransportEndpoint::tcp("127.0.0.1", 8080).is_local());
        assert!(TransportEndpoint::tcp("localhost", 8080).is_local());
        assert!(TransportEndpoint::tcp("::1", 8080).is_local());
        assert!(!TransportEndpoint::tcp("192.168.1.144", 7700).is_local());
        assert!(!TransportEndpoint::mesh_relay("strandgate", "security").is_local());
    }

    #[test]
    fn unknown_transport_tag_rejected() {
        let json = r#"{"transport":"quic","addr":"example.com:443"}"#;
        let result = serde_json::from_str::<TransportEndpoint>(json);
        assert!(result.is_err());
    }

    // ── G66 Transport Abstraction tests ──────────────────────────────

    #[test]
    fn is_relayed_classification() {
        assert!(!TransportEndpoint::uds("/tmp/test.sock").is_relayed());
        assert!(!TransportEndpoint::tcp("10.0.0.1", 8080).is_relayed());
        assert!(TransportEndpoint::mesh_relay("peer", "cap").is_relayed());
    }

    #[test]
    fn transport_name_matches_serde_tag() {
        assert_eq!(TransportEndpoint::uds("/x").transport_name(), "uds");
        assert_eq!(TransportEndpoint::tcp("h", 1).transport_name(), "tcp");
        assert_eq!(
            TransportEndpoint::mesh_relay("p", "c").transport_name(),
            "mesh_relay"
        );
    }

    #[test]
    fn display_uri_uds() {
        assert_eq!(
            TransportEndpoint::uds("/run/test.sock").display_uri(),
            "unix:///run/test.sock"
        );
    }

    #[test]
    fn display_uri_tcp_ipv4() {
        assert_eq!(
            TransportEndpoint::tcp("10.0.0.1", 8080).display_uri(),
            "tcp://10.0.0.1:8080"
        );
    }

    #[test]
    fn display_uri_tcp_ipv6() {
        assert_eq!(
            TransportEndpoint::tcp("::1", 443).display_uri(),
            "tcp://[::1]:443"
        );
    }

    #[test]
    fn display_uri_mesh() {
        assert_eq!(
            TransportEndpoint::mesh_relay("peer", "cap").display_uri(),
            "mesh://peer/cap"
        );
    }

    #[test]
    fn display_matches_display_uri() {
        let ep = TransportEndpoint::tcp("host", 1234);
        assert_eq!(format!("{ep}"), ep.display_uri());
    }

    #[test]
    fn uds_path_accessor() {
        let ep = TransportEndpoint::uds("/tmp/test.sock");
        assert_eq!(
            ep.uds_path().unwrap(),
            std::path::Path::new("/tmp/test.sock")
        );
        assert!(TransportEndpoint::tcp("h", 1).uds_path().is_none());
    }

    #[test]
    fn tcp_addr_accessor() {
        let ep = TransportEndpoint::tcp("192.168.1.5", 7700);
        assert_eq!(ep.tcp_addr(), Some(("192.168.1.5", 7700)));
        assert!(TransportEndpoint::uds("/x").tcp_addr().is_none());
    }

    #[test]
    fn mesh_peer_accessor() {
        let ep = TransportEndpoint::mesh_relay("east-gate", "crypto");
        assert_eq!(ep.mesh_peer(), Some(("east-gate", "crypto")));
        assert!(TransportEndpoint::uds("/x").mesh_peer().is_none());
    }

    #[test]
    fn tarpc_endpoint_from_uds() {
        let ep = TransportEndpoint::uds("/run/biomeos/beardog.sock");
        let tarpc = ep.tarpc_endpoint().unwrap();
        assert_eq!(
            tarpc.uds_path().unwrap(),
            std::path::Path::new("/run/biomeos/beardog.tarpc.sock")
        );
    }

    #[test]
    fn tarpc_endpoint_returns_none_for_tcp() {
        assert!(TransportEndpoint::tcp("h", 1).tarpc_endpoint().is_none());
    }

    #[test]
    fn tarpc_endpoint_returns_none_for_non_sock_uds() {
        assert!(TransportEndpoint::uds("/tmp/something")
            .tarpc_endpoint()
            .is_none());
    }

    #[test]
    fn from_primal_name_produces_uds() {
        let ep = TransportEndpoint::from_primal_name("beardog", None);
        let path = ep.uds_path().unwrap();
        assert!(path.to_string_lossy().contains("beardog"));
        assert_eq!(path.extension().and_then(|e| e.to_str()), Some("sock"));
    }

    #[test]
    fn from_primal_name_with_family() {
        let ep = TransportEndpoint::from_primal_name("beardog", Some("abc123"));
        let path_str = ep.uds_path().unwrap().to_string_lossy().to_string();
        assert!(path_str.contains("beardog"));
        assert!(path_str.contains("abc123"));
    }

    #[test]
    fn platform_default_unix_is_uds() {
        if cfg!(unix) {
            let ep = TransportEndpoint::platform_default("testprimal", None);
            assert!(matches!(ep, TransportEndpoint::Uds { .. }));
            assert!(ep.uds_path().unwrap().to_string_lossy().contains("testprimal"));
        }
    }

    #[test]
    fn serde_roundtrip_all_variants() {
        let variants = vec![
            TransportEndpoint::uds("/tmp/test.sock"),
            TransportEndpoint::tcp("192.168.1.5", 7700),
            TransportEndpoint::mesh_relay("east-gate", "storage"),
        ];
        for ep in variants {
            let json = serde_json::to_string(&ep).unwrap();
            let back: TransportEndpoint = serde_json::from_str(&json).unwrap();
            assert_eq!(ep, back);
        }
    }
}
