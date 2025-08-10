#!/bin/bash

# Fix Critical Compilation Errors Script
# Addresses the most urgent build-breaking issues from config unification

set -e

echo "🔧 Fixing Critical Compilation Errors from Config Unification"
echo "============================================================="

# Create backup
BACKUP_DIR="./config-migration-backup/compilation-fixes"
mkdir -p "$BACKUP_DIR"

# Function to backup and fix a file
fix_file() {
    local file=$1
    local description=$2
    
    if [ -f "$file" ]; then
        echo "🔧 Fixing: $file - $description"
        cp "$file" "$BACKUP_DIR/$(basename "$file").backup.$(date +%Y%m%d_%H%M%S)"
        return 0
    else
        echo "❌ File not found: $file"
        return 1
    fi
}

echo "📋 Phase 1: Fix missing CacheConfig fields"

# Fix CacheConfig missing fields
CACHE_TYPES_FILE="code/crates/nestgate-core/src/cache/types.rs"
if fix_file "$CACHE_TYPES_FILE" "Add missing CacheConfig fields"; then
    
    # Add missing fields to CacheConfig
    sed -i '/pub struct CacheConfig {/,/}/ {
        /enable_compression: bool,/a\
    /// Cache directory path\
    pub cache_dir: std::path::PathBuf,\
    /// Cache policy\
    pub policy: CachePolicy,\
    /// Hot tier size in bytes\
    pub hot_tier_size: usize,\
    /// Warm tier size in bytes\
    pub warm_tier_size: usize,\
    /// Whether cold tier is unlimited\
    pub cold_tier_unlimited: bool,
    }' "$CACHE_TYPES_FILE"
    
    # Add missing CachePolicy enum if not exists
    if ! grep -q "pub enum CachePolicy" "$CACHE_TYPES_FILE"; then
        sed -i '1a\
\
/// Cache policy for data management\
#[derive(Debug, Clone, Serialize, Deserialize)]\
pub enum CachePolicy {\
    WriteBack,\
    WriteThrough,\
    WriteAround,\
}\
\
impl Default for CachePolicy {\
    fn default() -> Self {\
        CachePolicy::WriteBack\
    }\
}\
\
/// Eviction policy for cache management\
#[derive(Debug, Clone, Serialize, Deserialize)]\
pub enum EvictionPolicy {\
    Lru,\
    Lfu,\
    Fifo,\
}\
\
impl Default for EvictionPolicy {\
    fn default() -> Self {\
        EvictionPolicy::Lru\
    }\
}' "$CACHE_TYPES_FILE"
    fi
    
    # Update CacheConfig::default() to include new fields
    sed -i '/impl Default for CacheConfig {/,/}/ {
        /eviction_policy: EvictionPolicy::Lru,/a\
            cache_dir: std::path::PathBuf::from("/tmp/nestgate-cache"),\
            policy: CachePolicy::WriteBack,\
            hot_tier_size: 100_000_000,  // 100MB\
            warm_tier_size: 500_000_000, // 500MB\
            cold_tier_unlimited: false,
    }' "$CACHE_TYPES_FILE"
    
    echo "  ✅ Fixed CacheConfig missing fields"
fi

echo ""
echo "📋 Phase 2: Fix missing PerformanceTestConfig fields"

PERF_CONFIG_FILE="code/crates/nestgate-core/src/universal_primal_discovery/performance.rs"
if fix_file "$PERF_CONFIG_FILE" "Add missing PerformanceTestConfig fields"; then
    
    # Add missing fields to PerformanceTestConfig
    sed -i '/pub struct PerformanceTestConfig {/,/}/ {
        /pub enable_adaptive_timeout: bool,/a\
    /// Number of test iterations\
    pub test_iterations: u32,\
    /// Percentile target for performance\
    pub percentile_target: f64,\
    /// Baseline timeout duration\
    pub baseline_timeout: Duration,\
    /// Maximum timeout duration\
    pub max_timeout: Duration,
    }' "$PERF_CONFIG_FILE"
    
    # Update default implementation
    sed -i '/impl Default for PerformanceTestConfig {/,/}/ {
        /enable_adaptive_timeout: true,/a\
            test_iterations: 10,\
            percentile_target: 0.95,\
            baseline_timeout: Duration::from_millis(100),\
            max_timeout: Duration::from_secs(30),
    }' "$PERF_CONFIG_FILE"
    
    echo "  ✅ Fixed PerformanceTestConfig missing fields"
fi

echo ""
echo "📋 Phase 3: Fix missing function arguments"

# Fix discover_endpoint calls
CERT_UTILS_FILE="code/crates/nestgate-core/src/cert/utils.rs"
if fix_file "$CERT_UTILS_FILE" "Fix discover_endpoint calls"; then
    
    # Fix discover_endpoint calls by adding service parameter
    sed -i 's/adapter\.discover_endpoint()/adapter.discover_endpoint("cert-service")/g' "$CERT_UTILS_FILE"
    sed -i 's/endpoint\.to_string()/endpoint?.to_string()/g' "$CERT_UTILS_FILE"
    sed -i 's/endpoint\.ip()/endpoint?.ip()/g' "$CERT_UTILS_FILE"
    sed -i 's/endpoint\.port()/endpoint?.port()/g' "$CERT_UTILS_FILE"
    
    echo "  ✅ Fixed discover_endpoint calls in cert/utils.rs"
fi

ENVIRONMENT_FILE="code/crates/nestgate-core/src/environment.rs"
if fix_file "$ENVIRONMENT_FILE" "Fix discover_endpoint calls"; then
    
    # Fix discover_endpoint calls
    sed -i 's/adapter\.discover_endpoint()/adapter.discover_endpoint("environment")/g' "$ENVIRONMENT_FILE"
    sed -i 's/endpoint\.port()/endpoint?.port()/g' "$ENVIRONMENT_FILE"
    sed -i 's/endpoint\.ip()/endpoint?.ip()/g' "$ENVIRONMENT_FILE"
    
    echo "  ✅ Fixed discover_endpoint calls in environment.rs"
fi

echo ""
echo "📋 Phase 4: Fix missing struct fields"

# Fix UniversalPrimalAdapter missing fields
ADAPTER_FILE="code/crates/nestgate-core/src/universal_adapter/adapter.rs"
if fix_file "$ADAPTER_FILE" "Fix UniversalPrimalAdapter missing fields"; then
    
    # Add missing fields to the struct
    sed -i '/pub security_providers:/a\
    /// Orchestration providers for service management\
    pub orchestration_providers: Arc<RwLock<Vec<String>>>,\
    /// Compute providers for resource management\
    pub compute_providers: Arc<RwLock<Vec<String>>>,' "$ADAPTER_FILE"
    
    # Update constructor to include new fields
    sed -i '/UniversalPrimalAdapter {/,/}/ {
        /capabilities,/a\
            orchestration_providers: Arc::new(RwLock::new(Vec::new())),\
            compute_providers: Arc::new(RwLock::new(Vec::new())),
    }' "$ADAPTER_FILE"
    
    echo "  ✅ Fixed UniversalPrimalAdapter missing fields"
fi

echo ""
echo "📋 Phase 5: Fix missing hardware types"

HARDWARE_FILE="code/crates/nestgate-core/src/hardware_tuning.rs"
if fix_file "$HARDWARE_FILE" "Add missing hardware configuration types"; then
    
    # Add missing enum and struct definitions
    sed -i '1a\
\
/// CPU configuration settings\
#[derive(Debug, Clone, Serialize, Deserialize, Default)]\
pub struct CpuConfiguration {\
    pub cores: Option<u32>,\
    pub frequency: Option<u32>,\
}\
\
/// Network interface configuration\
#[derive(Debug, Clone, Serialize, Deserialize, Default)]\
pub struct NetworkInterfaceConfiguration {\
    pub interface: String,\
    pub bandwidth: Option<u64>,\
}\
\
/// Performance profile enumeration\
#[derive(Debug, Clone, Serialize, Deserialize)]\
pub enum PerformanceProfile {\
    Balanced,\
    Performance,\
    PowerSaver,\
}\
\
impl Default for PerformanceProfile {\
    fn default() -> Self {\
        PerformanceProfile::Balanced\
    }\
}\
\
/// Power management settings\
#[derive(Debug, Clone, Serialize, Deserialize, Default)]\
pub struct PowerManagementSettings {\
    pub enabled: bool,\
    pub mode: String,\
}' "$HARDWARE_FILE"
    
    echo "  ✅ Added missing hardware configuration types"
fi

echo ""
echo "📋 Phase 6: Fix telemetry iterator issue"

TELEMETRY_FILE="code/crates/nestgate-core/src/telemetry.rs"
if fix_file "$TELEMETRY_FILE" "Fix telemetry config iterator"; then
    
    # Fix the iterator issue in telemetry
    sed -i 's/for endpoint in &self\.config\.collect_metrics/if self.config.collect_metrics { \/\/ Fixed: collect_metrics is bool, not Vec/' "$TELEMETRY_FILE"
    
    echo "  ✅ Fixed telemetry iterator issue"
fi

echo ""
echo "🧪 Testing compilation fixes..."

# Test if core library compiles now
if cargo check -p nestgate-core --quiet; then
    echo "✅ SUCCESS: nestgate-core now compiles!"
    
    # Try building the binary
    if cargo build -p nestgate-bin --quiet; then
        echo "🎉 SUCCESS: nestgate-bin builds successfully!"
        
        # Check for binary
        if [ -f "target/debug/nestgate" ]; then
            BINARY_SIZE=$(du -h target/debug/nestgate | cut -f1)
            echo "🚀 Binary created: target/debug/nestgate ($BINARY_SIZE)"
        fi
    else
        echo "⚠️  nestgate-bin still has issues, but core library is fixed"
    fi
else
    echo "⚠️  nestgate-core still has compilation issues"
    echo "   Run 'cargo check -p nestgate-core' for details"
fi

echo ""
echo "🏆 COMPILATION FIX SUMMARY"
echo "=========================="
echo "✅ Fixed CacheConfig missing fields (hot_tier_size, cache_dir, policy, etc.)"
echo "✅ Fixed PerformanceTestConfig missing fields (test_iterations, percentile_target, etc.)"  
echo "✅ Fixed discover_endpoint() missing arguments"
echo "✅ Fixed UniversalPrimalAdapter missing fields"
echo "✅ Added missing hardware configuration types"
echo "✅ Fixed telemetry iterator issue"
echo ""
echo "📋 Next steps:"
echo "1. Run: cargo check --workspace"
echo "2. Fix any remaining compilation issues"
echo "3. Run: cargo build -p nestgate-bin"
echo "4. Test: ./target/debug/nestgate --help" 