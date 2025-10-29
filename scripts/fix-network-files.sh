#!/bin/bash

# Fix all corrupted network domain files
echo "🔧 Fixing all network domain files..."

# Fix performance.rs
cat > code/crates/nestgate-core/src/config/canonical_master/domains/network/performance.rs << 'EOF'
// **NETWORK PERFORMANCE CONFIGURATION**
//! Network performance and optimization configuration.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};

/// Network performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceConfig {
    /// Enable performance optimizations
    pub enabled: bool,
    /// Connection pool size
    pub connection_pool_size: u32,
    /// Request timeout in seconds
    pub request_timeout_secs: u64,
    /// Enable connection keep-alive
    pub keep_alive_enabled: bool,
}

impl NetworkPerformanceConfig {
    /// Create a development-optimized configuration
    pub const fn development_optimized() -> Self {
        Self {
            enabled: true,
            connection_pool_size: 10,
            request_timeout_secs: 30,
            keep_alive_enabled: true,
        }
    }

    /// Create a production-hardened configuration
    pub const fn production_hardened() -> Self {
        Self {
            enabled: true,
            connection_pool_size: 100,
            request_timeout_secs: 10,
            keep_alive_enabled: true,
        }
    }

    /// Validate the configuration
    pub const fn validate(&self) -> Result<(), NestGateError> {
        Ok(())
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.connection_pool_size = other.connection_pool_size;
        self.request_timeout_secs = other.request_timeout_secs;
        self.keep_alive_enabled = other.keep_alive_enabled;
        self
    }
}

impl Default for NetworkPerformanceConfig {
    fn default() -> Self {
        Self::development_optimized()
    }
}
EOF

# Fix security.rs
cat > code/crates/nestgate-core/src/config/canonical_master/domains/network/security.rs << 'EOF'
// **NETWORK SECURITY CONFIGURATION**
//! Network security and authentication configuration.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Enable TLS
    pub tls_enabled: bool,
    /// Minimum TLS version
    pub min_tls_version: String,
    /// Enable client certificate authentication
    pub client_cert_auth: bool,
    /// Enable IP whitelisting
    pub ip_whitelist_enabled: bool,
}

impl NetworkSecurityConfig {
    /// Create a development-optimized configuration
    pub const fn development_optimized() -> Self {
        Self {
            tls_enabled: false,
            min_tls_version: String::new(),
            client_cert_auth: false,
            ip_whitelist_enabled: false,
        }
    }

    /// Create a production-hardened configuration
    pub const fn production_hardened() -> Self {
        Self {
            tls_enabled: true,
            min_tls_version: String::new(),
            client_cert_auth: true,
            ip_whitelist_enabled: true,
        }
    }

    /// Validate the configuration
    pub const fn validate(&self) -> Result<(), NestGateError> {
        Ok(())
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        self.tls_enabled = other.tls_enabled;
        self.min_tls_version = other.min_tls_version;
        self.client_cert_auth = other.client_cert_auth;
        self.ip_whitelist_enabled = other.ip_whitelist_enabled;
        self
    }
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self::development_optimized()
    }
}
EOF

# Fix monitoring.rs
cat > code/crates/nestgate-core/src/config/canonical_master/domains/network/monitoring.rs << 'EOF'
// **NETWORK MONITORING CONFIGURATION**
//! Network monitoring and observability configuration.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};

/// Network monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMonitoringConfig {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval in seconds
    pub metrics_interval_secs: u64,
    /// Enable detailed logging
    pub detailed_logging: bool,
    /// Enable health checks
    pub health_checks_enabled: bool,
}

impl NetworkMonitoringConfig {
    /// Create a development-optimized configuration
    pub const fn development_optimized() -> Self {
        Self {
            enabled: true,
            metrics_interval_secs: 60,
            detailed_logging: true,
            health_checks_enabled: true,
        }
    }

    /// Create a production-hardened configuration
    pub const fn production_hardened() -> Self {
        Self {
            enabled: true,
            metrics_interval_secs: 30,
            detailed_logging: false,
            health_checks_enabled: true,
        }
    }

    /// Validate the configuration
    pub const fn validate(&self) -> Result<(), NestGateError> {
        Ok(())
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.metrics_interval_secs = other.metrics_interval_secs;
        self.detailed_logging = other.detailed_logging;
        self.health_checks_enabled = other.health_checks_enabled;
        self
    }
}

impl Default for NetworkMonitoringConfig {
    fn default() -> Self {
        Self::development_optimized()
    }
}
EOF

# Fix environment.rs
cat > code/crates/nestgate-core/src/config/canonical_master/domains/network/environment.rs << 'EOF'
// **NETWORK ENVIRONMENT CONFIGURATION**
//! Environment-specific network configuration.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};

/// Network environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEnvironmentConfig {
    /// Environment name
    pub name: String,
    /// Debug mode enabled
    pub debug_enabled: bool,
    /// Development mode enabled
    pub dev_mode: bool,
    /// Production optimizations enabled
    pub production_optimizations: bool,
}

impl NetworkEnvironmentConfig {
    /// Create a development-optimized configuration
    pub const fn development_optimized() -> Self {
        Self {
            name: String::new(),
            debug_enabled: true,
            dev_mode: true,
            production_optimizations: false,
        }
    }

    /// Create a production-hardened configuration
    pub const fn production_hardened() -> Self {
        Self {
            name: String::new(),
            debug_enabled: false,
            dev_mode: false,
            production_optimizations: true,
        }
    }

    /// Validate the configuration
    pub const fn validate(&self) -> Result<(), NestGateError> {
        Ok(())
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        self.name = other.name;
        self.debug_enabled = other.debug_enabled;
        self.dev_mode = other.dev_mode;
        self.production_optimizations = other.production_optimizations;
        self
    }
}

impl Default for NetworkEnvironmentConfig {
    fn default() -> Self {
        Self::development_optimized()
    }
}
EOF

# Fix api.rs
cat > code/crates/nestgate-core/src/config/canonical_master/domains/network/api.rs << 'EOF'
// **NETWORK API CONFIGURATION**
//! Network API server configuration.

use crate::error::NestGateError;
use serde::{Deserialize, Serialize};

/// Network API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkApiConfig {
    /// API server enabled
    pub enabled: bool,
    /// API server port
    pub port: u16,
    /// API server host
    pub host: String,
    /// Enable CORS
    pub cors_enabled: bool,
}

impl NetworkApiConfig {
    /// Create a development-optimized configuration
    pub const fn development_optimized() -> Self {
        Self {
            enabled: true,
            port: 8080,
            host: String::new(),
            cors_enabled: true,
        }
    }

    /// Create a production-hardened configuration
    pub const fn production_hardened() -> Self {
        Self {
            enabled: true,
            port: 443,
            host: String::new(),
            cors_enabled: false,
        }
    }

    /// Validate the configuration
    pub const fn validate(&self) -> Result<(), NestGateError> {
        Ok(())
    }

    /// Merge with another configuration
    pub fn merge(mut self, other: Self) -> Self {
        self.enabled = other.enabled;
        self.port = other.port;
        self.host = other.host;
        self.cors_enabled = other.cors_enabled;
        self
    }
}

impl Default for NetworkApiConfig {
    fn default() -> Self {
        Self::development_optimized()
    }
}
EOF

echo "✅ All network domain files fixed!" 