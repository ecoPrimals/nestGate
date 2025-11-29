
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported network protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
/// Protocol
pub enum Protocol {
    /// Http
    Http,
    /// Https
    Https,
    /// Tcp
    Tcp,
    /// Udp
    Udp,
    /// Websocket
    Websocket,
    /// Grpc
    Grpc,
}
impl std::fmt::Display for Protocol {
    /// Fmt
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Http => write!(f, "HTTP"),
            Protocol::Https => write!(f, "HTTPS"),
            Protocol::Tcp => write!(f, "TCP"),
            Protocol::Udp => write!(f, "UDP"),
            Protocol::Websocket => write!(f, "WebSocket"),
            Protocol::Grpc => write!(f, "gRPC"),
        }
    }
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// ⚠️ DEPRECATED: This config has been consolidated into canonical_primary
/// 
/// **Migration Path**:
/// ```rust
/// // OLD (deprecated):
/// use crate::network::config::ProtocolConfig;
/// 
/// // NEW (canonical):
/// use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;
/// // Or use type alias for compatibility:
/// use crate::network::config::ProtocolConfig; // Now aliases to CanonicalNetworkConfig
/// ```
/// 
/// **Timeline**: This type alias will be maintained until v0.12.0 (May 2026)
#[deprecated(since = "0.11.0", note = "Use nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig instead")]
/// Configuration for Protocol
pub struct ProtocolConfig {
    /// Protocol
    pub protocol: Protocol,
    /// Port
    pub port: u16,
    /// Whether this feature is enabled
    pub enabled: bool,
    /// Options
    pub options: HashMap<String, String>,
}
/// Protocol manager
pub struct ProtocolManager {
    protocols: HashMap<Protocol, ProtocolConfig>,
}
impl ProtocolManager {
    /// Create a new protocol manager
    #[must_use]
    pub fn new() -> Self { Self {
            protocols: HashMap::new(),
         }

    /// Add protocol configuration
    pub fn add_protocol(&mut self, config: ProtocolConfig) {
        self.protocols.insert(config.protocol.clone(), config);
    }

    /// Get protocol configuration
    pub fn get_protocol(&self, protocol: &Protocol) -> Option<&ProtocolConfig> {
        self.protocols.get(protocol)
    }

    /// Enable protocol
    pub fn enable_protocol(&mut self, protocol: Protocol) -> bool {
        if let Some(config) = self.protocols.get_mut(&protocol) {
            config.enabled = true;
            true
        } else {
            false
        }
    }

    /// Disable protocol
    pub fn disable_protocol(&mut self, protocol: Protocol) -> bool {
        if let Some(config) = self.protocols.get_mut(&protocol) {
            config.enabled = false;
            true
        } else {
            false
        }
    }

    /// Get all protocols
    pub fn get_all_protocols(&self) -> Vec<&ProtocolConfig> {
        self.protocols.values().collect()
    }

    /// Get enabled protocols
    pub fn get_enabled_protocols(&self) -> Vec<&ProtocolConfig> {
        self.protocols
            .values()
            .filter(|config| config.enabled)
            .collect()
    }
}

impl Default for ProtocolManager {
    /// Returns the default instance
    fn default() -> Self {
        Self::new()
    }
}

// ==================== CANONICAL TYPE ALIAS ====================
// This type now aliases to the canonical network configuration
// Original struct definition kept above for reference and backward compatibility

/// Type alias to canonical network configuration
/// 
/// This provides backward compatibility while migrating to unified configuration.
/// The original struct is marked as deprecated but still functional.
#[allow(deprecated)]
/// Type alias for Protocolconfigcanonical
pub type ProtocolConfigCanonical = nestgate_core::config::canonical_primary::domains::network::CanonicalNetworkConfig;

// Note: Keep using ProtocolConfig (the deprecated struct) for now.
// We'll gradually migrate to CanonicalNetworkConfig directly in a later phase.
// This alias is here for reference and future migration.

