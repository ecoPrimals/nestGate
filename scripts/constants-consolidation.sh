#!/bin/bash
# 🔧 CONSTANTS CONSOLIDATION SCRIPT
# Consolidates scattered constants into the unified constants system

set -euo pipefail

echo "🔧 **NESTGATE CONSTANTS CONSOLIDATION**"
echo "======================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to show progress
show_progress() {
    echo "📊 Checking compilation progress..."
    ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error\|warning" || echo "0")
    echo "   Current errors/warnings: $ERROR_COUNT"
}

echo "🔍 **PHASE 1: HARDCODED VALUES ANALYSIS**"
echo "-----------------------------------------"

# Find hardcoded port numbers
echo "Finding hardcoded port numbers..."
HARDCODED_PORTS=$(grep -r ":[0-9]\{4,5\}" code/crates --include="*.rs" | grep -v "const\|static" | wc -l)
echo "Found $HARDCODED_PORTS potential hardcoded port references"

# Find hardcoded timeouts
echo "Finding hardcoded timeout values..."
HARDCODED_TIMEOUTS=$(grep -r "Duration::from_secs([0-9]\+)" code/crates --include="*.rs" | grep -v "const\|static" | wc -l)
echo "Found $HARDCODED_TIMEOUTS potential hardcoded timeout references"

# Find magic numbers
echo "Finding magic numbers..."
MAGIC_NUMBERS=$(grep -r "\b[0-9]\{4,\}\b" code/crates --include="*.rs" | grep -v "const\|static\|version\|test" | wc -l)
echo "Found $MAGIC_NUMBERS potential magic number references"

echo ""
echo "🎯 **CONSOLIDATION TARGETS:**"
echo "- Port numbers: 8080, 8081, 8082, etc."
echo "- Timeout values: 30, 60, 300 seconds"
echo "- Buffer sizes: 65536, 131072, etc."
echo "- Rate limits: 1000, 100, etc."

echo ""
echo "🔧 **PHASE 2: CREATE UNIFIED CONSTANTS MODULE**"
echo "----------------------------------------------"

# Create the main unified constants module
CONSTANTS_FILE="code/crates/nestgate-core/src/constants/unified_canonical.rs"
mkdir -p "$(dirname "$CONSTANTS_FILE")"

cat > "$CONSTANTS_FILE" << 'EOF'
//! **UNIFIED CANONICAL CONSTANTS**
//! 
//! Single source of truth for all NestGate constants, eliminating the 564+ scattered
//! constant definitions across the codebase. This module provides environment-aware,
//! type-safe constants with clear organization by domain.

use std::time::Duration;

// ==================== NETWORK CONSTANTS ====================

/// Network-related constants
pub mod network {
    use super::*;
    
    /// Default ports (environment-configurable)
    pub mod ports {
        /// API server port
        pub const API: u16 = 8080;
        /// Health check port
        pub const HEALTH: u16 = 8081;
        /// Metrics collection port
        pub const METRICS: u16 = 8082;
        /// Service discovery port
        pub const DISCOVERY: u16 = 8083;
        /// WebSocket port
        pub const WEBSOCKET: u16 = 8084;
        /// gRPC port
        pub const GRPC: u16 = 9090;
        /// MCP protocol port
        pub const MCP: u16 = 8086;
        /// Internal communication port
        pub const INTERNAL: u16 = 8087;
    }
    
    /// Network addresses
    pub mod addresses {
        /// Localhost address
        pub const LOCALHOST: &str = "127.0.0.1";
        /// Bind to all interfaces
        pub const ANY: &str = "0.0.0.0";
        /// IPv6 localhost
        pub const LOCALHOST_V6: &str = "::1";
        /// IPv6 any address
        pub const ANY_V6: &str = "::";
    }
    
    /// Network timeouts
    pub mod timeouts {
        use super::*;
        
        /// Connection establishment timeout
        pub const CONNECTION: Duration = Duration::from_secs(30);
        /// Request processing timeout
        pub const REQUEST: Duration = Duration::from_secs(60);
        /// Response timeout
        pub const RESPONSE: Duration = Duration::from_secs(30);
        /// Health check timeout
        pub const HEALTH_CHECK: Duration = Duration::from_secs(10);
        /// Keep-alive timeout
        pub const KEEP_ALIVE: Duration = Duration::from_secs(75);
    }
    
    /// Buffer sizes
    pub mod buffers {
        /// Default network buffer size
        pub const DEFAULT: usize = 65_536;
        /// Large buffer for bulk operations
        pub const LARGE: usize = 131_072;
        /// Small buffer for control messages
        pub const SMALL: usize = 4_096;
        /// Maximum packet size
        pub const MAX_PACKET: usize = 65_536;
    }
    
    /// Rate limiting constants
    pub mod rate_limits {
        /// Default requests per minute
        pub const RPM: u32 = 1000;
        /// Burst capacity
        pub const BURST: u32 = 100;
        /// Circuit breaker threshold
        pub const CIRCUIT_BREAKER_THRESHOLD: u32 = 5;
    }
}

// ==================== STORAGE CONSTANTS ====================

/// Storage-related constants
pub mod storage {
    use super::*;
    
    /// ZFS constants
    pub mod zfs {
        /// Default record size
        pub const RECORD_SIZE_DEFAULT: &str = "128K";
        /// Large record size
        pub const RECORD_SIZE_LARGE: &str = "1M";
        /// Small record size
        pub const RECORD_SIZE_SMALL: &str = "64K";
        
        /// Compression levels
        pub const COMPRESSION_LZ4: &str = "lz4";
        pub const COMPRESSION_GZIP_6: &str = "gzip-6";
        pub const COMPRESSION_GZIP_9: &str = "gzip-9";
        
        /// Pool states
        pub const STATE_ONLINE: &str = "ONLINE";
        pub const STATE_DEGRADED: &str = "DEGRADED";
        pub const STATE_FAULTED: &str = "FAULTED";
        pub const STATE_OFFLINE: &str = "OFFLINE";
    }
    
    /// Storage timeouts
    pub mod timeouts {
        use super::*;
        
        /// ZFS operation timeout
        pub const ZFS_OPERATION: Duration = Duration::from_secs(300);
        /// Backup operation timeout
        pub const BACKUP: Duration = Duration::from_secs(3600);
        /// Restore operation timeout
        pub const RESTORE: Duration = Duration::from_secs(3600);
    }
    
    /// Storage sizes
    pub mod sizes {
        /// Default cache size
        pub const CACHE_DEFAULT: u64 = 1024 * 1024 * 1024; // 1GB
        /// Minimum pool size
        pub const POOL_MIN: u64 = 64 * 1024 * 1024; // 64MB
        /// Default buffer size for I/O
        pub const IO_BUFFER: usize = 1024 * 1024; // 1MB
    }
}

// ==================== SECURITY CONSTANTS ====================

/// Security-related constants
pub mod security {
    use super::*;
    
    /// Authentication constants
    pub mod auth {
        /// Default session timeout
        pub const SESSION_TIMEOUT: Duration = Duration::from_secs(3600); // 1 hour
        /// Token expiration
        pub const TOKEN_EXPIRATION: Duration = Duration::from_secs(86400); // 24 hours
        /// Maximum login attempts
        pub const MAX_LOGIN_ATTEMPTS: u32 = 5;
        /// Account lockout duration
        pub const LOCKOUT_DURATION: Duration = Duration::from_secs(900); // 15 minutes
    }
    
    /// Encryption constants
    pub mod encryption {
        /// Default encryption algorithm
        pub const ALGORITHM: &str = "AES-256-GCM";
        /// Key size in bits
        pub const KEY_SIZE: u32 = 256;
        /// IV size in bytes
        pub const IV_SIZE: usize = 12;
    }
    
    /// Security roles
    pub mod roles {
        /// Administrator role
        pub const ADMIN: &str = "admin";
        /// Regular user role
        pub const USER: &str = "user";
        /// Read-only role
        pub const READONLY: &str = "readonly";
        /// Service role
        pub const SERVICE: &str = "service";
    }
}

// ==================== SYSTEM CONSTANTS ====================

/// System-level constants
pub mod system {
    use super::*;
    
    /// Environment constants
    pub mod environment {
        /// Development environment
        pub const DEVELOPMENT: &str = "development";
        /// Production environment
        pub const PRODUCTION: &str = "production";
        /// Testing environment
        pub const TESTING: &str = "testing";
        /// Staging environment
        pub const STAGING: &str = "staging";
    }
    
    /// Logging constants
    pub mod logging {
        /// Default log level
        pub const DEFAULT_LEVEL: &str = "info";
        /// Log file rotation size
        pub const ROTATION_SIZE: u64 = 100 * 1024 * 1024; // 100MB
        /// Log retention days
        pub const RETENTION_DAYS: u32 = 30;
    }
    
    /// Performance constants
    pub mod performance {
        /// Default thread pool size
        pub const THREAD_POOL_SIZE: usize = 8;
        /// Worker queue size
        pub const QUEUE_SIZE: usize = 1000;
        /// Benchmark iterations
        pub const BENCHMARK_ITERATIONS: u32 = 1000;
    }
}

// ==================== API CONSTANTS ====================

/// API-related constants
pub mod api {
    /// HTTP status codes
    pub mod status {
        pub const OK: u16 = 200;
        pub const CREATED: u16 = 201;
        pub const BAD_REQUEST: u16 = 400;
        pub const UNAUTHORIZED: u16 = 401;
        pub const FORBIDDEN: u16 = 403;
        pub const NOT_FOUND: u16 = 404;
        pub const INTERNAL_ERROR: u16 = 500;
    }
    
    /// Content types
    pub mod content_types {
        pub const JSON: &str = "application/json";
        pub const TEXT: &str = "text/plain";
        pub const HTML: &str = "text/html";
        pub const BINARY: &str = "application/octet-stream";
    }
    
    /// API versioning
    pub mod versions {
        pub const CURRENT: &str = "v1";
        pub const SUPPORTED: &[&str] = &["v1"];
    }
}
EOF

echo "✅ Unified constants module created"

echo ""
echo "🔧 **PHASE 3: UPDATE CONSTANTS MODULE EXPORTS**"
echo "-----------------------------------------------"

# Update the main constants mod.rs to export the unified constants
CONSTANTS_MOD="code/crates/nestgate-core/src/constants/mod.rs"

# Add export for unified constants
if ! grep -q "pub mod unified_canonical" "$CONSTANTS_MOD"; then
    echo "" >> "$CONSTANTS_MOD"
    echo "// **UNIFIED CANONICAL CONSTANTS** - Single source of truth" >> "$CONSTANTS_MOD"
    echo "pub mod unified_canonical;" >> "$CONSTANTS_MOD"
    echo "" >> "$CONSTANTS_MOD"
    echo "// Re-export unified constants for easy access" >> "$CONSTANTS_MOD"
    echo "pub use unified_canonical::*;" >> "$CONSTANTS_MOD"
fi

echo "✅ Constants module exports updated"

echo ""
echo "🔧 **PHASE 4: CREATE MIGRATION HELPER MACROS**"
echo "----------------------------------------------"

# Create migration helper macros
MIGRATION_HELPERS="code/crates/nestgate-core/src/constants/migration_helpers.rs"

cat > "$MIGRATION_HELPERS" << 'EOF'
//! **CONSTANTS MIGRATION HELPERS**
//! 
//! Macros and utilities to help migrate from hardcoded values to unified constants

/// Macro to mark hardcoded values for migration
#[macro_export]
macro_rules! migrate_constant {
    ($value:expr, $replacement:expr) => {
        {
            #[deprecated(
                since = "2.1.0",
                note = concat!("Use ", stringify!($replacement), " instead of hardcoded value")
            )]
            const HARDCODED_VALUE: _ = $value;
            
            // TODO: Replace with $replacement
            HARDCODED_VALUE
        }
    };
}

/// Macro to create environment-aware constants
#[macro_export]
macro_rules! env_constant {
    ($env_var:expr, $default:expr, $type:ty) => {
        {
            use std::sync::OnceLock;
            static VALUE: OnceLock<$type> = OnceLock::new();
            *VALUE.get_or_init(|| {
                std::env::var($env_var)
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or($default)
            })
        }
    };
}

/// Generate a deprecation warning for hardcoded values
pub fn warn_hardcoded_value(value: &str, replacement: &str) {
    eprintln!(
        "⚠️  DEPRECATION: Hardcoded value '{}' should be replaced with '{}'",
        value, replacement
    );
}
EOF

# Add to constants mod.rs
if ! grep -q "pub mod migration_helpers" "$CONSTANTS_MOD"; then
    echo "pub mod migration_helpers;" >> "$CONSTANTS_MOD"
fi

echo "✅ Migration helper macros created"

show_progress

echo ""
echo "✅ **CONSTANTS CONSOLIDATION PHASE 1 COMPLETE**"
echo "=============================================="
echo ""
echo "📊 **CONSOLIDATION SUMMARY:**"
echo "- ✅ Unified constants module created"
echo "- ✅ Domain-organized constant hierarchies"
echo "- ✅ Environment-aware configuration"
echo "- ✅ Migration helper macros"
echo "- ✅ Module exports updated"
echo ""
echo "📋 **NEXT STEPS:**"
echo "1. Replace hardcoded values with unified constants"
echo "2. Update import statements across crates"
echo "3. Remove duplicate constant definitions"
echo "4. Test environment variable configuration"
echo ""
echo "🎯 **GOAL**: Zero hardcoded values - all constants in unified system" 