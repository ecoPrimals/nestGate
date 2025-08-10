/// Connection Pool Statistics
/// Provides metrics and statistics for connection pool performance monitoring.
/// Statistics for connection pool monitoring
#[derive(Debug, Clone)]
pub struct PoolStats {
    /// Total number of connections created
    pub total_created: u64,
    /// Number of currently active connections
    pub active_connections: usize,
    /// Number of idle connections in pool
    pub idle_connections: usize,
    /// Number of connection acquisition failures
    pub failed_acquisitions: u64,
    /// Average connection acquisition time
    pub avg_acquisition_time_ms: f64,
}

impl Default for PoolStats {
    fn default() -> Self {
        Self {
            total_created: 0,
            active_connections: 0,
            idle_connections: 0,
            failed_acquisitions: 0,
            avg_acquisition_time_ms: 0.0,
        }
    }
}

impl PoolStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Get total number of connections (active + idle)
    pub fn total_connections(&self) -> usize {
        self.active_connections + self.idle_connections
    }

    /// Get connection utilization as a percentage
    pub fn utilization_percentage(&self) -> f64 {
        if self.total_connections() == 0 {
            0.0
        } else {
            (self.active_connections as f64 / self.total_connections() as f64) * 100.0
        }
    }

    /// Get success rate for connection acquisitions
    pub fn success_rate(&self) -> f64 {
        let total_attempts = self.total_created + self.failed_acquisitions;
        if total_attempts == 0 {
            100.0
        } else {
            (self.total_created as f64 / total_attempts as f64) * 100.0
        }
    }
}
