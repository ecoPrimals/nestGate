//! HTTP Client Delegation Layer - ecoBin Architecture
//!
//! **NestGate does NOT make direct HTTP calls.** This module provides type stubs
//! that satisfy compile-time type requirements while enforcing the ecoBin principle:
//! all external HTTP is delegated to the network-capability primal (e.g. Songbird)
//! via JSON-RPC over Unix sockets.
//!
//! ## ecoBin Compliance
//!
//! Per `wateringHole/ECOBIN_ARCHITECTURE_STANDARD.md`:
//! - NestGate is a **storage** primal (not a network primal)
//! - External HTTP requires the `network` capability
//! - Capability is discovered at runtime, not imported as a dependency
//!
//! ## Usage
//!
//! Modules that historically used `reqwest` should instead:
//! 1. Discover the network-capability primal via socket scanning
//! 2. Send an `http.get` / `http.post` JSON-RPC request
//! 3. Receive the response via IPC
//!
//! ```rust,ignore
//! use nestgate_core::rpc::isomorphic_ipc::atomic::discover_primal_socket;
//!
//! // Discover network-capable primal
//! let socket = discover_primal_socket("network-provider")?;
//! // Send JSON-RPC: {"method": "http.get", "params": {"url": "..."}}
//! ```

use crate::error::Result;

/// HTTP client placeholder - use JSON-RPC delegation for actual HTTP requests
///
/// This type exists for compile-time compatibility. All methods are no-ops.
/// Real HTTP should be delegated via IPC to a network-capability primal.
#[derive(Debug, Clone)]
pub struct Client;

impl Client {
    /// Create a new client (compile-time placeholder)
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builder pattern (compile-time placeholder)
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Client builder placeholder
#[derive(Debug)]
pub struct ClientBuilder;

impl ClientBuilder {
    /// Set timeout (placeholder - actual timeout configured on IPC call)
    #[must_use]
    pub fn timeout(self, _duration: std::time::Duration) -> Self {
        self
    }

    /// Build client
    ///
    /// # Errors
    ///
    /// This placeholder never fails.
    pub fn build(self) -> Result<Client> {
        Ok(Client)
    }
}

/// HTTP method enum for type compatibility
///
/// Used by modules that reference HTTP methods in data structures.
/// Actual HTTP method selection happens in the network-capability primal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    /// HTTP GET
    Get,
    /// HTTP POST
    Post,
    /// HTTP PUT
    Put,
    /// HTTP DELETE
    Delete,
    /// HTTP PATCH
    Patch,
    /// HTTP HEAD
    Head,
    /// HTTP OPTIONS
    Options,
}

impl Method {
    /// HTTP GET constant
    pub const GET: Self = Self::Get;
    /// HTTP POST constant
    pub const POST: Self = Self::Post;
    /// HTTP PUT constant
    pub const PUT: Self = Self::Put;
    /// HTTP DELETE constant
    pub const DELETE: Self = Self::Delete;
    /// HTTP PATCH constant
    pub const PATCH: Self = Self::Patch;
    /// HTTP HEAD constant
    pub const HEAD: Self = Self::Head;
    /// HTTP OPTIONS constant
    pub const OPTIONS: Self = Self::Options;
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Get => write!(f, "GET"),
            Self::Post => write!(f, "POST"),
            Self::Put => write!(f, "PUT"),
            Self::Delete => write!(f, "DELETE"),
            Self::Patch => write!(f, "PATCH"),
            Self::Head => write!(f, "HEAD"),
            Self::Options => write!(f, "OPTIONS"),
        }
    }
}
