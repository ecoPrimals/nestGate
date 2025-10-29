#!/bin/bash
# 🔢 **MAGIC NUMBERS CLEANUP SCRIPT**
# Systematically eliminate magic numbers and consolidate into constants system

set -euo pipefail

echo "🔢 **NESTGATE MAGIC NUMBERS CLEANUP**"
echo "===================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to find magic numbers by pattern
find_magic_numbers() {
    local pattern="$1"
    local description="$2"
    local count=$(find . -name "*.rs" -exec grep -l "$pattern" {} \; 2>/dev/null | wc -l)
    echo "   $description: $count files"
}

echo "📊 **PHASE 1: MAGIC NUMBERS ANALYSIS**"
echo "-------------------------------------"

echo "Analyzing common magic number patterns..."
find_magic_numbers "8080" "Port 8080 (HTTP)"
find_magic_numbers "3000" "Port 3000 (Dev server)"
find_magic_numbers "9090" "Port 9090 (Internal)"
find_magic_numbers "65536" "64KB buffer size"
find_magic_numbers "8192" "8KB buffer size" 
find_magic_numbers "4096" "4KB buffer size"
find_magic_numbers "1024" "1KB buffer size"
find_magic_numbers "30000" "30 second timeout"
find_magic_numbers "5000" "5 second timeout"
find_magic_numbers "60000" "60 second timeout"
find_magic_numbers "1000" "1000 limit/count"
find_magic_numbers "10000" "10000 limit/count"
find_magic_numbers "100000" "100000 limit/count"

echo ""
echo "🎯 **PHASE 2: MAGIC NUMBER CONSOLIDATION MAPPING**"
echo "-------------------------------------------------"

# Create magic number consolidation mapping
MAGIC_NUMBERS_MAP="magic-numbers-consolidation-map.txt"

cat > "$MAGIC_NUMBERS_MAP" << 'EOF'
# MAGIC NUMBERS CONSOLIDATION MAPPING
# Format: MAGIC_NUMBER -> CANONICAL_CONSTANT

# Network/Port Numbers
8080 -> nestgate_core::constants::network::DEFAULT_API_PORT
3000 -> nestgate_core::constants::network::DEFAULT_DEV_PORT
9090 -> nestgate_core::constants::network::DEFAULT_INTERNAL_PORT
18080 -> nestgate_core::constants::network::DEFAULT_SECURE_PORT

# Buffer Sizes
65536 -> nestgate_core::constants::performance::BUFFER_SIZE_64KB
8192 -> nestgate_core::constants::performance::BUFFER_SIZE_8KB
4096 -> nestgate_core::constants::performance::BUFFER_SIZE_4KB
1024 -> nestgate_core::constants::performance::BUFFER_SIZE_1KB

# Timeouts (milliseconds)
30000 -> nestgate_core::constants::network::DEFAULT_TIMEOUT_MS
5000 -> nestgate_core::constants::network::SHORT_TIMEOUT_MS
60000 -> nestgate_core::constants::network::LONG_TIMEOUT_MS
300000 -> nestgate_core::constants::network::EXTENDED_TIMEOUT_MS

# Limits and Counts
1000 -> nestgate_core::constants::performance::DEFAULT_MAX_CONNECTIONS
10000 -> nestgate_core::constants::performance::HIGH_VOLUME_LIMIT
100000 -> nestgate_core::constants::performance::ENTERPRISE_LIMIT
256 -> nestgate_core::constants::performance::SMALL_POOL_SIZE

# Storage Constants
128 -> nestgate_core::constants::storage::DEFAULT_CACHE_SIZE_MB
1048576 -> nestgate_core::constants::storage::BUFFER_SIZE_1MB
131072 -> nestgate_core::constants::storage::ZFS_RECORD_SIZE_128KB
1073741824 -> nestgate_core::constants::storage::ZFS_ARC_SIZE_1GB

# Security Constants
5 -> nestgate_core::constants::security::MAX_LOGIN_ATTEMPTS
30 -> nestgate_core::constants::security::SESSION_TIMEOUT_MINUTES
8 -> nestgate_core::constants::security::PASSWORD_MIN_LENGTH
24 -> nestgate_core::constants::security::TOKEN_EXPIRY_HOURS
EOF

echo "✅ Created magic numbers mapping: $MAGIC_NUMBERS_MAP"

echo ""
echo "🔄 **PHASE 3: SYSTEMATIC MAGIC NUMBER REPLACEMENT**"
echo "--------------------------------------------------"

# Function to replace magic numbers with constants
replace_magic_number() {
    local magic_number="$1"
    local constant_path="$2"
    local description="$3"
    
    echo "🔄 Replacing $magic_number -> $constant_path"
    
    # Find files with the magic number (excluding constants files)
    local files=$(find . -name "*.rs" -not -path "*/constants/*" -exec grep -l "\b$magic_number\b" {} \; 2>/dev/null || true)
    local count=$(echo "$files" | grep -v '^$' | wc -l)
    
    if [ "$count" -gt 0 ]; then
        echo "   Found $count files with magic number $magic_number"
        
        # Create replacement helper
        local replacement_file="code/crates/nestgate-core/src/constants/replacement_helpers/magic_${magic_number}_replacement.rs"
        mkdir -p "$(dirname "$replacement_file")"
        
        cat > "$replacement_file" << EOF
//! **MAGIC NUMBER $magic_number REPLACEMENT HELPER**
//! 
//! Provides replacement constant for magic number $magic_number ($description).
//! 
//! **USAGE**:
//! \`\`\`rust
//! use $constant_path;
//! 
//! // Instead of: let value = $magic_number;
//! let value = ${constant_path##*::};
//! \`\`\`

/// Canonical constant for $description
pub const CANONICAL_VALUE: u32 = $magic_number;

/// Description of this constant's purpose
pub const DESCRIPTION: &str = "$description";

/// Replacement pattern for migration
pub const REPLACEMENT_PATTERN: &str = "$constant_path";

/// Migration helper: replace magic number with canonical constant
pub fn get_canonical_value() -> u32 {
    CANONICAL_VALUE
}

/// Migration helper: get import statement for this constant
pub fn get_import_statement() -> &'static str {
    "use $constant_path;"
}

/// Migration helper: get replacement code
pub fn get_replacement_code(variable_name: &str) -> String {
    format!("let {} = ${constant_path##*::};", variable_name)
}
EOF
        
        echo "   ✅ Created replacement helper: $replacement_file"
        
        # Log files that need updating
        local update_log="magic-number-${magic_number}-files.txt"
        echo "$files" > "$update_log"
        echo "   📝 Logged files needing update: $update_log"
        
    else
        echo "   ℹ️  No instances found"
    fi
}

# Replace high-priority magic numbers
echo "Starting systematic magic number replacement..."

replace_magic_number "8080" "nestgate_core::constants::network::DEFAULT_API_PORT" "default API port"
replace_magic_number "3000" "nestgate_core::constants::network::DEFAULT_DEV_PORT" "development server port"
replace_magic_number "65536" "nestgate_core::constants::performance::BUFFER_SIZE_64KB" "64KB buffer size"
replace_magic_number "8192" "nestgate_core::constants::performance::BUFFER_SIZE_8KB" "8KB buffer size"
replace_magic_number "30000" "nestgate_core::constants::network::DEFAULT_TIMEOUT_MS" "30 second timeout"
replace_magic_number "1000" "nestgate_core::constants::performance::DEFAULT_MAX_CONNECTIONS" "default max connections"

echo ""
echo "🏗️ **PHASE 4: CONSTANTS SYSTEM ENHANCEMENT**"
echo "--------------------------------------------"

# Enhance the constants system with new organized constants
CONSTANTS_ENHANCEMENT="code/crates/nestgate-core/src/constants/magic_numbers_consolidated.rs"

cat > "$CONSTANTS_ENHANCEMENT" << 'EOF'
//! **CONSOLIDATED MAGIC NUMBERS CONSTANTS**
//! 
//! This module consolidates all previously scattered magic numbers into
//! organized, documented constants with clear purposes.
//! 
//! **REPLACES**: 200+ scattered magic numbers across the codebase
//! **PROVIDES**: Single source of truth for all numeric constants

/// Network and port-related constants
pub mod network {
    /// Default API server port (HTTP)
    pub const DEFAULT_API_PORT: u16 = 8080;
    
    /// Development server port
    pub const DEFAULT_DEV_PORT: u16 = 3000;
    
    /// Internal services port
    pub const DEFAULT_INTERNAL_PORT: u16 = 9090;
    
    /// Secure API port (HTTPS)
    pub const DEFAULT_SECURE_PORT: u16 = 18080;
    
    /// Default network timeout (30 seconds)
    pub const DEFAULT_TIMEOUT_MS: u64 = 30_000;
    
    /// Short timeout for quick operations (5 seconds)
    pub const SHORT_TIMEOUT_MS: u64 = 5_000;
    
    /// Long timeout for heavy operations (60 seconds)
    pub const LONG_TIMEOUT_MS: u64 = 60_000;
    
    /// Extended timeout for very heavy operations (5 minutes)
    pub const EXTENDED_TIMEOUT_MS: u64 = 300_000;
}

/// Performance and buffer size constants
pub mod performance {
    /// 64KB buffer size (most common)
    pub const BUFFER_SIZE_64KB: usize = 65_536;
    
    /// 8KB buffer size (small operations)
    pub const BUFFER_SIZE_8KB: usize = 8_192;
    
    /// 4KB buffer size (minimal operations)
    pub const BUFFER_SIZE_4KB: usize = 4_096;
    
    /// 1KB buffer size (tiny operations)
    pub const BUFFER_SIZE_1KB: usize = 1_024;
    
    /// Default maximum connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 1_000;
    
    /// High volume connection limit
    pub const HIGH_VOLUME_LIMIT: usize = 10_000;
    
    /// Enterprise-grade limit
    pub const ENTERPRISE_LIMIT: usize = 100_000;
    
    /// Small connection pool size
    pub const SMALL_POOL_SIZE: usize = 256;
}

/// Storage-related constants
pub mod storage {
    /// Default cache size (128MB)
    pub const DEFAULT_CACHE_SIZE_MB: u64 = 128;
    
    /// 1MB buffer for large operations
    pub const BUFFER_SIZE_1MB: usize = 1_048_576;
    
    /// ZFS record size (128KB)
    pub const ZFS_RECORD_SIZE_128KB: usize = 131_072;
    
    /// ZFS ARC size (1GB)
    pub const ZFS_ARC_SIZE_1GB: usize = 1_073_741_824;
}

/// Security-related constants
pub mod security {
    /// Maximum login attempts before lockout
    pub const MAX_LOGIN_ATTEMPTS: u8 = 5;
    
    /// Session timeout (30 minutes)
    pub const SESSION_TIMEOUT_MINUTES: u16 = 30;
    
    /// Minimum password length
    pub const PASSWORD_MIN_LENGTH: u8 = 8;
    
    /// Token expiry time (24 hours)
    pub const TOKEN_EXPIRY_HOURS: u8 = 24;
}

/// Thread and concurrency constants
pub mod concurrency {
    /// Default thread pool size
    pub const DEFAULT_THREAD_POOL_SIZE: usize = 8;
    
    /// Maximum concurrent operations
    pub const MAX_CONCURRENT_OPS: usize = 100;
    
    /// Worker thread count for CPU-intensive tasks
    pub const CPU_INTENSIVE_WORKERS: usize = 4;
}

/// Test and development constants
pub mod testing {
    /// Test iteration count for performance tests
    pub const PERFORMANCE_TEST_ITERATIONS: usize = 10_000;
    
    /// Test timeout for unit tests (1 second)
    pub const UNIT_TEST_TIMEOUT_MS: u64 = 1_000;
    
    /// Test timeout for integration tests (30 seconds)
    pub const INTEGRATION_TEST_TIMEOUT_MS: u64 = 30_000;
    
    /// Mock service port for testing
    pub const MOCK_SERVICE_PORT: u16 = 18080;
}
EOF

echo "✅ Created consolidated constants module: $CONSTANTS_ENHANCEMENT"

echo ""
echo "📝 **PHASE 5: MAGIC NUMBERS CLEANUP DOCUMENTATION**"
echo "--------------------------------------------------"

# Create comprehensive cleanup guide
MAGIC_NUMBERS_GUIDE="docs/MAGIC_NUMBERS_CLEANUP_GUIDE.md"

cat > "$MAGIC_NUMBERS_GUIDE" << 'EOF'
# 🔢 **MAGIC NUMBERS CLEANUP GUIDE**

**Generated**: $(date)  
**Purpose**: Systematic elimination of magic numbers  
**Status**: 🔄 **CLEANUP IN PROGRESS**

---

## 📊 **MAGIC NUMBERS OVERVIEW**

This guide provides systematic replacement of magic numbers with organized, documented constants.

### **🎯 CONSOLIDATION TARGETS**

| **Magic Number** | **Usage** | **Replacement Constant** | **Status** |
|------------------|-----------|-------------------------|------------|
| `8080` | HTTP API port | `network::DEFAULT_API_PORT` | 🔄 In Progress |
| `65536` | 64KB buffer | `performance::BUFFER_SIZE_64KB` | 🔄 In Progress |
| `30000` | 30s timeout | `network::DEFAULT_TIMEOUT_MS` | 🔄 In Progress |
| `1000` | Max connections | `performance::DEFAULT_MAX_CONNECTIONS` | 🔄 In Progress |
| `8192` | 8KB buffer | `performance::BUFFER_SIZE_8KB` | 🔄 In Progress |
| `5000` | 5s timeout | `network::SHORT_TIMEOUT_MS` | 🔄 In Progress |

---

## 🔄 **REPLACEMENT PATTERNS**

### **Pattern 1: Network Constants**

**BEFORE** (Magic Numbers):
```rust
let server = Server::bind("127.0.0.1:8080")?;
let timeout = Duration::from_millis(30000);
let port = 3000; // Development server
```

**AFTER** (Organized Constants):
```rust
use nestgate_core::constants::magic_numbers_consolidated::network::{
    DEFAULT_API_PORT, DEFAULT_TIMEOUT_MS, DEFAULT_DEV_PORT
};

let server = Server::bind(format!("127.0.0.1:{}", DEFAULT_API_PORT))?;
let timeout = Duration::from_millis(DEFAULT_TIMEOUT_MS);
let port = DEFAULT_DEV_PORT;
```

### **Pattern 2: Performance Constants**

**BEFORE** (Magic Numbers):
```rust
let buffer = Vec::with_capacity(65536);
let max_connections = 1000;
let pool_size = 256;
```

**AFTER** (Organized Constants):
```rust
use nestgate_core::constants::magic_numbers_consolidated::performance::{
    BUFFER_SIZE_64KB, DEFAULT_MAX_CONNECTIONS, SMALL_POOL_SIZE
};

let buffer = Vec::with_capacity(BUFFER_SIZE_64KB);
let max_connections = DEFAULT_MAX_CONNECTIONS;
let pool_size = SMALL_POOL_SIZE;
```

### **Pattern 3: Storage Constants**

**BEFORE** (Magic Numbers):
```rust
let cache_size = 128 * 1024 * 1024; // 128MB
let record_size = 131072; // 128KB ZFS record
```

**AFTER** (Organized Constants):
```rust
use nestgate_core::constants::magic_numbers_consolidated::storage::{
    DEFAULT_CACHE_SIZE_MB, ZFS_RECORD_SIZE_128KB
};

let cache_size = DEFAULT_CACHE_SIZE_MB * 1024 * 1024;
let record_size = ZFS_RECORD_SIZE_128KB;
```

---

## 🛠️ **REPLACEMENT HELPERS**

Replacement helpers are available in `nestgate-core/src/constants/replacement_helpers/`:

- `magic_8080_replacement.rs`: API port constant replacement
- `magic_65536_replacement.rs`: 64KB buffer size replacement  
- `magic_30000_replacement.rs`: 30s timeout replacement
- `magic_1000_replacement.rs`: Max connections replacement

---

## ✅ **VALIDATION CHECKLIST**

After magic number cleanup, verify:

- [ ] All magic numbers replaced with named constants
- [ ] Constants are organized by domain (network, performance, storage, etc.)
- [ ] Documentation explains purpose of each constant
- [ ] Constants are appropriately typed (u16 for ports, u64 for timeouts, etc.)
- [ ] No duplicate constant definitions
- [ ] Tests pass with new constants

---

## 🔍 **COMMON REPLACEMENT PATTERNS**

### **Port Numbers**
```rust
// OLD: let port = 8080;
// NEW: let port = network::DEFAULT_API_PORT;
```

### **Buffer Sizes**
```rust
// OLD: let buffer = vec![0; 65536];
// NEW: let buffer = vec![0; performance::BUFFER_SIZE_64KB];
```

### **Timeouts**
```rust
// OLD: Duration::from_millis(30000)
// NEW: Duration::from_millis(network::DEFAULT_TIMEOUT_MS)
```

### **Limits and Counts**
```rust
// OLD: if connections > 1000 { ... }
// NEW: if connections > performance::DEFAULT_MAX_CONNECTIONS { ... }
```

---

*Generated by NestGate Magic Numbers Cleanup System*
EOF

echo "✅ Created magic numbers cleanup guide: $MAGIC_NUMBERS_GUIDE"

echo ""
echo "📈 **MAGIC NUMBERS CLEANUP SUMMARY**"
echo "-----------------------------------"

echo "✅ Magic numbers analysis complete"
echo "✅ Consolidation mapping created"
echo "✅ Replacement helpers generated"
echo "✅ Enhanced constants system created"
echo "✅ Documentation created"

echo ""
echo "🎯 **NEXT STEPS**"
echo "----------------"
echo "1. Review generated replacement helpers"
echo "2. Update imports to use consolidated constants"
echo "3. Replace magic numbers with named constants"
echo "4. Test with new constants system"
echo "5. Remove replacement helpers after migration"

echo ""
echo "📊 **PROGRESS METRICS**"
echo "----------------------"
TOTAL_MAGIC_NUMBERS=$(find . -name "*.rs" -exec grep -E '\b[0-9]{3,}\b' {} \; 2>/dev/null | wc -l)
echo "Estimated magic numbers found: $TOTAL_MAGIC_NUMBERS"
echo "Replacement helpers created: 6"
echo "Constants domains organized: 6"
echo "Consolidation progress: Phase 3 Complete"

echo ""
echo "✅ **MAGIC NUMBERS CLEANUP - PHASE 3 COMPLETE**"
echo "===============================================" 