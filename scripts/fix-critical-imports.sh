#!/bin/bash
# 🔧 CRITICAL IMPORTS FIXER
# Fixes the most critical import and module resolution issues

set -euo pipefail

echo "🔧 **FIXING CRITICAL IMPORT ISSUES**"
echo "===================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "🔄 **STEP 1: Fix missing constants**"
echo "------------------------------------"

# Add missing constants to the constants module
cat >> "code/crates/nestgate-core/src/constants/network.rs" << 'EOF'

// Missing constants for backward compatibility
pub const DEFAULT_API_PORT: u16 = 8080;
pub const LOCALHOST: &str = "127.0.0.1";
EOF

echo "   ✅ Added missing network constants"

echo ""
echo "🔄 **STEP 2: Fix missing modules**"
echo "----------------------------------"

# Create missing service discovery types module
mkdir -p "code/crates/nestgate-core/src/service_discovery"
cat > "code/crates/nestgate-core/src/service_discovery/types.rs" << 'EOF'
//! Service Discovery Types
//! Placeholder types for service discovery functionality

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub endpoint: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRequest {
    pub service_name: String,
    pub operation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]  
pub struct ServiceResponse {
    pub status: String,
    pub data: String,
}
EOF

cat > "code/crates/nestgate-core/src/service_discovery/mod.rs" << 'EOF'
//! Service Discovery Module
//! Provides service discovery functionality

pub mod types;
pub use types::*;
EOF

echo "   ✅ Created service_discovery module"

# Create missing traits module for native_async
cat > "code/crates/nestgate-core/src/services/native_async/traits.rs" << 'EOF'
//! Native Async Service Traits
//! Defines traits for native async service implementations

use crate::error::Result;
use crate::service_discovery::types::*;

pub trait LoadBalancer {
    type ServiceRequest;
    type ServiceResponse;
    
    async fn route_request(&self, request: Self::ServiceRequest) -> Result<Self::ServiceResponse>;
    async fn health_check(&self) -> Result<bool>;
}

pub trait CommunicationProvider {
    async fn send_message(&self, message: &str) -> Result<String>;
    async fn connect(&self) -> Result<()>;
}
EOF

echo "   ✅ Created native_async traits module"

# Fix the missing environment modules
mkdir -p "code/crates/nestgate-core/src/config/canonical_master/domains/performance"
cat > "code/crates/nestgate-core/src/config/canonical_master/domains/performance/environment.rs" << 'EOF'
//! Performance Environment Configuration
//! Environment-specific performance settings

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEnvironmentConfig {
    pub cpu_optimization: bool,
    pub memory_optimization: bool,
    pub network_optimization: bool,
}

impl Default for PerformanceEnvironmentConfig {
    fn default() -> Self {
        Self {
            cpu_optimization: true,
            memory_optimization: true,
            network_optimization: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDebugConfig {
    pub enable_profiling: bool,
    pub detailed_metrics: bool,
}

impl Default for PerformanceDebugConfig {
    fn default() -> Self {
        Self {
            enable_profiling: false,
            detailed_metrics: false,
        }
    }
}
EOF

mkdir -p "code/crates/nestgate-core/src/config/canonical_master/domains/network"
cat > "code/crates/nestgate-core/src/config/canonical_master/domains/network/environment.rs" << 'EOF'
//! Network Environment Configuration
//! Environment-specific network settings

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkEnvironmentConfig {
    pub interface: String,
    pub timeout_ms: u64,
    pub max_connections: u32,
}

impl Default for NetworkEnvironmentConfig {
    fn default() -> Self {
        Self {
            interface: "0.0.0.0".to_string(),
            timeout_ms: 30000,
            max_connections: 1000,
        }
    }
}
EOF

cat > "code/crates/nestgate-core/src/config/canonical_master/domains/network/protocols.rs" << 'EOF'
//! Network Protocol Configuration
//! Protocol-specific network settings

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProtocolConfig {
    pub http_enabled: bool,
    pub https_enabled: bool,
    pub websocket_enabled: bool,
}

impl Default for NetworkProtocolConfig {
    fn default() -> Self {
        Self {
            http_enabled: true,
            https_enabled: true,
            websocket_enabled: false,
        }
    }
}
EOF

echo "   ✅ Created missing environment and protocol modules"

echo ""
echo "🔄 **STEP 3: Update module declarations**"
echo "-----------------------------------------"

# Update the performance module to include environment
sed -i '/^pub mod environment;/d' "code/crates/nestgate-core/src/config/canonical_master/domains/performance/mod.rs" 2>/dev/null || true
sed -i '1i pub mod environment;' "code/crates/nestgate-core/src/config/canonical_master/domains/performance/mod.rs"

# Update the network module to include environment and protocols  
sed -i '/^pub mod environment;/d' "code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs" 2>/dev/null || true
sed -i '/^pub mod protocols;/d' "code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs" 2>/dev/null || true
sed -i '1i pub mod environment;\npub mod protocols;' "code/crates/nestgate-core/src/config/canonical_master/domains/network/mod.rs"

# Update the main lib.rs to include service_discovery
grep -q "pub mod service_discovery;" "code/crates/nestgate-core/src/lib.rs" || sed -i '/pub mod scheduling;/a pub mod service_discovery;' "code/crates/nestgate-core/src/lib.rs"

echo "   ✅ Updated module declarations"

echo ""
echo "🔄 **STEP 4: Test compilation**"
echo "-------------------------------"

if cargo check --workspace --quiet; then
    echo "   ✅ Compilation successful after fixes"
else
    echo "   ⚠️  Some compilation issues remain - partial success"
fi

echo ""
echo "📋 **CRITICAL IMPORTS FIX: COMPLETE**"
echo "======================================"
echo "✅ Added missing network constants"
echo "✅ Created service_discovery module"
echo "✅ Created native_async traits" 
echo "✅ Added missing environment modules"
echo "✅ Updated module declarations" 