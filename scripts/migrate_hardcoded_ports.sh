#!/bin/bash
# Systematic Port Migration Script
# Migrates hardcoded port numbers to use port_config module
# Usage: ./migrate_hardcoded_ports.sh [--dry-run]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

DRY_RUN=false
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
    echo -e "${YELLOW}DRY RUN MODE - No files will be modified${NC}"
fi

# Migration statistics
TOTAL_FILES=0
TOTAL_REPLACEMENTS=0
ERRORS=0

# Log file
LOG_FILE="migration_log_$(date +%Y%m%d_%H%M%S).txt"
echo "Port Migration Log - $(date)" > "$LOG_FILE"

# Port mapping: hardcoded_port -> replacement_function
declare -A PORT_MAP=(
    ["8080"]="crate::config::port_config::api_port()"
    ["8081"]="crate::config::port_config::admin_port()"
    ["8082"]="crate::config::port_config::health_port()"
    ["8083"]="crate::config::port_config::websocket_port()"
    ["8084"]="crate::config::port_config::orchestration_port()"
    ["8085"]="crate::config::port_config::storage_discovery_port()"
    ["8086"]="crate::config::port_config::compute_port()"
    ["3000"]="crate::config::port_config::grafana_port()"
    ["3010"]="crate::config::port_config::discovery_port()"
    ["5000"]="crate::config::port_config::storage_port()"
    ["5432"]="crate::config::port_config::postgres_port()"
    ["6379"]="crate::config::port_config::redis_port()"
    ["7070"]="crate::config::port_config::security_port()"
    ["9090"]="crate::config::port_config::prometheus_port()"
    ["9092"]="crate::config::port_config::kafka_port()"
    ["9100"]="crate::config::port_config::metrics_port()"
    ["27017"]="crate::config::port_config::mongodb_port()"
    ["50051"]="crate::config::port_config::grpc_port()"
)

# Function to check if file needs migration
needs_migration() {
    local file="$1"
    for port in "${!PORT_MAP[@]}"; do
        if grep -q "\b${port}\b" "$file" 2>/dev/null; then
            return 0
        fi
    done
    return 1
}

# Function to add use statement if missing
add_use_statement() {
    local file="$1"
    
    # Check if use statement already exists
    if grep -q "use.*config::port_config" "$file"; then
        return 0
    fi
    
    # Find appropriate place to add use statement
    if grep -q "^use " "$file"; then
        # Add after existing use statements
        sed -i '/^use /a use crate::config::port_config;' "$file"
    else
        # Add at beginning after module docs
        sed -i '1i use crate::config::port_config;' "$file"
    fi
    
    echo "  Added use statement" | tee -a "$LOG_FILE"
}

# Function to migrate a single file
migrate_file() {
    local file="$1"
    local replacements=0
    
    echo -e "\n${GREEN}Processing:${NC} $file" | tee -a "$LOG_FILE"
    
    if [[ "$DRY_RUN" == true ]]; then
        # Dry run - just count potential replacements
        for port in "${!PORT_MAP[@]}"; do
            local count=$(grep -o "\b${port}\b" "$file" 2>/dev/null | wc -l)
            if [[ $count -gt 0 ]]; then
                echo "  Would replace $count instances of port $port" | tee -a "$LOG_FILE"
                replacements=$((replacements + count))
            fi
        done
    else
        # Create backup
        cp "$file" "${file}.bak"
        
        # Add use statement if needed
        add_use_statement "$file"
        
        # Replace each port
        for port in "${!PORT_MAP[@]}"; do
            local replacement="${PORT_MAP[$port]}"
            local count=$(grep -o "\b${port}\b" "$file" | wc -l)
            
            if [[ $count -gt 0 ]]; then
                # Special handling for different contexts
                # In format strings: "...:{}" needs special care
                # In assertions: assert_eq!(port, 8080) -> assert_eq!(port, port_config::api_port())
                
                # Simple numeric replacement (works for most cases)
                sed -i "s/\b${port}\b/${replacement}/g" "$file"
                
                echo "  Replaced $count instances of port $port" | tee -a "$LOG_FILE"
                replacements=$((replacements + count))
            fi
        done
        
        # Format the file
        if command -v rustfmt &> /dev/null; then
            rustfmt "$file" 2>/dev/null || true
        fi
    fi
    
    TOTAL_REPLACEMENTS=$((TOTAL_REPLACEMENTS + replacements))
    return 0
}

# Main migration logic
main() {
    echo -e "${GREEN}=== Port Migration Script ===${NC}"
    echo "Starting migration at $(date)" | tee -a "$LOG_FILE"
    
    # Priority 1: Core config files
    echo -e "\n${YELLOW}Phase 1: Core Configuration Files${NC}"
    core_files=(
        "code/crates/nestgate-core/src/config/network_defaults_tests.rs"
        "code/crates/nestgate-core/src/config/defaults.rs"
        "code/crates/nestgate-core/src/config/defaults_config.rs"
        "code/crates/nestgate-core/src/config/runtime.rs"
    )
    
    for file in "${core_files[@]}"; do
        if [[ -f "$file" ]] && needs_migration "$file"; then
            migrate_file "$file"
            TOTAL_FILES=$((TOTAL_FILES + 1))
        fi
    done
    
    # Priority 2: Test files (safer to modify)
    echo -e "\n${YELLOW}Phase 2: Test Files${NC}"
    test_files=$(find code/crates/nestgate-core/src -name "*tests.rs" -o -name "*test*.rs" | head -20)
    
    for file in $test_files; do
        if needs_migration "$file"; then
            migrate_file "$file"
            TOTAL_FILES=$((TOTAL_FILES + 1))
        fi
    done
    
    # Summary
    echo -e "\n${GREEN}=== Migration Summary ===${NC}"
    echo "Files processed: $TOTAL_FILES"
    echo "Total replacements: $TOTAL_REPLACEMENTS"
    echo "Errors: $ERRORS"
    
    if [[ "$DRY_RUN" == false ]]; then
        echo -e "\n${YELLOW}Running tests to validate...${NC}"
        if cargo test --lib --quiet 2>&1 | tee -a "$LOG_FILE"; then
            echo -e "${GREEN}✓ Tests passed!${NC}"
        else
            echo -e "${RED}✗ Tests failed - check log${NC}"
            ERRORS=$((ERRORS + 1))
        fi
        
        echo -e "\nBackup files created with .bak extension"
        echo "Log file: $LOG_FILE"
    fi
}

# Run main
main

exit $ERRORS

