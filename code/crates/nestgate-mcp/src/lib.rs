//! `NestGate` MCP (Model Context Protocol) Integration
//!
//! This crate provides MCP protocol support for the `NestGate` ecosystem.

// Remove all the invalid imports and use the corrected ones
use nestgate_core::Result;

// Re-export the error handling functions
pub use error::{
    extract_mcp_context, extract_method, extract_session_id, mcp_connection_error, method_error,
    protocol_error, serialization_error, session_error, transport_error, McpErrorExt,
};

pub mod config;
pub mod error;

// ==================== SECTION ====================

/// MCP service trait using canonical error patterns
#[allow(async_fn_in_trait)]
pub trait McpService {
    /// Initialize the MCP service
    fn initialize(&mut self) -> Result<()>;
    /// Get service health status
    async fn health_check(&self) -> Result<McpHealthStatus>;

    /// Shutdown the service gracefully
    async fn shutdown(&mut self) -> Result<()>;
}

/// MCP health status using canonical patterns
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpHealthStatus {
    pub is_healthy: bool,
    pub message: String,
    pub details: Option<std::collections::HashMap<String, String>>,
}
impl McpHealthStatus {
    #[must_use]
    pub fn healthy() -> Self {
        Self {
            is_healthy: true,
            message: "MCP service is healthy".to_string(),
            details: None,
        }
    }

    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            is_healthy: false,
            message: message.into(),
            details: None,
        }
    }

    #[must_use]
    pub fn with_details(mut self, details: std::collections::HashMap<String, String>) -> Self {
        self.details = Some(details);
        self
    }
}

// ==================== SECTION ====================

/// Default MCP configuration constants
pub mod constants {
    use nestgate_core::canonical_modernization::canonical_constants::network::{
        CONNECTION_TIMEOUT_SECS, REQUEST_TIMEOUT_SECS,
    };
    use std::time::Duration;
    /// Default connection timeout - **CANONICAL MODERNIZATION**
    pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(CONNECTION_TIMEOUT_SECS);

    /// Default request timeout (extended for MCP operations) - **CANONICAL MODERNIZATION**
    pub const DEFAULT_REQUEST_TIMEOUT: Duration = Duration::from_secs(REQUEST_TIMEOUT_SECS);

    /// Default retry attempts - **CANONICAL MODERNIZATION**
    pub const DEFAULT_RETRY_ATTEMPTS: u32 = 3; // Reasonable default for MCP operations

    /// Default retry delay - **CANONICAL MODERNIZATION**
    pub const DEFAULT_RETRY_DELAY: Duration = Duration::from_millis(1000); // 1 second initial delay

    /// Default protocol version
    pub const DEFAULT_PROTOCOL_VERSION: &str = "2024-11-05";

    /// Default user agent
    pub const DEFAULT_USER_AGENT: &str = "nestgate-mcp/0.1.0";
}

// ==================== SECTION ====================

/// Create a canonical MCP client configuration
/// **MODERNIZED**: Uses canonical configuration system
#[must_use]
pub fn create_default_config() -> nestgate_core::config::canonical_primary::McpConfig {
    nestgate_core::config::canonical_primary::McpConfig::default()
}
/// Validate MCP protocol version compatibility
#[must_use]
pub fn is_protocol_version_supported(version: &str) -> bool {
    matches!(version, "2024-11-05" | "2024-10-07" | "2024-09-25")
}
/// Create a standardized MCP error response
pub fn create_error_response(
    error_type: nestgate_core::NestGateError,
    message: impl Into<String>,
) -> serde_json::Value {
    serde_json::json!({
        "error": {
            "type": error_type.to_string(),
            "message": message.into(),
            "timestamp": std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        }
    })
}
