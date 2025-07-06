//! Service Connections
//!
//! Management of dynamic connections to ecosystem services with intelligent load balancing

use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Service connection pool for managing dynamic ecosystem connections
#[derive(Debug)]
pub struct ServiceConnectionPool {
    pub nestgate_peers: HashMap<String, String>, // peer_id -> endpoint
    squirrel_connections: HashMap<String, SquirrelConnection>,
    last_health_check: SystemTime,
    health_check_interval: Duration,
}

/// Connection to a Squirrel MCP service with health metrics
#[derive(Debug, Clone)]
pub struct SquirrelConnection {
    pub squirrel_id: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub last_seen: SystemTime,
    pub response_time_ms: u64,
    pub success_rate: f64,
    pub request_count: u64,
    pub error_count: u64,
    pub is_healthy: bool,
    #[cfg(feature = "network-integration")]
    pub client: reqwest::Client,
}

impl SquirrelConnection {
    pub fn new(squirrel_id: String, endpoint: String) -> Self {
        Self {
            squirrel_id,
            endpoint,
            capabilities: vec![],
            last_seen: SystemTime::now(),
            response_time_ms: 100, // Default to healthy response time
            success_rate: 1.0,     // Start optimistic
            request_count: 0,
            error_count: 0,
            is_healthy: true,
            #[cfg(feature = "network-integration")]
            client: reqwest::Client::new(),
        }
    }

    /// Update connection health metrics after a request
    pub fn update_metrics(&mut self, response_time_ms: u64, success: bool) {
        self.last_seen = SystemTime::now();
        self.request_count += 1;

        if success {
            // Weighted average for response time (80% old, 20% new)
            self.response_time_ms = (self.response_time_ms * 4 + response_time_ms) / 5;
        } else {
            self.error_count += 1;
        }

        // Calculate success rate
        self.success_rate = if self.request_count > 0 {
            (self.request_count - self.error_count) as f64 / self.request_count as f64
        } else {
            1.0
        };

        // Update health status based on metrics
        self.is_healthy = self.success_rate > 0.8 && self.response_time_ms < 5000;
    }

    /// Calculate service score for load balancing (higher is better)
    pub fn calculate_score(&self) -> f64 {
        if !self.is_healthy {
            return 0.0;
        }

        // Score based on success rate (70%) and response time (30%)
        let success_score = self.success_rate * 0.7;
        let speed_score = if self.response_time_ms > 0 {
            (1000.0 / self.response_time_ms as f64).min(1.0) * 0.3
        } else {
            0.3
        };

        success_score + speed_score
    }
}

impl ServiceConnectionPool {
    pub fn new() -> Self {
        Self {
            nestgate_peers: HashMap::new(),
            squirrel_connections: HashMap::new(),
            last_health_check: SystemTime::now(),
            health_check_interval: Duration::from_secs(30),
        }
    }

    /// Get the best available Squirrel MCP service with intelligent load balancing
    pub fn get_best_squirrel(&self) -> Option<String> {
        if self.squirrel_connections.is_empty() {
            return None;
        }

        // Find the squirrel with the highest score
        let best_squirrel = self
            .squirrel_connections
            .values()
            .filter(|conn| conn.is_healthy)
            .max_by(|a, b| {
                a.calculate_score()
                    .partial_cmp(&b.calculate_score())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        best_squirrel.map(|conn| conn.endpoint.clone())
    }

    /// Add a squirrel connection
    pub fn add_squirrel(&mut self, squirrel_id: String, endpoint: String) {
        let connection = SquirrelConnection::new(squirrel_id.clone(), endpoint);
        self.squirrel_connections.insert(squirrel_id, connection);
    }

    /// Update squirrel health metrics
    pub fn update_squirrel_health(
        &mut self,
        squirrel_id: &str,
        response_time_ms: u64,
        success: bool,
    ) {
        if let Some(connection) = self.squirrel_connections.get_mut(squirrel_id) {
            connection.update_metrics(response_time_ms, success);
        }
    }

    /// Perform health check on all connections (should be called periodically)
    pub async fn perform_health_check(&mut self) {
        let now = SystemTime::now();
        if now
            .duration_since(self.last_health_check)
            .unwrap_or_default()
            < self.health_check_interval
        {
            return;
        }

        self.last_health_check = now;

        // Mark connections as unhealthy if not seen recently
        let stale_threshold = Duration::from_secs(120);
        for connection in self.squirrel_connections.values_mut() {
            if now.duration_since(connection.last_seen).unwrap_or_default() > stale_threshold {
                connection.is_healthy = false;
            }
        }
    }

    /// Get squirrel connection statistics
    pub fn get_squirrel_stats(&self) -> HashMap<String, (f64, u64, bool)> {
        self.squirrel_connections
            .iter()
            .map(|(id, conn)| {
                (
                    id.clone(),
                    (conn.success_rate, conn.response_time_ms, conn.is_healthy),
                )
            })
            .collect()
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
