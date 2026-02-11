/// Network and Protocol Classification Enums
/// This module contains enums related to network protocols, integration types,
/// and networking infrastructure.
use serde::{Deserialize, Serialize};
use std::fmt;
// ==================== SECTION ====================

/// **THE** `ProtocolType` - unified across all modules
/// Replaces `ProtocolType` definitions in network and API modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of UnifiedProtocol
pub enum UnifiedProtocolType {
    /// HTTP protocol
    Http,
    /// HTTPS protocol
    Https,
    /// WebSocket protocol
    WebSocket,
    /// TCP protocol
    Tcp,
    /// UDP protocol
    Udp,
    /// gRPC protocol
    Grpc,
    /// REST API protocol
    Rest,
    /// GraphQL protocol
    GraphQL,
    /// Message queue protocol
    MessageQueue,
    /// Custom protocol
    Custom(String),
}
impl Default for UnifiedProtocolType {
    /// Returns the default instance
    fn default() -> Self {
        Self::Http
    }
}

impl fmt::Display for UnifiedProtocolType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
            Self::WebSocket => write!(f, "websocket"),
            Self::Tcp => write!(f, "tcp"),
            Self::Udp => write!(f, "udp"),
            Self::Grpc => write!(f, "grpc"),
            Self::Rest => write!(f, "rest"),
            Self::GraphQL => write!(f, "graphql"),
            Self::MessageQueue => write!(f, "message_queue"),
            Self::Custom(protocol) => write!(f, "{protocol}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `IntegrationType` - unified across all modules
/// Replaces `IntegrationType` definitions in ecosystem integration modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of UnifiedIntegration
pub enum UnifiedIntegrationType {
    /// Direct API integration
    DirectApi,
    /// Database integration
    Database,
    /// File system integration
    FileSystem,
    /// Message queue integration
    MessageQueue,
    /// Webhook integration
    Webhook,
    /// Event-driven integration
    EventDriven,
    /// Batch processing integration
    BatchProcessing,
    /// Real-time streaming integration
    RealTimeStreaming,
    /// Custom integration type
    Custom(String),
}
impl Default for UnifiedIntegrationType {
    /// Returns the default instance
    fn default() -> Self {
        Self::DirectApi
    }
}

impl fmt::Display for UnifiedIntegrationType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DirectApi => write!(f, "direct_api"),
            Self::Database => write!(f, "database"),
            Self::FileSystem => write!(f, "filesystem"),
            Self::MessageQueue => write!(f, "message_queue"),
            Self::Webhook => write!(f, "webhook"),
            Self::EventDriven => write!(f, "event_driven"),
            Self::BatchProcessing => write!(f, "batch_processing"),
            Self::RealTimeStreaming => write!(f, "real_time_streaming"),
            Self::Custom(integration_type) => write!(f, "{integration_type}"),
        }
    }
}

// ==================== SECTION ====================

/// **THE** `ProxyType` - unified across all modules
/// Replaces `ProxyType` definitions in ecosystem integration and network modules
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of UnifiedProxy
pub enum UnifiedProxyType {
    /// HTTP proxy
    Http,
    /// HTTPS proxy
    Https,
    /// SOCKS4 proxy
    Socks4,
    /// SOCKS5 proxy
    Socks5,
    /// Transparent proxy
    Transparent,
    /// Reverse proxy
    Reverse,
    /// Load balancer proxy
    LoadBalancer,
    /// No proxy
    None,
    /// Custom proxy type
    Custom(String),
}
impl Default for UnifiedProxyType {
    /// Returns the default instance
    fn default() -> Self {
        Self::None
    }
}

impl fmt::Display for UnifiedProxyType {
    /// Fmt
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http => write!(f, "http"),
            Self::Https => write!(f, "https"),
            Self::Socks4 => write!(f, "socks4"),
            Self::Socks5 => write!(f, "socks5"),
            Self::Transparent => write!(f, "transparent"),
            Self::Reverse => write!(f, "reverse"),
            Self::LoadBalancer => write!(f, "load_balancer"),
            Self::None => write!(f, "none"),
            Self::Custom(proxy_type) => write!(f, "{proxy_type}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_protocol_type_default() {
        assert!(matches!(
            UnifiedProtocolType::default(),
            UnifiedProtocolType::Http
        ));
    }

    #[test]
    fn test_unified_protocol_type_custom() {
        let pt = UnifiedProtocolType::Custom("mqtt".to_string());
        assert_eq!(pt.to_string(), "mqtt");
    }

    #[test]
    fn test_unified_protocol_type_serialization() {
        let pt = UnifiedProtocolType::WebSocket;
        let json = serde_json::to_string(&pt).unwrap();
        let parsed: UnifiedProtocolType = serde_json::from_str(&json).unwrap();
        assert_eq!(pt, parsed);
    }

    #[test]
    fn test_unified_integration_type_display() {
        assert_eq!(UnifiedIntegrationType::DirectApi.to_string(), "direct_api");
        assert_eq!(UnifiedIntegrationType::Database.to_string(), "database");
    }

    #[test]
    fn test_unified_proxy_type_default() {
        assert!(matches!(
            UnifiedProxyType::default(),
            UnifiedProxyType::None
        ));
    }
}
