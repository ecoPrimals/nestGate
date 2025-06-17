/*!
 * Port allocation and management for the Port Manager
 */

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener};
use std::sync::{Arc, Mutex};

use crate::errors::{Error, Result};
use crate::service::ServiceType;

/// Port range for a service type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    /// Start of port range (inclusive)
    pub start: u16,
    
    /// End of port range (inclusive)
    pub end: u16,
}

impl PortRange {
    /// Create a new port range
    pub fn new(start: u16, end: u16) -> Self {
        Self { start, end }
    }
    
    /// Check if a port is within this range
    pub fn contains(&self, port: u16) -> bool {
        port >= self.start && port <= self.end
    }
    
    /// Get the number of ports in this range
    pub fn size(&self) -> u16 {
        self.end - self.start + 1
    }
}

/// Allocated port information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocatedPort {
    /// The port number
    pub port: u16,
    
    /// The service ID that allocated this port
    pub service_id: String,
    
    /// When this port was allocated
    pub allocated_at: chrono::DateTime<chrono::Utc>,
    
    /// For which service type (e.g., UI, API)
    pub service_type: ServiceType,
}

/// Port allocator for managing available ports
#[derive(Clone)]
pub struct PortAllocator {
    /// Port ranges by service type
    ranges: HashMap<ServiceType, PortRange>,
    
    /// Currently allocated ports
    allocated_ports: Arc<Mutex<HashMap<u16, AllocatedPort>>>,
    
    /// Excluded ports (e.g., system services)
    excluded_ports: Arc<Mutex<HashSet<u16>>>,
}

impl PortAllocator {
    /// Create a new port allocator
    pub fn new(ranges: HashMap<ServiceType, PortRange>) -> Self {
        Self {
            ranges,
            allocated_ports: Arc::new(Mutex::new(HashMap::new())),
            excluded_ports: Arc::new(Mutex::new(HashSet::new())),
        }
    }
    
    /// Initialize the port allocator
    pub async fn initialize(&self) -> Result<()> {
        // Scan for system ports that should be excluded
        self.scan_system_ports().await?;
        Ok(())
    }
    
    /// Scan for system ports that should be excluded
    async fn scan_system_ports(&self) -> Result<()> {
        let mut excluded = self.excluded_ports.lock().unwrap();
        
        // Common system ports to exclude
        let system_ports = vec![
            80, 443, 22, 21, 25, 587, 3306, 5432, 6379, 27017, 
            8080, 8443, 1433, 3389
        ];
        
        for port in system_ports {
            excluded.insert(port);
        }
        
        // Skip the slow port scanning for now
        tracing::debug!("Excluded {} system ports", excluded.len());
        Ok(())
    }
    
    /// Check if a port is in use
    pub async fn is_port_in_use(&self, port: u16) -> bool {
        // First check if we already allocated this port
        {
            let allocated = self.allocated_ports.lock().unwrap();
            if allocated.contains_key(&port) {
                return true;
            }
        }
        
        // Then check if it's an excluded port
        {
            let excluded = self.excluded_ports.lock().unwrap();
            if excluded.contains(&port) {
                return true;
            }
        }
        
        // Finally, try to bind to the port
        let addrs = [
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port),
        ];
        
        for addr in addrs {
            match TcpListener::bind(addr) {
                Ok(_) => return false, // Port is available
                Err(_) => continue,    // Try next binding
            }
        }
        
        // If we couldn't bind to any address, the port is in use
        true
    }
    
    /// Allocate a port for a service
    pub async fn allocate_port(
        &self,
        service_id: &str,
        service_type: ServiceType,
        preferred_port: Option<u16>,
    ) -> Result<u16> {
        // First try the preferred port if specified
        if let Some(port) = preferred_port {
            if !self.is_port_in_use(port).await {
                self.register_allocated_port(port, service_id.to_string(), service_type.clone())?;
                return Ok(port);
            }
            
            tracing::warn!("Preferred port {} is already in use", port);
        }
        
        // If no preferred port or it's in use, try to find one in the service type's range
        if let Some(range) = self.ranges.get(&service_type) {
            for port in range.start..=range.end {
                if !self.is_port_in_use(port).await {
                    self.register_allocated_port(port, service_id.to_string(), service_type.clone())?;
                    return Ok(port);
                }
            }
            
            return Err(Error::NoAvailablePorts { 
                start: range.start, 
                end: range.end 
            });
        }
        
        // If no range for this service type, find any available port
        for port in 10000..65535 {
            if !self.is_port_in_use(port).await {
                self.register_allocated_port(port, service_id.to_string(), service_type.clone())?;
                return Ok(port);
            }
        }
        
        Err(Error::PortAllocation("No available ports".to_string()))
    }
    
    /// Register a port as allocated
    fn register_allocated_port(
        &self,
        port: u16,
        service_id: String,
        service_type: ServiceType,
    ) -> Result<()> {
        let mut allocated = self.allocated_ports.lock().unwrap();
        
        let service_id_for_log = service_id.clone();
        
        allocated.insert(port, AllocatedPort {
            port,
            service_id,
            allocated_at: chrono::Utc::now(),
            service_type,
        });
        
        tracing::debug!("Allocated port {} for service {}", port, service_id_for_log);
        Ok(())
    }
    
    /// Deallocate a port
    pub fn deallocate_port(&self, port: u16) -> Result<()> {
        let mut allocated = self.allocated_ports.lock().unwrap();
        
        if allocated.remove(&port).is_some() {
            tracing::debug!("Deallocated port {}", port);
            Ok(())
        } else {
            Err(Error::PortAllocation(format!("Port {} not allocated", port)))
        }
    }
    
    /// Deallocate all ports for a service
    pub fn deallocate_service_ports(&self, service_id: &str) -> Result<Vec<u16>> {
        let mut allocated = self.allocated_ports.lock().unwrap();
        let mut deallocated = Vec::new();
        
        allocated.retain(|&port, info| {
            let keep = info.service_id != service_id;
            if !keep {
                deallocated.push(port);
            }
            keep
        });
        
        if !deallocated.is_empty() {
            tracing::debug!("Deallocated {} ports for service {}", deallocated.len(), service_id);
        }
        
        Ok(deallocated)
    }
    
    /// Get information about an allocated port
    pub fn get_port_info(&self, port: u16) -> Option<AllocatedPort> {
        let allocated = self.allocated_ports.lock().unwrap();
        allocated.get(&port).cloned()
    }
    
    /// Get all allocated ports
    pub fn get_all_allocated_ports(&self) -> HashMap<u16, AllocatedPort> {
        let allocated = self.allocated_ports.lock().unwrap();
        allocated.clone()
    }
    
    /// Get all allocated ports for a service
    pub fn get_service_ports(&self, service_id: &str) -> Vec<AllocatedPort> {
        let allocated = self.allocated_ports.lock().unwrap();
        allocated
            .values()
            .filter(|info| info.service_id == service_id)
            .cloned()
            .collect()
    }
    
    /// Check if a specific port is available
    pub async fn is_port_available(&self, port: u16) -> bool {
        !self.is_port_in_use(port).await
    }
    
    /// Find the next available port in a range
    pub async fn find_next_available_port(
        &self,
        start: u16,
        end: u16,
    ) -> Option<u16> {
        for port in start..=end {
            if !self.is_port_in_use(port).await {
                return Some(port);
            }
        }
        
        None
    }
    
    /// Check if a port is suitable for a service type
    pub fn is_port_in_range_for_service(&self, port: u16, service_type: &ServiceType) -> bool {
        if let Some(range) = self.ranges.get(service_type) {
            range.contains(port)
        } else {
            // If no range is defined for this service type, any port is allowed
            true
        }
    }
    
    /// Get all port allocations as a mapping of service_id -> port
    pub async fn get_all_allocations(&self) -> HashMap<String, u16> {
        let allocated = self.allocated_ports.lock().unwrap();
        allocated
            .values()
            .map(|info| (info.service_id.clone(), info.port))
            .collect()
    }
} 