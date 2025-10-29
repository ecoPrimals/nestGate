
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Supported network protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Protocol {
    Http,
    Https,
    Tcp,
    Udp,
    Websocket,
    Grpc,
}
impl std::fmt::Display for Protocol {
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
pub struct ProtocolConfig {
    pub protocol: Protocol,
    pub port: u16,
    pub enabled: bool,
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
    fn default() -> Self {
        Self::new()
    }
}
