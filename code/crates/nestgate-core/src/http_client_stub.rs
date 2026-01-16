//! HTTP Client Stub - BiomeOS Pure Rust Evolution
//!
//! **DEPRECATED**: External HTTP should go through Songbird (concentrated gap).
//!
//! For external HTTP requests, use:
//! ```rust
//! let songbird = discover_orchestration().await?;
//! // Use Songbird's RPC methods for external HTTP
//! ```

use crate::error::{NestGateError, Result};

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
#[derive(Debug, Clone, Copy)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl Method {
    pub const GET: Method = Method::Get;
    pub const POST: Method = Method::Post;
    pub const PUT: Method = Method::Put;
    pub const DELETE: Method = Method::Delete;
    pub const PATCH: Method = Method::Patch;
    pub const HEAD: Method = Method::Head;
    pub const OPTIONS: Method = Method::Options;
}
