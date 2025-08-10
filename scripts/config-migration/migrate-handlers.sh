#!/bin/bash

# Migrate API Handler Configs Script
# Migrates API handler configuration structs to unified types

set -e

echo "🔧 Migrating API Handler Configurations to Unified Types"
echo "========================================================"

# Configuration
HANDLERS_DIR="code/crates/nestgate-api/src/handlers"
BACKUP_DIR="./config-migration-backup"

# Create backup directory
mkdir -p "$BACKUP_DIR"

echo "📁 API Handler directories:"
find "$HANDLERS_DIR" -name "*.rs" -path "*/handlers/*" | head -10

# Function to migrate a config struct in a handler file
migrate_handler_config() {
    local file=$1
    local struct_name=$2
    local unified_type=$3
    
    echo "🔄 Migrating $struct_name in $file to $unified_type"
    
    # Backup original file
    cp "$file" "$BACKUP_DIR/$(basename "$file").backup.$(date +%Y%m%d_%H%M%S)"
    
    # Add migration methods (this is a template - actual implementation would be more complex)
    cat >> "$file" << EOF

impl $struct_name {
    /// Convert to unified configuration type
    /// 🚀 ECOSYSTEM UNIFICATION: Migration to unified type system
    pub fn to_unified(&self) -> $unified_type {
        // TODO: Implement specific conversion logic for $struct_name
        Default::default()
    }
}

/// Modern unified type alias for $struct_name
pub type Modern$struct_name = $unified_type;
EOF
    
    echo "✅ Added migration methods to $struct_name"
}

# List of handler configs to migrate
declare -A HANDLER_CONFIGS=(
    ["load_testing.rs"]="LoadTestConfig=UnifiedConfig TestDataConfig=UnifiedConfig"
    ["dashboard_types.rs"]="DashboardConfig=UnifiedMonitoringConfig"
    ["performance_analytics/types.rs"]="PerformanceConfig=UnifiedMonitoringConfig"
    ["performance_dashboard/types.rs"]="DashboardConfig=UnifiedMonitoringConfig"
    ["zfs/types.rs"]="PoolConfig=UnifiedConfig"
    ["zfs/universal_zfs/types.rs"]="PoolConfig=UnifiedConfig DatasetConfig=UnifiedConfig SnapshotConfig=UnifiedConfig"
    ["zfs/universal_zfs/config.rs"]="ZfsServiceConfig=UnifiedConfig FailSafeConfig=UnifiedConfig ObservabilityConfig=UnifiedMonitoringConfig PerformanceConfig=UnifiedMonitoringConfig SecurityConfig=UnifiedSecurityConfig"
)

echo ""
echo "🎯 Handler configs to migrate:"
for handler_file in "${!HANDLER_CONFIGS[@]}"; do
    full_path="$HANDLERS_DIR/$handler_file"
    if [ -f "$full_path" ]; then
        echo "  ✅ $handler_file: ${HANDLER_CONFIGS[$handler_file]}"
    else
        echo "  ❌ $handler_file: FILE NOT FOUND"
    fi
done

echo ""
echo "📋 Migration Strategy:"
echo "1. Each config struct gets a to_unified() method"
echo "2. Add ModernConfigName type alias"
echo "3. Update imports to use unified types"
echo "4. Gradually replace usage with unified types"

echo ""
echo "🚀 Next phase: Run ./scripts/config-migration/migrate-services.sh"
echo "   This will migrate service-specific configs (MCP, NAS, FSMonitor, etc.)" 