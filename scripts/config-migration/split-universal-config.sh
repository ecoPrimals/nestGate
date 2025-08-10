#!/bin/bash

# Split Universal Primal Config Script
# Splits the massive 1229-line config file into organized modules

set -e

echo "🔨 Splitting universal_primal_config.rs into organized modules"
echo "============================================================="

# Configuration
SOURCE_FILE="code/crates/nestgate-api/src/universal_primal_config.rs"
CONFIG_DIR="code/crates/nestgate-api/src/config"
BACKUP_DIR="./config-migration-backup"

# Create backup
mkdir -p "$BACKUP_DIR"
cp "$SOURCE_FILE" "$BACKUP_DIR/universal_primal_config.rs.backup.$(date +%Y%m%d_%H%M%S)"

# Create new module structure
mkdir -p "$CONFIG_DIR"/{primal,network,storage,monitoring,security}

echo "📁 Created module directories:"
ls -la "$CONFIG_DIR"

# Extract header and imports for reuse
cat << 'EOF' > "$CONFIG_DIR/mod.rs"
/// Unified Configuration Modules
///
/// This module provides organized configuration types that integrate with
/// the unified configuration system from nestgate-core.
///
/// **ECOSYSTEM UNIFICATION**: All configs now use unified types for consistency

pub mod primal;
pub mod network;
pub mod storage;
pub mod monitoring;
pub mod security;

// Re-export for backward compatibility
pub use primal::*;
pub use network::*;
pub use storage::*;
pub use monitoring::*;
pub use security::*;

// 🚀 ECOSYSTEM UNIFICATION: Import unified types
use nestgate_core::unified_types::{
    UnifiedConfig, UnifiedNetworkConfig, UnifiedSecurityConfig, 
    UnifiedMonitoringConfig, UnifiedServiceConfig
};
EOF

# Create primal.rs - Core primal ecosystem configs
cat << 'EOF' > "$CONFIG_DIR/primal.rs"
/// Primal Ecosystem Configuration
///
/// Configuration types for primal discovery, integration, and ecosystem management.
/// **ECOSYSTEM UNIFICATION**: Uses unified types for consistency.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// 🚀 ECOSYSTEM UNIFICATION: Import unified types
use nestgate_core::unified_types::{
    UnifiedConfig, UnifiedNetworkConfig, UnifiedSecurityConfig,
    UnifiedMonitoringConfig, UnifiedServiceConfig
};

/// **MIGRATING TO UNIFIED**: Use UnifiedConfig with custom primal fields
/// This config will be absorbed into UnifiedConfig.custom
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimalEcosystemConfig {
    /// Enable primal discovery
    pub discovery_enabled: bool,
    /// Discovery interval
    pub discovery_interval: Duration,
    /// Known primal endpoints
    pub known_primals: HashMap<String, String>,
    /// Integration timeout
    pub integration_timeout: Duration,
}

impl PrimalEcosystemConfig {
    /// Convert to unified configuration
    pub fn to_unified(&self) -> UnifiedConfig {
        let mut custom = HashMap::new();
        custom.insert("discovery_enabled".to_string(), 
                     serde_json::Value::Bool(self.discovery_enabled));
        custom.insert("discovery_interval".to_string(), 
                     serde_json::Value::Number(self.discovery_interval.as_secs().into()));
        custom.insert("known_primals".to_string(), 
                     serde_json::to_value(&self.known_primals).unwrap());
        custom.insert("integration_timeout".to_string(), 
                     serde_json::Value::Number(self.integration_timeout.as_secs().into()));
        
        UnifiedConfig {
            service: UnifiedServiceConfig {
                name: "primal-ecosystem".to_string(),
                service_type: "discovery".to_string(),
                ..Default::default()
            },
            network: UnifiedNetworkConfig {
                discovery_enabled: self.discovery_enabled,
                service_endpoints: self.known_primals.clone(),
                ..Default::default()
            },
            custom,
            ..Default::default()
        }
    }
}

impl Default for PrimalEcosystemConfig {
    fn default() -> Self {
        Self {
            discovery_enabled: true,
            discovery_interval: Duration::from_secs(30),
            known_primals: HashMap::new(),
            integration_timeout: Duration::from_secs(10),
        }
    }
}

/// Modern unified type alias
pub type ModernPrimalEcosystemConfig = UnifiedConfig;
EOF

# Create network.rs - Network and discovery configs
cat << 'EOF' > "$CONFIG_DIR/network.rs"
/// Network Configuration
///
/// Network-related configuration types for NestGate services.
/// **ECOSYSTEM UNIFICATION**: Migrating to UnifiedNetworkConfig.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::Duration;

// 🚀 ECOSYSTEM UNIFICATION: Import unified types
use nestgate_core::unified_types::{
    UnifiedConfig, UnifiedNetworkConfig, UnifiedSecurityConfig,
    UnifiedMonitoringConfig, UnifiedServiceConfig
};

/// **MIGRATING TO UNIFIED**: Use UnifiedNetworkConfig instead
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<u32>,
    pub max_connections: Option<u32>,
    pub request_timeout_ms: Option<u64>,
}

impl ServerConfig {
    /// Convert to unified network configuration
    pub fn to_unified(&self) -> UnifiedNetworkConfig {
        UnifiedNetworkConfig {
            bind_address: self.host.parse().unwrap_or_else(|_| 
                std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST)),
            api_port: self.port,
            websocket_port: None,
            service_name: "nestgate".to_string(),
            max_connections: self.max_connections.unwrap_or(1000) as usize,
            connection_timeout: Duration::from_millis(
                self.request_timeout_ms.unwrap_or(30000)),
            keep_alive: true,
            discovery_enabled: true,
            service_endpoints: HashMap::new(),
            compression: true,
            buffer_size: 8192,
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            workers: None,
            max_connections: Some(1000),
            request_timeout_ms: Some(30000),
        }
    }
}

/// Modern unified type alias
pub type ModernServerConfig = UnifiedNetworkConfig;
EOF

echo "✅ Created base module structure"
echo "📊 Module file sizes:"
find "$CONFIG_DIR" -name "*.rs" -exec wc -l {} +

echo ""
echo "🎯 Next steps:"
echo "1. Update imports in universal_primal_config.rs"
echo "2. Move remaining structs to appropriate modules"
echo "3. Update all usages to use unified types"
echo "4. Test compilation: cargo check" 