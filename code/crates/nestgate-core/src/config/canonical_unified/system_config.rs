use std::collections::HashMap;
//
// System-level configuration structures for the canonical unified configuration system.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical_modernization::canonical_constants::{
    system::{DEFAULT_SERVICE_NAME, DEFAULT_TIMEOUT_SECS, DEFAULT_LOG_LEVEL},
};

// ==================== SECTION ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    /// Service name
    pub service_name: String,
    /// Service version
    pub version: String,
    /// Environment (development, staging, production)
    /// Log level
    pub log_level: String,
    /// Working directory
    pub working_directory: PathBuf,
    /// Process ID file
    pub pid_file: Option<PathBuf>,
    /// Maximum memory usage (bytes)
    pub max_memory_mb: Option<u64>,
    /// Maximum CPU cores
    pub max_cpu_cores: Option<usize>,
    /// Startup timeout
    pub startup_timeout: Duration,
    /// Shutdown timeout
    pub shutdown_timeout: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub enum DeploymentEnvironment {
    #[default]
    Development,
    Testing,
    Staging,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvironmentConfig {
    pub name: String,
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeatureFlags {
    pub enable_streaming: bool,
    pub enable_websockets: bool,
    pub enable_dashboard: bool,
    pub enable_metrics: bool,
    pub enable_tracing: bool,
    pub enable_security_scanning: bool,
    pub enable_auto_scaling: bool,
    pub enable_load_balancing: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConfigMetadata {
    pub version: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub created_by: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
}

// ==================== SECTION ====================

impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            service_name: DEFAULT_SERVICE_NAME.to_string(),
            version: "2.0.0".to_string(),
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            working_directory: PathBuf::from("./"),
            pid_file: None,
            max_memory_mb: None,
            max_cpu_cores: None,
            startup_timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            shutdown_timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            health_check_interval: Duration::from_secs(30),
        }
    }
}

 