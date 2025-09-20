#!/bin/bash

# 🎯 PRIMAL ENVIRONMENT VARIABLE MIGRATION SCRIPT
# Converts hardcoded primal endpoints to capability-based discovery patterns

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BACKUP_DIR="$PROJECT_ROOT/migration-backup-$(date +%Y%m%d-%H%M%S)"

echo "🚀 Starting Primal Environment Variable Migration"
echo "📂 Project Root: $PROJECT_ROOT"
echo "💾 Backup Directory: $BACKUP_DIR"

# Create backup
mkdir -p "$BACKUP_DIR"

# Function to backup and migrate a file
migrate_file() {
    local file="$1"
    local description="$2"
    
    if [[ ! -f "$file" ]]; then
        echo "⚠️  File not found: $file"
        return
    fi
    
    echo "🔄 Migrating $description: $file"
    
    # Create backup
    cp "$file" "$BACKUP_DIR/$(basename "$file").backup"
    
    # Apply migrations
    sed -i.tmp \
        -e 's/NESTGATE_SONGBIRD_ENDPOINT=/ORCHESTRATION_DISCOVERY_ENDPOINT=/g' \
        -e 's/NESTGATE_BEARDOG_ENDPOINT=/SECURITY_DISCOVERY_ENDPOINT=/g' \
        -e 's/NESTGATE_SQUIRREL_ENDPOINT=/AI_DISCOVERY_ENDPOINT=/g' \
        -e 's/NESTGATE_TOADSTOOL_ENDPOINT=/COMPUTE_DISCOVERY_ENDPOINT=/g' \
        -e 's/NESTGATE_BIOMEOS_ENDPOINT=/MANAGEMENT_DISCOVERY_ENDPOINT=/g' \
        -e 's/http:\/\/songbird:8081/http:\/\/discovery:8081\/capabilities\/orchestration/g' \
        -e 's/http:\/\/beardog:8082/http:\/\/discovery:8082\/capabilities\/security/g' \
        -e 's/http:\/\/squirrel:8083/http:\/\/discovery:8083\/capabilities\/artificial_intelligence/g' \
        -e 's/http:\/\/toadstool:8084/http:\/\/discovery:8084\/capabilities\/compute/g' \
        -e 's/http:\/\/biomeos:8085/http:\/\/discovery:8085\/capabilities\/management/g' \
        "$file"
    
    # Remove temporary file
    rm -f "$file.tmp"
    
    echo "✅ Migrated: $file"
}

# Function to add universal adapter configuration
add_universal_adapter_config() {
    local file="$1"
    
    if [[ ! -f "$file" ]]; then
        return
    fi
    
    echo "🔧 Adding universal adapter configuration to: $file"
    
    # Add universal adapter section if it doesn't exist
    if ! grep -q "\[universal_adapter\]" "$file"; then
        cat >> "$file" << 'EOF'

# ✅ UNIVERSAL ADAPTER CONFIGURATION
# Replaces hardcoded primal endpoints with capability-based discovery
[universal_adapter]
enabled = true
discovery_methods = ["environment", "network_scan", "service_registry", "capability_broadcast"]
discovery_timeout_ms = 5000
capability_cache_ttl_seconds = 300
fallback_strategy = "graceful_degradation"

[universal_adapter.capabilities]
# Dynamic capability discovery - no hardcoded primal names
orchestration = { required = false, fallback_strategy = "local" }
security = { required = false, fallback_strategy = "basic_auth" }
artificial_intelligence = { required = false, fallback_strategy = "mock" }
compute = { required = false, fallback_strategy = "local" }
management = { required = false, fallback_strategy = "self_managed" }

[universal_adapter.discovery]
# Service discovery endpoints (capability-based)
orchestration_discovery_endpoint = "${ORCHESTRATION_DISCOVERY_ENDPOINT:-http://discovery:8081/capabilities/orchestration}"
security_discovery_endpoint = "${SECURITY_DISCOVERY_ENDPOINT:-http://discovery:8082/capabilities/security}"
ai_discovery_endpoint = "${AI_DISCOVERY_ENDPOINT:-http://discovery:8083/capabilities/artificial_intelligence}"
compute_discovery_endpoint = "${COMPUTE_DISCOVERY_ENDPOINT:-http://discovery:8084/capabilities/compute}"
management_discovery_endpoint = "${MANAGEMENT_DISCOVERY_ENDPOINT:-http://discovery:8085/capabilities/management}"
EOF
    fi
}

# Function to add deprecation comments
add_deprecation_comments() {
    local file="$1"
    
    if [[ ! -f "$file" ]]; then
        return
    fi
    
    echo "📝 Adding deprecation comments to: $file"
    
    # Add deprecation header
    sed -i '1i\
# 🚨 PRIMAL HARDCODING MIGRATION IN PROGRESS\
# Legacy primal-specific endpoints are being replaced with capability-based discovery\
# See PRIMAL_HARDCODING_ELIMINATION_PLAN.md for details\
' "$file"
}

echo "🔄 Phase 1: Migrating Environment Files"

# Migrate deployment environment files
migrate_file "$PROJECT_ROOT/deploy/production.env" "Production Environment"
migrate_file "$PROJECT_ROOT/deploy/staging-universal-adapter.env" "Staging Environment"

echo "🔄 Phase 2: Migrating Docker Compose Files"

# Migrate Docker Compose files
migrate_file "$PROJECT_ROOT/deploy/production.yml" "Production Docker Compose"
migrate_file "$PROJECT_ROOT/deploy/unified-production.yml" "Unified Production Docker Compose"

echo "🔄 Phase 3: Migrating Configuration Files"

# Migrate TOML configuration files
migrate_file "$PROJECT_ROOT/config/canonical-master.toml" "Canonical Master Config"
migrate_file "$PROJECT_ROOT/config/enterprise-production.toml" "Enterprise Production Config"
migrate_file "$PROJECT_ROOT/config/production.toml" "Production Config"

# Add universal adapter configurations
add_universal_adapter_config "$PROJECT_ROOT/config/canonical-master.toml"

echo "🔄 Phase 4: Updating Documentation References"

# Find and update documentation files
find "$PROJECT_ROOT/docs" -name "*.md" -type f | while read -r doc_file; do
    if grep -q "NESTGATE_SONGBIRD\|NESTGATE_BEARDOG\|NESTGATE_SQUIRREL\|NESTGATE_TOADSTOOL\|NESTGATE_BIOMEOS" "$doc_file"; then
        echo "📝 Updating documentation: $doc_file"
        cp "$doc_file" "$BACKUP_DIR/$(basename "$doc_file").backup"
        
        sed -i \
            -e 's/NESTGATE_SONGBIRD_ENDPOINT/ORCHESTRATION_DISCOVERY_ENDPOINT/g' \
            -e 's/NESTGATE_BEARDOG_ENDPOINT/SECURITY_DISCOVERY_ENDPOINT/g' \
            -e 's/NESTGATE_SQUIRREL_ENDPOINT/AI_DISCOVERY_ENDPOINT/g' \
            -e 's/NESTGATE_TOADSTOOL_ENDPOINT/COMPUTE_DISCOVERY_ENDPOINT/g' \
            -e 's/NESTGATE_BIOMEOS_ENDPOINT/MANAGEMENT_DISCOVERY_ENDPOINT/g' \
            "$doc_file"
    fi
done

echo "🔄 Phase 5: Creating Migration Summary"

# Create migration summary
cat > "$PROJECT_ROOT/PRIMAL_MIGRATION_SUMMARY.md" << EOF
# 🎯 PRIMAL HARDCODING MIGRATION SUMMARY

**Migration Date**: $(date)
**Backup Location**: $BACKUP_DIR

## ✅ COMPLETED MIGRATIONS

### Environment Variables Migrated:
- \`NESTGATE_SONGBIRD_ENDPOINT\` → \`ORCHESTRATION_DISCOVERY_ENDPOINT\`
- \`NESTGATE_BEARDOG_ENDPOINT\` → \`SECURITY_DISCOVERY_ENDPOINT\`
- \`NESTGATE_SQUIRREL_ENDPOINT\` → \`AI_DISCOVERY_ENDPOINT\`
- \`NESTGATE_TOADSTOOL_ENDPOINT\` → \`COMPUTE_DISCOVERY_ENDPOINT\`
- \`NESTGATE_BIOMEOS_ENDPOINT\` → \`MANAGEMENT_DISCOVERY_ENDPOINT\`

### Files Updated:
- deploy/production.env
- deploy/staging-universal-adapter.env
- deploy/production.yml
- deploy/unified-production.yml
- config/canonical-master.toml
- config/enterprise-production.toml
- config/production.toml
- Documentation files in docs/

### Universal Adapter Configuration Added:
- Capability-based discovery system
- Fallback strategies for all capabilities
- Environment-driven discovery endpoints
- Graceful degradation patterns

## 🎯 NEXT STEPS

1. **Test Configuration**: Validate all services start with new environment variables
2. **Code Migration**: Update code references to use universal adapter patterns
3. **Validation**: Run sovereignty compliance tests
4. **Documentation**: Update remaining references

## 🔄 ROLLBACK PROCEDURE

If rollback is needed:
\`\`\`bash
# Restore from backup
cp $BACKUP_DIR/*.backup ./deploy/
cp $BACKUP_DIR/*.backup ./config/
\`\`\`

**Status**: ✅ Environment Variable Migration Complete
EOF

echo "🎉 Migration Complete!"
echo ""
echo "📊 SUMMARY:"
echo "  ✅ Environment variables migrated to capability-based discovery"
echo "  ✅ Configuration files updated with universal adapter patterns"
echo "  ✅ Documentation references updated"
echo "  ✅ Backup created at: $BACKUP_DIR"
echo ""
echo "🎯 NEXT STEPS:"
echo "  1. Review changes: git diff"
echo "  2. Test configuration: docker-compose -f deploy/production.yml config"
echo "  3. Run sovereignty tests: ./scripts/test_sovereignty_compliance.sh"
echo "  4. Proceed with code migration phase"
echo ""
echo "📖 See PRIMAL_MIGRATION_SUMMARY.md for detailed migration report" 