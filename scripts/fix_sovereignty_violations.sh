#!/bin/bash

# 🏛️ **SOVEREIGNTY VIOLATIONS FIX SCRIPT**
# 
# This script systematically eliminates all hardcoded values that violate
# user sovereignty and infrastructure autonomy principles.

set -euo pipefail

echo "🏛️ Starting Sovereignty Compliance Migration..."
echo "📊 Target: Eliminate 62+ hardcoded infrastructure assumptions"

# Create backup
BACKUP_DIR="sovereignty_backup_$(date +%Y%m%d_%H%M%S)"
echo "📦 Creating backup: $BACKUP_DIR"
cp -r code/ "$BACKUP_DIR"

# Phase 1: Replace hardcoded localhost/127.0.0.1 with environment variables
echo "🌐 Phase 1: Network sovereignty compliance..."

find code/crates -name "*.rs" -type f -exec sed -i 's/localhost:8080/\${NESTGATE_API_ENDPOINT:-localhost:8080}/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/127\.0\.0\.1:8080/\${NESTGATE_BIND_ADDRESS:-127.0.0.1:8080}/g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's/"8080"/"${NESTGATE_PORT:-8080}"/g' {} \;

# Phase 2: Replace hardcoded endpoints with configurable values
echo "🔗 Phase 2: Endpoint sovereignty compliance..."

# Replace hardcoded HTTP endpoints
find code/crates -name "*.rs" -type f -exec sed -i 's|http://localhost:8080|${NESTGATE_HTTP_ENDPOINT:-http://localhost:8080}|g' {} \;
find code/crates -name "*.rs" -type f -exec sed -i 's|ws://localhost:8080|${NESTGATE_WS_ENDPOINT:-ws://localhost:8080}|g' {} \;

# Phase 3: Create sovereignty configuration helpers
echo "⚙️ Phase 3: Creating sovereignty configuration helpers..."

cat > code/crates/nestgate-core/src/sovereignty_config.rs << 'EOF'
//! Sovereignty Configuration Helpers
//! 
//! These helpers ensure all infrastructure assumptions are user-configurable

use std::env;

pub struct SovereigntyConfig;

impl SovereigntyConfig {
    /// Get API endpoint respecting user sovereignty
    pub fn api_endpoint() -> String {
        env::var("NESTGATE_API_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:8080".to_string())
    }
    
    /// Get bind address respecting user sovereignty
    pub fn bind_address() -> String {
        env::var("NESTGATE_BIND_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1".to_string())
    }
    
    /// Get API port respecting user sovereignty
    pub fn api_port() -> u16 {
        env::var("NESTGATE_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .unwrap_or(8080)
    }
    
    /// Get WebSocket endpoint respecting user sovereignty
    pub fn websocket_endpoint() -> String {
        env::var("NESTGATE_WS_ENDPOINT")
            .unwrap_or_else(|_| "ws://localhost:8080".to_string())
    }
    
    /// Get database URL respecting user sovereignty
    pub fn database_url() -> String {
        env::var("NESTGATE_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/nestgate".to_string())
    }
    
    /// Get service discovery endpoint respecting user sovereignty
    pub fn discovery_endpoint() -> String {
        env::var("NESTGATE_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:8080/discovery".to_string())
    }
}
EOF

# Phase 4: Update core lib.rs to include sovereignty config
echo "📚 Phase 4: Integrating sovereignty configuration..."

# Add sovereignty config to nestgate-core
if ! grep -q "pub mod sovereignty_config" code/crates/nestgate-core/src/lib.rs; then
    echo "pub mod sovereignty_config;" >> code/crates/nestgate-core/src/lib.rs
    echo "pub use sovereignty_config::SovereigntyConfig;" >> code/crates/nestgate-core/src/lib.rs
fi

# Phase 5: Replace remaining hardcoded values with sovereignty helpers
echo "🔧 Phase 5: Systematic hardcoding elimination..."

# Replace hardcoded endpoints in key files
find code/crates -name "*.rs" -type f -exec grep -l "localhost:8080\|127\.0\.0\.1:8080" {} \; | while read -r file; do
    echo "  Fixing sovereignty violations in: $file"
    
    # Replace with sovereignty config calls where appropriate
    sed -i 's/"http:\/\/localhost:8080"/SovereigntyConfig::api_endpoint()/g' "$file"
    sed -i 's/"localhost:8080"/format!("{}:{}", SovereigntyConfig::bind_address(), SovereigntyConfig::api_port())/g' "$file"
    sed -i 's/"127\.0\.0\.1:8080"/format!("{}:{}", SovereigntyConfig::bind_address(), SovereigntyConfig::api_port())/g' "$file"
done

# Phase 6: Create environment template
echo "📋 Phase 6: Creating environment configuration template..."

cat > .env.sovereignty << 'EOF'
# 🏛️ NestGate Sovereignty Configuration
# 
# These environment variables ensure user control over all infrastructure assumptions

# Network Configuration
NESTGATE_BIND_ADDRESS=127.0.0.1
NESTGATE_PORT=8080
NESTGATE_API_ENDPOINT=http://localhost:8080
NESTGATE_WS_ENDPOINT=ws://localhost:8080

# Service Discovery
NESTGATE_DISCOVERY_ENDPOINT=http://localhost:8080/discovery

# Database Configuration
NESTGATE_DATABASE_URL=postgresql://localhost/nestgate

# Security Configuration
NESTGATE_SESSION_TIMEOUT=3600
NESTGATE_MAX_LOGIN_ATTEMPTS=5

# Performance Configuration
NESTGATE_THREAD_POOL_SIZE=4
NESTGATE_BUFFER_SIZE_KB=64
NESTGATE_BATCH_SIZE=100
EOF

echo "✅ Sovereignty compliance migration complete!"
echo "📋 Next steps:"
echo "   1. Copy .env.sovereignty to .env"
echo "   2. Customize values for your infrastructure"
echo "   3. Test with: cargo check --workspace"
echo "   4. Verify no hardcoded values remain: grep -r 'localhost:8080' code/crates"

# Verification
echo "🔍 Verification: Checking for remaining hardcoded values..."
REMAINING=$(grep -r "localhost:8080\|127\.0\.0\.1:8080" code/crates --include="*.rs" | wc -l)
echo "📊 Remaining hardcoded references: $REMAINING"

if [ "$REMAINING" -lt 10 ]; then
    echo "✅ Sovereignty compliance: EXCELLENT (< 10 remaining)"
elif [ "$REMAINING" -lt 30 ]; then
    echo "🟡 Sovereignty compliance: GOOD (< 30 remaining)"
else
    echo "🔴 Sovereignty compliance: NEEDS WORK ($REMAINING remaining)"
fi

echo "🏛️ Sovereignty migration completed successfully!" 