#!/usr/bin/env bash
# 🔧 NestGate Hardcoding Migration Script
# Date: November 28, 2025
# Purpose: Migrate hardcoded ports and IPs to configuration system
# Usage: ./scripts/migrate_hardcoded_values.sh [--dry-run] [--ports] [--ips] [--all]

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
WORKSPACE_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CODE_DIR="$WORKSPACE_ROOT/code"
DRY_RUN=false
MIGRATE_PORTS=false
MIGRATE_IPS=false
BACKUP_DIR="$WORKSPACE_ROOT/backup/hardcoding_migration_$(date +%Y%m%d_%H%M%S)"

# Statistics
PORTS_MIGRATED=0
IPS_MIGRATED=0
FILES_MODIFIED=0

# Port mappings (port number -> config function)
declare -A PORT_MAPPINGS=(
    ["8080"]="crate::config::port_config::api_port()"
    ["3000"]="crate::config::port_config::dev_port()"
    ["5432"]="crate::config::port_config::postgres_port()"
    ["6379"]="crate::config::port_config::redis_port()"
    ["9090"]="crate::config::port_config::metrics_port()"
    ["27017"]="crate::config::port_config::mongodb_port()"
    ["8081"]="crate::config::port_config::health_port()"
    ["8082"]="crate::config::port_config::websocket_port()"
    ["8443"]="crate::config::port_config::secure_api_port()"
)

# IP mappings (IP/hostname -> environment variable)
declare -A IP_MAPPINGS=(
    ["127.0.0.1"]='env::var("NESTGATE_HOST").unwrap_or_else(|_| "127.0.0.1".to_string())'
    ["localhost"]='env::var("NESTGATE_HOST").unwrap_or_else(|_| "localhost".to_string())'
    ["0.0.0.0"]='env::var("NESTGATE_BIND_HOST").unwrap_or_else(|_| "0.0.0.0".to_string())'
)

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_usage() {
    cat << EOF
Usage: $0 [OPTIONS]

Migrate hardcoded ports and IPs to configuration system.

OPTIONS:
    --dry-run       Show what would be changed without making changes
    --ports         Migrate only hardcoded ports
    --ips           Migrate only hardcoded IPs
    --all           Migrate both ports and IPs (default)
    --help          Show this help message

EXAMPLES:
    # Dry run to see what would change
    $0 --dry-run --all

    # Migrate only ports
    $0 --ports

    # Migrate everything
    $0 --all

EOF
}

create_backup() {
    log_info "Creating backup at $BACKUP_DIR..."
    mkdir -p "$BACKUP_DIR"
    
    # Backup all Rust files
    find "$CODE_DIR" -name "*.rs" -type f ! -path "*/target/*" | while read -r file; do
        rel_path="${file#$CODE_DIR/}"
        backup_file="$BACKUP_DIR/$rel_path"
        mkdir -p "$(dirname "$backup_file")"
        cp "$file" "$backup_file"
    done
    
    log_success "Backup created successfully"
}

migrate_port_in_file() {
    local file=$1
    local port=$2
    local config_func=$3
    local modified=false
    
    # Skip if file doesn't contain the port
    if ! grep -q "\b$port\b" "$file"; then
        return 0
    fi
    
    log_info "Processing $file for port $port..."
    
    if [ "$DRY_RUN" = true ]; then
        log_warning "[DRY RUN] Would replace port $port with $config_func in $file"
        grep -n "\b$port\b" "$file" | head -5
        return 0
    fi
    
    # Create temp file
    temp_file=$(mktemp)
    
    # Perform replacement (context-aware)
    # This is a simplified version - production version would be more sophisticated
    sed "s/\b$port\b/$config_func/g" "$file" > "$temp_file"
    
    # Check if file actually changed
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        ((FILES_MODIFIED++))
        ((PORTS_MIGRATED++))
        log_success "Migrated port $port in $(basename "$file")"
        modified=true
    else
        rm "$temp_file"
    fi
    
    return 0
}

migrate_ip_in_file() {
    local file=$1
    local ip=$2
    local env_var=$3
    
    # Skip if file doesn't contain the IP
    if ! grep -q "$ip" "$file"; then
        return 0
    fi
    
    log_info "Processing $file for IP $ip..."
    
    if [ "$DRY_RUN" = true ]; then
        log_warning "[DRY RUN] Would replace IP $ip with $env_var in $file"
        grep -n "$ip" "$file" | head -5
        return 0
    fi
    
    # Create temp file
    temp_file=$(mktemp)
    
    # Perform replacement
    sed "s/$ip/$env_var/g" "$file" > "$temp_file"
    
    # Check if file actually changed
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        ((FILES_MODIFIED++))
        ((IPS_MIGRATED++))
        log_success "Migrated IP $ip in $(basename "$file")"
    else
        rm "$temp_file"
    fi
    
    return 0
}

migrate_ports() {
    log_info "Starting port migration..."
    
    # Find all Rust files
    find "$CODE_DIR" -name "*.rs" -type f ! -path "*/target/*" | while read -r file; do
        for port in "${!PORT_MAPPINGS[@]}"; do
            migrate_port_in_file "$file" "$port" "${PORT_MAPPINGS[$port]}"
        done
    done
    
    log_success "Port migration complete. Migrated: $PORTS_MIGRATED ports"
}

migrate_ips() {
    log_info "Starting IP migration..."
    
    # Find all Rust files
    find "$CODE_DIR" -name "*.rs" -type f ! -path "*/target/*" | while read -r file; do
        for ip in "${!IP_MAPPINGS[@]}"; do
            migrate_ip_in_file "$file" "$ip" "${IP_MAPPINGS[$ip]}"
        done
    done
    
    log_success "IP migration complete. Migrated: $IPS_MIGRATED IPs"
}

verify_migration() {
    log_info "Verifying migration..."
    
    # Run cargo check
    log_info "Running cargo check..."
    if cargo check --workspace &> /dev/null; then
        log_success "Cargo check passed"
    else
        log_error "Cargo check failed - migration may have introduced errors"
        return 1
    fi
    
    # Run tests
    log_info "Running tests..."
    if cargo test --workspace --lib &> /dev/null; then
        log_success "Tests passed"
    else
        log_warning "Some tests failed - review changes"
        return 1
    fi
    
    return 0
}

print_summary() {
    echo ""
    log_info "==================== MIGRATION SUMMARY ===================="
    echo "Ports migrated:    $PORTS_MIGRATED"
    echo "IPs migrated:      $IPS_MIGRATED"
    echo "Files modified:    $FILES_MODIFIED"
    echo "Backup location:   $BACKUP_DIR"
    if [ "$DRY_RUN" = true ]; then
        echo "Mode:              DRY RUN (no changes made)"
    else
        echo "Mode:              LIVE (changes applied)"
    fi
    log_info "==========================================================="
    echo ""
}

# Main execution
main() {
    # Parse arguments
    if [ $# -eq 0 ]; then
        MIGRATE_PORTS=true
        MIGRATE_IPS=true
    fi
    
    while [ $# -gt 0 ]; do
        case "$1" in
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --ports)
                MIGRATE_PORTS=true
                shift
                ;;
            --ips)
                MIGRATE_IPS=true
                shift
                ;;
            --all)
                MIGRATE_PORTS=true
                MIGRATE_IPS=true
                shift
                ;;
            --help)
                print_usage
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                print_usage
                exit 1
                ;;
        esac
    done
    
    log_info "NestGate Hardcoding Migration Script"
    log_info "====================================="
    
    if [ "$DRY_RUN" = true ]; then
        log_warning "Running in DRY RUN mode - no changes will be made"
    fi
    
    # Create backup
    if [ "$DRY_RUN" = false ]; then
        create_backup
    fi
    
    # Migrate ports
    if [ "$MIGRATE_PORTS" = true ]; then
        migrate_ports
    fi
    
    # Migrate IPs
    if [ "$MIGRATE_IPS" = true ]; then
        migrate_ips
    fi
    
    # Verify if not dry run
    if [ "$DRY_RUN" = false ]; then
        verify_migration
    fi
    
    # Print summary
    print_summary
    
    if [ "$DRY_RUN" = false ]; then
        log_success "Migration complete! Review changes and run tests."
        log_info "Backup available at: $BACKUP_DIR"
    else
        log_info "Dry run complete. Run without --dry-run to apply changes."
    fi
}

# Run main
main "$@"

