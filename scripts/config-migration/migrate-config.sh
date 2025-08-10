#!/bin/bash

# Config Migration Script for NestGate Unified Configuration
# Helps migrate fragmented configs to UnifiedConfig system

set -e

echo "🚀 NestGate Config Unification Migration Script"
echo "================================================"

# Configuration
BACKUP_DIR="./config-migration-backup"
API_CONFIG_DIR="code/crates/nestgate-api/src/config"
CORE_UNIFIED_TYPES="code/crates/nestgate-core/src/unified_types.rs"

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Function: Backup original file
backup_file() {
    local file=$1
    local backup_name=$(basename "$file").backup.$(date +%Y%m%d_%H%M%S)
    cp "$file" "$BACKUP_DIR/$backup_name"
    echo "✅ Backed up: $file -> $BACKUP_DIR/$backup_name"
}

# Function: Extract config struct from file
extract_config_struct() {
    local file=$1
    local struct_name=$2
    local start_line=$3
    local end_line=$4
    
    echo "📤 Extracting $struct_name from $file (lines $start_line-$end_line)"
    sed -n "${start_line},${end_line}p" "$file"
}

# Function: Create unified config conversion
create_unified_conversion() {
    local struct_name=$1
    local unified_type=$2
    
    cat << EOF

impl $struct_name {
    /// Convert to unified configuration type
    /// 🚀 ECOSYSTEM UNIFICATION: Migration to unified type system
    pub fn to_unified(&self) -> $unified_type {
        // TODO: Implement conversion logic
        Default::default()
    }
    
    /// Create from unified configuration type
    /// 🚀 ECOSYSTEM UNIFICATION: Support unified type system
    pub fn from_unified(config: &$unified_type) -> Self {
        // TODO: Implement conversion logic
        Default::default()
    }
}

/// Modern unified type alias for $struct_name
/// 🚀 ECOSYSTEM UNIFICATION: Use this type for new code
pub type Modern$struct_name = $unified_type;

EOF
}

# Phase 1: Split the massive universal_primal_config.rs file
echo "📋 Phase 1: Analyzing universal_primal_config.rs"
UNIVERSAL_CONFIG="code/crates/nestgate-api/src/universal_primal_config.rs"

if [ -f "$UNIVERSAL_CONFIG" ]; then
    echo "📊 File size: $(wc -l < "$UNIVERSAL_CONFIG") lines"
    echo "🎯 Target: Split into modules under 1000 lines each"
    
    # Backup the original
    backup_file "$UNIVERSAL_CONFIG"
    
    # Count config structs
    STRUCT_COUNT=$(grep -c "pub struct.*Config" "$UNIVERSAL_CONFIG")
    echo "🔍 Found $STRUCT_COUNT config structs to migrate"
    
    # List all structs
    echo "📝 Config structs found:"
    grep "pub struct.*Config" "$UNIVERSAL_CONFIG" | sed 's/pub struct /- /' | sed 's/ {.*//'
else
    echo "❌ universal_primal_config.rs not found!"
    exit 1
fi

echo ""
echo "🚀 Ready to proceed with migration!"
echo "Next steps:"
echo "1. Run: ./scripts/config-migration/split-universal-config.sh"
echo "2. Run: ./scripts/config-migration/migrate-handlers.sh" 
echo "3. Run: ./scripts/config-migration/migrate-services.sh"
echo "4. Run: ./scripts/config-migration/cleanup-deprecated.sh" 