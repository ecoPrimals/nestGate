// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! IPC Protocol Abstraction for G65 Protocol Negotiation
//!
//! Defines the protocol selector used by the G65 wire handshake to choose
//! between JSON-RPC 2.0 and tarpc at connection time on a single socket.

use serde::{Deserialize, Serialize};
use std::fmt;

/// RPC protocol for IPC connections.
///
/// Supports automatic protocol selection via G65 protocol negotiation.
/// `JsonRpc` is the default (backward-compatible); `Tarpc` provides
/// high-performance binary RPC for primal-to-primal communication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum IpcProtocol {
    /// JSON-RPC 2.0 — default, backward-compatible, human-readable.
    #[default]
    JsonRpc,

    /// tarpc — binary, type-safe, high-performance primal-to-primal RPC.
    Tarpc,
}

impl fmt::Display for IpcProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.negotiation_name())
    }
}

impl IpcProtocol {
    /// Parse protocol from a wire-name string (case-insensitive).
    #[expect(
        clippy::should_implement_trait,
        reason = "Custom from_str avoids FromStr trait conflict with std"
    )]
    #[must_use]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "jsonrpc" | "json-rpc" | "json_rpc" => Some(Self::JsonRpc),
            "tarpc" => Some(Self::Tarpc),
            _ => None,
        }
    }

    /// All protocols supported by this build.
    #[must_use]
    pub fn supported() -> Vec<Self> {
        vec![Self::JsonRpc, Self::Tarpc]
    }

    /// Wire name used in the G65 `PROTOCOLS:` / `PROTOCOL:` handshake.
    #[must_use]
    pub const fn negotiation_name(&self) -> &'static str {
        match self {
            Self::JsonRpc => "jsonrpc",
            Self::Tarpc => "tarpc",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_jsonrpc() {
        assert_eq!(IpcProtocol::default(), IpcProtocol::JsonRpc);
    }

    #[test]
    fn from_str_known_names() {
        assert_eq!(IpcProtocol::from_str("jsonrpc"), Some(IpcProtocol::JsonRpc));
        assert_eq!(
            IpcProtocol::from_str("json-rpc"),
            Some(IpcProtocol::JsonRpc)
        );
        assert_eq!(
            IpcProtocol::from_str("JSON_RPC"),
            Some(IpcProtocol::JsonRpc)
        );
        assert_eq!(IpcProtocol::from_str("tarpc"), Some(IpcProtocol::Tarpc));
        assert_eq!(IpcProtocol::from_str("TARPC"), Some(IpcProtocol::Tarpc));
    }

    #[test]
    fn from_str_unknown_returns_none() {
        assert_eq!(IpcProtocol::from_str("grpc"), None);
        assert_eq!(IpcProtocol::from_str(""), None);
    }

    #[test]
    fn supported_includes_both() {
        let s = IpcProtocol::supported();
        assert!(s.contains(&IpcProtocol::JsonRpc));
        assert!(s.contains(&IpcProtocol::Tarpc));
    }

    #[test]
    fn display_matches_wire_name() {
        assert_eq!(IpcProtocol::JsonRpc.to_string(), "jsonrpc");
        assert_eq!(IpcProtocol::Tarpc.to_string(), "tarpc");
    }

    #[test]
    fn negotiation_name_roundtrips() {
        for proto in IpcProtocol::supported() {
            let name = proto.negotiation_name();
            assert_eq!(IpcProtocol::from_str(name), Some(proto));
        }
    }
}
