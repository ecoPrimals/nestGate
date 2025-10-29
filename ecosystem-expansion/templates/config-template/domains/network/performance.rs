//! **NETWORK PERFORMANCE CONFIGURATION**

use serde::{Deserialize, Serialize};
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NetworkPerformanceConfig {
    pub buffer_size: u32,
    pub tcp_nodelay: bool,
    pub keep_alive: bool,
}

impl NetworkPerformanceConfig {
    pub fn development_optimized() -> Self {
        Self { buffer_size: 8192, tcp_nodelay: false, keep_alive: true }
    }

    pub fn production_hardened() -> Self {
        Self { buffer_size: 65536, tcp_nodelay: true, keep_alive: true }
    }

    pub fn validate(&self) -> Result<()> { Ok(()) }
    
    pub fn merge(mut self, other: Self) -> Self {
        self.buffer_size = other.buffer_size;
        self.tcp_nodelay = other.tcp_nodelay;
        self.keep_alive = other.keep_alive;
        self
    }
} 