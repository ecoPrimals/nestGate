//! HTTP Client Stub - BiomeOS Pure Rust Evolution
//!
//! **DEPRECATED**: External HTTP should go through Songbird (concentrated gap).
//!
//! For external HTTP requests, use:
//! ```rust
//! let songbird = discover_orchestration().await?;
//! // Use Songbird's RPC methods for external HTTP
//! ```

use crate::error::Result;

/// Stubbed HTTP client (use Songbird RPC for external HTTP)
#[derive(Debug, Clone)]
pub struct Client;

impl Client {
    /// Create a new client (stubbed)
    pub fn new() -> Self {
        Self
    }

    /// Builder (stubbed)
    pub fn builder() -> ClientBuilder {
        ClientBuilder
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

/// Stubbed client builder
#[derive(Debug)]
pub struct ClientBuilder;

impl ClientBuilder {
    /// Set timeout (stubbed)
    pub fn timeout(self, _duration: std::time::Duration) -> Self {
        self
    }

    /// Build client (stubbed)
    pub fn build(self) -> Result<Client> {
        Ok(Client)
    }
}

/// Stubbed HTTP method enum
///
/// **ecoBin Compliance**: NestGate does NOT make HTTP calls directly.
/// All external HTTP is delegated to Songbird primal via JSON-RPC over Unix sockets.
#[derive(Debug, Clone, Copy)]
pub enum Method {
    /// HTTP GET method
    Get,
    /// HTTP POST method
    Post,
    /// HTTP PUT method
    Put,
    /// HTTP DELETE method
    Delete,
    /// HTTP PATCH method
    Patch,
    /// HTTP HEAD method
    Head,
    /// HTTP OPTIONS method
    Options,
}

impl Method {
    /// HTTP GET method constant
    pub const GET: Method = Method::Get;
    /// HTTP POST method constant
    pub const POST: Method = Method::Post;
    /// HTTP PUT method constant
    pub const PUT: Method = Method::Put;
    /// HTTP DELETE method constant
    pub const DELETE: Method = Method::Delete;
    /// HTTP PATCH method constant
    pub const PATCH: Method = Method::Patch;
    /// HTTP HEAD method constant
    pub const HEAD: Method = Method::Head;
    /// HTTP OPTIONS method constant
    pub const OPTIONS: Method = Method::Options;
}
