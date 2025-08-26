
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Port allocation manager
pub struct PortManager {
    allocated_ports: HashMap<u16, String>,
    next_port: u16,
}

impl PortManager {
    /// Create a new port manager
    pub fn new() -> Self {
        Self {
            allocated_ports: HashMap::new(),
            next_port: 8080,
        }
    }

    /// Allocate a port for a service
    pub fn allocate_port(&mut self, service_name: &str) -> u16 {
        let port = self.next_port;
        self.allocated_ports.insert(port, service_name.to_string());
        self.next_port += 1;
        port
    }

    /// Release a port
    pub fn release_port(&mut self, port: u16) -> bool {
        self.allocated_ports.remove(&port).is_some()
    }

    /// Check if a port is allocated
    pub fn is_allocated(&self, port: u16) -> bool {
        self.allocated_ports.contains_key(&port)
    }

    /// Get service name for a port
    pub fn get_service(&self, port: u16) -> Option<&String> {
        self.allocated_ports.get(&port)
    }

    /// Get all allocated ports
    pub fn get_allocated_ports(&self) -> &HashMap<u16, String> {
        &self.allocated_ports
    }
}

impl Default for PortManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Port allocation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortAllocation {
    pub port: u16,
    pub service_name: String,
    pub port_type: String,
    pub allocated_at: std::time::SystemTime,
}
