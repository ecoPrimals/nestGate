//
// System-level configuration for NestGate including service identity,
// resource limits, and operational parameters.

use serde::{Deserialize, Serialize};
use std::time::Duration;

// Import unified constants
use crate::canonical_modernization::canonical_constants::{
    system::{DEFAULT_SERVICE_NAME, DEFAULT_TIMEOUT_SECS, DEFAULT_LOG_LEVEL},
};

/// System-level configuration
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
/// Deployment environment enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum DeploymentEnvironment {
    /// Development environment
    #[default]
    Development,
    /// Testing environment
    Testing,
    /// Staging environment
    Staging,
    /// Production environment
    Production,
}
impl Default for SystemConfig {
    fn default() -> Self {
        Self {
            service_name: DEFAULT_SERVICE_NAME.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            log_level: DEFAULT_LOG_LEVEL.to_string(),
            working_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            pid_file: None,
            max_memory_mb: None,
            max_cpu_cores: None,
            startup_timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            shutdown_timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            health_check_interval: Duration::from_secs(30),
        }
    }
}


impl std::fmt::Display for DeploymentEnvironment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeploymentEnvironment::Development => write!(f, "development"),
            DeploymentEnvironment::Testing => write!(f, "testing"),
            DeploymentEnvironment::Staging => write!(f, "staging"),
            DeploymentEnvironment::Production => write!(f, "production"),
        }
    }
}

impl std::str::FromStr for DeploymentEnvironment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "development" | "dev" => Ok(DeploymentEnvironment::Development),
            "testing" | "test" => Ok(DeploymentEnvironment::Testing),
            "staging" | "stage" => Ok(DeploymentEnvironment::Staging),
            "production" | "prod" => Ok(DeploymentEnvironment::Production),
        }
    }
}

impl SystemConfig {
    /// Create a new system configuration with custom service name
    pub const fn with_service_name(service_name: impl Into<String>) -> Self {
        Self {
            service_name: service_name.into(),
            ..Default::default()
        }
    }

    /// Set the deployment environment
        self.environment = environment;
        self
    }

    /// Set the log level
    #[must_use]
    pub fn with_log_level(mut self, log_level: impl Into<String>) -> Self {
        self.log_level = log_level.into();
        self
    }

    /// Set resource limits
    #[must_use]
    pub fn with_resource_limits(mut self, max_memory_mb: Option<u64>, max_cpu_cores: Option<usize>) -> Self {
        self.max_memory_mb = max_memory_mb;
        self.max_cpu_cores = max_cpu_cores;
        self
    }

    /// Check if running in production environment
    pub const fn is_production(&self) -> bool {
        self.environment == DeploymentEnvironment::Production
    }

    /// Check if running in development environment
    pub const fn is_development(&self) -> bool {
        self.environment == DeploymentEnvironment::Development
    }

    /// Get effective memory limit in bytes
    pub const fn effective_memory_limit_bytes(&self) -> Option<u64> {
        self.max_memory_mb.map(|mb| mb * 1024 * 1024)
    }
} 