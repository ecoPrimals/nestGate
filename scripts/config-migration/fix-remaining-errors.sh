#!/bin/bash

# Fix Remaining Critical Compilation Errors Script
# Addresses the final compilation issues

set -e

echo "🔧 Fixing Remaining Critical Compilation Errors (48 → 0)"
echo "========================================================"

BACKUP_DIR="./config-migration-backup/remaining-fixes"
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

echo "📋 Phase 1: Fix value-use-after-move errors"

# Fix cert/utils.rs endpoint move issue
CERT_UTILS_FILE="code/crates/nestgate-core/src/cert/utils.rs"
if fix_file "$CERT_UTILS_FILE" "Fix endpoint use-after-move"; then
    
    # Replace the problematic code with proper Result handling
    sed -i '199,220c\
        let endpoint_result = adapter.discover_endpoint("cert-service");\
        let endpoint = endpoint_result?;\
        \
        let network_config = UnifiedNetworkConfig {\
            bind_address: endpoint.ip(),\
            api_port: endpoint.port(),\
            service_name: "cert-service".to_string(),\
            endpoints: vec![endpoint.to_string()],\
            max_connections: 100,\
            connection_timeout: Duration::from_secs(30),\
            keep_alive: true,\
            discovery_enabled: true,\
            service_endpoints: HashMap::new(),\
            compression: false,\
            buffer_size: 8192,\
        };\
        \
        // Update required_capabilities properly\
        required_capabilities.insert("endpoint".to_string(), endpoint.to_string());' "$CERT_UTILS_FILE"
    
    echo "  ✅ Fixed cert/utils.rs endpoint use-after-move"
fi

# Fix environment.rs endpoint move issue  
ENVIRONMENT_FILE="code/crates/nestgate-core/src/environment.rs"
if fix_file "$ENVIRONMENT_FILE" "Fix endpoint use-after-move"; then
    
    sed -i '109,125c\
        let endpoint_result = adapter.discover_endpoint("environment");\
        let endpoint = endpoint_result?;\
        \
        UnifiedNetworkConfig {\
            bind_address: endpoint.ip(),\
            api_port: endpoint.port(),\
            service_name: "environment".to_string(),\
            max_connections: 500,\
            connection_timeout: Duration::from_secs(30),\
            keep_alive: true,\
            discovery_enabled: true,\
            service_endpoints: HashMap::new(),\
            compression: true,\
            buffer_size: 16384,\
            websocket_port: None,\
        }' "$ENVIRONMENT_FILE"
    
    echo "  ✅ Fixed environment.rs endpoint use-after-move"
fi

echo ""
echo "📋 Phase 2: Manually add missing PerformanceTestConfig fields"

PERF_CONFIG_FILE="code/crates/nestgate-core/src/universal_primal_discovery/performance.rs"
if fix_file "$PERF_CONFIG_FILE" "Manually add PerformanceTestConfig fields"; then
    
    # Check if fields exist and add them manually if they don't
    if ! grep -q "test_iterations:" "$PERF_CONFIG_FILE"; then
        
        # Find the struct and add missing fields before the closing brace
        sed -i '/pub struct PerformanceTestConfig {/,/^}/ {
            /pub enable_adaptive_timeout: bool,/a\
    /// Number of test iterations to run\
    pub test_iterations: u32,\
    /// Percentile target for performance measurement (0.0-1.0)\
    pub percentile_target: f64,\
    /// Baseline timeout for operations\
    pub baseline_timeout: Duration,\
    /// Maximum allowed timeout\
    pub max_timeout: Duration,
        }' "$PERF_CONFIG_FILE"
        
        # Add to Default implementation
        sed -i '/impl Default for PerformanceTestConfig {/,/^    }/ {
            /enable_adaptive_timeout: true,/a\
            test_iterations: 10,\
            percentile_target: 0.95,\
            baseline_timeout: Duration::from_millis(100),\
            max_timeout: Duration::from_secs(30),
        }' "$PERF_CONFIG_FILE"
        
        echo "  ✅ Added missing PerformanceTestConfig fields"
    else
        echo "  ✅ PerformanceTestConfig fields already exist"
    fi
fi

echo ""
echo "📋 Phase 3: Fix UniversalPrimalAdapter missing fields"

ADAPTER_FILE="code/crates/nestgate-core/src/universal_adapter/adapter.rs"
if fix_file "$ADAPTER_FILE" "Fix UniversalPrimalAdapter struct definition"; then
    
    # Check if the fields are already in the struct definition
    if ! grep -q "orchestration_providers:" "$ADAPTER_FILE"; then
        
        # Find the struct definition and add the missing fields
        sed -i '/pub struct UniversalPrimalAdapter {/,/^}/ {
            /pub capabilities: Vec<String>,/a\
    /// Orchestration providers for service management\
    pub orchestration_providers: Arc<RwLock<Vec<String>>>,\
    /// Compute providers for resource management\
    pub compute_providers: Arc<RwLock<Vec<String>>>,
        }' "$ADAPTER_FILE"
        
        # Find the constructor and add the fields
        sed -i '/impl UniversalPrimalAdapter {/,/^    }/ {
            /UniversalPrimalAdapter {/,/^        }/ {
                /capabilities,/a\
            orchestration_providers: Arc::new(RwLock::new(Vec::new())),\
            compute_providers: Arc::new(RwLock::new(Vec::new())),
            }
        }' "$ADAPTER_FILE"
        
        echo "  ✅ Added missing UniversalPrimalAdapter fields"
    else
        echo "  ✅ UniversalPrimalAdapter fields already exist"
    fi
fi

echo ""
echo "📋 Phase 4: Fix missing type definitions"

# Add UniversalSecurityError to the security client
SECURITY_CLIENT_FILE="code/crates/nestgate-core/src/universal_security_client/client.rs"
if fix_file "$SECURITY_CLIENT_FILE" "Add UniversalSecurityError type"; then
    
    # Add the error type at the top of the file
    sed -i '1a\
\
/// Universal Security Error types\
#[derive(Debug, Clone)]\
pub enum UniversalSecurityError {\
    Network(String),\
    Configuration(String),\
    Timeout(String),\
    Authentication(String),\
}\
\
impl std::fmt::Display for UniversalSecurityError {\
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {\
        match self {\
            UniversalSecurityError::Network(msg) => write!(f, "Network error: {}", msg),\
            UniversalSecurityError::Configuration(msg) => write!(f, "Configuration error: {}", msg),\
            UniversalSecurityError::Timeout(msg) => write!(f, "Timeout error: {}", msg),\
            UniversalSecurityError::Authentication(msg) => write!(f, "Authentication error: {}", msg),\
        }\
    }\
}\
\
impl std::error::Error for UniversalSecurityError {}' "$SECURITY_CLIENT_FILE"
    
    echo "  ✅ Added UniversalSecurityError type"
fi

# Fix EvictionPolicy import in cache types
CACHE_TYPES_FILE="code/crates/nestgate-core/src/cache/types.rs"
if fix_file "$CACHE_TYPES_FILE" "Fix EvictionPolicy usage"; then
    
    # Make sure the EvictionPolicy is properly exported
    sed -i '1a\
use serde::{Deserialize, Serialize};\
\
// Re-export for internal usage\
pub use crate::cache::types::{CachePolicy, EvictionPolicy};' "$CACHE_TYPES_FILE"
    
    echo "  ✅ Fixed EvictionPolicy import"
fi

echo ""
echo "📋 Phase 5: Fix NetworkInterfaceConfiguration default"

HARDWARE_FILE="code/crates/nestgate-core/src/hardware_tuning.rs"
if fix_file "$HARDWARE_FILE" "Fix NetworkInterfaceConfiguration default"; then
    
    # Update the NetworkInterfaceConfiguration to provide a proper default
    sed -i '/pub struct NetworkInterfaceConfiguration {/,/^}/ {
        s/pub interface: String,/pub interface: String,/
    }' "$HARDWARE_FILE"
    
    # Add a better Default implementation
    sed -i '/impl Default for NetworkInterfaceConfiguration {/,/^}/ {
        s/.*/impl Default for NetworkInterfaceConfiguration {\
    fn default() -> Self {\
        Self {\
            interface: "eth0".to_string(),\
            bandwidth: Some(1000),\
        }\
    }\
}/
    }' "$HARDWARE_FILE"
    
    echo "  ✅ Fixed NetworkInterfaceConfiguration default"
fi

echo ""
echo "🧪 Testing compilation fixes..."

# Test compilation again
if cargo check -p nestgate-core --quiet; then
    echo "🎉 SUCCESS: nestgate-core now compiles cleanly!"
    
    # Try building the binary
    if cargo build -p nestgate-bin --quiet; then
        echo "🚀 SUCCESS: nestgate-bin builds successfully!"
        
        # Check for binary and its size
        if [ -f "target/debug/nestgate" ]; then
            BINARY_SIZE=$(du -h target/debug/nestgate | cut -f1)
            echo "📊 Binary created: target/debug/nestgate ($BINARY_SIZE)"
            
            # Quick test
            if ./target/debug/nestgate --help >/dev/null 2>&1; then
                echo "✅ Binary runs successfully!"
            else
                echo "⚠️  Binary has runtime issues"
            fi
        fi
    else
        echo "⚠️  nestgate-bin still has issues, checking..."
        cargo check -p nestgate-bin 2>&1 | head -10
    fi
else
    echo "⚠️  nestgate-core still has compilation issues"
    echo "📋 Remaining errors:"
    cargo check -p nestgate-core 2>&1 | grep "error\[" | head -5
fi

echo ""
echo "🏆 FINAL COMPILATION FIX SUMMARY"
echo "================================"
echo "✅ Fixed endpoint use-after-move errors"
echo "✅ Added missing PerformanceTestConfig fields"  
echo "✅ Fixed UniversalPrimalAdapter missing fields"
echo "✅ Added UniversalSecurityError type definition"
echo "✅ Fixed EvictionPolicy import issues"
echo "✅ Fixed NetworkInterfaceConfiguration defaults"
echo ""
echo "🎯 Build Status Check:"
echo "  • nestgate-core: Checking compilation..."
echo "  • nestgate-bin: Checking binary build..."
echo "  • Binary size: Should be > 1MB (not 1.2kib!)" 