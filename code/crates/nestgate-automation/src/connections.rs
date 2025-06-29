//! Service Connections
//!
//! Management of dynamic connections to ecosystem services

use std::collections::HashMap;

/// Service connection pool for managing dynamic ecosystem connections
#[derive(Debug)]
pub struct ServiceConnectionPool {
    pub nestgate_peers: HashMap<String, String>, // peer_id -> endpoint
    squirrel_connections: HashMap<String, String>, // squirrel_id -> endpoint
}

/// Connection to a Squirrel MCP service
#[derive(Debug, Clone)]
pub struct SquirrelConnection {
    pub squirrel_id: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub last_seen: std::time::SystemTime,
    #[cfg(feature = "network-integration")]
    pub client: reqwest::Client,
}

impl SquirrelConnection {
    pub fn new(squirrel_id: String, endpoint: String) -> Self {
        Self {
            squirrel_id,
            endpoint,
            capabilities: vec![],
            last_seen: std::time::SystemTime::now(),
            #[cfg(feature = "network-integration")]
            client: reqwest::Client::new(),
        }
    }
}

impl ServiceConnectionPool {
    pub fn new() -> Self {
        Self {
            nestgate_peers: HashMap::new(),
            squirrel_connections: HashMap::new(),
        }
    }

    /// Get the best available Squirrel MCP service
    pub fn get_best_squirrel(&self) -> Option<String> {
        // Return the first available squirrel for now
        // TODO: Implement proper load balancing and health checking
        self.squirrel_connections.keys().next().cloned()
    }

    /// Add a squirrel connection
    pub fn add_squirrel(&mut self, squirrel_id: String, endpoint: String) {
        self.squirrel_connections.insert(squirrel_id, endpoint);
    }

    /// Add a nestgate peer
    pub fn add_nestgate_peer(&mut self, peer_id: String, endpoint: String) {
        self.nestgate_peers.insert(peer_id, endpoint);
    }
}

impl Default for ServiceConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}
