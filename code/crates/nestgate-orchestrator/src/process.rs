/*!
 * Process management for the Port Manager
 */

use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::errors::{Error, Result};
use crate::service::ServiceInstance;

/// Process information
#[derive(Debug, Clone)]
pub struct ProcessInfo {
    /// Process ID
    pub pid: u32,
    
    /// Service ID
    pub service_id: String,
    
    /// Start time
    pub start_time: chrono::DateTime<chrono::Utc>,
    
    /// Command that was executed
    pub command: String,
    
    /// Whether this is a managed process
    pub managed: bool,
}

/// Process manager for controlling service processes
#[derive(Clone)]
pub struct ProcessManager {
    /// Currently running processes
    processes: Arc<Mutex<HashMap<u32, ProcessInfo>>>,
}

impl ProcessManager {
    /// Create a new process manager
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    /// Initialize the process manager
    pub async fn initialize(&self) -> Result<()> {
        tracing::info!("Initializing process manager");
        // Clean up any orphaned processes
        self.cleanup_orphaned_processes().await?;
        Ok(())
    }
    
    /// Clean up any orphaned processes that might be running
    async fn cleanup_orphaned_processes(&self) -> Result<()> {
        tracing::info!("Cleaning up orphaned processes");
        
        // Kill any existing processes that might conflict
        self.kill_processes_by_command("node dist/server.js").await?;
        self.kill_processes_by_command("node dist/api-server.js").await?;
        self.kill_processes_by_command("npm start").await?;
        
        Ok(())
    }
    
    /// Kill existing processes by service ID (for restart scenarios)
    pub async fn kill_existing_service_processes(&self, service_id: &str) -> Result<()> {
        let existing_pids: Vec<u32> = {
            let processes = self.processes.lock().unwrap();
            processes.iter()
                .filter(|(_, info)| info.service_id == service_id)
                .map(|(pid, _)| *pid)
                .collect()
        };
        
        for pid in existing_pids {
            tracing::info!("Killing existing process {} for service {}", pid, service_id);
            if let Err(e) = self.kill_process(pid) {
                tracing::warn!("Failed to kill process {}: {}", pid, e);
            }
            
            // Wait a moment for the process to terminate
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        Ok(())
    }
    
    /// Kill processes by command pattern (for cleanup)
    pub async fn kill_processes_by_command(&self, command_pattern: &str) -> Result<()> {
        tracing::info!("Killing processes matching command: {}", command_pattern);
        
        // Use system command to find and kill processes
        if cfg!(target_os = "windows") {
            // Windows: Use taskkill
            let output = Command::new("cmd")
                .args(["/C", &format!("taskkill /F /IM node.exe")])
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    tracing::info!("Successfully killed Windows processes");
                }
            }
        } else {
            // Unix: Use pkill
            let patterns = [
                &format!("pkill -f '{}'", command_pattern),
                &format!("killall -9 node"),  // Fallback for node processes
            ];
            
            for pattern in &patterns {
                let output = Command::new("sh")
                    .args(["-c", pattern])
                    .output();
                
                if let Ok(output) = output {
                    if output.status.success() {
                        tracing::info!("Successfully killed processes with pattern: {}", pattern);
                    }
                }
                
                // Wait between attempts
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
        
        Ok(())
    }
    
    /// Start a service process with proper cleanup
    pub async fn start_service(&self, service: &ServiceInstance) -> Result<u32> {
        let service_id = service.definition.id.clone();
        
        // Step 1: Kill any existing processes for this service
        self.kill_existing_service_processes(&service_id).await?;
        
        // Step 2: For certain service types, kill by command pattern too
        match service.definition.service_type {
            crate::service::ServiceType::WebSocket => {
                self.kill_processes_by_command("node dist/server.js").await?;
            },
            crate::service::ServiceType::API => {
                self.kill_processes_by_command("node dist/api-server.js").await?;
            },
            crate::service::ServiceType::UI => {
                self.kill_processes_by_command("npm start").await?;
                // Also kill react-scripts processes
                self.kill_processes_by_command("react-scripts start").await?;
            },
            _ => {}
        }
        
        // Step 3: Wait for cleanup to complete
        tokio::time::sleep(Duration::from_millis(1000)).await;
        
        // Step 4: Check if service is still running (shouldn't be after cleanup)
        if let Some(pid) = service.pid {
            if self.is_process_running(pid) {
                tracing::warn!("Service {} still running with PID {}, force killing", service_id, pid);
                let _ = self.kill_process(pid);
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
        }
        
        let definition = &service.definition;
        
        // Step 5: Prepare and start the new process
        let mut command = if cfg!(target_os = "windows") {
            // On Windows, use cmd.exe
            let mut cmd = Command::new("cmd");
            cmd.args(["/C", &definition.startup_command]);
            cmd
        } else {
            // On Unix, use sh
            let mut cmd = Command::new("sh");
            cmd.args(["-c", &definition.startup_command]);
            cmd
        };
        
        // Set working directory if specified
        if let Some(working_dir) = &definition.working_directory {
            command.current_dir(working_dir);
        }
        
        // Set environment variables
        for (key, value) in &definition.environment {
            command.env(key, value);
        }
        
        // Add service-type-specific port environment variables
        if let Some(port) = service.port {
            // Always set PORT for compatibility
            command.env("PORT", port.to_string());
            
            // Set service-type-specific environment variables
            match definition.service_type {
                crate::service::ServiceType::API => {
                    command.env("API_PORT", port.to_string());
                },
                crate::service::ServiceType::WebSocket => {
                    command.env("WEBSOCKET_PORT", port.to_string());
                    command.env("SERVER_PORT", port.to_string());
                },
                crate::service::ServiceType::UI => {
                    command.env("UI_PORT", port.to_string());
                    command.env("REACT_APP_PORT", port.to_string());
                },
                crate::service::ServiceType::Database => {
                    command.env("DB_PORT", port.to_string());
                    command.env("DATABASE_PORT", port.to_string());
                },
                _ => {
                    // For other service types, just use PORT
                }
            }
        }
        
        // Configure standard streams
        command
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        // Start the process
        tracing::info!("Starting service: {}", service_id);
        tracing::debug!("Command: {:?}", command);
        
        let mut child = command.spawn().map_err(|e| {
            Error::ServiceStartup(format!("Failed to start service {}: {}", service_id, e))
        })?;
        
        let pid = child.id();
        
        // Register the process
        self.register_process(
            pid,
            service_id.clone(),
            definition.startup_command.clone(),
            true,
        )?;
        
        tracing::info!("Started service {} with PID {}", service_id, pid);
        
        // Spawn a task to wait for the child process to prevent zombies
        let processes = Arc::clone(&self.processes);
        let service_id_clone = service_id.clone();
        tokio::task::spawn_blocking(move || {
            // Wait for the child process to complete
            match child.wait() {
                Ok(exit_status) => {
                    tracing::info!(
                        "Process {} (PID {}) exited with status: {}",
                        service_id_clone,
                        pid,
                        exit_status
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Error waiting for process {} (PID {}): {}",
                        service_id_clone,
                        pid,
                        e
                    );
                }
            }
            
            // Remove from process registry
            let mut procs = processes.lock().unwrap();
            procs.remove(&pid);
            
            tracing::debug!("Process {} (PID {}) cleaned up", service_id_clone, pid);
        });
        
        Ok(pid)
    }
    
    /// Stop a service process with proper cleanup
    pub async fn stop_service(&self, service: &ServiceInstance) -> Result<()> {
        let service_id = service.definition.id.clone();
        
        if let Some(pid) = service.pid {
            if self.is_process_running(pid) {
                // Try to use shutdown command if specified
                if let Some(shutdown_cmd) = &service.definition.shutdown_command {
                    tracing::info!("Stopping service {} with shutdown command", service_id);
                    let status = if cfg!(target_os = "windows") {
                        Command::new("cmd")
                            .args(["/C", shutdown_cmd])
                            .status()
                    } else {
                        Command::new("sh")
                            .args(["-c", shutdown_cmd])
                            .status()
                    };
                    
                    match status {
                        Ok(exit_status) if exit_status.success() => {
                            // Wait for process to actually terminate
                            for _ in 0..10 {
                                if !self.is_process_running(pid) {
                                    break;
                                }
                                tokio::time::sleep(Duration::from_millis(500)).await;
                            }
                        },
                        _ => {
                            tracing::warn!("Shutdown command failed for service {}, using SIGTERM", service_id);
                        }
                    }
                }
                
                // If process is still running, terminate it
                if self.is_process_running(pid) {
                    tracing::info!("Terminating service {} with PID {}", service_id, pid);
                    self.kill_process(pid)?;
                    
                    // Wait for process to terminate
                    for _ in 0..20 {
                        if !self.is_process_running(pid) {
                            break;
                        }
                        tokio::time::sleep(Duration::from_millis(250)).await;
                    }
                    
                    // Force kill if still running
                    if self.is_process_running(pid) {
                        tracing::warn!("Force killing stubborn process {} for service {}", pid, service_id);
                        self.force_kill_process(pid)?;
                    }
                }
                
                // Clean up process registry
                let mut processes = self.processes.lock().unwrap();
                processes.remove(&pid);
            }
        }
        
        // Also clean up any other processes that might be related to this service
        self.kill_existing_service_processes(&service_id).await?;
        
        tracing::info!("Service {} stopped and cleaned up", service_id);
        Ok(())
    }
    
    /// Register a process
    fn register_process(
        &self,
        pid: u32,
        service_id: String,
        command: String,
        managed: bool,
    ) -> Result<()> {
        let info = ProcessInfo {
            pid,
            service_id,
            start_time: chrono::Utc::now(),
            command,
            managed,
        };
        
        let mut processes = self.processes.lock().unwrap();
        processes.insert(pid, info);
        
        Ok(())
    }
    
    /// Kill a process using SIGTERM first, then SIGKILL if needed
    fn kill_process(&self, pid: u32) -> Result<()> {
        if !self.is_process_running(pid) {
            return Ok(());
        }
        
        tracing::info!("Killing process with PID {}", pid);
        
        if cfg!(target_os = "windows") {
            // On Windows, use taskkill
            let status = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F"])
                .status()
                .map_err(|e| Error::ProcessTermination(format!("Failed to kill process {}: {}", pid, e)))?;
            
            if !status.success() {
                return Err(Error::ProcessTermination(format!("Failed to kill process {}", pid)));
            }
        } else {
            // On Unix, try SIGTERM first
            let status = Command::new("kill")
                .args(["-TERM", &pid.to_string()])
                .status()
                .map_err(|e| Error::ProcessTermination(format!("Failed to send SIGTERM to process {}: {}", pid, e)))?;
            
            if status.success() {
                // Wait a moment for graceful shutdown
                std::thread::sleep(Duration::from_millis(2000));
                
                // Check if still running
                if self.is_process_running(pid) {
                    // Use SIGKILL as fallback
                    let kill_status = Command::new("kill")
                        .args(["-KILL", &pid.to_string()])
                        .status()
                        .map_err(|e| Error::ProcessTermination(format!("Failed to send SIGKILL to process {}: {}", pid, e)))?;
                    
                    if !kill_status.success() {
                        return Err(Error::ProcessTermination(format!("Failed to force kill process {}", pid)));
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Force kill a process using SIGKILL immediately
    fn force_kill_process(&self, pid: u32) -> Result<()> {
        if !self.is_process_running(pid) {
            return Ok(());
        }
        
        tracing::warn!("Force killing process with PID {}", pid);
        
        if cfg!(target_os = "windows") {
            // On Windows, use taskkill with force
            let status = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/F", "/T"])
                .status()
                .map_err(|e| Error::ProcessTermination(format!("Failed to force kill process {}: {}", pid, e)))?;
            
            if !status.success() {
                return Err(Error::ProcessTermination(format!("Failed to force kill process {}", pid)));
            }
        } else {
            // On Unix, use SIGKILL
            let status = Command::new("kill")
                .args(["-KILL", &pid.to_string()])
                .status()
                .map_err(|e| Error::ProcessTermination(format!("Failed to send SIGKILL to process {}: {}", pid, e)))?;
            
            if !status.success() {
                return Err(Error::ProcessTermination(format!("Failed to force kill process {}", pid)));
            }
        }
        
        Ok(())
    }
    
    /// Check if a process is running
    fn is_process_running(&self, pid: u32) -> bool {
        if cfg!(target_os = "windows") {
            // On Windows, use tasklist
            let output = Command::new("tasklist")
                .args(["/FI", &format!("PID eq {}", pid)])
                .output();
            
            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    stdout.contains(&pid.to_string())
                }
                Err(_) => false,
            }
        } else {
            // On Unix, check /proc
            std::path::Path::new(&format!("/proc/{}", pid)).exists() ||
            // Fallback: use kill -0
            Command::new("kill")
                .args(["-0", &pid.to_string()])
                .status()
                .map(|status| status.success())
                .unwrap_or(false)
        }
    }
    
    /// Stop all services in the specified order
    pub async fn stop_services_in_order(&self, service_ids: &[String]) -> Result<()> {
        for service_id in service_ids.iter().rev() {
            // Find the PIDs for this service
            let pids = self.get_service_pids(service_id);
            
            for pid in pids {
                tracing::info!("Stopping service {} with PID {}", service_id, pid);
                
                // Kill the process
                if let Err(e) = self.kill_process(pid) {
                    tracing::warn!("Failed to kill process {} for service {}: {}", pid, service_id, e);
                }
                
                // Remove from process registry
                let mut processes = self.processes.lock().unwrap();
                processes.remove(&pid);
            }
        }
        
        Ok(())
    }
    
    /// Stop all services
    pub async fn stop_all_services(&self) -> Result<()> {
        let process_ids: Vec<u32> = {
            let processes = self.processes.lock().unwrap();
            processes.keys().copied().collect()
        };
        
        for pid in process_ids {
            tracing::info!("Killing process with PID {}", pid);
            let _ = self.kill_process(pid);
        }
        
        // Clear process registry
        let mut processes = self.processes.lock().unwrap();
        processes.clear();
        
        tracing::info!("Stopped all services");
        Ok(())
    }
    
    /// Find process PIDs by service ID
    fn get_service_pids(&self, service_id: &str) -> Vec<u32> {
        let processes = self.processes.lock().unwrap();
        
        processes.iter()
            .filter(|(_, info)| info.service_id == service_id)
            .map(|(pid, _)| *pid)
            .collect()
    }

    /// Get all running processes
    pub fn get_all_processes(&self) -> HashMap<u32, ProcessInfo> {
        let processes = self.processes.lock().unwrap();
        processes.clone()
    }
    
    /// Get process information for a specific PID
    pub fn get_process_info(&self, pid: u32) -> Option<ProcessInfo> {
        let processes = self.processes.lock().unwrap();
        processes.get(&pid).cloned()
    }
    
    /// Get all processes for a specific service
    pub fn get_service_processes(&self, service_id: &str) -> Vec<ProcessInfo> {
        let processes = self.processes.lock().unwrap();
        
        processes.values()
            .filter(|info| info.service_id == service_id)
            .cloned()
            .collect()
    }
} 