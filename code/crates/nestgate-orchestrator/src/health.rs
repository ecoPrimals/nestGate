/*!
 * Health monitoring for the Port Manager
 */

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tokio::time::interval;

use crate::errors::Result;
use crate::service::HealthCheck;

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    /// Enable health monitoring
    pub enabled: bool,
    
    /// Default health check interval in seconds
    pub default_interval: u64,
    
    /// Default timeout for health checks in seconds
    pub default_timeout: u64,
    
    /// Default failure threshold
    pub default_failure_threshold: u32,
    
    /// Default success threshold
    pub default_success_threshold: u32,
    
    /// Enable detailed health reports
    pub detailed_reports: bool,
    
    /// Health history retention in hours
    pub history_retention_hours: u32,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            default_interval: 30,
            default_timeout: 5,
            default_failure_threshold: 3,
            default_success_threshold: 2,
            detailed_reports: true,
            history_retention_hours: 24,
        }
    }
}

/// Health status of a service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    
    /// Service is unhealthy
    Unhealthy,
    
    /// Service health is unknown
    Unknown,
    
    /// Health check failed
    Failed(String),
}

/// Health check result
#[derive(Debug, Clone, Serialize)]
pub struct HealthCheckResult {
    /// Service ID
    pub service_id: String,
    
    /// Check timestamp
    pub timestamp: chrono::DateTime<chrono::Utc>,
    
    /// Health status
    pub status: HealthStatus,
    
    /// Response time in milliseconds
    pub response_time_ms: u64,
    
    /// Additional details
    pub details: Option<String>,
    
    /// Check type
    pub check_type: String,
}

/// Health monitor for services
#[derive(Clone)]
pub struct HealthMonitor {
    /// Configuration
    config: HealthConfig,
    
    /// Service health results
    health_results: Arc<RwLock<HashMap<String, HealthCheckResult>>>,
    
    /// Health check tasks
    check_tasks: Arc<RwLock<HashMap<String, tokio::task::JoinHandle<()>>>>,
}

impl HealthMonitor {
    /// Create a new health monitor
    pub fn new(config: HealthConfig) -> Self {
        Self {
            config,
            health_results: Arc::new(RwLock::new(HashMap::new())),
            check_tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Initialize the health monitor
    pub async fn initialize(&self) -> Result<()> {
        if !self.config.enabled {
            tracing::info!("Health monitoring disabled");
            return Ok(());
        }
        
        tracing::info!("Initializing health monitor");
        Ok(())
    }
    
    /// Shutdown the health monitor
    pub async fn shutdown(&self) -> Result<()> {
        tracing::info!("Shutting down health monitor");
        
        // Cancel all health check tasks
        let mut tasks = self.check_tasks.write().await;
        for (service_id, task) in tasks.drain() {
            tracing::debug!("Cancelling health check task for service: {}", service_id);
            task.abort();
        }
        
        Ok(())
    }
    
    /// Start monitoring a service
    pub async fn start_monitoring(&self, service_id: &str, health_checks: Vec<HealthCheck>) -> Result<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        if health_checks.is_empty() {
            tracing::debug!("No health checks configured for service: {}", service_id);
            return Ok(());
        }
        
        tracing::info!("Starting health monitoring for service: {}", service_id);
        
        // Cancel existing monitoring if any
        self.stop_monitoring(service_id).await;
        
        // Start new monitoring task
        let service_id_for_task = service_id.to_string();
        let service_id_for_map = service_id.to_string();
        let health_results = Arc::clone(&self.health_results);
        let config = self.config.clone();
        
        let task = tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config.default_interval));
            
            loop {
                interval.tick().await;
                
                for health_check in &health_checks {
                    let result = perform_health_check(&service_id_for_task, health_check, &config).await;
                    
                    // Store the result
                    let mut results = health_results.write().await;
                    results.insert(service_id_for_task.clone(), result);
                }
            }
        });
        
        // Store the task
        let mut tasks = self.check_tasks.write().await;
        tasks.insert(service_id_for_map, task);
        
        Ok(())
    }
    
    /// Stop monitoring a service
    pub async fn stop_monitoring(&self, service_id: &str) {
        tracing::debug!("Stopping health monitoring for service: {}", service_id);
        
        let mut tasks = self.check_tasks.write().await;
        if let Some(task) = tasks.remove(service_id) {
            task.abort();
        }
        
        // Remove health results
        let mut results = self.health_results.write().await;
        results.remove(service_id);
    }
    
    /// Get health status for a service
    pub async fn get_health_status(&self, service_id: &str) -> Option<HealthCheckResult> {
        let results = self.health_results.read().await;
        results.get(service_id).cloned()
    }
    
    /// Get health status for all services
    pub async fn get_all_health_status(&self) -> HashMap<String, HealthCheckResult> {
        let results = self.health_results.read().await;
        results.clone()
    }
}

/// Perform a health check
async fn perform_health_check(
    service_id: &str,
    health_check: &HealthCheck,
    config: &HealthConfig,
) -> HealthCheckResult {
    let start_time = Instant::now();
    let timestamp = chrono::Utc::now();
    
    let (status, details) = match &health_check.check_type {
        crate::service::HealthCheckType::HttpGet => {
            perform_http_health_check(&health_check.target, config).await
        },
        crate::service::HealthCheckType::TcpSocket => {
            perform_tcp_health_check(&health_check.target, config).await
        },
        crate::service::HealthCheckType::ProcessExistence => {
            perform_process_health_check(&health_check.target, config).await
        },
        crate::service::HealthCheckType::Command(cmd) => {
            perform_command_health_check(cmd, config).await
        },
    };
    
    let response_time_ms = start_time.elapsed().as_millis() as u64;
    
    HealthCheckResult {
        service_id: service_id.to_string(),
        timestamp,
        status,
        response_time_ms,
        details,
        check_type: format!("{:?}", health_check.check_type),
    }
}

/// Perform HTTP health check
async fn perform_http_health_check(target: &str, _config: &HealthConfig) -> (HealthStatus, Option<String>) {
    // In a real implementation, this would make an HTTP request
    // For now, return a mock result
    (HealthStatus::Healthy, Some(format!("HTTP check for {}", target)))
}

/// Perform TCP health check
async fn perform_tcp_health_check(target: &str, _config: &HealthConfig) -> (HealthStatus, Option<String>) {
    // In a real implementation, this would attempt a TCP connection
    // For now, return a mock result
    (HealthStatus::Healthy, Some(format!("TCP check for {}", target)))
}

/// Perform process existence health check
async fn perform_process_health_check(target: &str, _config: &HealthConfig) -> (HealthStatus, Option<String>) {
    // In a real implementation, this would check if a process is running
    // For now, return a mock result
    (HealthStatus::Healthy, Some(format!("Process check for {}", target)))
}

/// Perform command health check
async fn perform_command_health_check(command: &str, _config: &HealthConfig) -> (HealthStatus, Option<String>) {
    // In a real implementation, this would execute the command
    // For now, return a mock result
    (HealthStatus::Healthy, Some(format!("Command check: {}", command)))
} 