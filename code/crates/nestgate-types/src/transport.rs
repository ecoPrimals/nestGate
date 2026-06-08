// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! Ecosystem-standard transport endpoint types.
//!
//! Wire-compatible with the sourDough `TransportEndpoint` canonical standard.
//! Primals accept `TRANSPORT_ENDPOINT` as a JSON-encoded env var; the launcher
//! (Tower Atomic / membrane) decides the transport — primals never self-bind.
//!
//! ## Wire Format
//!
//! ```json
//! { "transport": "uds", "path": "/run/user/1000/biomeos/beardog.sock" }
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
    /// Mesh relay via songBird (cross-gate federation).
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
    pub fn from_env_with(env: &(impl crate::EnvSource + ?Sized)) -> Result<Self, TransportEndpointError> {
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
            Self::Tcp { host, .. } => {
                host == "127.0.0.1" || host == "::1" || host == "localhost"
            }
            Self::MeshRelay { .. } => false,
        }
    }
}

impl fmt::Display for TransportEndpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uds { path } => write!(f, "uds:{}", path.display()),
            Self::Tcp { host, port } => write!(f, "tcp:{host}:{port}"),
            Self::MeshRelay {
                peer_id,
                capability,
            } => write!(f, "mesh_relay:{peer_id}/{capability}"),
        }
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
        assert!(matches!(ep, TransportEndpoint::Uds { ref path } if path.to_str().unwrap().ends_with("beardog.sock")));
    }

    #[test]
    fn wire_format_compatibility_tcp() {
        let json = r#"{"transport":"tcp","host":"127.0.0.1","port":9100}"#;
        let ep: TransportEndpoint = serde_json::from_str(json).unwrap();
        assert!(matches!(ep, TransportEndpoint::Tcp { ref host, port } if host == "127.0.0.1" && port == 9100));
    }

    #[test]
    fn wire_format_compatibility_mesh_relay() {
        let json = r#"{"transport":"mesh_relay","peer_id":"strandgate","capability":"security"}"#;
        let ep: TransportEndpoint = serde_json::from_str(json).unwrap();
        assert!(matches!(ep, TransportEndpoint::MeshRelay { ref peer_id, ref capability } if peer_id == "strandgate" && capability == "security"));
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
        assert_eq!(
            ep,
            TransportEndpoint::uds("/run/membrane/nestgate.sock")
        );
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
    fn display_formatting() {
        assert_eq!(
            TransportEndpoint::uds("/run/test.sock").to_string(),
            "uds:/run/test.sock"
        );
        assert_eq!(
            TransportEndpoint::tcp("127.0.0.1", 9100).to_string(),
            "tcp:127.0.0.1:9100"
        );
        assert_eq!(
            TransportEndpoint::mesh_relay("peer1", "compute").to_string(),
            "mesh_relay:peer1/compute"
        );
    }

    #[test]
    fn unknown_transport_tag_rejected() {
        let json = r#"{"transport":"quic","addr":"example.com:443"}"#;
        let result = serde_json::from_str::<TransportEndpoint>(json);
        assert!(result.is_err());
    }
}
