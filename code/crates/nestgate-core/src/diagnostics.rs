//! Diagnostics module for NestGate
//!
//! This module provides system diagnostics and monitoring functionality
//! for the NestGate system.

use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::time::{Duration, SystemTime};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::error::{NestGateError, Result};

/// System diagnostic level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticLevel {
    /// Informational diagnostic
    Info,
    /// Warning diagnostic
    Warning,
    /// Error diagnostic
    Error,
    /// Critical error diagnostic
    Critical,
}

impl fmt::Display for DiagnosticLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiagnosticLevel::Info => write!(f, "INFO"),
            DiagnosticLevel::Warning => write!(f, "WARNING"),
            DiagnosticLevel::Error => write!(f, "ERROR"),
            DiagnosticLevel::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// System component type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    /// CPU
    Cpu,
    /// Memory
    Memory,
    /// Storage
    Storage,
    /// Network
    Network,
    /// File system
    FileSystem,
    /// NFS service
    Nfs,
    /// SMB service
    Smb,
    /// ZFS
    Zfs,
    /// System
    System,
}

impl fmt::Display for ComponentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComponentType::Cpu => write!(f, "CPU"),
            ComponentType::Memory => write!(f, "Memory"),
            ComponentType::Storage => write!(f, "Storage"),
            ComponentType::Network => write!(f, "Network"),
            ComponentType::FileSystem => write!(f, "FileSystem"),
            ComponentType::Nfs => write!(f, "NFS"),
            ComponentType::Smb => write!(f, "SMB"),
            ComponentType::Zfs => write!(f, "ZFS"),
            ComponentType::System => write!(f, "System"),
        }
    }
}

/// Diagnostic entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Diagnostic ID
    pub id: String,
    
    /// Diagnostic level
    pub level: DiagnosticLevel,
    
    /// Component type
    pub component: ComponentType,
    
    /// Diagnostic message
    pub message: String,
    
    /// Timestamp when the diagnostic was created
    pub timestamp: SystemTime,
    
    /// Optional details
    pub details: Option<String>,
    
    /// Optional associated resource
    pub resource: Option<String>,
    
    /// Whether the diagnostic is resolved
    pub resolved: bool,
    
    /// Timestamp when the diagnostic was resolved (if resolved)
    pub resolved_at: Option<SystemTime>,
}

impl Diagnostic {
    /// Create a new diagnostic
    pub fn new(level: DiagnosticLevel, component: ComponentType, message: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            level,
            component,
            message,
            timestamp: SystemTime::now(),
            details: None,
            resource: None,
            resolved: false,
            resolved_at: None,
        }
    }
    
    /// Create a new info diagnostic
    pub fn info(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Info, component, message)
    }
    
    /// Create a new warning diagnostic
    pub fn warning(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Warning, component, message)
    }
    
    /// Create a new error diagnostic
    pub fn error(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Error, component, message)
    }
    
    /// Create a new critical diagnostic
    pub fn critical(component: ComponentType, message: String) -> Self {
        Self::new(DiagnosticLevel::Critical, component, message)
    }
    
    /// Set the details for the diagnostic
    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }
    
    /// Set the resource for the diagnostic
    pub fn with_resource(mut self, resource: String) -> Self {
        self.resource = Some(resource);
        self
    }
    
    /// Mark the diagnostic as resolved
    pub fn resolve(&mut self) {
        self.resolved = true;
        self.resolved_at = Some(SystemTime::now());
    }
}

/// System health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// System is healthy
    Healthy,
    /// System has warnings
    Warning,
    /// System has errors
    Error,
    /// System is in critical failure state
    Critical,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "Healthy"),
            HealthStatus::Warning => write!(f, "Warning"),
            HealthStatus::Error => write!(f, "Error"),
            HealthStatus::Critical => write!(f, "Critical"),
        }
    }
}

/// System metrics structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_usage: f64,
    
    /// Memory usage in bytes
    pub memory_used: u64,
    
    /// Total memory in bytes
    pub memory_total: u64,
    
    /// Storage used in bytes
    pub storage_used: u64,
    
    /// Total storage in bytes
    pub storage_total: u64,
    
    /// Network traffic in bytes per second
    pub network_traffic: u64,
    
    /// System load average (1 minute)
    pub load_avg_1m: f64,
    
    /// System load average (5 minutes)
    pub load_avg_5m: f64,
    
    /// System load average (15 minutes)
    pub load_avg_15m: f64,
    
    /// System uptime in seconds
    pub uptime: u64,
    
    /// Number of active processes
    pub process_count: u32,
    
    /// Timestamp when metrics were collected
    pub timestamp: SystemTime,
    
    /// Per-disk metrics
    pub disk_metrics: HashMap<String, DiskMetrics>,
    
    /// Per-network interface metrics
    pub network_metrics: HashMap<String, NetworkMetrics>,
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_used: 0,
            memory_total: 0,
            storage_used: 0,
            storage_total: 0,
            network_traffic: 0,
            load_avg_1m: 0.0,
            load_avg_5m: 0.0,
            load_avg_15m: 0.0,
            uptime: 0,
            process_count: 0,
            timestamp: SystemTime::now(),
            disk_metrics: HashMap::new(),
            network_metrics: HashMap::new(),
        }
    }
}

/// Disk metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    /// Disk name
    pub name: String,
    
    /// Space used in bytes
    pub used: u64,
    
    /// Total space in bytes
    pub total: u64,
    
    /// I/O operations per second
    pub iops: u64,
    
    /// Read throughput in bytes per second
    pub read_throughput: u64,
    
    /// Write throughput in bytes per second
    pub write_throughput: u64,
    
    /// Disk type (e.g., SSD, HDD)
    pub disk_type: String,
    
    /// Mount point (if applicable)
    pub mount_point: Option<PathBuf>,
}

/// Network interface metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Interface name
    pub name: String,
    
    /// Bytes received
    pub rx_bytes: u64,
    
    /// Bytes transmitted
    pub tx_bytes: u64,
    
    /// Packets received
    pub rx_packets: u64,
    
    /// Packets transmitted
    pub tx_packets: u64,
    
    /// Errors during receive
    pub rx_errors: u64,
    
    /// Errors during transmit
    pub tx_errors: u64,
    
    /// Receive throughput in bytes per second
    pub rx_throughput: u64,
    
    /// Transmit throughput in bytes per second
    pub tx_throughput: u64,
}

/// Service status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceStatus {
    /// Service is running
    Running,
    /// Service is starting up
    Starting,
    /// Service is stopping
    Stopping,
    /// Service is stopped
    Stopped,
    /// Service failed to start or crashed
    Failed,
    /// Service state is unknown
    Unknown,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceStatus::Running => write!(f, "Running"),
            ServiceStatus::Starting => write!(f, "Starting"),
            ServiceStatus::Stopping => write!(f, "Stopping"),
            ServiceStatus::Stopped => write!(f, "Stopped"),
            ServiceStatus::Failed => write!(f, "Failed"),
            ServiceStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    /// Service name
    pub name: String,
    
    /// Service display name
    pub display_name: String,
    
    /// Service status
    pub status: ServiceStatus,
    
    /// Service uptime in seconds (if running)
    pub uptime: Option<u64>,
    
    /// Process ID (if running)
    pub pid: Option<u32>,
    
    /// Memory usage in bytes (if running)
    pub memory_usage: Option<u64>,
    
    /// CPU usage percentage (if running)
    pub cpu_usage: Option<f64>,
    
    /// Last restart time (if applicable)
    pub last_restart: Option<SystemTime>,
    
    /// Number of restarts
    pub restart_count: u32,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Diagnostics manager
#[derive(Debug)]
pub struct DiagnosticsManager {
    /// Diagnostics storage
    diagnostics: Arc<RwLock<Vec<Diagnostic>>>,
    
    /// Metrics history
    metrics_history: Arc<RwLock<Vec<SystemMetrics>>>,
    
    /// Services information
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
    
    /// Diagnostics broadcast channel
    diagnostic_tx: broadcast::Sender<Diagnostic>,
    
    /// Metrics broadcast channel
    metrics_tx: broadcast::Sender<SystemMetrics>,
    
    /// Maximum metrics history length
    max_metrics_history: usize,
}

impl DiagnosticsManager {
    /// Create a new diagnostics manager
    pub fn new() -> Self {
        let (diagnostic_tx, _) = broadcast::channel(100);
        let (metrics_tx, _) = broadcast::channel(100);
        
        Self {
            diagnostics: Arc::new(RwLock::new(Vec::new())),
            metrics_history: Arc::new(RwLock::new(Vec::new())),
            services: Arc::new(RwLock::new(HashMap::new())),
            diagnostic_tx,
            metrics_tx,
            max_metrics_history: 1000,
        }
    }
    
    /// Set the maximum metrics history length
    pub fn set_max_metrics_history(&mut self, max: usize) {
        self.max_metrics_history = max;
    }
    
    /// Add a diagnostic
    pub fn add_diagnostic(&self, diagnostic: Diagnostic) -> Result<()> {
        // Add to diagnostics storage
        {
            let mut diagnostics = match self.diagnostics.write() {
                Ok(d) => d,
                Err(_) => return Err(NestGateError::Internal("Diagnostics lock poisoned".to_string())),
            };
            
            diagnostics.push(diagnostic.clone());
        }
        
        // Broadcast the diagnostic
        let _ = self.diagnostic_tx.send(diagnostic);
        
        Ok(())
    }
    
    /// Get all diagnostics
    pub fn get_diagnostics(&self) -> Result<Vec<Diagnostic>> {
        let diagnostics = match self.diagnostics.read() {
            Ok(d) => d.clone(),
            Err(_) => return Err(NestGateError::Internal("Diagnostics lock poisoned".to_string())),
        };
        
        Ok(diagnostics)
    }
    
    /// Get diagnostics by level
    pub fn get_diagnostics_by_level(&self, level: DiagnosticLevel) -> Result<Vec<Diagnostic>> {
        let diagnostics = match self.diagnostics.read() {
            Ok(d) => d.clone(),
            Err(_) => return Err(NestGateError::Internal("Diagnostics lock poisoned".to_string())),
        };
        
        Ok(diagnostics.into_iter().filter(|d| d.level == level).collect())
    }
    
    /// Get diagnostics by component
    pub fn get_diagnostics_by_component(&self, component: ComponentType) -> Result<Vec<Diagnostic>> {
        let diagnostics = match self.diagnostics.read() {
            Ok(d) => d.clone(),
            Err(_) => return Err(NestGateError::Internal("Diagnostics lock poisoned".to_string())),
        };
        
        Ok(diagnostics.into_iter().filter(|d| d.component == component).collect())
    }
    
    /// Get unresolved diagnostics
    pub fn get_unresolved_diagnostics(&self) -> Result<Vec<Diagnostic>> {
        let diagnostics = match self.diagnostics.read() {
            Ok(d) => d.clone(),
            Err(_) => return Err(NestGateError::Internal("Diagnostics lock poisoned".to_string())),
        };
        
        Ok(diagnostics.into_iter().filter(|d| !d.resolved).collect())
    }
    
    /// Resolve a diagnostic by ID
    pub fn resolve_diagnostic(&self, id: &str) -> Result<bool> {
        let mut diagnostics = match self.diagnostics.write() {
            Ok(d) => d,
            Err(_) => return Err(NestGateError::Internal("Diagnostics lock poisoned".to_string())),
        };
        
        if let Some(diagnostic) = diagnostics.iter_mut().find(|d| d.id == id) {
            diagnostic.resolve();
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Add system metrics
    pub fn add_metrics(&self, metrics: SystemMetrics) -> Result<()> {
        // Add to metrics history
        {
            let mut history = match self.metrics_history.write() {
                Ok(h) => h,
                Err(_) => return Err(NestGateError::Internal("Metrics history lock poisoned".to_string())),
            };
            
            // Trim history if it gets too long
            let history_len = history.len();
            if history_len >= self.max_metrics_history {
                let items_to_remove = history_len - self.max_metrics_history + 1;
                history.drain(0..items_to_remove);
            }
            
            // Add the new metrics
            history.push(metrics.clone());
        }
        
        // Broadcast the metrics
        let _ = self.metrics_tx.send(metrics);
        
        Ok(())
    }
    
    /// Get the latest metrics
    pub fn get_latest_metrics(&self) -> Result<Option<SystemMetrics>> {
        let history = match self.metrics_history.read() {
            Ok(h) => h,
            Err(_) => return Err(NestGateError::Internal("Metrics history lock poisoned".to_string())),
        };
        
        Ok(history.last().cloned())
    }
    
    /// Get metrics history
    pub fn get_metrics_history(&self) -> Result<Vec<SystemMetrics>> {
        let history = match self.metrics_history.read() {
            Ok(h) => h.clone(),
            Err(_) => return Err(NestGateError::Internal("Metrics history lock poisoned".to_string())),
        };
        
        Ok(history)
    }
    
    /// Get metrics history for a time range
    pub fn get_metrics_history_range(&self, duration: Duration) -> Result<Vec<SystemMetrics>> {
        let history = match self.metrics_history.read() {
            Ok(h) => h.clone(),
            Err(_) => return Err(NestGateError::Internal("Metrics history lock poisoned".to_string())),
        };
        
        let now = SystemTime::now();
        let cutoff = now - duration;
        
        Ok(history.into_iter().filter(|m| m.timestamp >= cutoff).collect())
    }
    
    /// Update a service's status
    pub fn update_service_status(&self, name: &str, status: ServiceStatus) -> Result<()> {
        let mut services = match self.services.write() {
            Ok(s) => s,
            Err(_) => return Err(NestGateError::Internal("Services lock poisoned".to_string())),
        };
        
        if let Some(service) = services.get_mut(name) {
            service.status = status;
            
            // Update uptime and last restart if needed
            match status {
                ServiceStatus::Running => {
                    if service.uptime.is_none() {
                        service.uptime = Some(0);
                    }
                }
                ServiceStatus::Starting => {
                    service.uptime = None;
                }
                ServiceStatus::Failed => {
                    service.uptime = None;
                    service.last_restart = Some(SystemTime::now());
                    service.restart_count += 1;
                }
                _ => {
                    service.uptime = None;
                }
            }
            
            Ok(())
        } else {
            Err(NestGateError::NotFound(format!("Service {} not found", name)))
        }
    }
    
    /// Register a new service
    pub fn register_service(&self, service: ServiceInfo) -> Result<()> {
        let mut services = match self.services.write() {
            Ok(s) => s,
            Err(_) => return Err(NestGateError::Internal("Services lock poisoned".to_string())),
        };
        
        services.insert(service.name.clone(), service);
        
        Ok(())
    }
    
    /// Get service information
    pub fn get_service(&self, name: &str) -> Result<Option<ServiceInfo>> {
        let services = match self.services.read() {
            Ok(s) => s,
            Err(_) => return Err(NestGateError::Internal("Services lock poisoned".to_string())),
        };
        
        Ok(services.get(name).cloned())
    }
    
    /// Get all service information
    pub fn get_all_services(&self) -> Result<Vec<ServiceInfo>> {
        let services = match self.services.read() {
            Ok(s) => s.clone(),
            Err(_) => return Err(NestGateError::Internal("Services lock poisoned".to_string())),
        };
        
        Ok(services.into_values().collect())
    }
    
    /// Update service metrics (memory, CPU usage)
    pub fn update_service_metrics(&self, name: &str, memory_usage: u64, cpu_usage: f64) -> Result<()> {
        let mut services = match self.services.write() {
            Ok(s) => s,
            Err(_) => return Err(NestGateError::Internal("Services lock poisoned".to_string())),
        };
        
        if let Some(service) = services.get_mut(name) {
            service.memory_usage = Some(memory_usage);
            service.cpu_usage = Some(cpu_usage);
            Ok(())
        } else {
            Err(NestGateError::NotFound(format!("Service {} not found", name)))
        }
    }
    
    /// Get a diagnostics broadcast receiver
    pub fn subscribe_diagnostics(&self) -> broadcast::Receiver<Diagnostic> {
        self.diagnostic_tx.subscribe()
    }
    
    /// Get a metrics broadcast receiver
    pub fn subscribe_metrics(&self) -> broadcast::Receiver<SystemMetrics> {
        self.metrics_tx.subscribe()
    }
    
    /// Calculate the overall system health status
    pub fn calculate_health_status(&self) -> Result<HealthStatus> {
        // Get unresolved diagnostics
        let unresolved = self.get_unresolved_diagnostics()?;
        
        // Check for critical diagnostics
        if unresolved.iter().any(|d| d.level == DiagnosticLevel::Critical) {
            return Ok(HealthStatus::Critical);
        }
        
        // Check for error diagnostics
        if unresolved.iter().any(|d| d.level == DiagnosticLevel::Error) {
            return Ok(HealthStatus::Error);
        }
        
        // Check for warning diagnostics
        if unresolved.iter().any(|d| d.level == DiagnosticLevel::Warning) {
            return Ok(HealthStatus::Warning);
        }
        
        // No problems found
        Ok(HealthStatus::Healthy)
    }
}

impl Default for DiagnosticsManager {
    fn default() -> Self {
        Self::new()
    }
} 