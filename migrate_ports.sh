#!/bin/bash
# Systematic Port Migration Script
# Migrates hardcoded ports to use PortConfiguration

set -e

cd "$(dirname "$0")"

echo "🔧 Starting systematic port migration..."
echo ""

# Statistics
TOTAL_FILES=0
MIGRATED_FILES=0

# Function to migrate a file
migrate_file() {
    local file=$1
    local description=$2
    
    echo "📝 Migrating: $file"
    echo "   Purpose: $description"
    
    # Backup
    cp "$file" "$file.backup"
    
    # Count changes
    local before=$(grep -c "localhost:8080\|127.0.0.1:8080\|8080\"" "$file" || true)
    
    # Perform migration (this is a template - actual implementation per file)
    # Each file needs custom migration based on context
    
    local after=$(grep -c "localhost:8080\|127.0.0.1:8080\|8080\"" "$file" || true)
    
    echo "   Changed: $before → $after instances"
    echo ""
    
    TOTAL_FILES=$((TOTAL_FILES + 1))
    if [ "$after" -lt "$before" ]; then
        MIGRATED_FILES=$((MIGRATED_FILES + 1))
    fi
}

echo "Phase 1: High-Priority Files (Tests and Config)"
echo "================================================"

# Priority 1: Test files (safe to modify)
echo ">> Test Files"
# migrate_file "code/crates/nestgate-zfs/src/manager/tests.rs" "ZFS manager tests"
# migrate_file "code/crates/nestgate-core/src/load_balancing/tests.rs" "Load balancing tests"

echo ""
echo "Phase 2: Configuration Files"
echo "============================="

# Priority 2: Config files
echo ">> Configuration"
# migrate_file "code/crates/nestgate-core/src/constants/canonical_defaults.rs" "Canonical defaults"

echo ""
echo "Phase 3: Discovery and Integration"
echo "==================================="

# Priority 3: Service discovery
echo ">> Service Discovery"
# migrate_file "code/crates/nestgate-core/src/service_discovery/discovery_expanded_tests.rs" "Discovery tests"

echo ""
echo "📊 Migration Summary"
echo "===================="
echo "Total files processed: $TOTAL_FILES"
echo "Files migrated: $MIGRATED_FILES"
echo "Remaining: $((20 - MIGRATED_FILES)) files with localhost:8080"
echo ""
echo "✅ Migration script template ready"
echo "⚠️  Each file needs custom migration based on context"
echo "📋 Use this script as a guide for systematic migration"


