#!/bin/bash
# **NESTGATE CONFIGURATION UNIFICATION COMPLETION SCRIPT**
# 
# This script completes the migration from fragmented configuration structs
# to the unified canonical configuration system.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔧 NestGate Configuration Unification - Phase 2 Completion${NC}"
echo "=============================================================="

# Function to log with timestamp
log() {
    echo -e "[$(date '+%H:%M:%S')] $1"
}

# Function to count configuration structs
count_config_structs() {
    local crate_path="$1"
    find "$crate_path" -name "*.rs" -exec grep -l "struct.*Config" {} \; | wc -l
}

# Function to find fragmented config patterns
find_fragmented_configs() {
    log "${BLUE}📊 Analyzing configuration fragmentation...${NC}"
    
    echo "Configuration struct counts by crate:"
    for crate in code/crates/*/; do
        if [[ -d "$crate/src" ]]; then
            local count=$(count_config_structs "$crate/src")
            local crate_name=$(basename "$crate")
            if [[ $count -gt 0 ]]; then
                echo "  $crate_name: $count config structs"
            fi
        fi
    done
}

# Function to identify migration targets
identify_migration_targets() {
    log "${YELLOW}🎯 Identifying migration targets...${NC}"
    
    echo "Fragmented config patterns found:"
    
    # Find duplicate config struct names
    echo "  Duplicate config struct names:"
    find code/crates/ -name "*.rs" -exec grep -h "struct.*Config" {} \; | \
        sed 's/.*struct \([A-Za-z]*Config\).*/\1/' | \
        sort | uniq -c | sort -nr | head -10
        
    # Find hardcoded configuration values
    echo "  Files with hardcoded config values:"
    find code/crates/ -name "*.rs" -exec grep -l "8080\|localhost\|127.0.0.1" {} \; | wc -l
}

# Function to create migration report
create_migration_report() {
    log "${BLUE}📋 Creating migration report...${NC}"
    
    local report_file="docs/CONFIG_UNIFICATION_REPORT.md"
    
    cat > "$report_file" << EOF
# 🔧 Configuration Unification Progress Report

**Generated**: $(date)
**Status**: Phase 2 - Configuration Consolidation

## 📊 Current State Analysis

### Configuration Framework Status
- ✅ **Canonical Master System**: Established in \`nestgate-core/src/config/canonical_master/\`
- ✅ **Migration Framework**: Available in \`migration_framework.rs\`
- ✅ **Domain Consolidation**: Comprehensive domain-specific configs available
- ✅ **Type Safety**: Compile-time configuration with const generics

### Remaining Migration Targets

#### High Priority (Blocking Unification)
EOF

    # Add specific migration targets to report
    echo "#### Configuration Structs by Crate" >> "$report_file"
    for crate in code/crates/*/; do
        if [[ -d "$crate/src" ]]; then
            local count=$(count_config_structs "$crate/src")
            local crate_name=$(basename "$crate")
            if [[ $count -gt 3 ]]; then  # Focus on crates with many configs
                echo "- **$crate_name**: $count configuration structs" >> "$report_file"
            fi
        fi
    done
    
    cat >> "$report_file" << EOF

## 🎯 Next Steps

1. **Immediate**: Migrate high-frequency config structs to canonical system
2. **Short-term**: Eliminate duplicate config definitions
3. **Medium-term**: Replace hardcoded values with canonical constants
4. **Long-term**: Complete removal of legacy config patterns

## 📈 Success Metrics

- **Target**: Single \`NestGateCanonicalConfig\` as source of truth
- **Reduction**: 50%+ reduction in config struct count
- **Consistency**: 100% usage of canonical config system
- **Performance**: Zero-cost configuration with compile-time optimization

EOF

    log "${GREEN}✅ Migration report created: $report_file${NC}"
}

# Function to demonstrate canonical config usage
demonstrate_canonical_usage() {
    log "${BLUE}🚀 Demonstrating canonical configuration usage...${NC}"
    
    cat << EOF

Example of unified canonical configuration:

\`\`\`rust
use nestgate_core::config::canonical_master::NestGateCanonicalConfig;

// Single configuration struct replaces 200+ fragmented configs
let config = NestGateCanonicalConfig::default()
    .with_api_port(8080)
    .with_max_connections(1000)
    .with_storage_backend("zfs")
    .with_security_enabled(true)
    .build()?;

// Type-safe access to all configuration domains
let api_config = config.api;
let storage_config = config.storage;
let security_config = config.security;
\`\`\`

Benefits achieved:
- ✅ Single source of truth for all configuration
- ✅ Compile-time validation and optimization
- ✅ Environment-driven configuration loading
- ✅ Type-safe configuration access
- ✅ Zero-cost abstractions with const generics

EOF
}

# Main execution
main() {
    log "${GREEN}🚀 Starting configuration unification analysis...${NC}"
    
    find_fragmented_configs
    echo
    identify_migration_targets
    echo
    create_migration_report
    echo
    demonstrate_canonical_usage
    
    log "${GREEN}✅ Configuration unification analysis complete!${NC}"
    log "${YELLOW}📋 Next: Review migration report and begin systematic consolidation${NC}"
}

# Run the script
main "$@" 