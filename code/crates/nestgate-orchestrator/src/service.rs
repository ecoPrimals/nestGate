/*!
 * Service registry and management for the Port Manager
 */

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::errors::{Error, Result};

/// Service type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    /// User interface services
    UI,
    
    /// API services
    API,
    
    /// WebSocket services
    WebSocket,
    
    /// Database services
    Database,
    
    /// Metrics and monitoring services
    Metrics,
    
    /// Admin services
    Admin,
    
    /// Other services
    Other(String),
}

/// Service status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    /// Not started
    Idle,
    
    /// In the process of starting
    Starting,
    
    /// Running normally
    Running,
    
    /// In the process of stopping
    Stopping,
    
    /// Stopped normally
    Stopped,
    
    /// Failed to start
    Failed(String),
    
    /// Crashed after starting
    Crashed(String),
    
    /// Restarting after failure
    Restarting,
}

/// Health check information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Type of health check
    pub check_type: HealthCheckType,
    
    /// Endpoint or resource to check
    pub target: String,
    
    /// How often to check (in seconds)
    pub interval_secs: u64,
    
    /// Timeout for the check (in seconds)
    pub timeout_secs: u64,
    
    /// Number of failures before marking unhealthy
    pub failure_threshold: u32,
    
    /// Number of successes before marking healthy
    pub success_threshold: u32,
}

/// Health check type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthCheckType {
    /// HTTP GET request
    HttpGet,
    
    /// TCP connection
    TcpSocket,
    
    /// Process existence
    ProcessExistence,
    
    /// Custom command
    Command(String),
}

/// Service definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDefinition {
    /// Unique identifier for the service
    pub id: String,
    
    /// Human-readable name
    pub name: String,
    
    /// Service type
    pub service_type: ServiceType,
    
    /// Command to start the service
    pub startup_command: String,
    
    /// Command to stop the service (optional)
    pub shutdown_command: Option<String>,
    
    /// Working directory for the service
    pub working_directory: Option<PathBuf>,
    
    /// Environment variables to inject
    pub environment: HashMap<String, String>,
    
    /// Preferred port to use
    pub preferred_port: Option<u16>,
    
    /// Custom port range
    pub port_range: Option<(u16, u16)>,
    
    /// Service dependencies (IDs)
    pub dependencies: Vec<String>,
    
    /// Health checks
    pub health_checks: Vec<HealthCheck>,
    
    /// Whether the service should restart automatically on failure
    pub auto_restart: bool,
    
    /// Maximum restart attempts before giving up
    pub max_restart_attempts: u32,
    
    /// Created timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// Updated timestamp
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Service instance information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInstance {
    /// Service definition
    pub definition: ServiceDefinition,
    
    /// Current status
    pub status: ServiceStatus,
    
    /// Allocated port
    pub port: Option<u16>,
    
    /// Process ID if running
    pub pid: Option<u32>,
    
    /// URLs for the service
    pub urls: HashMap<String, String>,
    
    /// Start time if running
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Stop time if stopped
    pub stopped_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Last error if any
    pub last_error: Option<String>,
    
    /// Restart count
    pub restart_count: u32,
    
    /// Health status (true = healthy)
    pub is_healthy: bool,
    
    /// Last health check time
    pub last_health_check: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Health check details
    pub health_details: Option<String>,
}

/// Service registry for tracking all services
#[derive(Clone)]
pub struct ServiceRegistry {
    /// All registered services
    services: Arc<Mutex<HashMap<String, ServiceInstance>>>,
}

impl ServiceRegistry {
    /// Create a new service registry
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Initialize the service registry
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing service registry");
        Ok(())
    }
    
    /// Shutdown the service registry
    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down service registry");
        Ok(())
    }
    
    /// Register a new service
    pub fn register_service(&self, definition: ServiceDefinition) -> Result<String> {
        let service_id = if definition.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            definition.id.clone()
        };
        
        let mut services = self.services.lock().unwrap();
        
        // Check if service already exists
        if services.contains_key(&service_id) {
            return Err(Error::ServiceAlreadyExists(service_id));
        }
        
        // Create service instance
        let service = ServiceInstance {
            definition: ServiceDefinition {
                id: service_id.clone(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                ..definition
            },
            status: ServiceStatus::Idle,
            port: None,
            pid: None,
            urls: HashMap::new(),
            started_at: None,
            stopped_at: None,
            last_error: None,
            restart_count: 0,
            is_healthy: false,
            last_health_check: None,
            health_details: None,
        };
        
        // Store the service
        services.insert(service_id.clone(), service);
        tracing::info!("Registered service: {}", service_id);
        
        Ok(service_id)
    }
    
    /// Unregister a service
    pub fn unregister_service(&self, service_id: &str) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Remove the service
        services.remove(service_id);
        tracing::info!("Unregistered service: {}", service_id);
        
        Ok(())
    }
    
    /// Update a service definition
    pub fn update_service(&self, definition: ServiceDefinition) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(&definition.id) {
            return Err(Error::ServiceNotFound(definition.id.clone()));
        }
        
        // Get existing service
        let mut service = services.get(&definition.id).unwrap().clone();
        
        // Keep the service_id variable and also clone the definition.id for logging
        let service_id = definition.id.clone();
        let service_id_for_log = service_id.clone(); // Clone for logging
        
        // Update service definition
        service.definition = ServiceDefinition {
            updated_at: chrono::Utc::now(),
            created_at: service.definition.created_at,
            ..definition
        };
        
        // Store updated service
        services.insert(service_id, service);
        tracing::info!("Updated service: {}", service_id_for_log);
        
        Ok(())
    }
    
    /// Get a service by ID
    pub fn get_service(&self, service_id: &str) -> Result<ServiceInstance> {
        let services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Return service instance
        Ok(services.get(service_id).unwrap().clone())
    }
    
    /// Get all services
    pub fn get_all_services(&self) -> Vec<ServiceInstance> {
        let services = self.services.lock().unwrap();
        services.values().cloned().collect()
    }
    
    /// Get all services of a specific type
    pub fn get_services_by_type(&self, service_type: &ServiceType) -> Vec<ServiceInstance> {
        let services = self.services.lock().unwrap();
        services
            .values()
            .filter(|s| s.definition.service_type == *service_type)
            .cloned()
            .collect()
    }
    
    /// Update service status
    pub fn update_service_status(&self, service_id: &str, status: ServiceStatus) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Get existing service
        let mut service = services.get(service_id).unwrap().clone();
        
        // Update status
        service.status = status;
        
        // Update timestamps based on status
        match service.status {
            ServiceStatus::Running => {
                service.started_at = Some(chrono::Utc::now());
                service.stopped_at = None;
            }
            ServiceStatus::Stopped | ServiceStatus::Failed(_) | ServiceStatus::Crashed(_) => {
                service.stopped_at = Some(chrono::Utc::now());
            }
            ServiceStatus::Restarting => {
                service.restart_count += 1;
            }
            _ => {}
        }
        
        // Store updated service
        services.insert(service_id.to_string(), service);
        
        Ok(())
    }
    
    /// Build a dependency graph for services
    pub fn build_dependency_graph(&self) -> Result<Vec<String>> {
        let services = self.services.lock().unwrap();
        
        // Build dependency graph
        let mut graph = petgraph::Graph::<String, ()>::new();
        let mut node_indices = HashMap::new();
        
        // Add nodes
        for service_id in services.keys() {
            let node_idx = graph.add_node(service_id.clone());
            node_indices.insert(service_id.clone(), node_idx);
        }
        
        // Add edges
        for (service_id, service) in services.iter() {
            let node_idx = node_indices.get(service_id).unwrap();
            
            for dep_id in &service.definition.dependencies {
                if let Some(dep_idx) = node_indices.get(dep_id) {
                    graph.add_edge(*dep_idx, *node_idx, ());
                } else {
                    return Err(Error::DependencyCycle(format!(
                        "Dependency {} not found for service {}",
                        dep_id, service_id
                    )));
                }
            }
        }
        
        // Check for cycles
        if petgraph::algo::is_cyclic_directed(&graph) {
            return Err(Error::DependencyCycle(
                "Cyclic dependency detected in service graph".to_string(),
            ));
        }
        
        // Perform topological sort
        let sorted = petgraph::algo::toposort(&graph, None)
            .map_err(|_| Error::DependencyCycle("Failed to sort service dependencies".to_string()))?;
        
        // Convert back to service IDs
        let service_order = sorted
            .iter()
            .map(|&node_idx| graph[node_idx].clone())
            .collect();
        
        Ok(service_order)
    }
    
    /// Update service port
    pub fn update_service_port(&self, service_id: &str, port: u16) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Get existing service
        let mut service = services.get(service_id).unwrap().clone();
        
        // Update port
        service.port = Some(port);
        
        // Update URLs
        let hostname = "localhost";
        let protocol = if service.definition.service_type == ServiceType::UI {
            "http"
        } else {
            match service.definition.service_type {
                ServiceType::WebSocket => "ws",
                _ => "http",
            }
        };
        
        let url = format!("{}://{}:{}", protocol, hostname, port);
        service.urls.insert("main".to_string(), url);
        
        // Store updated service
        services.insert(service_id.to_string(), service);
        
        Ok(())
    }
    
    /// Update service process ID
    pub fn update_service_pid(&self, service_id: &str, pid: u32) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Get existing service
        let mut service = services.get(service_id).unwrap().clone();
        
        // Update PID (0 means clear the PID)
        if pid == 0 {
            service.pid = None;
            service.stopped_at = Some(chrono::Utc::now());
        } else {
            service.pid = Some(pid);
            service.started_at = Some(chrono::Utc::now());
        }
        
        // Store updated service
        services.insert(service_id.to_string(), service);
        
        Ok(())
    }
    
    /// Update service health
    pub fn update_service_health(
        &self,
        service_id: &str,
        is_healthy: bool,
        details: Option<String>,
    ) -> Result<()> {
        let mut services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Get existing service
        let mut service = services.get(service_id).unwrap().clone();
        
        // Update health
        service.is_healthy = is_healthy;
        service.last_health_check = Some(chrono::Utc::now());
        service.health_details = details;
        
        // Store updated service
        services.insert(service_id.to_string(), service);
        
        Ok(())
    }
    
    /// Check if a service has dependencies on another service
    pub fn has_dependency(&self, service_id: &str, depends_on_id: &str) -> Result<bool> {
        let services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Check if dependency exists
        if !services.contains_key(depends_on_id) {
            return Err(Error::ServiceNotFound(depends_on_id.to_string()));
        }
        
        // Get service
        let service = services.get(service_id).unwrap();
        
        // Check direct dependencies
        if service.definition.dependencies.contains(&depends_on_id.to_string()) {
            return Ok(true);
        }
        
        // Check transitive dependencies
        for dep_id in &service.definition.dependencies {
            if self.has_dependency(dep_id, depends_on_id)? {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Get all services that depend on a service
    pub fn get_dependent_services(&self, service_id: &str) -> Result<HashSet<String>> {
        let services = self.services.lock().unwrap();
        
        // Check if service exists
        if !services.contains_key(service_id) {
            return Err(Error::ServiceNotFound(service_id.to_string()));
        }
        
        // Find all services that depend on this service
        let mut dependent_services = HashSet::new();
        
        for (id, service) in services.iter() {
            if service.definition.dependencies.contains(&service_id.to_string()) {
                dependent_services.insert(id.clone());
            }
        }
        
        Ok(dependent_services)
    }
}

/// Service Module
/// 
/// Service-related types and functionality
pub struct Service {
    pub id: String,
    pub name: String,
} 