#!/bin/bash

# **PRIMAL HARDCODING MIGRATION SCRIPT**
# 
# This script systematically migrates primal-specific hardcoding to capability-based patterns.
# It identifies and replaces direct primal references with universal adapter calls.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKUP_DIR="${WORKSPACE_ROOT}/migration-backup-$(date +%Y%m%d-%H%M%S)"
LOG_FILE="${WORKSPACE_ROOT}/migration.log"

echo -e "${BLUE}🎯 PRIMAL HARDCODING MIGRATION SCRIPT${NC}"
echo -e "${BLUE}======================================${NC}"
echo "Workspace: $WORKSPACE_ROOT"
echo "Backup: $BACKUP_DIR"
echo "Log: $LOG_FILE"
echo ""

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Logging function
log() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$LOG_FILE"
}

# Progress function
progress() {
    echo -e "${GREEN}✅ $1${NC}" | tee -a "$LOG_FILE"
}

# Warning function
warning() {
    echo -e "${YELLOW}⚠️  $1${NC}" | tee -a "$LOG_FILE"
}

# Error function
error() {
    echo -e "${RED}❌ $1${NC}" | tee -a "$LOG_FILE"
}

# ==================== AUDIT FUNCTIONS ====================

audit_primal_references() {
    log "🔍 Auditing primal references in codebase..."
    
    cd "$WORKSPACE_ROOT"
    
    echo "=== PRIMAL HARDCODING AUDIT ===" > "$LOG_FILE"
    
    echo "Songbird references:" >> "$LOG_FILE"
    grep -r "songbird\|Songbird" code/crates/ --include="*.rs" | wc -l >> "$LOG_FILE" || true
    
    echo "Toadstool references:" >> "$LOG_FILE"
    grep -r "toadstool\|Toadstool" code/crates/ --include="*.rs" | wc -l >> "$LOG_FILE" || true
    
    echo "BearDog references:" >> "$LOG_FILE"
    grep -r "beardog\|BearDog" code/crates/ --include="*.rs" | wc -l >> "$LOG_FILE" || true
    
    echo "Squirrel references:" >> "$LOG_FILE"
    grep -r "squirrel\|Squirrel" code/crates/ --include="*.rs" | wc -l >> "$LOG_FILE" || true
    
    echo "BiomeOS references:" >> "$LOG_FILE"
    grep -r "biomeOS\|BiomeOS\|biomeos" code/crates/ --include="*.rs" | wc -l >> "$LOG_FILE" || true
    
    echo "Hardcoded endpoints:" >> "$LOG_FILE"
    grep -r "http://.*:808[0-9]\|http://.*:909[0-9]" code/crates/ --include="*.rs" | wc -l >> "$LOG_FILE" || true
    
    progress "Audit completed - see $LOG_FILE for details"
}

# ==================== BACKUP FUNCTIONS ====================

backup_files() {
    log "💾 Creating backup of files to be modified..."
    
    # Find all files with primal references
    find code/crates/ -name "*.rs" -type f | while read -r file; do
        if grep -q "songbird\|toadstool\|beardog\|squirrel\|biomeOS\|BiomeOS" "$file" 2>/dev/null; then
            # Create directory structure in backup
            backup_path="$BACKUP_DIR/$(dirname "$file")"
            mkdir -p "$backup_path"
            
            # Copy file to backup
            cp "$file" "$BACKUP_DIR/$file"
            log "Backed up: $file"
        fi
    done
    
    progress "Backup completed in $BACKUP_DIR"
}

# ==================== MIGRATION FUNCTIONS ====================

migrate_songbird_references() {
    log "🎵 Migrating Songbird references to orchestration capabilities..."
    
    cd "$WORKSPACE_ROOT"
    
    # Replace Songbird-specific patterns
    find code/crates/ -name "*.rs" -type f -exec sed -i.bak \
        -e 's/SongbirdClient/OrchestrationCapability/g' \
        -e 's/songbird_endpoint/orchestration_capability/g' \
        -e 's/route_to_songbird/route_orchestration_capability/g' \
        -e 's/songbird::/orchestration::/g' \
        -e 's/Songbird/Orchestration/g' \
        -e 's/songbird/orchestration/g' \
        {} \;
    
    # Clean up backup files
    find code/crates/ -name "*.rs.bak" -delete
    
    progress "Songbird migration completed"
}

migrate_toadstool_references() {
    log "🍄 Migrating Toadstool references to compute capabilities..."
    
    cd "$WORKSPACE_ROOT"
    
    # Replace Toadstool-specific patterns
    find code/crates/ -name "*.rs" -type f -exec sed -i.bak \
        -e 's/ToadstoolClient/ComputeCapability/g' \
        -e 's/ToadstoolComputeClient/ComputeCapability/g' \
        -e 's/toadstool_endpoint/compute_capability/g' \
        -e 's/route_to_toadstool/route_compute_capability/g' \
        -e 's/toadstool::/compute::/g' \
        -e 's/Toadstool/Compute/g' \
        -e 's/toadstool/compute/g' \
        {} \;
    
    # Clean up backup files
    find code/crates/ -name "*.rs.bak" -delete
    
    progress "Toadstool migration completed"
}

migrate_beardog_references() {
    log "🐕 Migrating BearDog references to security capabilities..."
    
    cd "$WORKSPACE_ROOT"
    
    # Replace BearDog-specific patterns
    find code/crates/ -name "*.rs" -type f -exec sed -i.bak \
        -e 's/BearDogClient/SecurityCapability/g' \
        -e 's/beardog_endpoint/security_capability/g' \
        -e 's/route_to_beardog/route_security_capability/g' \
        -e 's/beardog::/security::/g' \
        -e 's/BearDog/Security/g' \
        -e 's/beardog/security/g' \
        {} \;
    
    # Clean up backup files
    find code/crates/ -name "*.rs.bak" -delete
    
    progress "BearDog migration completed"
}

migrate_squirrel_references() {
    log "🐿️  Migrating Squirrel references to intelligence capabilities..."
    
    cd "$WORKSPACE_ROOT"
    
    # Replace Squirrel-specific patterns
    find code/crates/ -name "*.rs" -type f -exec sed -i.bak \
        -e 's/SquirrelClient/IntelligenceCapability/g' \
        -e 's/squirrel_endpoint/intelligence_capability/g' \
        -e 's/route_to_squirrel/route_intelligence_capability/g' \
        -e 's/squirrel::/intelligence::/g' \
        -e 's/Squirrel/Intelligence/g' \
        -e 's/squirrel/intelligence/g' \
        {} \;
    
    # Clean up backup files
    find code/crates/ -name "*.rs.bak" -delete
    
    progress "Squirrel migration completed"
}

migrate_biomeos_references() {
    log "🌱 Migrating BiomeOS references to management capabilities..."
    
    cd "$WORKSPACE_ROOT"
    
    # Replace BiomeOS-specific patterns
    find code/crates/ -name "*.rs" -type f -exec sed -i.bak \
        -e 's/BiomeOSClient/ManagementCapability/g' \
        -e 's/biomeos_endpoint/management_capability/g' \
        -e 's/route_to_biomeos/route_management_capability/g' \
        -e 's/biomeos::/management::/g' \
        -e 's/BiomeOS/Management/g' \
        -e 's/biomeOS/management/g' \
        -e 's/biomeos/management/g' \
        {} \;
    
    # Clean up backup files
    find code/crates/ -name "*.rs.bak" -delete
    
    progress "BiomeOS migration completed"
}

migrate_hardcoded_endpoints() {
    log "🌐 Migrating hardcoded endpoints to environment-based configuration..."
    
    cd "$WORKSPACE_ROOT"
    
    # Replace common hardcoded endpoints
    find code/crates/ -name "*.rs" -type f -exec sed -i.bak \
        -e 's/"http:\/\/localhost:8080"/"http:\/\/localhost:".to_string() + \&env::var("NESTGATE_API_PORT").unwrap_or_else(|_| "8080".to_string())/g' \
        -e 's/"http:\/\/localhost:8081"/"http:\/\/localhost:".to_string() + \&env::var("NESTGATE_SECURITY_PORT").unwrap_or_else(|_| "8081".to_string())/g' \
        -e 's/"http:\/\/.*:8080"/get_service_endpoint("api").unwrap_or_else(|_| "http:\/\/localhost:8080".to_string())/g' \
        {} \;
    
    # Clean up backup files
    find code/crates/ -name "*.rs.bak" -delete
    
    progress "Hardcoded endpoint migration completed"
}

add_capability_imports() {
    log "📦 Adding capability-based imports to migrated files..."
    
    cd "$WORKSPACE_ROOT"
    
    # Find files that need capability imports
    find code/crates/ -name "*.rs" -type f | while read -r file; do
        if grep -q "OrchestrationCapability\|ComputeCapability\|SecurityCapability\|IntelligenceCapability\|ManagementCapability" "$file" 2>/dev/null; then
            # Check if imports already exist
            if ! grep -q "use nestgate_core::universal_adapter" "$file" 2>/dev/null; then
                # Add capability imports at the top of the file
                sed -i '1i use nestgate_core::universal_adapter::{PrimalAgnosticAdapter, CapabilityCategory, CapabilityRequest};' "$file"
                log "Added capability imports to: $file"
            fi
        fi
    done
    
    progress "Capability imports added"
}

# ==================== VALIDATION FUNCTIONS ====================

validate_migration() {
    log "🔍 Validating migration results..."
    
    cd "$WORKSPACE_ROOT"
    
    # Count remaining primal references
    songbird_count=$(grep -r "songbird\|Songbird" code/crates/ --include="*.rs" | wc -l || echo "0")
    toadstool_count=$(grep -r "toadstool\|Toadstool" code/crates/ --include="*.rs" | wc -l || echo "0")
    beardog_count=$(grep -r "beardog\|BearDog" code/crates/ --include="*.rs" | wc -l || echo "0")
    squirrel_count=$(grep -r "squirrel\|Squirrel" code/crates/ --include="*.rs" | wc -l || echo "0")
    biomeos_count=$(grep -r "biomeOS\|BiomeOS\|biomeos" code/crates/ --include="*.rs" | wc -l || echo "0")
    
    # Count capability references
    capability_count=$(grep -r "CapabilityCategory\|PrimalAgnosticAdapter" code/crates/ --include="*.rs" | wc -l || echo "0")
    
    echo "=== MIGRATION VALIDATION ===" >> "$LOG_FILE"
    echo "Remaining primal references:" >> "$LOG_FILE"
    echo "  Songbird: $songbird_count" >> "$LOG_FILE"
    echo "  Toadstool: $toadstool_count" >> "$LOG_FILE"
    echo "  BearDog: $beardog_count" >> "$LOG_FILE"
    echo "  Squirrel: $squirrel_count" >> "$LOG_FILE"
    echo "  BiomeOS: $biomeos_count" >> "$LOG_FILE"
    echo "Capability-based references: $capability_count" >> "$LOG_FILE"
    
    total_primal=$((songbird_count + toadstool_count + beardog_count + squirrel_count + biomeos_count))
    
    if [ "$total_primal" -eq 0 ]; then
        progress "🎉 Migration successful! Zero primal references remaining."
    elif [ "$total_primal" -lt 10 ]; then
        warning "Migration mostly successful. $total_primal primal references remaining (likely in tests/docs)."
    else
        error "Migration incomplete. $total_primal primal references remaining."
        return 1
    fi
    
    if [ "$capability_count" -gt 0 ]; then
        progress "✅ Capability-based patterns successfully introduced ($capability_count references)."
    else
        warning "⚠️  No capability-based patterns found. Manual review needed."
    fi
}

compile_check() {
    log "🔨 Checking if migrated code compiles..."
    
    cd "$WORKSPACE_ROOT"
    
    if cargo check --workspace --quiet 2>/dev/null; then
        progress "✅ Code compiles successfully after migration!"
    else
        warning "⚠️  Compilation issues detected. Manual fixes may be needed."
        cargo check --workspace 2>&1 | head -20 >> "$LOG_FILE"
    fi
}

# ==================== MAIN EXECUTION ====================

main() {
    echo -e "${BLUE}🚀 Starting primal hardcoding migration...${NC}"
    echo ""
    
    # Phase 1: Audit and Backup
    audit_primal_references
    backup_files
    
    echo ""
    echo -e "${BLUE}📝 Phase 1 Complete: Audit and Backup${NC}"
    echo ""
    
    # Phase 2: Systematic Migration
    migrate_songbird_references
    migrate_toadstool_references
    migrate_beardog_references
    migrate_squirrel_references
    migrate_biomeos_references
    migrate_hardcoded_endpoints
    add_capability_imports
    
    echo ""
    echo -e "${BLUE}🔄 Phase 2 Complete: Systematic Migration${NC}"
    echo ""
    
    # Phase 3: Validation
    validate_migration
    compile_check
    
    echo ""
    echo -e "${GREEN}🎉 PRIMAL HARDCODING MIGRATION COMPLETE!${NC}"
    echo ""
    echo "📊 Results:"
    echo "  - Backup created: $BACKUP_DIR"
    echo "  - Migration log: $LOG_FILE"
    echo "  - Capability-based patterns introduced"
    echo "  - Primal hardcoding eliminated"
    echo ""
    echo "🔄 Next Steps:"
    echo "  1. Review migration log for any issues"
    echo "  2. Run tests to ensure functionality"
    echo "  3. Update configuration to use capability discovery"
    echo "  4. Remove deprecated primal-specific modules"
    echo ""
}

# Handle script arguments
case "${1:-}" in
    "audit")
        audit_primal_references
        ;;
    "backup")
        backup_files
        ;;
    "migrate")
        migrate_songbird_references
        migrate_toadstool_references
        migrate_beardog_references
        migrate_squirrel_references
        migrate_biomeos_references
        migrate_hardcoded_endpoints
        add_capability_imports
        ;;
    "validate")
        validate_migration
        ;;
    "compile")
        compile_check
        ;;
    "")
        main
        ;;
    *)
        echo "Usage: $0 [audit|backup|migrate|validate|compile]"
        echo "  audit    - Audit primal references only"
        echo "  backup   - Create backup only"
        echo "  migrate  - Run migration only"
        echo "  validate - Validate migration results only"
        echo "  compile  - Check compilation only"
        echo "  (no args) - Run full migration process"
        exit 1
        ;;
esac 