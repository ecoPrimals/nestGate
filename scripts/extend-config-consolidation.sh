#!/bin/bash
# 🔧 **EXTENDED CONFIG CONSOLIDATION IMPLEMENTATION**
# Extend config consolidation to network and storage configs

set -euo pipefail

echo "🔧 **NESTGATE EXTENDED CONFIG CONSOLIDATION**"
echo "============================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📊 **PHASE 1: NETWORK CONFIG CONSOLIDATION**"
echo "-------------------------------------------"

# Create network config consolidation implementation
NETWORK_CONSOLIDATION="code/crates/nestgate-core/src/config/migration_helpers/networkconfig_consolidation.rs"

cat > "$NETWORK_CONSOLIDATION" << 'EOF'
//! **NETWORK CONFIG CONSOLIDATION IMPLEMENTATION**
//! 
//! Extends the config consolidation framework to handle NetworkConfig patterns.

use crate::config::ConsolidatedCanonicalConfig;
use crate::config::migration_helpers::config_consolidation_implementation::{
    ConfigConsolidationBuilder, NetworkConfigFragment
};
use crate::error::NestGateUnifiedError;
use serde::{Deserialize, Serialize};

/// Enhanced network configuration fragment for complex patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedNetworkConfigFragment {
    // Basic network settings
    pub host: Option<String>,
    pub port: Option<u16>,
    pub api_port: Option<u16>,
    pub websocket_port: Option<u16>,
    pub internal_port: Option<u16>,
    
    // Connection settings
    pub max_connections: Option<usize>,
    pub timeout_secs: Option<u64>,
    pub connection_timeout_secs: Option<u64>,
    pub keepalive_timeout_secs: Option<u64>,
    pub buffer_size: Option<usize>,
    
    // Protocol settings
    pub enable_tls: Option<bool>,
    pub enable_ipv6: Option<bool>,
    pub tcp_keepalive: Option<bool>,
    pub keep_alive: Option<bool>,
    
    // Performance settings
    pub worker_threads: Option<u32>,
    pub retry_attempts: Option<u32>,
    pub pool_size: Option<usize>,
    
    // Service settings
    pub service_name: Option<String>,
    pub bind_endpoint: Option<String>,
}

impl From<EnhancedNetworkConfigFragment> for NetworkConfigFragment {
    fn from(enhanced: EnhancedNetworkConfigFragment) -> Self {
        Self {
            host: enhanced.host,
            port: enhanced.port,
            max_connections: enhanced.max_connections,
            timeout_secs: enhanced.timeout_secs.or(enhanced.connection_timeout_secs),
            enable_tls: enhanced.enable_tls,
        }
    }
}

/// Migrate common NetworkConfig patterns
pub fn migrate_network_config_comprehensive(
    host: Option<&str>,
    port: Option<u16>,
    api_port: Option<u16>,
    max_connections: Option<usize>,
    timeout_secs: Option<u64>,
    worker_threads: Option<u32>,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    let fragment = EnhancedNetworkConfigFragment {
        host: host.map(|s| s.to_string()),
        port,
        api_port,
        websocket_port: None,
        internal_port: None,
        max_connections,
        timeout_secs,
        connection_timeout_secs: None,
        keepalive_timeout_secs: None,
        buffer_size: None,
        enable_tls: Some(false),
        enable_ipv6: Some(false),
        tcp_keepalive: Some(true),
        keep_alive: Some(true),
        worker_threads,
        retry_attempts: Some(3),
        pool_size: Some(10),
        service_name: None,
        bind_endpoint: None,
    };
    
    ConfigConsolidationBuilder::new()
        .with_network(fragment.into())
        .build()
}

/// Migrate server config patterns
pub fn migrate_server_config(
    bind_addr: &str,
    port: u16,
    max_connections: Option<usize>,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    migrate_network_config_comprehensive(
        Some(bind_addr),
        Some(port),
        None,
        max_connections,
        Some(30),
        Some(4)
    )
}

/// Migrate mock network config patterns (common in tests)
pub fn migrate_mock_network_config(
    test_port: Option<u16>,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    migrate_network_config_comprehensive(
        Some("127.0.0.1"),
        test_port,
        None,
        Some(10),
        Some(5),
        Some(1)
    )
}

/// Migrate external network config patterns
pub fn migrate_external_network_config(
    endpoints: &[String],
    protocol: &str,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    // Use first endpoint as primary
    let primary_endpoint = endpoints.first()
        .map(|ep| ep.split(':').next().unwrap_or("127.0.0.1"))
        .unwrap_or("127.0.0.1");
    
    let port = endpoints.first()
        .and_then(|ep| ep.split(':').nth(1))
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);
    
    migrate_network_config_comprehensive(
        Some(primary_endpoint),
        Some(port),
        None,
        Some(100),
        Some(60),
        Some(8)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_config_consolidation() {
        let config = migrate_network_config_comprehensive(
            Some("0.0.0.0"),
            Some(8080),
            Some(3000),
            Some(1000),
            Some(30),
            Some(4)
        );
        assert!(config.is_ok());
    }
    
    #[test]
    fn test_server_config_migration() {
        let config = migrate_server_config("127.0.0.1", 8080, Some(100));
        assert!(config.is_ok());
    }
    
    #[test]
    fn test_mock_network_config_migration() {
        let config = migrate_mock_network_config(Some(9000));
        assert!(config.is_ok());
    }
}
EOF

echo "✅ Created network config consolidation: $NETWORK_CONSOLIDATION"

echo ""
echo "📊 **PHASE 2: STORAGE CONFIG CONSOLIDATION**"
echo "-------------------------------------------"

# Create storage config consolidation implementation
STORAGE_CONSOLIDATION="code/crates/nestgate-core/src/config/migration_helpers/storageconfig_consolidation.rs"

cat > "$STORAGE_CONSOLIDATION" << 'EOF'
//! **STORAGE CONFIG CONSOLIDATION IMPLEMENTATION**
//! 
//! Extends the config consolidation framework to handle StorageConfig patterns.

use crate::config::ConsolidatedCanonicalConfig;
use crate::config::migration_helpers::config_consolidation_implementation::{
    ConfigConsolidationBuilder, StorageConfigFragment
};
use crate::error::NestGateUnifiedError;
use serde::{Deserialize, Serialize};

/// Enhanced storage configuration fragment for complex patterns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedStorageConfigFragment {
    // Backend settings
    pub backend_type: Option<String>,
    pub primary_backend: Option<String>,
    pub backup_backends: Option<Vec<String>>,
    
    // Path settings
    pub base_path: Option<String>,
    pub data_dir: Option<String>,
    pub root_path: Option<String>,
    
    // Performance settings
    pub cache_size_mb: Option<u64>,
    pub cache_size: Option<usize>,
    pub max_size_mb: Option<u64>,
    pub buffer_size: Option<usize>,
    pub io_threads: Option<usize>,
    
    // Compression settings
    pub compression: Option<String>,
    pub compression_level: Option<u32>,
    pub enable_compression: Option<bool>,
    
    // Encryption settings
    pub encryption: Option<bool>,
    pub enable_encryption: Option<bool>,
    
    // Replication settings
    pub replication_enabled: Option<bool>,
    pub replication_factor: Option<u32>,
    
    // Monitoring settings
    pub enable_metrics: Option<bool>,
    pub enable_monitoring: Option<bool>,
    
    // ZFS specific
    pub pool_name: Option<String>,
    pub dataset_prefix: Option<String>,
    pub enable_snapshots: Option<bool>,
}

impl From<EnhancedStorageConfigFragment> for StorageConfigFragment {
    fn from(enhanced: EnhancedStorageConfigFragment) -> Self {
        Self {
            backend_type: enhanced.backend_type
                .or(enhanced.primary_backend)
                .unwrap_or_else(|| "filesystem".to_string())
                .into(),
            base_path: enhanced.base_path
                .or(enhanced.data_dir)
                .or(enhanced.root_path),
            cache_size_mb: enhanced.cache_size_mb
                .or_else(|| enhanced.cache_size.map(|s| s as u64))
                .or(enhanced.max_size_mb),
            compression: enhanced.compression,
            encryption: enhanced.encryption
                .or(enhanced.enable_encryption),
        }
    }
}

/// Migrate comprehensive storage config patterns
pub fn migrate_storage_config_comprehensive(
    backend_type: Option<&str>,
    base_path: Option<&str>,
    cache_size_mb: Option<u64>,
    compression: Option<&str>,
    encryption: Option<bool>,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    let fragment = EnhancedStorageConfigFragment {
        backend_type: backend_type.map(|s| s.to_string()),
        primary_backend: None,
        backup_backends: None,
        base_path: base_path.map(|s| s.to_string()),
        data_dir: None,
        root_path: None,
        cache_size_mb,
        cache_size: None,
        max_size_mb: None,
        buffer_size: None,
        io_threads: Some(4),
        compression: compression.map(|s| s.to_string()),
        compression_level: Some(6),
        enable_compression: compression.map(|_| true),
        encryption,
        enable_encryption: encryption,
        replication_enabled: Some(false),
        replication_factor: Some(1),
        enable_metrics: Some(true),
        enable_monitoring: Some(true),
        pool_name: None,
        dataset_prefix: None,
        enable_snapshots: Some(false),
    };
    
    ConfigConsolidationBuilder::new()
        .with_storage(fragment.into())
        .build()
}

/// Migrate ZFS storage config patterns
pub fn migrate_zfs_storage_config(
    pool_name: &str,
    dataset_prefix: Option<&str>,
    enable_snapshots: bool,
    compression: Option<&str>,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    let fragment = EnhancedStorageConfigFragment {
        backend_type: Some("zfs".to_string()),
        primary_backend: None,
        backup_backends: None,
        base_path: Some(format!("/{}", pool_name)),
        data_dir: None,
        root_path: None,
        cache_size_mb: Some(512),
        cache_size: None,
        max_size_mb: None,
        buffer_size: None,
        io_threads: Some(8),
        compression: compression.map(|s| s.to_string()).or_else(|| Some("lz4".to_string())),
        compression_level: Some(6),
        enable_compression: Some(true),
        encryption: Some(false),
        enable_encryption: Some(false),
        replication_enabled: Some(false),
        replication_factor: Some(1),
        enable_metrics: Some(true),
        enable_monitoring: Some(true),
        pool_name: Some(pool_name.to_string()),
        dataset_prefix: dataset_prefix.map(|s| s.to_string()),
        enable_snapshots: Some(enable_snapshots),
    };
    
    ConfigConsolidationBuilder::new()
        .with_storage(fragment.into())
        .build()
}

/// Migrate universal storage config patterns
pub fn migrate_universal_storage_config(
    primary_backend: &str,
    backup_backends: &[String],
    cache_size_mb: Option<u64>,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    let fragment = EnhancedStorageConfigFragment {
        backend_type: Some(primary_backend.to_string()),
        primary_backend: Some(primary_backend.to_string()),
        backup_backends: Some(backup_backends.to_vec()),
        base_path: Some("/var/lib/nestgate".to_string()),
        data_dir: None,
        root_path: None,
        cache_size_mb,
        cache_size: None,
        max_size_mb: None,
        buffer_size: Some(8192),
        io_threads: Some(4),
        compression: Some("lz4".to_string()),
        compression_level: Some(6),
        enable_compression: Some(true),
        encryption: Some(true),
        enable_encryption: Some(true),
        replication_enabled: Some(true),
        replication_factor: Some(3),
        enable_metrics: Some(true),
        enable_monitoring: Some(true),
        pool_name: None,
        dataset_prefix: None,
        enable_snapshots: Some(true),
    };
    
    ConfigConsolidationBuilder::new()
        .with_storage(fragment.into())
        .build()
}

/// Migrate test storage config patterns
pub fn migrate_test_storage_config(
    test_name: &str,
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    migrate_storage_config_comprehensive(
        Some("memory"),
        Some(&format!("/tmp/nestgate-test-{}", test_name)),
        Some(64),
        Some("none"),
        Some(false)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_config_consolidation() {
        let config = migrate_storage_config_comprehensive(
            Some("filesystem"),
            Some("/data"),
            Some(256),
            Some("lz4"),
            Some(true)
        );
        assert!(config.is_ok());
    }
    
    #[test]
    fn test_zfs_storage_config_migration() {
        let config = migrate_zfs_storage_config(
            "tank",
            Some("nestgate"),
            true,
            Some("zstd")
        );
        assert!(config.is_ok());
    }
    
    #[test]
    fn test_universal_storage_config_migration() {
        let config = migrate_universal_storage_config(
            "zfs",
            &["s3".to_string(), "filesystem".to_string()],
            Some(1024)
        );
        assert!(config.is_ok());
    }
}
EOF

echo "✅ Created storage config consolidation: $STORAGE_CONSOLIDATION"

echo ""
echo "🔄 **PHASE 3: SYSTEMATIC CONFIG FILE CONSOLIDATION**"
echo "---------------------------------------------------"

# Function to consolidate network configs in a specific file
consolidate_network_configs() {
    local file_path="$1"
    local backup_path="${file_path}.network_backup"
    
    echo "🔄 Consolidating network configs in: $file_path"
    
    # Create backup
    cp "$file_path" "$backup_path"
    
    # Apply network config consolidation patterns
    sed -i \
        -e 's|pub struct NetworkConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with network fragment")]\npub struct LegacyNetworkConfig|g' \
        -e 's|pub struct ServerConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with network fragment")]\npub struct LegacyServerConfig|g' \
        -e 's|pub struct MockNetworkConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with network fragment")]\npub struct LegacyMockNetworkConfig|g' \
        -e 's|pub struct ExternalNetworkConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with network fragment")]\npub struct LegacyExternalNetworkConfig|g' \
        "$file_path"
    
    echo "   ✅ Consolidated network configs: $file_path (backup: $backup_path)"
}

# Function to consolidate storage configs in a specific file
consolidate_storage_configs() {
    local file_path="$1"
    local backup_path="${file_path}.storage_backup"
    
    echo "🔄 Consolidating storage configs in: $file_path"
    
    # Create backup
    cp "$file_path" "$backup_path"
    
    # Apply storage config consolidation patterns
    sed -i \
        -e 's|pub struct StorageConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with storage fragment")]\npub struct LegacyStorageConfig|g' \
        -e 's|pub struct UniversalStorageConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with storage fragment")]\npub struct LegacyUniversalStorageConfig|g' \
        -e 's|pub struct ZfsStorageConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with storage fragment")]\npub struct LegacyZfsStorageConfig|g' \
        -e 's|pub struct TestStorageConfig|#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig with storage fragment")]\npub struct LegacyTestStorageConfig|g' \
        "$file_path"
    
    echo "   ✅ Consolidated storage configs: $file_path (backup: $backup_path)"
}

# Find and consolidate network config files
echo "Finding files with network config patterns..."
NETWORK_CONFIG_FILES=$(find . -name "*.rs" -exec grep -l "struct.*NetworkConfig\|struct.*ServerConfig" {} \; | head -15)
NETWORK_COUNT=$(echo "$NETWORK_CONFIG_FILES" | grep -v '^$' | wc -l)

echo "Found $NETWORK_COUNT network config files to consolidate"

# Consolidate network configs
BATCH_SIZE=5
CURRENT_BATCH=0

for file in $NETWORK_CONFIG_FILES; do
    if [ -n "$file" ] && [ -f "$file" ]; then
        consolidate_network_configs "$file"
        CURRENT_BATCH=$((CURRENT_BATCH + 1))
        
        if [ $((CURRENT_BATCH % BATCH_SIZE)) -eq 0 ]; then
            echo "   📊 Consolidated $CURRENT_BATCH / $NETWORK_COUNT network config files..."
        fi
    fi
done

echo ""
echo "Finding files with storage config patterns..."
STORAGE_CONFIG_FILES=$(find . -name "*.rs" -exec grep -l "struct.*StorageConfig" {} \; | head -15)
STORAGE_COUNT=$(echo "$STORAGE_CONFIG_FILES" | grep -v '^$' | wc -l)

echo "Found $STORAGE_COUNT storage config files to consolidate"

# Consolidate storage configs
CURRENT_BATCH=0

for file in $STORAGE_CONFIG_FILES; do
    if [ -n "$file" ] && [ -f "$file" ]; then
        consolidate_storage_configs "$file"
        CURRENT_BATCH=$((CURRENT_BATCH + 1))
        
        if [ $((CURRENT_BATCH % BATCH_SIZE)) -eq 0 ]; then
            echo "   📊 Consolidated $CURRENT_BATCH / $STORAGE_COUNT storage config files..."
        fi
    fi
done

echo ""
echo "🔧 **PHASE 4: UPDATE MODULE EXPORTS**"
echo "------------------------------------"

# Update config migration helpers module
CONFIG_MIGRATION_MOD="code/crates/nestgate-core/src/config/migration_helpers/mod.rs"

# Add new consolidation modules
if ! grep -q "pub mod networkconfig_consolidation;" "$CONFIG_MIGRATION_MOD"; then
    echo "pub mod networkconfig_consolidation;" >> "$CONFIG_MIGRATION_MOD"
    echo "✅ Added networkconfig_consolidation to: $CONFIG_MIGRATION_MOD"
fi

if ! grep -q "pub mod storageconfig_consolidation;" "$CONFIG_MIGRATION_MOD"; then
    echo "pub mod storageconfig_consolidation;" >> "$CONFIG_MIGRATION_MOD"
    echo "✅ Added storageconfig_consolidation to: $CONFIG_MIGRATION_MOD"
fi

# Add re-exports for enhanced consolidation functions
cat >> "$CONFIG_MIGRATION_MOD" << 'EOF'

// Re-export enhanced consolidation functions
pub use networkconfig_consolidation::{
    migrate_network_config_comprehensive, migrate_server_config, 
    migrate_mock_network_config, migrate_external_network_config,
    EnhancedNetworkConfigFragment
};

pub use storageconfig_consolidation::{
    migrate_storage_config_comprehensive, migrate_zfs_storage_config,
    migrate_universal_storage_config, migrate_test_storage_config,
    EnhancedStorageConfigFragment
};
EOF

echo "✅ Added enhanced consolidation exports to: $CONFIG_MIGRATION_MOD"

echo ""
echo "📝 **PHASE 5: CREATE EXTENDED CONSOLIDATION EXAMPLES**"
echo "-----------------------------------------------------"

# Create extended consolidation examples
EXTENDED_EXAMPLES="docs/EXTENDED_CONFIG_CONSOLIDATION_EXAMPLES.md"

cat > "$EXTENDED_EXAMPLES" << 'EOF'
# 🔧 **EXTENDED CONFIG CONSOLIDATION EXAMPLES**

**Generated**: $(date)  
**Purpose**: Extended examples for network and storage config consolidation  
**Status**: 🔄 **IMPLEMENTATION READY**

---

## 📋 **NETWORK CONFIG CONSOLIDATION EXAMPLES**

### **Example 1: Basic NetworkConfig Consolidation**

**BEFORE** (Scattered Network Configs):
```rust
// Multiple files with different NetworkConfig structs
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
    pub timeout: Duration,
}

pub struct ServerConfig {
    pub bind_addr: String,
    pub max_connections: usize,
}

pub struct MockNetworkConfig {
    pub test_port: u16,
}
```

**AFTER** (Consolidated):
```rust
use crate::config::migration_helpers::migrate_network_config_comprehensive;

// Unified network configuration
let network_config = migrate_network_config_comprehensive(
    Some("0.0.0.0"),           // host
    Some(8080),                // port
    Some(3000),                // api_port
    Some(1000),                // max_connections
    Some(30),                  // timeout_secs
    Some(4)                    // worker_threads
)?;
```

### **Example 2: Server Config Migration**

**BEFORE** (Server-specific Config):
```rust
pub struct ServerConfig {
    pub bind_addr: String,
    pub port: u16,
    pub max_connections: usize,
}

let server_config = ServerConfig {
    bind_addr: "127.0.0.1".to_string(),
    port: 8080,
    max_connections: 100,
};
```

**AFTER** (Consolidated):
```rust
use crate::config::migration_helpers::migrate_server_config;

let server_config = migrate_server_config(
    "127.0.0.1",
    8080,
    Some(100)
)?;
```

### **Example 3: Test Network Config Migration**

**BEFORE** (Test-specific Config):
```rust
pub struct MockNetworkConfig {
    pub test_port: u16,
}

let mock_config = MockNetworkConfig {
    test_port: 9000,
};
```

**AFTER** (Consolidated):
```rust
use crate::config::migration_helpers::migrate_mock_network_config;

let test_config = migrate_mock_network_config(Some(9000))?;
```

---

## 📋 **STORAGE CONFIG CONSOLIDATION EXAMPLES**

### **Example 1: Basic StorageConfig Consolidation**

**BEFORE** (Scattered Storage Configs):
```rust
// Multiple files with different StorageConfig structs
pub struct StorageConfig {
    pub backend: String,
    pub path: String,
    pub cache_size: usize,
}

pub struct UniversalStorageConfig {
    pub primary_backend: String,
    pub backup_backends: Vec<String>,
    pub compression: bool,
}
```

**AFTER** (Consolidated):
```rust
use crate::config::migration_helpers::migrate_storage_config_comprehensive;

// Unified storage configuration
let storage_config = migrate_storage_config_comprehensive(
    Some("filesystem"),         // backend_type
    Some("/var/lib/nestgate"),  // base_path
    Some(256),                  // cache_size_mb
    Some("lz4"),                // compression
    Some(true)                  // encryption
)?;
```

### **Example 2: ZFS Storage Config Migration**

**BEFORE** (ZFS-specific Config):
```rust
pub struct ZfsStorageConfig {
    pub pool_name: String,
    pub dataset_prefix: String,
    pub enable_snapshots: bool,
    pub compression: String,
}

let zfs_config = ZfsStorageConfig {
    pool_name: "tank".to_string(),
    dataset_prefix: "nestgate".to_string(),
    enable_snapshots: true,
    compression: "zstd".to_string(),
};
```

**AFTER** (Consolidated):
```rust
use crate::config::migration_helpers::migrate_zfs_storage_config;

let zfs_config = migrate_zfs_storage_config(
    "tank",
    Some("nestgate"),
    true,
    Some("zstd")
)?;
```

### **Example 3: Universal Storage Config Migration**

**BEFORE** (Universal Storage Config):
```rust
pub struct UniversalStorageConfig {
    pub primary_backend: String,
    pub backup_backends: Vec<String>,
    pub cache_config: CacheConfig,
}

let universal_config = UniversalStorageConfig {
    primary_backend: "zfs".to_string(),
    backup_backends: vec!["s3".to_string(), "filesystem".to_string()],
    cache_config: CacheConfig::default(),
};
```

**AFTER** (Consolidated):
```rust
use crate::config::migration_helpers::migrate_universal_storage_config;

let universal_config = migrate_universal_storage_config(
    "zfs",
    &["s3".to_string(), "filesystem".to_string()],
    Some(1024)
)?;
```

---

## 🛠️ **ADVANCED CONSOLIDATION PATTERNS**

### **Pattern 1: Combined Network + Storage Consolidation**

```rust
use crate::config::migration_helpers::{
    ConfigConsolidationBuilder, migrate_network_config_comprehensive, 
    migrate_storage_config_comprehensive
};

// Build comprehensive configuration
let network_fragment = migrate_network_config_comprehensive(
    Some("0.0.0.0"), Some(8080), None, Some(1000), Some(30), Some(4)
)?;

let storage_fragment = migrate_storage_config_comprehensive(
    Some("zfs"), Some("/tank/nestgate"), Some(512), Some("lz4"), Some(true)
)?;

// Combine into single consolidated config
let combined_config = ConfigConsolidationBuilder::new()
    .with_network(network_fragment.network)
    .with_storage(storage_fragment.storage)
    .build()?;
```

### **Pattern 2: Test Environment Consolidation**

```rust
use crate::config::migration_helpers::{migrate_test_storage_config, migrate_mock_network_config};

// Consolidated test configuration
let test_network = migrate_mock_network_config(Some(9000))?;
let test_storage = migrate_test_storage_config("integration_test")?;

let test_config = ConfigConsolidationBuilder::new()
    .with_network(test_network.network)
    .with_storage(test_storage.storage)
    .build()?;
```

---

*Generated by NestGate Extended Config Consolidation System*
EOF

echo "✅ Created extended consolidation examples: $EXTENDED_EXAMPLES"

echo ""
echo "📈 **EXTENDED CONFIG CONSOLIDATION SUMMARY**"
echo "-------------------------------------------"

echo "✅ Extended config consolidation implementation complete"
echo "✅ $NETWORK_COUNT network config files processed"
echo "✅ $STORAGE_COUNT storage config files processed"
echo "✅ Enhanced consolidation frameworks created"
echo "✅ Extended documentation with examples"

echo ""
echo "🎯 **CONSOLIDATION PROGRESS UPDATE**"
echo "-----------------------------------"
echo "Network configs processed: $NETWORK_COUNT"
echo "Storage configs processed: $STORAGE_COUNT"
echo "Enhanced frameworks: 2 (network + storage)"
echo "Total config consolidation coverage: Test + Network + Storage"

echo ""
echo "✅ **EXTENDED CONFIG CONSOLIDATION COMPLETE**"
echo "=============================================" 