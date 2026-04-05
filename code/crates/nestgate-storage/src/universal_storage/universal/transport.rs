// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright (c) 2025-2026 ecoPrimals Collective

//! # Transport Layer
//!
//! Defines how data moves between client and storage, independent of vendor.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// How does data move between client and storage?
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportProtocol {
    /// HTTP/HTTPS transport
    Http {
        /// HTTP version
        version: HttpVersion,
        /// TLS configuration (if HTTPS)
        tls: Option<TlsConfig>,
    },

    /// Raw TCP with custom framing
    Tcp {
        /// How are messages framed?
        framing: FramingProtocol,
    },

    /// QUIC transport (HTTP/3)
    Quic {
        /// QUIC-specific configuration
        config: QuicConfig,
    },

    /// Unix domain socket
    UnixSocket {
        /// Socket path
        path: String,
    },

    /// Custom transport protocol
    Custom {
        /// Protocol identifier
        protocol_id: String,
        /// Protocol description
        description: String,
    },
}

impl TransportProtocol {
    /// Get a human-readable description
    #[must_use]
    pub fn description(&self) -> String {
        match self {
            Self::Http { version, tls } => {
                format!(
                    "HTTP/{} {}",
                    version.as_str(),
                    if tls.is_some() { "+ TLS" } else { "" }
                )
            }
            Self::Tcp { framing } => format!("TCP ({})", framing.description()),
            Self::Quic { .. } => "QUIC (HTTP/3)".to_string(),
            Self::UnixSocket { path } => format!("Unix Socket ({path})"),
            Self::Custom { protocol_id, .. } => format!("Custom ({protocol_id})"),
        }
    }

    /// Is this transport secure by default?
    #[must_use]
    pub const fn is_secure(&self) -> bool {
        match self {
            Self::Http { tls, .. } => tls.is_some(),
            Self::Quic { .. } | Self::UnixSocket { .. } => true, // QUIC encrypts; local socket is trusted
            Self::Tcp { .. } | Self::Custom { .. } => false,
        }
    }
}

/// HTTP protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpVersion {
    /// HTTP/1.0
    Http1_0,
    /// HTTP/1.1 (most common)
    Http1_1,
    /// HTTP/2
    Http2,
    /// HTTP/3 (over QUIC)
    Http3,
}

impl HttpVersion {
    /// Get version string
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Http1_0 => "1.0",
            Self::Http1_1 => "1.1",
            Self::Http2 => "2",
            Self::Http3 => "3",
        }
    }
}

/// TLS configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Minimum TLS version
    pub min_version: TlsVersion,
    /// Verify server certificate
    pub verify_cert: bool,
    /// Client certificate (for mutual TLS)
    pub client_cert: Option<String>,
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            min_version: TlsVersion::Tls1_2,
            verify_cert: true,
            client_cert: None,
        }
    }
}

/// TLS version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TlsVersion {
    /// TLS 1.0 (deprecated, insecure)
    Tls1_0,
    /// TLS 1.1 (deprecated, insecure)
    Tls1_1,
    /// TLS 1.2 (minimum recommended)
    Tls1_2,
    /// TLS 1.3 (modern, secure)
    Tls1_3,
}

/// TCP framing protocol
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FramingProtocol {
    /// Length-prefixed messages
    LengthPrefixed {
        /// Length field size in bytes
        length_bytes: usize,
    },

    /// Delimiter-based framing
    Delimited {
        /// Delimiter byte(s)
        delimiter: Vec<u8>,
    },

    /// Fixed-size messages
    FixedSize {
        /// Message size in bytes
        size: usize,
    },

    /// Custom framing
    Custom {
        /// Framing description
        description: String,
    },
}

impl FramingProtocol {
    /// Get description
    #[must_use]
    pub fn description(&self) -> &str {
        match self {
            Self::LengthPrefixed { .. } => "length-prefixed",
            Self::Delimited { .. } => "delimited",
            Self::FixedSize { .. } => "fixed-size",
            Self::Custom { description } => description,
        }
    }
}

/// QUIC configuration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct QuicConfig {
    /// Maximum idle timeout
    pub max_idle_timeout: Duration,
    /// Enable 0-RTT
    pub enable_0rtt: bool,
}

impl Default for QuicConfig {
    fn default() -> Self {
        Self {
            max_idle_timeout: Duration::from_secs(30),
            enable_0rtt: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_transport_description() {
        let transport = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: Some(TlsConfig::default()),
        };

        assert_eq!(transport.description(), "HTTP/1.1 + TLS");
        assert!(transport.is_secure());
    }

    #[test]
    fn test_http_version_strings() {
        assert_eq!(HttpVersion::Http1_1.as_str(), "1.1");
        assert_eq!(HttpVersion::Http2.as_str(), "2");
        assert_eq!(HttpVersion::Http3.as_str(), "3");
    }

    #[test]
    fn test_transport_security() {
        let secure = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: Some(TlsConfig::default()),
        };
        assert!(secure.is_secure());

        let insecure = TransportProtocol::Http {
            version: HttpVersion::Http1_1,
            tls: None,
        };
        assert!(!insecure.is_secure());

        let quic = TransportProtocol::Quic {
            config: QuicConfig::default(),
        };
        assert!(quic.is_secure());
    }
}
