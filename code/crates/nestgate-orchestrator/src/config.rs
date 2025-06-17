/*!
 * Configuration management for the Port Manager
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::service::ServiceType;
use crate::health::HealthConfig;
use crate::security::SecurityConfig;
use crate::metrics::MetricsConfig;
use crate::network::NetworkConfig;
use crate::port::PortRange;

/// Main configuration structure for the Port Manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server configuration
    pub server: ServerConfig,
    
    /// Port range configurations for different service types
    pub port_ranges: HashMap<ServiceType, PortRange>,
    
    /// Health monitoring configuration
    pub health: HealthConfig,
    
    /// Security configuration
    pub security: SecurityConfig,
    
    /// Metrics collection configuration
    pub metrics: MetricsConfig,
    
    /// Network and proxy configuration
    pub network: NetworkConfig,
    
    /// Storage configuration
    pub storage: StorageConfig,
    
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    
    /// Port to bind to
    pub port: u16,
    
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    
    /// Request timeout in seconds
    pub request_timeout: u64,
    
    /// Enable graceful shutdown
    pub graceful_shutdown: bool,
    
    /// Shutdown timeout in seconds
    pub shutdown_timeout: u64,
}

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    /// Base directory for data storage
    pub data_dir: PathBuf,
    
    /// Configuration file path
    pub config_file: PathBuf,
    
    /// Service registry file path
    pub registry_file: PathBuf,
    
    /// Backup configuration
    pub backup: BackupConfig,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automatic backups
    pub enabled: bool,
    
    /// Backup interval in hours
    pub interval_hours: u32,
    
    /// Number of backups to retain
    pub retention_count: u32,
    
    /// Backup directory
    pub backup_dir: PathBuf,
    
    /// Compress backups
    pub compress: bool,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    
    /// Log format
    pub format: LogFormat,
    
    /// Log output destinations
    pub outputs: Vec<LogOutput>,
    
    /// Enable JSON logging
    pub json: bool,
    
    /// Enable file rotation
    pub rotation: LogRotation,
}

/// Log format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogFormat {
    /// Human readable format
    Human,
    
    /// JSON format
    Json,
    
    /// Compact format
    Compact,
}

/// Log output destination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogOutput {
    /// Standard output
    Stdout,
    
    /// Standard error
    Stderr,
    
    /// File output
    File { path: PathBuf },
    
    /// Syslog output
    Syslog,
}

/// Log rotation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogRotation {
    /// Enable rotation
    pub enabled: bool,
    
    /// Maximum file size in MB
    pub max_size_mb: u64,
    
    /// Maximum number of files to keep
    pub max_files: u32,
    
    /// Rotation strategy
    pub strategy: RotationStrategy,
}

/// Log rotation strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationStrategy {
    /// Rotate by size
    Size,
    
    /// Rotate by time
    Time { interval_hours: u32 },
    
    /// Rotate by both size and time
    SizeAndTime { interval_hours: u32 },
}

impl Default for Config {
    fn default() -> Self {
        let mut port_ranges = HashMap::new();
        
        // Default port ranges for different service types
        port_ranges.insert(ServiceType::UI, PortRange {
            start: 3000,
            end: 3099,
        });
        
        port_ranges.insert(ServiceType::API, PortRange {
            start: 3100,
            end: 3199,
        });
        
        port_ranges.insert(ServiceType::WebSocket, PortRange {
            start: 3100,
            end: 3199,
        });
        
        port_ranges.insert(ServiceType::Database, PortRange {
            start: 5432,
            end: 5532,
        });
        
        port_ranges.insert(ServiceType::Metrics, PortRange {
            start: 9000,
            end: 9099,
        });
        
        port_ranges.insert(ServiceType::Admin, PortRange {
            start: 8080,
            end: 8180,
        });
        
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 9000,
                max_connections: 1000,
                request_timeout: 30,
                graceful_shutdown: true,
                shutdown_timeout: 30,
            },
            port_ranges,
            health: HealthConfig::default(),
            security: SecurityConfig::default(),
            metrics: MetricsConfig::default(),
            network: NetworkConfig::default(),
            storage: StorageConfig {
                data_dir: PathBuf::from("./data"),
                config_file: PathBuf::from("./config/port-manager.toml"),
                registry_file: PathBuf::from("./data/service-registry.json"),
                backup: BackupConfig {
                    enabled: true,
                    interval_hours: 24,
                    retention_count: 7,
                    backup_dir: PathBuf::from("./backups"),
                    compress: true,
                },
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: LogFormat::Human,
                outputs: vec![LogOutput::Stdout],
                json: false,
                rotation: LogRotation {
                    enabled: true,
                    max_size_mb: 100,
                    max_files: 10,
                    strategy: RotationStrategy::SizeAndTime { interval_hours: 24 },
                },
            },
        }
    }
}

impl Config {
    /// Load configuration from a YAML file
    pub fn from_file(path: &std::path::Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    /// Save configuration to a YAML file
    pub fn to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate server configuration
        if self.server.port == 0 {
            return Err("Server port cannot be 0".to_string());
        }
        
        if self.server.host.is_empty() {
            return Err("Server host cannot be empty".to_string());
        }
        
        // Validate port ranges
        for (service_type, range) in &self.port_ranges {
            if range.start >= range.end {
                return Err(format!(
                    "Invalid port range for {:?}: start ({}) must be less than end ({})",
                    service_type, range.start, range.end
                ));
            }
            
            if range.start == 0 || range.end == 0 {
                return Err(format!(
                    "Port range for {:?} cannot include port 0",
                    service_type
                ));
            }
        }
        
        // Validate storage paths
        if let Some(parent) = self.storage.data_dir.parent() {
            if !parent.exists() {
                return Err(format!(
                    "Data directory parent does not exist: {}",
                    parent.display()
                ));
            }
        }
        
        // Validate network configuration
        if self.network.reverse_proxy_enabled && self.network.reverse_proxy_port == 0 {
            return Err("Reverse proxy port cannot be 0 when enabled".to_string());
        }
        
        if self.network.ssl_termination {
            if self.network.ssl_cert_path.is_none() || self.network.ssl_key_path.is_none() {
                return Err("SSL certificate and key paths must be provided when SSL termination is enabled".to_string());
            }
        }
        
        Ok(())
    }
    
    /// Get port range for a service type
    pub fn get_port_range(&self, service_type: &ServiceType) -> Option<&PortRange> {
        self.port_ranges.get(service_type)
    }
    
    /// Create data directory if it doesn't exist
    pub fn ensure_data_directory(&self) -> Result<(), std::io::Error> {
        if !self.storage.data_dir.exists() {
            std::fs::create_dir_all(&self.storage.data_dir)?;
        }
        
        if self.storage.backup.enabled && !self.storage.backup.backup_dir.exists() {
            std::fs::create_dir_all(&self.storage.backup.backup_dir)?;
        }
        
        Ok(())
    }
} 