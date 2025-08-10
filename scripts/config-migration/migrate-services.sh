#!/bin/bash

# Migrate Service Configs Script
# Migrates service-specific configuration structs to unified types

set -e

echo "⚙️  Migrating Service-Specific Configurations to Unified Types"
echo "=============================================================="

# Configuration
BACKUP_DIR="./config-migration-backup"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Service config locations and their configs to migrate
declare -A SERVICE_CONFIGS=(
    # MCP Configs (nestgate-mcp crate)
    ["code/crates/nestgate-mcp/src/types.rs"]="AuthConfig=UnifiedSecurityConfig ProviderConfig=UnifiedServiceConfig PerformanceConfig=UnifiedMonitoringConfig QosConfig=UnifiedMonitoringConfig RateLimitConfig=UnifiedSecurityConfig"
    ["code/crates/nestgate-mcp/src/storage.rs"]="VolumeConfig=UnifiedConfig"
    ["code/crates/nestgate-mcp/src/security.rs"]="SecurityConfig=UnifiedSecurityConfig"
    ["code/crates/nestgate-mcp/src/adapter.rs"]="AdapterConfig=UnifiedConfig"
    ["code/crates/nestgate-mcp/src/config.rs"]="McpConfig=UnifiedConfig"
    ["code/crates/nestgate-mcp/src/lib.rs"]="McpConfig=UnifiedConfig EnhancedMcpConfig=UnifiedConfig RetryConfig=UnifiedConfig"
    
    # Service Configs (various crates)
    ["code/crates/nestgate-nas/src/config.rs"]="NasConfig=UnifiedConfig ShareConfig=UnifiedConfig"
    ["code/crates/nestgate-nas/src/lib.rs"]="NasConfig=UnifiedConfig NasServerConfig=UnifiedConfig"
    ["code/crates/nestgate-fsmonitor/src/config.rs"]="FileSystemConfig=UnifiedConfig"
    ["code/crates/nestgate-fsmonitor/src/lib.rs"]="FsMonitorConfig=UnifiedConfig"
    ["code/crates/nestgate-automation/src/types/config.rs"]="AutomationConfig=UnifiedConfig DiscoveryConfig=UnifiedConfig"
    ["code/crates/nestgate-automation/src/manager.rs"]="AutomationConfig=UnifiedConfig"
    ["code/crates/nestgate-automation/src/lifecycle.rs"]="HealthCheckConfig=UnifiedMonitoringConfig LifecycleConfig=UnifiedConfig"
    ["code/crates/nestgate-automation/src/connections.rs"]="TimeoutConfig=UnifiedConfig RetryConfig=UnifiedConfig TlsConfig=UnifiedSecurityConfig ConnectionConfig=UnifiedNetworkConfig"
    ["code/crates/nestgate-installer/src/config.rs"]="InstallerConfig=UnifiedConfig"
    ["code/crates/nestgate-middleware/src/lib.rs"]="MiddlewareConfig=UnifiedConfig"
    ["code/crates/nestgate-ui/src/config.rs"]="UiConfig=UnifiedConfig NotificationConfig=UnifiedConfig"
    
    # ZFS Configs
    ["code/crates/nestgate-zfs/src/config.rs"]="ZfsConfig=UnifiedConfig"
    ["code/crates/nestgate-zfs/src/mcp_integration.rs"]="ZfsMcpConfig=UnifiedConfig TierConfig=UnifiedConfig"
    ["code/crates/nestgate-zfs/src/failover.rs"]="FailoverConfig=UnifiedConfig"
    
    # Network Configs
    ["code/crates/nestgate-network/src/lib.rs"]="NetworkConfig=UnifiedNetworkConfig"
    ["code/crates/nestgate-network/src/protocol.rs"]="ProtocolConfig=UnifiedNetworkConfig"
)

# Function to add unified conversion methods to a config struct
add_unified_conversion() {
    local file_path=$1
    local struct_name=$2
    local unified_type=$3
    
    echo "🔄 Adding unified conversion to $struct_name in $(basename "$file_path")"
    
    # Check if file exists
    if [ ! -f "$file_path" ]; then
        echo "  ❌ File not found: $file_path"
        return 1
    fi
    
    # Backup original file
    cp "$file_path" "$BACKUP_DIR/$(basename "$file_path").backup.$(date +%Y%m%d_%H%M%S)"
    
    # Add migration methods to the file
    cat >> "$file_path" << EOF

// 🚀 ECOSYSTEM UNIFICATION: Migration methods for $struct_name
#[allow(dead_code)]
impl $struct_name {
    /// Convert to unified configuration type
    /// **MIGRATION PATH**: Use this method to transition to unified types
    pub fn to_unified(&self) -> $unified_type {
        // TODO: Implement specific conversion logic for $struct_name
        // This is a placeholder - actual implementation should map fields appropriately
        $unified_type::default()
    }
    
    /// Create from unified configuration type
    /// **MIGRATION PATH**: Use this method when receiving unified configs
    pub fn from_unified(config: &$unified_type) -> Self {
        // TODO: Implement specific conversion logic from unified type
        // This is a placeholder - actual implementation should extract relevant fields
        Default::default()
    }
}

/// **MODERN TYPE ALIAS**: Use this type for new code instead of $struct_name
/// 🚀 ECOSYSTEM UNIFICATION: Future-proof type alias
pub type Modern$struct_name = $unified_type;
EOF
    
    echo "  ✅ Added unified conversion methods to $struct_name"
}

# Process each service config file
echo "🎯 Processing service configuration files:"
echo ""

total_configs=0
processed_configs=0

for file_path in "${!SERVICE_CONFIGS[@]}"; do
    if [ -f "$file_path" ]; then
        echo "📄 Processing: $file_path"
        
        # Parse the configs for this file
        configs_string="${SERVICE_CONFIGS[$file_path]}"
        
        # Split the configs by space and process each one
        IFS=' ' read -ra CONFIGS <<< "$configs_string"
        for config_mapping in "${CONFIGS[@]}"; do
            # Split struct_name=unified_type
            IFS='=' read -ra MAPPING <<< "$config_mapping"
            struct_name="${MAPPING[0]}"
            unified_type="${MAPPING[1]}"
            
            echo "  🔧 $struct_name → $unified_type"
            
            # Check if the struct actually exists in the file
            if grep -q "pub struct $struct_name" "$file_path"; then
                add_unified_conversion "$file_path" "$struct_name" "$unified_type"
                ((processed_configs++))
            else
                echo "    ⚠️  Struct $struct_name not found in file"
            fi
            
            ((total_configs++))
        done
        
        echo ""
    else
        echo "❌ File not found: $file_path"
        echo ""
    fi
done

echo ""
echo "📊 Migration Summary:"
echo "  📋 Total configs identified: $total_configs"
echo "  ✅ Configs processed: $processed_configs"
echo "  ⚠️  Configs skipped: $((total_configs - processed_configs))"

echo ""
echo "🎯 Next Steps:"
echo "1. Update import statements to use unified types"
echo "2. Replace constructor calls with unified types"
echo "3. Update serialization/deserialization code"
echo "4. Test compilation with: cargo check --workspace"

echo ""
echo "🚀 Final phase: Run ./scripts/config-migration/cleanup-deprecated.sh"
echo "   This will remove deprecated configs and finalize migration" 