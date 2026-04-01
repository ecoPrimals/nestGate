// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (c) 2025-2026 ecoPrimals Collective

//! **MCP DOMAIN CONFIGURATION**
//!
//! Consolidates all MCP protocol configurations:
//! - Protocol settings and streaming configuration
//! - Connection management and security settings
//!
//! ## Usage
//!
//! ```rust,ignore
//! use nestgate_core::config::canonical_primary::domains::consolidated_domains::mcp::*;
//!
//! let mcp_config = McpDomainConfig::default();
//! assert_eq!(mcp_config.protocol.version, "1.0");
//! ```

use super::validation::DomainConfigValidation;
use serde::{Deserialize, Serialize};
use std::time::Duration;

// ==================== MCP DOMAIN CONFIGURATION ====================

/// **MCP DOMAIN CONFIGURATION**
///
/// Consolidates all MCP protocol configurations including protocol settings,
/// connection management, and security.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpDomainConfig {
    /// MCP protocol configuration
    pub protocol: McpProtocolConfig,

    /// Streaming configuration
    pub streaming: McpStreamingConfig,

    /// Connection configuration
    pub connection: McpConnectionConfig,

    /// Security configuration
    pub security: McpSecurityConfig,

    /// Performance configuration
    pub performance: McpPerformanceConfig,
}

// ==================== PROTOCOL CONFIGURATION ====================

/// MCP protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpProtocolConfig {
    /// Protocol version
    pub version: String,

    /// Message format
    pub message_format: McpMessageFormat,

    /// Enable compression
    pub compression: bool,

    /// Heartbeat interval
    pub heartbeat_interval: Duration,

    /// Timeout configuration
    pub timeouts: McpTimeoutConfig,
}

impl Default for McpProtocolConfig {
    fn default() -> Self {
        Self {
            version: "1.0".to_string(),
            message_format: McpMessageFormat::Json,
            compression: false,
            heartbeat_interval: Duration::from_secs(30),
            timeouts: McpTimeoutConfig::default(),
        }
    }
}

/// MCP message format
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum McpMessageFormat {
    /// JSON format
    #[default]
    Json,
    /// `MessagePack` format
    MessagePack,
    /// Protobuf format
    Protobuf,
}

/// MCP timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTimeoutConfig {
    /// Connection timeout
    pub connect: Duration,
    /// Read timeout
    pub read: Duration,
    /// Write timeout
    pub write: Duration,
}

impl Default for McpTimeoutConfig {
    fn default() -> Self {
        Self {
            connect: Duration::from_secs(5),
            read: Duration::from_secs(30),
            write: Duration::from_secs(30),
        }
    }
}

// ==================== ADDITIONAL MCP CONFIGURATION ====================

/// Configuration for MCP streaming settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpStreamingConfig {}

/// Configuration for MCP connection management
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpConnectionConfig {}

/// Configuration for MCP security settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpSecurityConfig {}

/// Configuration for MCP performance tuning
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpPerformanceConfig {}

// ==================== VALIDATION ====================

impl DomainConfigValidation for McpDomainConfig {
    fn validate(&self) -> nestgate_types::error::Result<Vec<String>> {
        let mut warnings = Vec::new();

        // Validate protocol version
        if self.protocol.version.is_empty() {
            warnings.push("Protocol version is empty".to_string());
        }

        Ok(warnings)
    }

    fn validate_for_environment(&self, _env: &str) -> nestgate_types::error::Result<()> {
        Ok(())
    }

    fn required_fields() -> Vec<&'static str> {
        vec!["protocol.version"]
    }

    fn optional_fields() -> Vec<&'static str> {
        vec!["streaming", "security"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_domain_config_default() {
        let config = McpDomainConfig::default();
        assert_eq!(config.protocol.version, "1.0");
    }

    #[test]
    fn test_mcp_protocol_config() {
        let protocol = McpProtocolConfig::default();
        assert!(!protocol.compression);
        assert_eq!(protocol.heartbeat_interval, Duration::from_secs(30));
    }

    #[test]
    fn test_mcp_message_format() {
        let format = McpMessageFormat::default();
        assert!(matches!(format, McpMessageFormat::Json));
    }

    #[test]
    fn test_validation() {
        let config = McpDomainConfig::default();
        let warnings = config.validate().expect("Should validate");
        assert!(warnings.is_empty());
    }
}
