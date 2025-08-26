//
// Connection management and pooling configuration.

use serde::{Deserialize, Serialize};

/// Network connection management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnectionSettings {
    /// Maximum concurrent connections
    pub max_connections: u32,
    /// Connection timeout
    pub connection_timeout: std::time::Duration,
}

impl Default for NetworkConnectionSettings {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            connection_timeout: std::time::Duration::from_secs(30),
        }
    }
}
