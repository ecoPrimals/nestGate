// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! NAT Traversal Persistence
//!
//! Stores NAT traversal info, port allocation patterns, relay preferences,
//! and known beacon records so they survive restarts. Other primals (e.g. a
//! networking/STUN provider and an ecosystem coordinator) produce this data;
//! NestGate persists it.
//!
//! ## JSON-RPC Methods
//!
//! | Method | Description |
//! |--------|-------------|
//! | `nat.store_traversal_info` | Persist this device's NAT traversal info |
//! | `nat.retrieve_traversal_info` | Retrieve persisted NAT traversal info |
//! | `beacon.store` | Store a known beacon (discovered peer) |
//! | `beacon.retrieve` | Retrieve a beacon by peer_id |
//! | `beacon.list` | List all known beacon peer_ids |
//! | `beacon.delete` | Delete a beacon by peer_id |
//!
//! ## Storage Layout
//!
//! Uses the existing `StorageManagerService` with reserved datasets:
//!
//! - `_nat_traversal` — Our own NAT info (single key: `"self"`)
//! - `_known_beacons` — One object per discovered peer, keyed by `peer_id`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Reserved dataset name for NAT traversal info
pub const NAT_DATASET: &str = "_nat_traversal";

/// Reserved dataset name for known beacon records
pub const BEACON_DATASET: &str = "_known_beacons";

/// Key used for this device's own NAT info within the NAT dataset
pub const NAT_SELF_KEY: &str = "self";

// ─── NAT Classification ───────────────────────────────────────────────

/// NAT type classification as detected by STUN probing.
///
/// The networking/STUN capability's `stun.detect_nat_type` produces this value; NestGate persists it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NatType {
    /// Endpoint-Independent Mapping — any external host can reach mapped port
    FullCone,
    /// Address-Restricted — only hosts we've sent to can reply
    RestrictedCone,
    /// Port-Restricted — only host:port pairs we've sent to can reply
    PortRestricted,
    /// Symmetric — different mapping per destination, hardest to punch through
    Symmetric,
    /// Detection failed or not yet performed
    #[default]
    Unknown,
}

impl std::fmt::Display for NatType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FullCone => write!(f, "full_cone"),
            Self::RestrictedCone => write!(f, "restricted_cone"),
            Self::PortRestricted => write!(f, "port_restricted"),
            Self::Symmetric => write!(f, "symmetric"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

// ─── Port Pattern ─────────────────────────────────────────────────────

/// Port allocation pattern detected by repeated STUN probes.
///
/// The networking/STUN capability's `stun.probe_port_pattern` sends N probes from the same socket
/// and observes the external port assigned by the NAT. The pattern determines
/// whether port prediction is feasible for coordinated punch.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PortPattern {
    /// Sequential allocation (step between consecutive ports is constant).
    /// Prediction is feasible with `confidence` probability.
    Sequential {
        /// Delta between consecutive allocated ports (e.g., 1 or 5)
        step: i32,
        /// Last port observed during probing
        last_port: u16,
        /// Predicted next port based on the pattern
        predicted_next: u16,
        /// Confidence in the prediction (0.0 – 1.0)
        confidence: f64,
    },
    /// Random or unpredictable allocation. Punch requires spray approach.
    Random {
        /// Observed ports from STUN probes
        observed: Vec<u16>,
    },
    /// Not yet probed or detection failed
    #[default]
    Unknown,
}

// ─── Relay Endpoint ───────────────────────────────────────────────────

/// A known relay node's endpoints (relay + STUN).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelayEndpoint {
    /// UDP address for relay data forwarding (e.g., "192.0.2.10:3479")
    pub relay_addr: String,
    /// STUN server address on the same host (e.g., "192.0.2.10:3478")
    pub stun_addr: String,
}

// ─── NAT Traversal Info ───────────────────────────────────────────────

/// This device's NAT traversal information, persisted for fast reconnection.
///
/// After STUN probes run via the networking capability, the results are stored here so future
/// connections can skip the probe phase and jump straight to relay or punch.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NatTraversalInfo {
    /// Detected NAT type for this device
    pub our_nat_type: NatType,
    /// Port allocation pattern from STUN probing
    pub port_pattern: PortPattern,
    /// ISO 8601 timestamp of the last probe
    pub last_probed: String,
    /// Known family relay nodes, keyed by relay name (e.g., "tower")
    pub family_relay: HashMap<String, RelayEndpoint>,
}

// ─── Known Beacon ─────────────────────────────────────────────────────

/// A known beacon record — represents a previously discovered peer.
///
/// Beacons are created when peers meet at the rendezvous. NestGate persists
/// them so that future connections to the same peer can skip rendezvous and
/// use cached NAT info and relay preferences.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KnownBeacon {
    /// Unique identifier for this peer
    pub peer_id: String,
    /// Family this peer belongs to
    pub family_id: String,
    /// ISO 8601 timestamp of the last time this peer was seen
    pub last_seen: String,
    /// Peer's public address (from STUN)
    pub public_addr: Option<String>,
    /// Peer's NAT type (if known)
    pub nat_type: Option<NatType>,
    /// Peer's port allocation pattern (if probed)
    pub port_pattern: Option<PortPattern>,
    /// Preferred relay for reaching this peer
    pub relay_preference: Option<RelayPreference>,
    /// History of connection attempts to this peer
    #[serde(default)]
    pub connection_history: Vec<ConnectionRecord>,
}

// ─── Relay Preference ─────────────────────────────────────────────────

/// Preferred relay configuration for reaching a specific peer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelayPreference {
    /// Human-readable name of the preferred relay (e.g., "tower")
    pub preferred_relay: String,
    /// Relay's data forwarding address
    pub relay_addr: String,
    /// STUN server at the relay
    pub stun_addr: String,
}

// ─── Connection Record ────────────────────────────────────────────────

/// Record of a connection attempt or successful connection to a peer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConnectionRecord {
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// How the connection was established (or attempted)
    pub method: ConnectionMethod,
    /// Whether the connection succeeded
    pub success: bool,
    /// Round-trip time in milliseconds (if connection succeeded)
    pub duration_ms: Option<u64>,
}

/// Method used for a connection attempt.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionMethod {
    /// Direct connection (same LAN or public IP)
    Direct,
    /// UDP hole punch (blind or basic)
    HolePunch,
    /// Relay-assisted coordinated punch (the new protocol)
    RelayAssisted,
    /// Pure relay (fallback, data flows through relay)
    PureRelay,
    /// Local area network discovery
    Lan,
}

impl std::fmt::Display for ConnectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Direct => write!(f, "direct"),
            Self::HolePunch => write!(f, "hole_punch"),
            Self::RelayAssisted => write!(f, "relay_assisted"),
            Self::PureRelay => write!(f, "pure_relay"),
            Self::Lan => write!(f, "lan"),
        }
    }
}

// ─── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nat_type_serialization() {
        let nat = NatType::Symmetric;
        let json = serde_json::to_string(&nat).expect("serialize");
        assert_eq!(json, r#""symmetric""#);

        let deser: NatType = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deser, NatType::Symmetric);
    }

    #[test]
    fn test_nat_type_default() {
        assert_eq!(NatType::default(), NatType::Unknown);
    }

    #[test]
    fn test_nat_type_display() {
        assert_eq!(NatType::FullCone.to_string(), "full_cone");
        assert_eq!(NatType::Symmetric.to_string(), "symmetric");
        assert_eq!(NatType::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_port_pattern_sequential_serde() {
        let pattern = PortPattern::Sequential {
            step: 1,
            last_port: 41204,
            predicted_next: 41205,
            confidence: 0.85,
        };
        let json = serde_json::to_value(&pattern).expect("serialize");
        assert_eq!(json["type"], "sequential");
        assert_eq!(json["step"], 1);
        assert_eq!(json["predicted_next"], 41205);
        assert!((json["confidence"].as_f64().unwrap_or(0.0) - 0.85).abs() < f64::EPSILON);

        let deser: PortPattern = serde_json::from_value(json).expect("deserialize");
        assert_eq!(deser, pattern);
    }

    #[test]
    fn test_port_pattern_random_serde() {
        let pattern = PortPattern::Random {
            observed: vec![52100, 52105, 52110],
        };
        let json = serde_json::to_value(&pattern).expect("serialize");
        assert_eq!(json["type"], "random");
        assert_eq!(json["observed"].as_array().unwrap().len(), 3);

        let deser: PortPattern = serde_json::from_value(json).expect("deserialize");
        assert_eq!(deser, pattern);
    }

    #[test]
    fn test_port_pattern_unknown_serde() {
        let pattern = PortPattern::Unknown;
        let json = serde_json::to_value(&pattern).expect("serialize");
        assert_eq!(json["type"], "unknown");
    }

    #[test]
    fn test_nat_traversal_info_roundtrip() {
        let mut relays = HashMap::new();
        relays.insert(
            "tower".to_string(),
            RelayEndpoint {
                relay_addr: "192.0.2.10:3479".to_string(),
                stun_addr: "192.0.2.10:3478".to_string(),
            },
        );

        let info = NatTraversalInfo {
            our_nat_type: NatType::Symmetric,
            port_pattern: PortPattern::Sequential {
                step: 1,
                last_port: 41204,
                predicted_next: 41205,
                confidence: 0.85,
            },
            last_probed: "2026-02-11T20:00:00Z".to_string(),
            family_relay: relays,
        };

        let json = serde_json::to_vec(&info).expect("serialize");
        let deser: NatTraversalInfo = serde_json::from_slice(&json).expect("deserialize");
        assert_eq!(deser, info);
    }

    #[test]
    fn test_known_beacon_roundtrip() {
        let beacon = KnownBeacon {
            peer_id: "pixel-abc123".to_string(),
            family_id: "nat0".to_string(),
            last_seen: "2026-02-11T20:30:00Z".to_string(),
            public_addr: Some("1.2.3.4:41200".to_string()),
            nat_type: Some(NatType::Symmetric),
            port_pattern: Some(PortPattern::Sequential {
                step: 1,
                last_port: 41204,
                predicted_next: 41205,
                confidence: 0.85,
            }),
            relay_preference: Some(RelayPreference {
                preferred_relay: "tower".to_string(),
                relay_addr: "192.0.2.10:3479".to_string(),
                stun_addr: "192.0.2.10:3478".to_string(),
            }),
            connection_history: vec![ConnectionRecord {
                timestamp: "2026-02-11T20:30:00Z".to_string(),
                method: ConnectionMethod::RelayAssisted,
                success: true,
                duration_ms: Some(150),
            }],
        };

        let json = serde_json::to_vec(&beacon).expect("serialize");
        let deser: KnownBeacon = serde_json::from_slice(&json).expect("deserialize");
        assert_eq!(deser, beacon);
    }

    #[test]
    fn test_known_beacon_minimal() {
        let beacon = KnownBeacon {
            peer_id: "usb-xyz".to_string(),
            family_id: "nat0".to_string(),
            last_seen: "2026-02-11T21:00:00Z".to_string(),
            public_addr: None,
            nat_type: None,
            port_pattern: None,
            relay_preference: None,
            connection_history: vec![],
        };

        let json = serde_json::to_vec(&beacon).expect("serialize");
        let deser: KnownBeacon = serde_json::from_slice(&json).expect("deserialize");
        assert_eq!(deser, beacon);
    }

    #[test]
    fn test_connection_method_display() {
        assert_eq!(ConnectionMethod::Direct.to_string(), "direct");
        assert_eq!(ConnectionMethod::HolePunch.to_string(), "hole_punch");
        assert_eq!(
            ConnectionMethod::RelayAssisted.to_string(),
            "relay_assisted"
        );
        assert_eq!(ConnectionMethod::PureRelay.to_string(), "pure_relay");
        assert_eq!(ConnectionMethod::Lan.to_string(), "lan");
    }

    #[test]
    fn test_connection_method_serde() {
        let method = ConnectionMethod::RelayAssisted;
        let json = serde_json::to_string(&method).expect("serialize");
        assert_eq!(json, r#""relay_assisted""#);

        let deser: ConnectionMethod = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(deser, method);
    }
}
