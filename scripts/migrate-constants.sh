#!/bin/bash
# Constants Migration Script - Systematic replacement of hardcoded values
# Part of NestGate Unification Phase 1

set -e

echo "🔧 NestGate Constants Migration Script"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Counters
TOTAL_FILES=0
MIGRATED_FILES=0
ERRORS=0

# Function to log with colors
log_info() { echo -e "${BLUE}[INFO]${NC} $1"; }
log_success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
log_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
log_error() { echo -e "${RED}[ERROR]${NC} $1"; }

# Function to add import if not exists
add_import_if_needed() {
    local file="$1"
    local import_line="use nestgate_core::constants::ConstantsMigrationHelper;"
    
    if ! grep -q "ConstantsMigrationHelper" "$file"; then
        # Find the last use statement and add after it
        if grep -q "^use " "$file"; then
            sed -i "/^use /a\\$import_line" "$file"
            log_info "Added import to $file"
        else
            # Add at the beginning if no use statements
            sed -i "1i\\$import_line\n" "$file"
            log_info "Added import at beginning of $file"
        fi
    fi
}

# Function to migrate constants in a file
migrate_file_constants() {
    local file="$1"
    local changes_made=false
    
    log_info "Processing: $file"
    
    # Backup original file
    cp "$file" "$file.backup"
    
    # Add import if we're going to make changes
    if grep -q "localhost:8080\|\"8080\"\|127\.0\.0\.1:8080" "$file"; then
        add_import_if_needed "$file"
        changes_made=true
    fi
    
    # Replace hardcoded endpoints with canonical constants
    if sed -i 's|"localhost:8080"|ConstantsMigrationHelper::api_endpoint()|g' "$file"; then
        changes_made=true
    fi
    
    if sed -i 's|"127\.0\.0\.1:8080"|ConstantsMigrationHelper::api_endpoint()|g' "$file"; then
        changes_made=true
    fi
    
    if sed -i 's|"http://localhost:8080"|ConstantsMigrationHelper::http_api_endpoint()|g' "$file"; then
        changes_made=true
    fi
    
    if sed -i 's|"http://127\.0\.0\.1:8080"|ConstantsMigrationHelper::http_api_endpoint()|g' "$file"; then
        changes_made=true
    fi
    
    # Replace hardcoded port strings (be careful with context)
    if sed -i 's|"8080"|ConstantsMigrationHelper::api_port_string()|g' "$file"; then
        changes_made=true
    fi
    
    # Replace hardcoded port numbers in specific contexts
    if sed -i 's|port: 8080|port: nestgate_core::constants::canonical::network::DEFAULT_API_PORT|g' "$file"; then
        changes_made=true
    fi
    
    if [ "$changes_made" = true ]; then
        log_success "Migrated constants in $file"
        ((MIGRATED_FILES++))
    else
        # Remove backup if no changes made
        rm "$file.backup"
    fi
    
    ((TOTAL_FILES++))
}

# Function to validate file syntax after migration
validate_file() {
    local file="$1"
    
    # Basic Rust syntax check (if file is .rs)
    if [[ "$file" == *.rs ]]; then
        if ! cargo check --manifest-path "$(find . -name "Cargo.toml" | head -1)" --quiet 2>/dev/null; then
            log_warning "Syntax issues detected after migration in $file"
            # Restore backup
            if [ -f "$file.backup" ]; then
                mv "$file.backup" "$file"
                log_info "Restored backup for $file"
                ((ERRORS++))
                return 1
            fi
        else
            # Remove backup after successful validation
            [ -f "$file.backup" ] && rm "$file.backup"
        fi
    fi
    return 0
}

# Main migration process
main() {
    log_info "Starting constants migration..."
    log_info "Target: Replace hardcoded network values with canonical constants"
    
    # Find all Rust files with hardcoded values
    log_info "Scanning for files with hardcoded network values..."
    
    # Create array of files to process
    mapfile -t files_to_process < <(find code/ -name "*.rs" -exec grep -l "localhost:8080\|\"8080\"\|127\.0\.0\.1:8080\|http://localhost:8080" {} \;)
    
    log_info "Found ${#files_to_process[@]} files to process"
    
    if [ ${#files_to_process[@]} -eq 0 ]; then
        log_success "No files found with hardcoded values - migration may be complete!"
        exit 0
    fi
    
    # Process each file
    for file in "${files_to_process[@]}"; do
        if [ -f "$file" ]; then
            migrate_file_constants "$file"
            
            # Validate after migration
            if ! validate_file "$file"; then
                log_error "Validation failed for $file"
            fi
        fi
    done
    
    # Summary
    echo ""
    log_info "Migration Summary:"
    log_info "=================="
    log_info "Total files processed: $TOTAL_FILES"
    log_success "Successfully migrated: $MIGRATED_FILES"
    
    if [ $ERRORS -gt 0 ]; then
        log_error "Files with errors: $ERRORS"
        log_warning "Check backup files (.backup) for failed migrations"
    else
        log_success "All migrations completed successfully!"
    fi
    
    # Clean up any remaining backup files if no errors
    if [ $ERRORS -eq 0 ]; then
        find code/ -name "*.backup" -delete 2>/dev/null || true
        log_info "Cleaned up backup files"
    fi
    
    # Test compilation
    log_info "Testing compilation after migration..."
    if cargo check --quiet; then
        log_success "Compilation successful after migration!"
    else
        log_error "Compilation issues detected - check migration results"
        exit 1
    fi
}

# Run the migration
main "$@"

echo ""
log_success "Constants migration completed!"
log_info "Next step: NetworkConfig consolidation" 