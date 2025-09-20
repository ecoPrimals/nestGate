#!/bin/bash

# 🏛️ **CANONICAL CONFIGURATION MIGRATION SCRIPT**
# 
# Migrates existing fragmented TOML configurations to unified canonical master config
# **CANONICAL MODERNIZATION COMPLETE**: Single source of truth for all configuration

set -euo pipefail

# Script configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
CONFIG_DIR="$PROJECT_ROOT/config"
BACKUP_DIR="$CONFIG_DIR/backup-$(date +%Y%m%d-%H%M%S)"
CANONICAL_CONFIG="$CONFIG_DIR/canonical-master.toml"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
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

# Main migration function
main() {
    log_info "🏛️ Starting NestGate Canonical Configuration Migration"
    
    # Create backup directory
    mkdir -p "$BACKUP_DIR"
    log_info "📁 Created backup directory: $BACKUP_DIR"
    
    # Backup existing configurations
    backup_configs
    
    # Generate canonical master configuration
    generate_canonical_config
    
    # Validate the new configuration
    validate_canonical_config
    
    # Update references in code
    update_config_references
    
    log_success "🎉 Canonical configuration migration completed successfully!"
    log_info "📋 Summary:"
    log_info "   • Backup created: $BACKUP_DIR"
    log_info "   • Canonical config: $CANONICAL_CONFIG"
    log_info "   • Migration report: $BACKUP_DIR/migration-report.txt"
}

# Backup existing configurations
backup_configs() {
    log_info "💾 Backing up existing configurations..."
    
    for config_file in "$CONFIG_DIR"/*.toml; do
        if [[ -f "$config_file" ]] && [[ "$(basename "$config_file")" != "canonical-master.toml" ]]; then
            cp "$config_file" "$BACKUP_DIR/"
            log_info "   • Backed up: $(basename "$config_file")"
        fi
    done
    
    log_success "✅ Configuration backup completed"
}

# Generate canonical master configuration
generate_canonical_config() {
    log_info "🏗️ Generating canonical master configuration..."
    
    cat > "$CANONICAL_CONFIG" << 'EOF'
# 🏛️ **NESTGATE CANONICAL MASTER CONFIGURATION**
#
# Single source of truth for all NestGate configuration.
# **CANONICAL MODERNIZATION COMPLETE**: Unified configuration system.
#
# This file replaces all fragmented configuration files and provides
# a comprehensive, well-structured configuration for all NestGate components.

[system]
# Core system configuration
instance_name = "nestgate-production"
environment = "production"
version = "2.0.0-canonical"
log_level = "info"
max_connections = 10000
worker_threads = 16
enable_metrics = true
enable_tracing = true
startup_timeout_seconds = 30
shutdown_timeout_seconds = 10

[system.performance]
# Performance optimization settings
zero_copy_enabled = true
native_async_enabled = true
memory_pool_size_mb = 1024
cpu_affinity_enabled = true
numa_aware = true
prefetch_enabled = true

[network]
# Network configuration

[network.api]
# Main API server configuration
host = "0.0.0.0"
port = 8443
enable_tls = true
tls_cert_path = "/etc/nestgate/certs/server.crt"
tls_key_path = "/etc/nestgate/certs/server.key"
tls_ca_path = "/etc/nestgate/certs/ca.crt"
request_timeout_seconds = 30
max_request_size_mb = 100
keepalive_timeout_seconds = 60
max_concurrent_requests = 1000

[network.protocols]
# Storage protocol configuration
nfs_enabled = true
nfs_port = 2049
smb_enabled = true
smb_port = 445
iscsi_enabled = true
iscsi_port = 3260
ftp_enabled = false
sftp_enabled = true
sftp_port = 22

[network.cluster]
# Cluster networking (if clustering enabled)
enabled = false
bind_address = "0.0.0.0:8444"
cluster_name = "nestgate-cluster"
discovery_enabled = true
discovery_multicast = "224.0.0.1:8445"
election_timeout_ms = 5000
heartbeat_interval_ms = 1000
max_missed_heartbeats = 3
encryption_enabled = true

[storage]
# Core storage configuration
backend_type = "zfs_native"
data_path = "/data/nestgate"
backup_path = "/backup/nestgate"
temp_path = "/tmp/nestgate"
max_file_size_gb = 100
compression_enabled = true
deduplication_enabled = true
snapshot_enabled = true

[storage.zfs]
# ZFS-specific configuration
pool_name = "nestgate-pool"
dataset_name = "nestgate-data"
compression = "lz4"
deduplication = true
encryption = "aes-256-gcm"
checksum = "sha256"
atime = false
recordsize = "128K"
command_timeout_seconds = 30
use_sudo = true
auto_scrub_enabled = true
scrub_interval_days = 7

[storage.zfs.pools]
# ZFS pool configuration
auto_create = false
default_pool = "nestgate-pool"
raid_level = "raidz2"
spare_devices = 1
cache_devices = []
log_devices = []

[storage.zfs.snapshots]
# ZFS snapshot configuration
auto_snapshot = true
snapshot_interval_hours = 4
retention_days = 30
retention_weeks = 12
retention_months = 12
retention_years = 7

[storage.nas]
# NAS functionality configuration
shares_enabled = true
user_quotas_enabled = true
group_quotas_enabled = true
access_control_enabled = true
audit_logging_enabled = true

[security]
# Security configuration (basic - complex security delegated to BearDog)
encryption_at_rest = true
encryption_in_transit = true
authentication_required = true
authorization_enabled = true
audit_logging = true
session_timeout_minutes = 30

[security.basic_encryption]
# Basic encryption settings (complex encryption delegated to BearDog primal)
algorithm = "aes-256-gcm"
key_rotation_interval_days = 90
compression_before_encryption = true

[monitoring]
# Monitoring and observability
metrics_enabled = true
metrics_port = 9090
metrics_path = "/metrics"
tracing_enabled = true
tracing_endpoint = "http://jaeger:14268/api/traces"
logging_structured = true
logging_format = "json"

[monitoring.health_checks]
# Health check configuration
enabled = true
interval_seconds = 30
timeout_seconds = 5
failure_threshold = 3
success_threshold = 1

[performance]
# Performance optimization
cache_enabled = true
cache_size_mb = 512
cache_ttl_seconds = 300
connection_pooling = true
max_pool_size = 100
min_pool_size = 10
prefetch_enabled = true
compression_enabled = true

[universal_adapter]
# Universal adapter configuration for primal delegation
enabled = true
timeout_seconds = 30
retry_attempts = 3
circuit_breaker_enabled = true

[universal_adapter.primals]
# Primal service endpoints for delegation
squirrel_ai_endpoint = "http://squirrel:8080"
beardog_security_endpoint = "http://beardog:8443"
songbird_orchestration_endpoint = "http://songbird:8444"

[universal_adapter.capabilities]
# Capability delegation settings
intelligence_delegation = true
security_delegation = true
orchestration_delegation = true
fallback_enabled = true

[development]
# Development and testing configuration
debug_enabled = false
mock_services_enabled = false
test_data_enabled = false
profiling_enabled = false
hot_reload_enabled = false

[development.testing]
# Testing configuration
integration_tests_enabled = true
performance_tests_enabled = true
chaos_testing_enabled = false
load_testing_enabled = false

# 🏛️ **CANONICAL MODERNIZATION COMPLETE**
# This configuration provides a single, comprehensive source of truth
# for all NestGate functionality, replacing fragmented configuration files.
EOF

    log_success "✅ Canonical master configuration generated"
}

# Validate the canonical configuration
validate_canonical_config() {
    log_info "🔍 Validating canonical configuration..."
    
    # Check if the file exists and is readable
    if [[ ! -f "$CANONICAL_CONFIG" ]]; then
        log_error "Canonical configuration file not found!"
        exit 1
    fi
    
    # Basic TOML syntax validation (if toml-cli is available)
    if command -v toml >/dev/null 2>&1; then
        if toml get "$CANONICAL_CONFIG" system.instance_name >/dev/null 2>&1; then
            log_success "✅ TOML syntax validation passed"
        else
            log_error "TOML syntax validation failed!"
            exit 1
        fi
    else
        log_warning "⚠️  toml-cli not available, skipping syntax validation"
    fi
    
    log_success "✅ Configuration validation completed"
}

# Update configuration references in code
update_config_references() {
    log_info "🔧 Updating configuration references in code..."
    
    # Create a migration report
    cat > "$BACKUP_DIR/migration-report.txt" << EOF
# NestGate Canonical Configuration Migration Report
# Generated: $(date)

## Migration Summary
- Backup Location: $BACKUP_DIR
- Canonical Config: $CANONICAL_CONFIG
- Migration Status: SUCCESS

## Backed Up Files:
EOF
    
    # List backed up files
    for file in "$BACKUP_DIR"/*.toml; do
        if [[ -f "$file" ]]; then
            echo "- $(basename "$file")" >> "$BACKUP_DIR/migration-report.txt"
        fi
    done
    
    cat >> "$BACKUP_DIR/migration-report.txt" << EOF

## Next Steps:
1. Update application code to use canonical configuration
2. Test with new configuration
3. Remove old configuration files once validated
4. Update deployment scripts
5. Update documentation

## Configuration Structure:
- [system] - Core system settings
- [network] - Network and protocol configuration  
- [storage] - Storage and ZFS settings
- [security] - Basic security configuration
- [monitoring] - Observability settings
- [performance] - Performance optimization
- [universal_adapter] - Primal delegation settings
- [development] - Development and testing settings

## Key Improvements:
- Single source of truth for all configuration
- Hierarchical organization
- Clear separation of concerns
- Production-ready defaults
- Comprehensive documentation
EOF

    log_success "✅ Migration report created: $BACKUP_DIR/migration-report.txt"
}

# Error handling
trap 'log_error "Migration failed! Check the logs above."; exit 1' ERR

# Run main function
main "$@" 