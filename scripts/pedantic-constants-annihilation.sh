#!/bin/bash
# 🔥 PEDANTIC CONSTANTS ANNIHILATION - ZERO TOLERANCE
# Systematic elimination of ALL hardcoded references with surgical precision

set -e

# Colors for PEDANTIC output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# PEDANTIC counters
TOTAL_REFERENCES=0
ELIMINATED_REFERENCES=0
FAILED_MIGRATIONS=0
PERFECT_MIGRATIONS=0

# PEDANTIC logging functions
log_pedantic() { echo -e "${PURPLE}[PEDANTIC]${NC} $1"; }
log_annihilate() { echo -e "${RED}[ANNIHILATE]${NC} $1"; }
log_perfect() { echo -e "${GREEN}[PERFECT]${NC} $1"; }
log_surgical() { echo -e "${CYAN}[SURGICAL]${NC} $1"; }

echo "🔥 PEDANTIC CONSTANTS ANNIHILATION INITIATED"
echo "============================================="
log_pedantic "ZERO TOLERANCE MODE: ACTIVATED"
log_pedantic "SURGICAL PRECISION: ENABLED"
log_pedantic "PERFECTION STANDARD: MANDATORY"

# Function to add import with PEDANTIC precision
add_pedantic_import() {
    local file="$1"
    local import_line="use nestgate_core::constants::ConstantsMigrationHelper;"
    
    if ! grep -q "ConstantsMigrationHelper" "$file"; then
        log_surgical "Adding canonical import to: $file"
        
        # Find the best location for import
        if grep -q "^use nestgate_core::" "$file"; then
            # Add after existing nestgate_core imports
            sed -i "/^use nestgate_core::/a\\$import_line" "$file"
        elif grep -q "^use " "$file"; then
            # Add after last use statement
            sed -i "/^use /a\\$import_line" "$file"
        else
            # Add at beginning if no use statements
            sed -i "1i\\$import_line\n" "$file"
        fi
        
        log_perfect "Import added with surgical precision"
    else
        log_surgical "Import already exists - PERFECT"
    fi
}

# Function for PEDANTIC migration of a single file
pedantic_migrate_file() {
    local file="$1"
    local original_content
    local changes_made=false
    
    log_annihilate "PROCESSING: $file"
    
    # Create PEDANTIC backup with timestamp
    cp "$file" "$file.pedantic.backup.$(date +%s)"
    original_content=$(cat "$file")
    
    # Check if we need to add import
    if grep -q "localhost:8080\|\"8080\"\|127\.0\.0\.1:8080\|http://localhost:8080" "$file"; then
        add_pedantic_import "$file"
        changes_made=true
    fi
    
    # PEDANTIC replacements with surgical precision
    
    # Replace http://localhost:8080 (most common pattern)
    if sed -i 's|"http://localhost:8080"|ConstantsMigrationHelper::http_api_endpoint()|g' "$file"; then
        changes_made=true
        log_surgical "Replaced http://localhost:8080 endpoints"
    fi
    
    # Replace localhost:8080 (connection strings)
    if sed -i 's|"localhost:8080"|ConstantsMigrationHelper::api_endpoint()|g' "$file"; then
        changes_made=true
        log_surgical "Replaced localhost:8080 endpoints"
    fi
    
    # Replace 127.0.0.1:8080 (IP-based connections)
    if sed -i 's|"127\.0\.0\.1:8080"|ConstantsMigrationHelper::api_endpoint()|g' "$file"; then
        changes_made=true
        log_surgical "Replaced 127.0.0.1:8080 endpoints"
    fi
    
    # Replace port string "8080" in specific contexts
    if sed -i 's|default_value = "8080"|default_value_t = nestgate_core::constants::canonical::network::DEFAULT_API_PORT|g' "$file"; then
        changes_made=true
        log_surgical "Replaced default_value port arguments"
    fi
    
    # Replace environment variable defaults
    if sed -i 's|("NESTGATE_PORT", "8080")|("NESTGATE_PORT", &nestgate_core::constants::ConstantsMigrationHelper::api_port_string())|g' "$file"; then
        changes_made=true
        log_surgical "Replaced environment variable defaults"
    fi
    
    # Replace format! patterns
    if sed -i 's|format!("http://localhost:8080")|ConstantsMigrationHelper::http_api_endpoint()|g' "$file"; then
        changes_made=true
        log_surgical "Replaced format! patterns"
    fi
    
    # Replace unwrap_or_else patterns
    if sed -i 's|unwrap_or_else(|| format!("http://localhost:8080"))|unwrap_or_else(|| ConstantsMigrationHelper::http_api_endpoint())|g' "$file"; then
        changes_made=true
        log_surgical "Replaced unwrap_or_else patterns"
    fi
    
    if [ "$changes_made" = true ]; then
        log_perfect "MIGRATION APPLIED: $file"
        ((ELIMINATED_REFERENCES++))
        
        # PEDANTIC validation: Check if file still compiles
        if cargo check --quiet 2>/dev/null; then
            log_perfect "COMPILATION: PERFECT"
            ((PERFECT_MIGRATIONS++))
            # Remove backup after successful validation
            rm "$file.pedantic.backup."*
        else
            log_annihilate "COMPILATION: FAILED - RESTORING BACKUP"
            # Restore from backup
            mv "$file.pedantic.backup."* "$file"
            ((FAILED_MIGRATIONS++))
        fi
    else
        log_surgical "NO CHANGES NEEDED: $file"
        # Remove unnecessary backup
        rm "$file.pedantic.backup."*
    fi
    
    ((TOTAL_REFERENCES++))
}

# Main PEDANTIC execution
main() {
    log_pedantic "INITIATING SYSTEMATIC ANNIHILATION"
    
    # Read the audit file
    if [ ! -f "hardcoded_audit.txt" ]; then
        log_annihilate "ERROR: hardcoded_audit.txt not found"
        log_pedantic "Run: grep -r \"localhost:8080|\\\"8080\\\"|127\\.0\\.0\\.1:8080\" code/ --include=\"*.rs\" > hardcoded_audit.txt"
        exit 1
    fi
    
    # Extract unique files from audit
    cut -d: -f1 hardcoded_audit.txt | sort | uniq > files_to_annihilate.txt
    
    local total_files=$(wc -l < files_to_annihilate.txt)
    log_pedantic "TARGETS IDENTIFIED: $total_files files"
    
    # Process each file with PEDANTIC precision
    local file_count=0
    while IFS= read -r file; do
        ((file_count++))
        log_pedantic "PROGRESS: $file_count/$total_files"
        
        if [ -f "$file" ]; then
            pedantic_migrate_file "$file"
        else
            log_annihilate "FILE NOT FOUND: $file"
        fi
    done < files_to_annihilate.txt
    
    # PEDANTIC final validation
    log_pedantic "EXECUTING FINAL VALIDATION"
    
    # Check remaining hardcoded references
    remaining=$(grep -r "localhost:8080\|\"8080\"\|127\.0\.0\.1:8080" code/ --include="*.rs" | wc -l)
    
    echo ""
    log_pedantic "ANNIHILATION SUMMARY:"
    log_pedantic "===================="
    log_pedantic "Files processed: $file_count"
    log_perfect "Perfect migrations: $PERFECT_MIGRATIONS"
    log_annihilate "Failed migrations: $FAILED_MIGRATIONS"
    log_pedantic "Remaining hardcoded: $remaining"
    
    if [ "$remaining" -eq 0 ]; then
        log_perfect "🏆 PEDANTIC PERFECTION ACHIEVED!"
        log_perfect "🎯 ZERO HARDCODED REFERENCES REMAINING"
        log_perfect "⚔️ CONSTANTS ANNIHILATION: COMPLETE"
    else
        log_annihilate "🚨 PEDANTIC FAILURE: $remaining references still exist"
        log_pedantic "Manual intervention required for remaining references"
        
        # Show remaining references
        echo ""
        log_pedantic "REMAINING REFERENCES:"
        grep -r "localhost:8080\|\"8080\"\|127\.0\.0\.1:8080" code/ --include="*.rs"
        
        exit 1
    fi
    
    # Clean up temporary files
    rm -f files_to_annihilate.txt
    
    # Final compilation test
    log_pedantic "FINAL COMPILATION TEST"
    if cargo check --quiet; then
        log_perfect "🏆 COMPILATION: PERFECT"
    else
        log_annihilate "🚨 COMPILATION ISSUES DETECTED"
        log_pedantic "Running detailed check..."
        cargo check
        exit 1
    fi
}

# Execute PEDANTIC annihilation
main "$@"

echo ""
log_perfect "🔥 PEDANTIC CONSTANTS ANNIHILATION: COMPLETE"
log_perfect "🎯 READY FOR NETWORKCONFIG CONSOLIDATION" 