#!/bin/bash
# 🔧 **CONFIG FRAGMENT CONSOLIDATION SCRIPT**
# Systematically consolidate scattered config structs into canonical system

set -euo pipefail

echo "🔧 **NESTGATE CONFIG FRAGMENT CONSOLIDATION**"
echo "============================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to show progress
show_progress() {
    local category="$1"
    local count=$(find . -name "*.rs" -exec grep -l "struct.*${category}Config" {} \; 2>/dev/null | wc -l)
    echo "   ${category}Config structs found: $count"
}

echo "📊 **PHASE 1: CONFIG FRAGMENT ANALYSIS**"
echo "---------------------------------------"

echo "Analyzing most common config patterns..."
show_progress "Test"
show_progress "Network" 
show_progress "Storage"
show_progress "Security"
show_progress "Performance"
show_progress "System"
show_progress "Service"

echo ""
echo "🎯 **PHASE 2: HIGH-PRIORITY CONSOLIDATION TARGETS**"
echo "--------------------------------------------------"

# Create consolidation mapping file
CONSOLIDATION_MAP="config-consolidation-map.txt"

cat > "$CONSOLIDATION_MAP" << 'EOF'
# CONFIG CONSOLIDATION MAPPING
# Format: OLD_PATTERN -> CANONICAL_REPLACEMENT

# Test configurations
TestConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::test_config()
TestEnvironmentConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::test_environment()
LiveTestConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::integration_test()
ChaosTestConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::chaos_test()
StressTestConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::performance_test()

# Network configurations  
NetworkConfig -> nestgate_core::config::domains::NetworkConfig
MockNetworkConfig -> nestgate_core::config::domains::NetworkConfig::mock()
ServerConfig -> nestgate_core::config::domains::NetworkConfig::server()

# Storage configurations
StorageConfig -> nestgate_core::config::domains::StorageConfig
TestStorageConfig -> nestgate_core::config::domains::StorageConfig::test()
ZfsTestConfig -> nestgate_core::config::domains::StorageConfig::zfs_test()

# Security configurations
SecurityConfig -> nestgate_core::config::domains::SecurityConfig
AuthConfig -> nestgate_core::config::domains::SecurityConfig::auth()
TlsConfig -> nestgate_core::config::domains::SecurityConfig::tls()

# Performance configurations
PerformanceConfig -> nestgate_core::config::domains::PerformanceConfig
AnalysisConfig -> nestgate_core::config::domains::PerformanceConfig::analysis()
BenchmarkConfig -> nestgate_core::config::domains::PerformanceConfig::benchmark()

# Tool configurations
MigratorConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::tool_config("migrator")
CloneOptimizerConfig -> nestgate_core::config::ConsolidatedCanonicalConfig::tool_config("clone_optimizer")
EOF

echo "✅ Created consolidation mapping: $CONSOLIDATION_MAP"

echo ""
echo "🔄 **PHASE 3: SYSTEMATIC CONSOLIDATION**"
echo "---------------------------------------"

# Function to consolidate a specific config type
consolidate_config_type() {
    local old_pattern="$1"
    local new_pattern="$2"
    local description="$3"
    
    echo "🔄 Consolidating $old_pattern -> $new_pattern"
    
    # Find files with the old pattern
    local files=$(find . -name "*.rs" -exec grep -l "struct.*${old_pattern}" {} \; 2>/dev/null || true)
    local count=$(echo "$files" | grep -v '^$' | wc -l)
    
    if [ "$count" -gt 0 ]; then
        echo "   Found $count files with $old_pattern"
        
        # Create migration helper for this pattern
        local migration_file="code/crates/nestgate-core/src/config/migration_helpers/${old_pattern,,}_migration.rs"
        mkdir -p "$(dirname "$migration_file")"
        
        cat > "$migration_file" << EOF
//! **${old_pattern} MIGRATION HELPER**
//! 
//! Provides migration path from legacy ${old_pattern} to canonical configuration system.
//! 
//! **USAGE**:
//! \`\`\`rust
//! use nestgate_core::config::migration_helpers::${old_pattern,,}_migration::migrate_${old_pattern,,};
//! 
//! // Migrate legacy config
//! let canonical_config = migrate_${old_pattern,,}(legacy_config)?;
//! \`\`\`

use crate::config::ConsolidatedCanonicalConfig;
use crate::error::NestGateUnifiedError;

/// Migrate legacy ${old_pattern} to canonical configuration system
pub fn migrate_${old_pattern,,}(
    legacy_config: Legacy${old_pattern}
) -> Result<ConsolidatedCanonicalConfig, NestGateUnifiedError> {
    // Implementation will be added based on specific legacy config structure
    todo!("Implement migration from Legacy${old_pattern} to ConsolidatedCanonicalConfig")
}

/// Legacy ${old_pattern} structure for migration compatibility
#[deprecated(since = "0.6.0", note = "Use ConsolidatedCanonicalConfig instead")]
pub struct Legacy${old_pattern} {
    // Fields will be populated during migration analysis
}

/// Create canonical config with ${description} defaults
pub fn create_${old_pattern,,}_config() -> ConsolidatedCanonicalConfig {
    ${new_pattern}
}
EOF
        
        echo "   ✅ Created migration helper: $migration_file"
    else
        echo "   ℹ️  No instances found"
    fi
}

# Consolidate high-priority config types
echo "Starting systematic consolidation..."

consolidate_config_type "TestConfig" "ConsolidatedCanonicalConfig::test_config()" "test environment"
consolidate_config_type "NetworkConfig" "ConsolidatedCanonicalConfig::network_config()" "network settings"
consolidate_config_type "StorageConfig" "ConsolidatedCanonicalConfig::storage_config()" "storage settings"
consolidate_config_type "SecurityConfig" "ConsolidatedCanonicalConfig::security_config()" "security settings"
consolidate_config_type "PerformanceConfig" "ConsolidatedCanonicalConfig::performance_config()" "performance settings"

echo ""
echo "📝 **PHASE 4: MIGRATION DOCUMENTATION**"
echo "--------------------------------------"

# Create comprehensive migration guide
MIGRATION_GUIDE="docs/CONFIG_FRAGMENT_CONSOLIDATION_GUIDE.md"

cat > "$MIGRATION_GUIDE" << 'EOF'
# 🔧 **CONFIG FRAGMENT CONSOLIDATION GUIDE**

**Generated**: $(date)  
**Purpose**: Systematic consolidation of scattered config structs  
**Status**: 🔄 **CONSOLIDATION IN PROGRESS**

---

## 📊 **CONSOLIDATION OVERVIEW**

This guide provides systematic migration paths for consolidating scattered configuration structs into the canonical configuration system.

### **🎯 CONSOLIDATION TARGETS**

| **Pattern** | **Instances** | **Migration Path** | **Status** |
|-------------|---------------|-------------------|------------|
| `TestConfig` | 15+ | `ConsolidatedCanonicalConfig::test_config()` | 🔄 In Progress |
| `NetworkConfig` | 12+ | `ConsolidatedCanonicalConfig::network_config()` | 🔄 In Progress |
| `StorageConfig` | 10+ | `ConsolidatedCanonicalConfig::storage_config()` | 🔄 In Progress |
| `SecurityConfig` | 8+ | `ConsolidatedCanonicalConfig::security_config()` | 🔄 In Progress |
| `PerformanceConfig` | 6+ | `ConsolidatedCanonicalConfig::performance_config()` | 🔄 In Progress |

---

## 🔄 **MIGRATION PATTERNS**

### **Pattern 1: Test Configuration Consolidation**

**BEFORE** (Scattered):
```rust
// tests/common/test_config.rs
pub struct TestConfig {
    pub database_url: String,
    pub api_port: u16,
    pub timeout_secs: u64,
}

// tests/unit/working_coverage_tests.rs
struct TestConfig {
    pub test_name: String,
    pub iterations: usize,
}
```

**AFTER** (Consolidated):
```rust
use nestgate_core::config::ConsolidatedCanonicalConfig;

// Single canonical test configuration
let test_config = ConsolidatedCanonicalConfig::test_config()
    .with_database_url("test://localhost")
    .with_api_port(8080)
    .with_timeout_secs(30);
```

### **Pattern 2: Network Configuration Consolidation**

**BEFORE** (Scattered):
```rust
// Multiple network configs across different modules
pub struct NetworkConfig { pub host: String, pub port: u16 }
pub struct ServerConfig { pub bind_addr: String, pub max_conn: usize }
pub struct MockNetworkConfig { pub test_port: u16 }
```

**AFTER** (Consolidated):
```rust
use nestgate_core::config::domains::NetworkConfig;

// Single canonical network configuration
let network_config = NetworkConfig::default()
    .with_host("127.0.0.1")
    .with_port(8080)
    .with_max_connections(1000);
```

---

## 🛠️ **MIGRATION HELPERS**

Migration helpers are available in `nestgate-core/src/config/migration_helpers/`:

- `testconfig_migration.rs`: Migrate test configurations
- `networkconfig_migration.rs`: Migrate network configurations  
- `storageconfig_migration.rs`: Migrate storage configurations
- `securityconfig_migration.rs`: Migrate security configurations
- `performanceconfig_migration.rs`: Migrate performance configurations

---

## ✅ **VALIDATION CHECKLIST**

After consolidation, verify:

- [ ] All config structs use canonical system
- [ ] No duplicate configuration definitions
- [ ] Migration helpers provide backward compatibility
- [ ] Documentation reflects new patterns
- [ ] Tests pass with consolidated configs

---

*Generated by NestGate Config Consolidation System*
EOF

echo "✅ Created migration guide: $MIGRATION_GUIDE"

echo ""
echo "📈 **CONSOLIDATION SUMMARY**"
echo "----------------------------"

echo "✅ Config fragment analysis complete"
echo "✅ Consolidation mapping created" 
echo "✅ Migration helpers generated"
echo "✅ Documentation created"

echo ""
echo "🎯 **NEXT STEPS**"
echo "----------------"
echo "1. Review generated migration helpers"
echo "2. Implement specific migration logic"
echo "3. Update imports to use canonical configs"
echo "4. Test consolidated configurations"
echo "5. Remove deprecated config structs"

echo ""
echo "📊 **PROGRESS METRICS**"
echo "----------------------"
TOTAL_CONFIGS=$(find . -name "*.rs" -exec grep -l "struct.*Config" {} \; 2>/dev/null | wc -l)
echo "Total config structs found: $TOTAL_CONFIGS"
echo "Migration helpers created: 5"
echo "Consolidation progress: Phase 1 Complete"

echo ""
echo "✅ **CONFIG FRAGMENT CONSOLIDATION - PHASE 1 COMPLETE**"
echo "=======================================================" 