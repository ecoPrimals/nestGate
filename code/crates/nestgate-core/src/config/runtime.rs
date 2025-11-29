// 🌟 CENTRALIZED RUNTIME CONFIGURATION SYSTEM
// Eliminates 805+ hardcoded values throughout the codebase
// Environment-driven, flexible, production-ready

//! Runtime module

use crate::error::{NestGateError, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use std::sync::OnceLock;

// ==================== MAIN CONFIGURATION ====================

/// Centralized runtime configuration for all NestGate components.
///
/// This is the **single source of truth** for all runtime configuration,
/// systematically replacing 805+ hardcoded values throughout the codebase.
///
/// # Architecture
///
/// Configuration is loaded from environment variables on first access via
/// [`get_config()`], using [`OnceLock`](std::sync::OnceLock) for thread-safe
/// lazy initialization. This provides zero-cost abstraction with no runtime overhead.
///
/// # Organization
///
/// The configuration is organized into seven domain-specific sections:
/// - **Network**: API endpoints, ports, timeouts, connection pooling
/// - **Services**: Service discovery, registration, health checks
/// - **Storage**: Backend configuration, paths, quotas, retention
/// - **Database**: Connection pooling, query limits, credentials
/// - **Cache**: TTL, size limits, eviction policies (Redis, in-memory)
/// - **Monitoring**: Metrics, logging, tracing, alerting
/// - **Security**: Authentication, encryption, access control
///
/// # Example Usage
///
/// ```rust,ignore
/// use nestgate_core::config::runtime::get_config;
///
/// // Get global configuration (initialized once)
/// let config = get_config();
///
/// // Use configuration values
/// let api_url = format!("http://{}:{}", config.network.api_host, config.network.api_port);
/// let storage_path = &config.storage.base_path;
/// let db_pool_size = config.database.pool_size;
/// ```
///
/// # Environment Variables
///
/// All configuration can be overridden via environment variables with the `NESTGATE_` prefix:
/// - `NESTGATE_API_HOST` → `network.api_host` (default: "127.0.0.1")
/// - `NESTGATE_API_PORT` → `network.api_port` (default: 8080)
/// - `NESTGATE_STORAGE_PATH` → `storage.base_path` (default: "./data")
/// - See individual config sections for complete list
///
/// # Migration Status
///
/// **Active Migration** (Nov 19, 2025):
/// - **Total hardcoded values identified**: 805
/// - **Remaining to migrate**: ~390 files
/// - **Migration pattern**: Replace `"localhost"` with `get_config().network.api_host`
/// - **Progress tracking**: See `HARDCODING_ELIMINATION_GUIDE.md`
///
/// # Thread Safety
///
/// This struct is [`Clone`] and all configuration is immutable after initialization.
/// The global instance is stored in a [`OnceLock`] for safe concurrent access.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Configuration for NestGateRuntime
pub struct NestGateRuntimeConfig {
    /// Network configuration (ports, IPs, endpoints, timeouts)
    pub network: NetworkConfig,

    /// Service configuration (discovery, registration, health checks)
    pub services: ServicesConfig,

    /// Storage configuration (paths, backends, quotas)
    pub storage: StorageConfig,

    /// Database configuration (connection pooling, credentials)
    pub database: DatabaseConfig,

    /// Cache configuration (Redis, in-memory, TTL)
    pub cache: CacheConfig,

    /// Monitoring configuration (metrics, logs, traces)
    pub monitoring: MonitoringConfig,

    /// Security configuration (auth, encryption, access control)
    pub security: SecurityConfig,
}

impl NestGateRuntimeConfig {
    /// Load configuration from environment variables with fallback to defaults
    pub fn from_environment() -> Result<Self> {
        Ok(Self {
            network: NetworkConfig::from_environment(),
            services: ServicesConfig::from_environment(),
            storage: StorageConfig::from_environment(),
            database: DatabaseConfig::from_environment(),
            cache: CacheConfig::from_environment(),
            monitoring: MonitoringConfig::from_environment(),
            security: SecurityConfig::from_environment(),
        })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        self.network.validate()?;
        self.services.validate()?;
        self.storage.validate()?;
        Ok(())
    }
}

// ==================== NETWORK CONFIGURATION ====================

/// Network configuration - eliminates hardcoded ports and IPs
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Network
pub struct NetworkConfig {
    /// API host (default: 127.0.0.1)
    pub api_host: IpAddr,

    /// API HTTP port (default: 8080)
    pub api_port: u16,

    /// API HTTPS port (default: 8443)
    pub https_port: u16,

    /// Internal service port (default: 3000)
    pub internal_port: u16,

    /// Bind all interfaces (default: false, binds 127.0.0.1 only)
    pub bind_all: bool,

    /// Request timeout in seconds (default: 30)
    pub timeout_seconds: u64,

    /// Connection pool size (default: 10)
    pub connection_pool_size: usize,
}

impl NetworkConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        Self {
            api_host: env::var("NESTGATE_API_HOST")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1))),

            api_port: env::var("NESTGATE_API_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::constants::hardcoding::ports::HTTP_DEFAULT),

            https_port: env::var("NESTGATE_HTTPS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8443),

            internal_port: env::var("NESTGATE_INTERNAL_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(crate::constants::hardcoding::ports::API_DEFAULT),

            bind_all: env::var("NESTGATE_BIND_ALL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),

            timeout_seconds: env::var("NESTGATE_TIMEOUT_SECONDS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),

            connection_pool_size: env::var("NESTGATE_CONNECTION_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }

    /// Validates data
    fn validate(&self) -> Result<()> {
        if self.api_port == 0 {
            return Err(NestGateError::configuration_error(
                "api_port",
                "API port cannot be 0",
            ));
        }
        Ok(())
    }

    /// Get the full API base URL
    pub fn api_base_url(&self) -> String {
        format!("http://{}:{}", self.api_host, self.api_port)
    }

    /// Get the full HTTPS base URL
    pub fn https_base_url(&self) -> String {
        format!("https://{}:{}", self.api_host, self.https_port)
    }
}

impl Default for NetworkConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            api_host: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            api_port: 8080,
            https_port: 8443,
            internal_port: 3000,
            bind_all: false,
            timeout_seconds: 30,
            connection_pool_size: 10,
        }
    }
}

// ==================== SERVICES CONFIGURATION ====================

/// Service configuration - primal ecosystem endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Services
pub struct ServicesConfig {
    /// BearDog security service URL
    pub beardog_url: Option<String>,

    /// Songbird orchestration service URL
    pub songbird_url: Option<String>,

    /// Squirrel AI/ML service URL
    pub squirrel_url: Option<String>,

    /// ToadStool compute service URL
    pub toadstool_url: Option<String>,

    /// biomeOS system service URL
    pub biomeos_url: Option<String>,

    /// Service discovery enabled (default: true)
    pub discovery_enabled: bool,

    /// Service discovery port (default: 8500)
    pub discovery_port: u16,
}

impl ServicesConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        Self {
            beardog_url: env::var("NESTGATE_BEARDOG_URL").ok(),
            songbird_url: env::var("NESTGATE_SONGBIRD_URL").ok(),
            squirrel_url: env::var("NESTGATE_SQUIRREL_URL").ok(),
            toadstool_url: env::var("NESTGATE_TOADSTOOL_URL").ok(),
            biomeos_url: env::var("NESTGATE_BIOMEOS_URL").ok(),
            discovery_enabled: env::var("NESTGATE_DISCOVERY_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            discovery_port: env::var("NESTGATE_DISCOVERY_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8500),
        }
    }

    /// Validates data
    fn validate(&self) -> Result<()> {
        // Service URLs are optional - infant discovery can find them
        Ok(())
    }

    /// Get BearDog URL or default to local discovery
    pub fn beardog_url_or_default(&self) -> String {
        use crate::constants::hardcoding::{addresses, ports};
        self.beardog_url.clone().unwrap_or_else(|| {
            format!(
                "http://{}:{}",
                addresses::LOCALHOST_NAME,
                ports::BEARDOG_DEFAULT
            )
        })
    }

    /// Get Songbird URL or default to local discovery
    pub fn songbird_url_or_default(&self) -> String {
        use crate::constants::hardcoding::{addresses, ports};
        self.songbird_url.clone().unwrap_or_else(|| {
            format!(
                "http://{}:{}",
                addresses::LOCALHOST_NAME,
                ports::SONGBIRD_DEFAULT
            )
        })
    }
}

impl Default for ServicesConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            beardog_url: None,
            songbird_url: None,
            squirrel_url: None,
            toadstool_url: None,
            biomeos_url: None,
            discovery_enabled: true,
            discovery_port: 8500,
        }
    }
}

// ==================== STORAGE CONFIGURATION ====================

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Storage
pub struct StorageConfig {
    /// Storage data directory
    pub data_dir: PathBuf,

    /// Temporary directory
    pub temp_dir: PathBuf,

    /// ZFS pool name (default: "nestgate")
    pub zfs_pool_name: String,

    /// Storage backend (default: "auto")
    pub backend: String,
}

impl StorageConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        Self {
            data_dir: env::var("NESTGATE_DATA_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/var/lib/nestgate")),

            temp_dir: env::var("NESTGATE_TEMP_DIR")
                .or_else(|_| env::var("TMPDIR"))
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("/tmp/nestgate")),

            zfs_pool_name: env::var("NESTGATE_ZFS_POOL").unwrap_or_else(|_| "nestgate".to_string()),

            backend: env::var("NESTGATE_STORAGE_BACKEND").unwrap_or_else(|_| "auto".to_string()),
        }
    }

    /// Validates data
    fn validate(&self) -> Result<()> {
        if self.zfs_pool_name.is_empty() {
            return Err(NestGateError::configuration_error(
                "zfs_pool_name",
                "ZFS pool name cannot be empty",
            ));
        }
        Ok(())
    }
}

impl Default for StorageConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("/var/lib/nestgate"),
            temp_dir: PathBuf::from("/tmp/nestgate"),
            zfs_pool_name: "nestgate".to_string(),
            backend: "auto".to_string(),
        }
    }
}

// ==================== DATABASE CONFIGURATION ====================

/// Database configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Database
pub struct DatabaseConfig {
    /// PostgreSQL host (default: localhost)
    pub postgres_host: String,

    /// PostgreSQL port (default: 5432)
    pub postgres_port: u16,

    /// PostgreSQL database name (default: nestgate)
    pub postgres_database: String,

    /// PostgreSQL user (default: nestgate)
    pub postgres_user: String,

    /// PostgreSQL password (from env only, never default)
    pub postgres_password: Option<String>,

    /// Connection pool size (default: 10)
    pub pool_size: u32,
}

impl DatabaseConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        use crate::constants::hardcoding::addresses;

        Self {
            postgres_host: env::var("NESTGATE_POSTGRES_HOST")
                .unwrap_or_else(|_| addresses::LOCALHOST_NAME.to_string()),

            postgres_port: env::var("NESTGATE_POSTGRES_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5432),

            postgres_database: env::var("NESTGATE_POSTGRES_DATABASE")
                .unwrap_or_else(|_| "nestgate".to_string()),

            postgres_user: env::var("NESTGATE_POSTGRES_USER")
                .unwrap_or_else(|_| "nestgate".to_string()),

            postgres_password: env::var("NESTGATE_POSTGRES_PASSWORD").ok(),

            pool_size: env::var("NESTGATE_DB_POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
        }
    }

    /// Get PostgreSQL connection string
    pub fn postgres_url(&self) -> Result<String> {
        let password = self.postgres_password.as_ref().ok_or_else(|| {
            NestGateError::configuration_error(
                "postgres_password",
                "PostgreSQL password must be set via NESTGATE_POSTGRES_PASSWORD",
            )
        })?;

        Ok(format!(
            "postgresql://{}:{}@{}:{}/{}",
            self.postgres_user,
            password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        ))
    }
}

impl Default for DatabaseConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            postgres_host: crate::constants::hardcoding::addresses::LOCALHOST_NAME.to_string(),
            postgres_port: 5432,
            postgres_database: "nestgate".to_string(),
            postgres_user: "nestgate".to_string(),
            postgres_password: None,
            pool_size: 10,
        }
    }
}

// ==================== CACHE CONFIGURATION ====================

/// Cache configuration (Redis, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Cache
pub struct CacheConfig {
    /// Redis host (default: localhost)
    pub redis_host: String,

    /// Redis port (default: 6379)
    pub redis_port: u16,

    /// Redis password (from env only)
    pub redis_password: Option<String>,

    /// Redis database number (default: 0)
    pub redis_database: u8,

    /// Cache enabled (default: true)
    pub enabled: bool,
}

impl CacheConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        Self {
            redis_host: env::var("NESTGATE_REDIS_HOST").unwrap_or_else(|_| {
                crate::constants::hardcoding::addresses::LOCALHOST_NAME.to_string()
            }),

            redis_port: env::var("NESTGATE_REDIS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(6379),

            redis_password: env::var("NESTGATE_REDIS_PASSWORD").ok(),

            redis_database: env::var("NESTGATE_REDIS_DATABASE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0),

            enabled: env::var("NESTGATE_CACHE_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
        }
    }

    /// Get Redis connection URL
    pub fn redis_url(&self) -> String {
        match &self.redis_password {
            Some(password) => format!(
                "redis://:{}@{}:{}/{}",
                password, self.redis_host, self.redis_port, self.redis_database
            ),
            None => format!(
                "redis://{}:{}/{}",
                self.redis_host, self.redis_port, self.redis_database
            ),
        }
    }
}

impl Default for CacheConfig {
    /// Returns the default instance
    fn default() -> Self {
        use crate::constants::hardcoding::{addresses, ports};

        Self {
            redis_host: addresses::LOCALHOST_NAME.to_string(),
            redis_port: ports::REDIS_DEFAULT,
            redis_password: None,
            redis_database: 0,
            enabled: true,
        }
    }
}

// ==================== MONITORING CONFIGURATION ====================

/// Monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Monitoring
pub struct MonitoringConfig {
    /// Prometheus metrics port (default: 9090)
    pub metrics_port: u16,

    /// Grafana port (default: 3000)
    pub grafana_port: u16,

    /// Metrics enabled (default: true)
    pub enabled: bool,

    /// Health check endpoint (default: /health)
    pub health_endpoint: String,
}

impl MonitoringConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        Self {
            metrics_port: env::var("NESTGATE_METRICS_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(9090),

            grafana_port: env::var("NESTGATE_GRAFANA_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),

            enabled: env::var("NESTGATE_MONITORING_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),

            health_endpoint: env::var("NESTGATE_HEALTH_ENDPOINT")
                .unwrap_or_else(|_| "/health".to_string()),
        }
    }
}

impl Default for MonitoringConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            metrics_port: 9090,
            grafana_port: 3000,
            enabled: true,
            health_endpoint: "/health".to_string(),
        }
    }
}

// ==================== SECURITY CONFIGURATION ====================

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for Security
pub struct SecurityConfig {
    /// TLS enabled (default: false for localhost, true for production)
    pub tls_enabled: bool,

    /// TLS certificate path
    pub tls_cert_path: Option<PathBuf>,

    /// TLS key path
    pub tls_key_path: Option<PathBuf>,

    /// API key required (default: true for production)
    pub api_key_required: bool,

    /// JWT secret (from env only, never default)
    pub jwt_secret: Option<String>,
}

impl SecurityConfig {
    /// Creates from Environment
    fn from_environment() -> Self {
        Self {
            tls_enabled: env::var("NESTGATE_TLS_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),

            tls_cert_path: env::var("NESTGATE_TLS_CERT_PATH").ok().map(PathBuf::from),

            tls_key_path: env::var("NESTGATE_TLS_KEY_PATH").ok().map(PathBuf::from),

            api_key_required: env::var("NESTGATE_API_KEY_REQUIRED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),

            jwt_secret: env::var("NESTGATE_JWT_SECRET").ok(),
        }
    }
}

impl Default for SecurityConfig {
    /// Returns the default instance
    fn default() -> Self {
        Self {
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            api_key_required: true,
            jwt_secret: None,
        }
    }
}

// ==================== GLOBAL CONFIGURATION INSTANCE ====================

/// Global configuration instance
static GLOBAL_CONFIG: OnceLock<NestGateRuntimeConfig> = OnceLock::new();

/// Initialize the global configuration from environment variables
pub fn init_config() -> Result<()> {
    let config = NestGateRuntimeConfig::from_environment()?;
    config.validate()?;

    GLOBAL_CONFIG.set(config).map_err(|_| {
        NestGateError::configuration_error("global_config", "Configuration already initialized")
    })?;

    Ok(())
}

/// Get the global configuration reference (lazy initialization).
///
/// Returns a static reference to the global [`NestGateRuntimeConfig`]. If the
/// configuration hasn't been initialized yet (via [`init_config()`]), it will be
/// automatically initialized from environment variables on first access.
///
/// # Panics
///
/// Panics if environment variables contain invalid values during automatic initialization.
/// To handle errors explicitly, use [`init_config()`] at application startup.
///
/// # Example
///
/// ```rust,ignore
/// use nestgate_core::config::runtime::get_config;
///
/// // Get configuration (auto-initializes on first call)
/// let config = get_config();
///
/// // Use configuration values
/// let host = &config.network.api_host;
/// let port = config.network.api_port;
/// ```
///
/// # Performance
///
/// This function is extremely fast after first initialization due to [`OnceLock`].
/// There is zero runtime overhead for subsequent calls.
///
/// # Thread Safety
///
/// This function is thread-safe and can be called from multiple threads concurrently.
/// The configuration will only be initialized once.
pub fn get_config() -> &'static NestGateRuntimeConfig {
    GLOBAL_CONFIG.get_or_init(|| {
        NestGateRuntimeConfig::from_environment().unwrap_or_else(|e| {
            eprintln!("⚠️ Failed to load configuration from environment: {e}");
            eprintln!("   Using default configuration");
            NestGateRuntimeConfig::default()
        })
    })
}

/// Helper: Get API base URL
pub fn api_base_url() -> String {
    get_config().network.api_base_url()
}

/// Helper: Get API port
pub fn api_port() -> u16 {
    get_config().network.api_port
}

/// Helper: Get service URL by name
pub fn service_url(name: &str) -> Option<String> {
    let services = &get_config().services;
    match name {
        "beardog" => services.beardog_url.clone(),
        "songbird" => services.songbird_url.clone(),
        "squirrel" => services.squirrel_url.clone(),
        "toadstool" => services.toadstool_url.clone(),
        "biomeos" => services.biomeos_url.clone(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NestGateRuntimeConfig::default();
        assert_eq!(config.network.api_port, 8080);
        assert_eq!(config.network.https_port, 8443);
        assert_eq!(config.database.postgres_port, 5432);
        assert_eq!(config.cache.redis_port, 6379);
    }

    #[test]
    fn test_api_base_url() {
        let config = NetworkConfig::default();
        assert_eq!(config.api_base_url(), "http://127.0.0.1:8080");
    }

    #[test]
    fn test_redis_url_without_password() {
        let config = CacheConfig::default();
        assert_eq!(config.redis_url(), "redis://localhost:6379/0");
    }

    #[test]
    fn test_redis_url_with_password() {
        let config = CacheConfig {
            redis_password: Some("secret".to_string()),
            ..Default::default()
        };
        assert_eq!(config.redis_url(), "redis://:secret@localhost:6379/0");
    }
}
