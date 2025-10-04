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
