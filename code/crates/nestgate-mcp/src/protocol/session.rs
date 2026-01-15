//! **SESSION TYPES**
//!
//! MCP session, client info, and server capabilities.

use serde::{Deserialize, Serialize};

/// MCP session information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSession {
    /// Session identifier
    pub session_id: String,
    /// Client Info
    pub client_info: ClientInfo,
    /// Server Capabilities
    pub server_capabilities: ServerCapabilities,
}

/// Client information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    /// Name
    pub name: String,
    /// Version
    pub version: String,
}

/// Server capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    /// Tools
    pub tools: Vec<String>,
    /// Resources
    pub resources: Vec<String>,
    /// Prompts
    pub prompts: Vec<String>,
}
