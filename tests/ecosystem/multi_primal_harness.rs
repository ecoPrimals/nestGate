//! Multi-Primal Test Harness
//!
//! Provides infrastructure for starting, managing, and testing multiple
//! primals (NestGate, BearDog, Songbird, etc.) in integration tests.

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::{sleep, timeout};

/// Configuration for a primal instance
#[derive(Debug, Clone)]
pub struct PrimalConfig {
    /// Name of the primal (e.g., "nestgate", "beardog")
    pub name: String,
    
    /// Path to the primal's binary
    pub binary_path: PathBuf,
    
    /// Working directory for the primal
    pub working_dir: PathBuf,
    
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    
    /// Command-line arguments
    pub args: Vec<String>,
    
    /// Port to listen on (if applicable)
    pub port: Option<u16>,
    
    /// Health check endpoint (if applicable)
    pub health_endpoint: Option<String>,
    
    /// Startup timeout
    pub startup_timeout: Duration,
}

impl Default for PrimalConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            binary_path: PathBuf::new(),
            working_dir: PathBuf::from("."),
            env_vars: HashMap::new(),
            args: Vec::new(),
            port: None,
            health_endpoint: None,
            startup_timeout: Duration::from_secs(10),
        }
    }
}

/// Handle to a running primal instance
pub struct PrimalHandle {
    /// Configuration used to start this primal
    pub config: PrimalConfig,
    
    /// Process handle
    process: Option<Child>,
    
    /// Whether the primal is healthy
    healthy: Arc<RwLock<bool>>,
}

impl PrimalHandle {
    /// Create a new primal handle
    fn new(config: PrimalConfig, process: Child) -> Self {
        Self {
            config,
            process: Some(process),
            healthy: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Check if the primal is healthy
    pub async fn is_healthy(&self) -> bool {
        *self.healthy.read().await
    }
    
    /// Wait for the primal to become healthy
    pub async fn wait_for_health(&self, timeout_duration: Duration) -> Result<(), String> {
        let start = std::time::Instant::now();
        
        while start.elapsed() < timeout_duration {
            if let Some(endpoint) = &self.config.health_endpoint {
                // Try HTTP health check
                match reqwest::get(endpoint).await {
                    Ok(response) if response.status().is_success() => {
                        *self.healthy.write().await = true;
                        return Ok(());
                    }
                    _ => {
                        sleep(Duration::from_millis(100)).await;
                    }
                }
            } else {
                // No health check endpoint, assume healthy after startup timeout
                sleep(self.config.startup_timeout).await;
                *self.healthy.write().await = true;
                return Ok(());
            }
        }
        
        Err(format!("Primal {} failed to become healthy within {:?}", 
                   self.config.name, timeout_duration))
    }
    
    /// Stop the primal
    pub fn stop(&mut self) -> Result<(), String> {
        if let Some(mut process) = self.process.take() {
            process.kill()
                .map_err(|e| format!("Failed to kill process: {}", e))?;
            process.wait()
                .map_err(|e| format!("Failed to wait for process: {}", e))?;
        }
        Ok(())
    }
}

impl Drop for PrimalHandle {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

/// Test harness for multi-primal integration testing
pub struct MultiPrimalHarness {
    /// Running primal instances
    primals: Arc<RwLock<HashMap<String, PrimalHandle>>>,
    
    /// Base directory for test data
    test_data_dir: PathBuf,
}

impl MultiPrimalHarness {
    /// Create a new test harness
    pub async fn new() -> Self {
        let test_data_dir = std::env::temp_dir().join(format!(
            "nestgate-integration-test-{}",
            std::process::id()
        ));
        
        std::fs::create_dir_all(&test_data_dir)
            .expect("Failed to create test data directory");
        
        Self {
            primals: Arc::new(RwLock::new(HashMap::new())),
            test_data_dir,
        }
    }
    
    /// Start a primal instance
    pub async fn start_primal(&self, config: PrimalConfig) -> Result<(), String> {
        let name = config.name.clone();
        
        // Build command
        let mut cmd = Command::new(&config.binary_path);
        cmd.current_dir(&config.working_dir);
        cmd.args(&config.args);
        
        // Set environment variables
        for (key, value) in &config.env_vars {
            cmd.env(key, value);
        }
        
        // Enable discovery
        cmd.env(format!("{}_DISCOVERY_ENABLED", name.to_uppercase()), "true");
        
        // Redirect output for debugging
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        // Start process
        let process = cmd.spawn()
            .map_err(|e| format!("Failed to start {}: {}", name, e))?;
        
        let handle = PrimalHandle::new(config.clone(), process);
        
        // Wait for health
        handle.wait_for_health(config.startup_timeout).await?;
        
        // Store handle
        self.primals.write().await.insert(name, handle);
        
        Ok(())
    }
    
    /// Get a handle to a running primal
    pub async fn get_primal(&self, name: &str) -> Option<Arc<RwLock<PrimalHandle>>> {
        let primals = self.primals.read().await;
        primals.get(name).map(|_| {
            // Return an Arc to the handle (simplified for now)
            // In a real implementation, we'd need better handle management
            Arc::new(RwLock::new(PrimalHandle {
                config: PrimalConfig::default(),
                process: None,
                healthy: Arc::new(RwLock::new(true)),
            }))
        })
    }
    
    /// Stop a specific primal
    pub async fn stop_primal(&self, name: &str) -> Result<(), String> {
        let mut primals = self.primals.write().await;
        if let Some(mut handle) = primals.remove(name) {
            handle.stop()?;
        }
        Ok(())
    }
    
    /// Stop all primals and clean up
    pub async fn cleanup(&self) {
        let mut primals = self.primals.write().await;
        for (name, mut handle) in primals.drain() {
            if let Err(e) = handle.stop() {
                eprintln!("Warning: Failed to stop {}: {}", name, e);
            }
        }
        
        // Clean up test data directory
        if let Err(e) = std::fs::remove_dir_all(&self.test_data_dir) {
            eprintln!("Warning: Failed to clean up test data: {}", e);
        }
    }
    
    /// Get the test data directory
    pub fn test_data_dir(&self) -> &PathBuf {
        &self.test_data_dir
    }
}

impl Drop for MultiPrimalHarness {
    fn drop(&mut self) {
        // Cleanup is async, so we can't do it in Drop
        // Users should call cleanup() explicitly
    }
}

/// Result of an integration test
#[derive(Debug)]
pub struct IntegrationTestResult {
    pub success: bool,
    pub message: String,
    pub details: HashMap<String, serde_json::Value>,
}

impl IntegrationTestResult {
    pub fn success(message: impl Into<String>) -> Self {
        Self {
            success: true,
            message: message.into(),
            details: HashMap::new(),
        }
    }
    
    pub fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            message: message.into(),
            details: HashMap::new(),
        }
    }
    
    pub fn with_detail(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.details.insert(key.into(), value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_harness_creation() {
        let harness = MultiPrimalHarness::new().await;
        assert!(harness.test_data_dir().exists());
        harness.cleanup().await;
    }
    
    #[tokio::test]
    async fn test_primal_config_default() {
        let config = PrimalConfig::default();
        assert_eq!(config.name, "");
        assert_eq!(config.args.len(), 0);
        assert!(config.port.is_none());
    }
}

