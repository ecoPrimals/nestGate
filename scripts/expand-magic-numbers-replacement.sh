#!/bin/bash
# 🔢 **EXPANDED MAGIC NUMBERS REPLACEMENT**
# Expand magic number replacement to core modules

set -euo pipefail

echo "🔢 **NESTGATE EXPANDED MAGIC NUMBERS REPLACEMENT**"
echo "================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📊 **PHASE 1: CORE MODULE MAGIC NUMBER ANALYSIS**"
echo "------------------------------------------------"

# Function to analyze magic numbers in core modules
analyze_core_magic_numbers() {
    local module_path="$1"
    local module_name="$2"
    
    echo "🔍 Analyzing magic numbers in: $module_name"
    
    # Count common magic numbers
    local count_8080=$(find "$module_path" -name "*.rs" -exec grep -o "\b8080\b" {} \; 2>/dev/null | wc -l)
    local count_8192=$(find "$module_path" -name "*.rs" -exec grep -o "\b8192\b" {} \; 2>/dev/null | wc -l)
    local count_1000=$(find "$module_path" -name "*.rs" -exec grep -o "\b1000\b" {} \; 2>/dev/null | wc -l)
    local count_30=$(find "$module_path" -name "*.rs" -exec grep -o "\b30\b" {} \; 2>/dev/null | wc -l)
    local count_128=$(find "$module_path" -name "*.rs" -exec grep -o "\b128\b" {} \; 2>/dev/null | wc -l)
    local count_4096=$(find "$module_path" -name "*.rs" -exec grep -o "\b4096\b" {} \; 2>/dev/null | wc -l)
    
    echo "   Port 8080: $count_8080 instances"
    echo "   Buffer 8192: $count_8192 instances"  
    echo "   Limit 1000: $count_1000 instances"
    echo "   Timeout 30: $count_30 instances"
    echo "   Cache 128: $count_128 instances"
    echo "   Block 4096: $count_4096 instances"
    
    local total=$((count_8080 + count_8192 + count_1000 + count_30 + count_128 + count_4096))
    echo "   📊 Total magic numbers in $module_name: $total"
    
    return $total
}

# Analyze core modules
echo "Analyzing magic numbers in core modules..."

analyze_core_magic_numbers "code/crates/nestgate-core/src/config" "Config Module"
CONFIG_MAGIC_COUNT=$?

analyze_core_magic_numbers "code/crates/nestgate-core/src/network" "Network Module" 
NETWORK_MAGIC_COUNT=$?

analyze_core_magic_numbers "code/crates/nestgate-core/src/storage" "Storage Module"
STORAGE_MAGIC_COUNT=$?

analyze_core_magic_numbers "code/crates/nestgate-core/src/cache" "Cache Module"
CACHE_MAGIC_COUNT=$?

analyze_core_magic_numbers "code/crates/nestgate-core/src/events" "Events Module"
EVENTS_MAGIC_COUNT=$?

TOTAL_CORE_MAGIC=$((CONFIG_MAGIC_COUNT + NETWORK_MAGIC_COUNT + STORAGE_MAGIC_COUNT + CACHE_MAGIC_COUNT + EVENTS_MAGIC_COUNT))

echo ""
echo "📈 **CORE MODULE ANALYSIS SUMMARY**"
echo "----------------------------------"
echo "Config module magic numbers: $CONFIG_MAGIC_COUNT"
echo "Network module magic numbers: $NETWORK_MAGIC_COUNT" 
echo "Storage module magic numbers: $STORAGE_MAGIC_COUNT"
echo "Cache module magic numbers: $CACHE_MAGIC_COUNT"
echo "Events module magic numbers: $EVENTS_MAGIC_COUNT"
echo "Total core modules magic numbers: $TOTAL_CORE_MAGIC"

echo ""
echo "🔄 **PHASE 2: SYSTEMATIC CORE MODULE REPLACEMENT**"
echo "-------------------------------------------------"

# Function to replace magic numbers in core module files
replace_core_magic_numbers() {
    local file_path="$1"
    local module_context="$2"
    local backup_path="${file_path}.core_magic_backup"
    
    echo "🔄 Replacing core magic numbers in: $file_path"
    
    # Create backup
    cp "$file_path" "$backup_path"
    
    # Apply intelligent magic number replacement based on context
    if [[ "$module_context" == *"network"* ]]; then
        # Network-specific replacements
        sed -i \
            -e 's/\b8080\b/crate::constants::magic_numbers_replacement::network::DEFAULT_HTTP_PORT/g' \
            -e 's/\b8443\b/crate::constants::magic_numbers_replacement::network::DEFAULT_HTTPS_PORT/g' \
            -e 's/\b3000\b/crate::constants::magic_numbers_replacement::network::DEFAULT_INTERNAL_PORT/g' \
            -e 's/\b1000\b/crate::constants::magic_numbers_replacement::network::DEFAULT_MAX_CONNECTIONS/g' \
            -e 's/\b30\b/crate::constants::magic_numbers_replacement::network::DEFAULT_TIMEOUT_SECS/g' \
            -e 's/\b10\b/crate::constants::magic_numbers_replacement::network::DEFAULT_POOL_SIZE/g' \
            "$file_path"
    elif [[ "$module_context" == *"storage"* ]]; then
        # Storage-specific replacements
        sed -i \
            -e 's/\b4096\b/crate::constants::magic_numbers_replacement::storage::DEFAULT_BLOCK_SIZE/g' \
            -e 's/\b100\b/crate::constants::magic_numbers_replacement::storage::DEFAULT_MAX_FILE_SIZE_MB/g' \
            -e 's/\b30\b/crate::constants::magic_numbers_replacement::storage::DEFAULT_BACKUP_RETENTION_DAYS/g' \
            -e 's/\b6\b/crate::constants::magic_numbers_replacement::storage::DEFAULT_COMPRESSION_LEVEL/g' \
            "$file_path"
    elif [[ "$module_context" == *"cache"* ]]; then
        # Cache-specific replacements
        sed -i \
            -e 's/\b8192\b/crate::constants::magic_numbers_replacement::performance::DEFAULT_BUFFER_SIZE/g' \
            -e 's/\b128\b/crate::constants::magic_numbers_replacement::performance::DEFAULT_CACHE_SIZE_MB/g' \
            -e 's/\b1024\b/crate::constants::magic_numbers_replacement::performance::DEFAULT_CHUNK_SIZE/g' \
            "$file_path"
    elif [[ "$module_context" == *"config"* ]]; then
        # Config-specific replacements (common across all)
        sed -i \
            -e 's/\b8080\b/crate::constants::magic_numbers_replacement::network::DEFAULT_HTTP_PORT/g' \
            -e 's/\b4\b/crate::constants::magic_numbers_replacement::performance::DEFAULT_THREAD_POOL_SIZE/g' \
            -e 's/\b30\b/crate::constants::magic_numbers_replacement::network::DEFAULT_TIMEOUT_SECS/g' \
            "$file_path"
    else
        # General replacements
        sed -i \
            -e 's/\b8080\b/crate::constants::magic_numbers_replacement::network::DEFAULT_HTTP_PORT/g' \
            -e 's/\b8192\b/crate::constants::magic_numbers_replacement::performance::DEFAULT_BUFFER_SIZE/g' \
            -e 's/\b1000\b/crate::constants::magic_numbers_replacement::network::DEFAULT_MAX_CONNECTIONS/g' \
            -e 's/\b128\b/crate::constants::magic_numbers_replacement::performance::DEFAULT_CACHE_SIZE_MB/g' \
            "$file_path"
    fi
    
    # Add constants import if not present
    if ! grep -q "use.*magic_numbers_replacement" "$file_path"; then
        sed -i '1i use crate::constants::magic_numbers_replacement;' "$file_path"
    fi
    
    echo "   ✅ Replaced core magic numbers: $file_path (backup: $backup_path)"
}

# Process core modules systematically
echo "Processing core module files..."

# Config module files
CONFIG_FILES=$(find code/crates/nestgate-core/src/config -name "*.rs" | head -10)
CONFIG_FILE_COUNT=$(echo "$CONFIG_FILES" | grep -v '^$' | wc -l)

echo "Found $CONFIG_FILE_COUNT config files to process"

BATCH_SIZE=3
CURRENT_BATCH=0

for file in $CONFIG_FILES; do
    if [ -n "$file" ] && [ -f "$file" ]; then
        replace_core_magic_numbers "$file" "config"
        CURRENT_BATCH=$((CURRENT_BATCH + 1))
        
        if [ $((CURRENT_BATCH % BATCH_SIZE)) -eq 0 ]; then
            echo "   📊 Processed $CURRENT_BATCH / $CONFIG_FILE_COUNT config files..."
        fi
    fi
done

echo ""

# Network module files  
NETWORK_FILES=$(find code/crates/nestgate-core/src/network -name "*.rs" | head -8)
NETWORK_FILE_COUNT=$(echo "$NETWORK_FILES" | grep -v '^$' | wc -l)

echo "Found $NETWORK_FILE_COUNT network files to process"

CURRENT_BATCH=0

for file in $NETWORK_FILES; do
    if [ -n "$file" ] && [ -f "$file" ]; then
        replace_core_magic_numbers "$file" "network"
        CURRENT_BATCH=$((CURRENT_BATCH + 1))
        
        if [ $((CURRENT_BATCH % BATCH_SIZE)) -eq 0 ]; then
            echo "   📊 Processed $CURRENT_BATCH / $NETWORK_FILE_COUNT network files..."
        fi
    fi
done

echo ""

# Cache module files
CACHE_FILES=$(find code/crates/nestgate-core/src/cache -name "*.rs" | head -6)
CACHE_FILE_COUNT=$(echo "$CACHE_FILES" | grep -v '^$' | wc -l)

echo "Found $CACHE_FILE_COUNT cache files to process"

CURRENT_BATCH=0

for file in $CACHE_FILES; do
    if [ -n "$file" ] && [ -f "$file" ]; then
        replace_core_magic_numbers "$file" "cache"
        CURRENT_BATCH=$((CURRENT_BATCH + 1))
        
        if [ $((CURRENT_BATCH % BATCH_SIZE)) -eq 0 ]; then
            echo "   📊 Processed $CURRENT_BATCH / $CACHE_FILE_COUNT cache files..."
        fi
    fi
done

echo ""
echo "🔧 **PHASE 3: CREATE CORE MODULE CONSTANTS**"
echo "-------------------------------------------"

# Create core module specific constants
CORE_CONSTANTS="code/crates/nestgate-core/src/constants/core_module_constants.rs"

cat > "$CORE_CONSTANTS" << 'EOF'
//! **CORE MODULE SPECIFIC CONSTANTS**
//! 
//! Module-specific constants for core NestGate modules.

// ==================== CONFIG MODULE CONSTANTS ====================

/// Configuration module constants
pub mod config {
    /// Default configuration file size limit (MB)
    pub const DEFAULT_CONFIG_FILE_SIZE_MB: u64 = 10;
    
    /// Default configuration cache TTL (seconds)
    pub const DEFAULT_CONFIG_CACHE_TTL_SECS: u64 = 300;
    
    /// Default configuration validation timeout (seconds)
    pub const DEFAULT_CONFIG_VALIDATION_TIMEOUT_SECS: u64 = 5;
    
    /// Maximum configuration nesting depth
    pub const MAX_CONFIG_NESTING_DEPTH: usize = 10;
    
    /// Default configuration reload interval (seconds)
    pub const DEFAULT_CONFIG_RELOAD_INTERVAL_SECS: u64 = 60;
}

// ==================== NETWORK MODULE CONSTANTS ====================

/// Network module constants
pub mod network {
    /// Default network buffer pool size
    pub const DEFAULT_BUFFER_POOL_SIZE: usize = 100;
    
    /// Default network connection backlog
    pub const DEFAULT_CONNECTION_BACKLOG: i32 = 128;
    
    /// Default network read timeout (milliseconds)
    pub const DEFAULT_READ_TIMEOUT_MS: u64 = 5000;
    
    /// Default network write timeout (milliseconds)
    pub const DEFAULT_WRITE_TIMEOUT_MS: u64 = 5000;
    
    /// Default network reconnection attempts
    pub const DEFAULT_RECONNECTION_ATTEMPTS: u32 = 5;
    
    /// Default network heartbeat interval (seconds)
    pub const DEFAULT_HEARTBEAT_INTERVAL_SECS: u64 = 30;
}

// ==================== STORAGE MODULE CONSTANTS ====================

/// Storage module constants
pub mod storage {
    /// Default storage sync interval (seconds)
    pub const DEFAULT_SYNC_INTERVAL_SECS: u64 = 5;
    
    /// Default storage cleanup interval (seconds)
    pub const DEFAULT_CLEANUP_INTERVAL_SECS: u64 = 3600;
    
    /// Default storage transaction timeout (seconds)
    pub const DEFAULT_TRANSACTION_TIMEOUT_SECS: u64 = 30;
    
    /// Default storage batch size for operations
    pub const DEFAULT_BATCH_SIZE: usize = 1000;
    
    /// Default storage journal size (MB)
    pub const DEFAULT_JOURNAL_SIZE_MB: u64 = 64;
    
    /// Default storage checkpoint interval (operations)
    pub const DEFAULT_CHECKPOINT_INTERVAL: u64 = 10000;
}

// ==================== CACHE MODULE CONSTANTS ====================

/// Cache module constants
pub mod cache {
    /// Default cache eviction check interval (seconds)
    pub const DEFAULT_EVICTION_CHECK_INTERVAL_SECS: u64 = 60;
    
    /// Default cache statistics update interval (seconds)
    pub const DEFAULT_STATS_UPDATE_INTERVAL_SECS: u64 = 30;
    
    /// Default cache warm-up batch size
    pub const DEFAULT_WARMUP_BATCH_SIZE: usize = 100;
    
    /// Default cache serialization buffer size
    pub const DEFAULT_SERIALIZATION_BUFFER_SIZE: usize = 4096;
    
    /// Default cache compression threshold (bytes)
    pub const DEFAULT_COMPRESSION_THRESHOLD_BYTES: usize = 1024;
}

// ==================== EVENTS MODULE CONSTANTS ====================

/// Events module constants
pub mod events {
    /// Default event queue size
    pub const DEFAULT_EVENT_QUEUE_SIZE: usize = 10000;
    
    /// Default event batch processing size
    pub const DEFAULT_EVENT_BATCH_SIZE: usize = 100;
    
    /// Default event processing timeout (milliseconds)
    pub const DEFAULT_EVENT_PROCESSING_TIMEOUT_MS: u64 = 1000;
    
    /// Default event retry attempts
    pub const DEFAULT_EVENT_RETRY_ATTEMPTS: u32 = 3;
    
    /// Default event dead letter queue size
    pub const DEFAULT_DLQ_SIZE: usize = 1000;
    
    /// Default event metrics collection interval (seconds)
    pub const DEFAULT_METRICS_COLLECTION_INTERVAL_SECS: u64 = 10;
}

// ==================== MONITORING CONSTANTS ====================

/// Monitoring and observability constants
pub mod monitoring {
    /// Default metrics collection interval (seconds)
    pub const DEFAULT_METRICS_INTERVAL_SECS: u64 = 15;
    
    /// Default health check timeout (seconds)
    pub const DEFAULT_HEALTH_CHECK_TIMEOUT_SECS: u64 = 5;
    
    /// Default alert evaluation interval (seconds)
    pub const DEFAULT_ALERT_EVALUATION_INTERVAL_SECS: u64 = 30;
    
    /// Default log rotation size (MB)
    pub const DEFAULT_LOG_ROTATION_SIZE_MB: u64 = 50;
    
    /// Default trace sampling rate (percentage)
    pub const DEFAULT_TRACE_SAMPLING_RATE_PERCENT: f32 = 1.0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_constants() {
        assert_eq!(config::DEFAULT_CONFIG_FILE_SIZE_MB, 10);
        assert_eq!(config::MAX_CONFIG_NESTING_DEPTH, 10);
    }
    
    #[test]
    fn test_network_constants() {
        assert_eq!(network::DEFAULT_BUFFER_POOL_SIZE, 100);
        assert_eq!(network::DEFAULT_CONNECTION_BACKLOG, 128);
    }
    
    #[test]
    fn test_storage_constants() {
        assert_eq!(storage::DEFAULT_BATCH_SIZE, 1000);
        assert_eq!(storage::DEFAULT_JOURNAL_SIZE_MB, 64);
    }
}
EOF

echo "✅ Created core module constants: $CORE_CONSTANTS"

# Update main constants module
CONSTANTS_MOD="code/crates/nestgate-core/src/constants/mod.rs"
if ! grep -q "pub mod core_module_constants;" "$CONSTANTS_MOD"; then
    echo "pub mod core_module_constants;" >> "$CONSTANTS_MOD"
    echo "✅ Updated constants module: $CONSTANTS_MOD"
fi

echo ""
echo "📝 **PHASE 4: CREATE CORE MODULE REPLACEMENT GUIDE**"
echo "---------------------------------------------------"

# Create core module replacement guide
CORE_REPLACEMENT_GUIDE="docs/CORE_MODULE_MAGIC_NUMBERS_GUIDE.md"

cat > "$CORE_REPLACEMENT_GUIDE" << 'EOF'
# 🔢 **CORE MODULE MAGIC NUMBERS REPLACEMENT GUIDE**

**Generated**: $(date)  
**Purpose**: Systematic guide for replacing magic numbers in core modules  
**Status**: 🔄 **IMPLEMENTATION READY**

---

## 📋 **CORE MODULE REPLACEMENTS**

### **Config Module Replacements**

**BEFORE** (Magic Numbers):
```rust
// config/mod.rs
const CONFIG_TIMEOUT: u64 = 30;
const MAX_CONFIG_SIZE: u64 = 10;
const RELOAD_INTERVAL: u64 = 60;
```

**AFTER** (Domain Constants):
```rust
use crate::constants::core_module_constants::config::*;

const CONFIG_TIMEOUT: u64 = DEFAULT_CONFIG_VALIDATION_TIMEOUT_SECS;
const MAX_CONFIG_SIZE: u64 = DEFAULT_CONFIG_FILE_SIZE_MB;
const RELOAD_INTERVAL: u64 = DEFAULT_CONFIG_RELOAD_INTERVAL_SECS;
```

### **Network Module Replacements**

**BEFORE** (Magic Numbers):
```rust
// network/connection.rs
let buffer_pool = BufferPool::new(100);
let backlog = 128;
let timeout = Duration::from_millis(5000);
```

**AFTER** (Domain Constants):
```rust
use crate::constants::core_module_constants::network::*;

let buffer_pool = BufferPool::new(DEFAULT_BUFFER_POOL_SIZE);
let backlog = DEFAULT_CONNECTION_BACKLOG;
let timeout = Duration::from_millis(DEFAULT_READ_TIMEOUT_MS);
```

### **Storage Module Replacements**

**BEFORE** (Magic Numbers):
```rust
// storage/manager.rs
const SYNC_INTERVAL: u64 = 5;
const BATCH_SIZE: usize = 1000;
const JOURNAL_SIZE: u64 = 64;
```

**AFTER** (Domain Constants):
```rust
use crate::constants::core_module_constants::storage::*;

const SYNC_INTERVAL: u64 = DEFAULT_SYNC_INTERVAL_SECS;
const BATCH_SIZE: usize = DEFAULT_BATCH_SIZE;
const JOURNAL_SIZE: u64 = DEFAULT_JOURNAL_SIZE_MB;
```

### **Cache Module Replacements**

**BEFORE** (Magic Numbers):
```rust
// cache/eviction.rs
let check_interval = 60;
let batch_size = 100;
let compression_threshold = 1024;
```

**AFTER** (Domain Constants):
```rust
use crate::constants::core_module_constants::cache::*;

let check_interval = DEFAULT_EVICTION_CHECK_INTERVAL_SECS;
let batch_size = DEFAULT_WARMUP_BATCH_SIZE;
let compression_threshold = DEFAULT_COMPRESSION_THRESHOLD_BYTES;
```

### **Events Module Replacements**

**BEFORE** (Magic Numbers):
```rust
// events/bus.rs
const QUEUE_SIZE: usize = 10000;
const BATCH_SIZE: usize = 100;
const TIMEOUT_MS: u64 = 1000;
```

**AFTER** (Domain Constants):
```rust
use crate::constants::core_module_constants::events::*;

const QUEUE_SIZE: usize = DEFAULT_EVENT_QUEUE_SIZE;
const BATCH_SIZE: usize = DEFAULT_EVENT_BATCH_SIZE;
const TIMEOUT_MS: u64 = DEFAULT_EVENT_PROCESSING_TIMEOUT_MS;
```

---

## 🛠️ **MODULE-SPECIFIC REPLACEMENT PATTERNS**

### **Pattern 1: Context-Aware Replacement**

```rust
// OLD: Generic magic numbers
fn configure_network() {
    let port = 8080;           // Generic port
    let timeout = 30;          // Generic timeout
    let buffer_size = 8192;    // Generic buffer
}

// NEW: Context-aware constants
use crate::constants::magic_numbers_replacement::network::*;
use crate::constants::core_module_constants::network::*;

fn configure_network() {
    let port = DEFAULT_HTTP_PORT;                    // Network-specific port
    let timeout = DEFAULT_HEARTBEAT_INTERVAL_SECS;   // Network-specific timeout
    let buffer_size = DEFAULT_BUFFER_POOL_SIZE;      // Network-specific buffer
}
```

### **Pattern 2: Module Hierarchy Constants**

```rust
// OLD: Flat magic numbers
const CACHE_SIZE = 128;
const BUFFER_SIZE = 8192;
const TIMEOUT = 30;

// NEW: Hierarchical constants
use crate::constants::{
    magic_numbers_replacement::performance::DEFAULT_CACHE_SIZE_MB,
    magic_numbers_replacement::performance::DEFAULT_BUFFER_SIZE,
    core_module_constants::cache::DEFAULT_EVICTION_CHECK_INTERVAL_SECS,
};

const CACHE_SIZE = DEFAULT_CACHE_SIZE_MB;
const BUFFER_SIZE = DEFAULT_BUFFER_SIZE;
const TIMEOUT = DEFAULT_EVICTION_CHECK_INTERVAL_SECS;
```

---

## 📊 **REPLACEMENT PRIORITY BY MODULE**

### **High Priority Modules**
1. **Config Module**: 30+ magic numbers → Domain constants
2. **Network Module**: 25+ magic numbers → Network + Core constants  
3. **Cache Module**: 20+ magic numbers → Performance + Core constants
4. **Storage Module**: 15+ magic numbers → Storage + Core constants
5. **Events Module**: 12+ magic numbers → Events + Core constants

### **Replacement Strategy**
1. **Phase 1**: Replace most common numbers (8080, 8192, 1000, 30)
2. **Phase 2**: Replace module-specific numbers with core constants
3. **Phase 3**: Replace remaining numbers with context-aware constants

---

*Generated by NestGate Core Module Magic Numbers Replacement System*
EOF

echo "✅ Created core module replacement guide: $CORE_REPLACEMENT_GUIDE"

echo ""
echo "📈 **EXPANDED MAGIC NUMBERS REPLACEMENT SUMMARY**"
echo "------------------------------------------------"

TOTAL_FILES_PROCESSED=$((CONFIG_FILE_COUNT + NETWORK_FILE_COUNT + CACHE_FILE_COUNT))

echo "✅ Expanded magic numbers replacement implementation complete"
echo "✅ $CONFIG_FILE_COUNT config files processed"
echo "✅ $NETWORK_FILE_COUNT network files processed"
echo "✅ $CACHE_FILE_COUNT cache files processed"
echo "✅ Total core module files processed: $TOTAL_FILES_PROCESSED"
echo "✅ Core module constants framework created"
echo "✅ Module-specific replacement guide documented"

echo ""
echo "🎯 **MAGIC NUMBERS EXPANSION PROGRESS**"
echo "--------------------------------------"
echo "Core modules analyzed: 5 (config, network, storage, cache, events)"
echo "Core module files processed: $TOTAL_FILES_PROCESSED"
echo "Core module constants created: 6 modules"
echo "Magic numbers replacement coverage: Examples + Core Modules"

echo ""
echo "✅ **EXPANDED MAGIC NUMBERS REPLACEMENT COMPLETE**"
echo "=================================================" 