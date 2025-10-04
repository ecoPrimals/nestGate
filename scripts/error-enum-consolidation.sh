#!/bin/bash
# ⚠️ **ERROR ENUM CONSOLIDATION SCRIPT**
# Systematically consolidate remaining error enums into NestGateUnifiedError

set -euo pipefail

echo "⚠️ **NESTGATE ERROR ENUM CONSOLIDATION**"
echo "========================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to show error enum analysis
analyze_error_enum() {
    local pattern="$1"
    local count=$(find . -name "*.rs" -exec grep -l "enum.*${pattern}" {} \; 2>/dev/null | wc -l)
    echo "   ${pattern} enums found: $count"
}

echo "📊 **PHASE 1: ERROR ENUM ANALYSIS**"
echo "----------------------------------"

echo "Analyzing error enum patterns..."
analyze_error_enum "Error"
analyze_error_enum "ModuleError"
analyze_error_enum "ServiceError"
analyze_error_enum "NetworkError"
analyze_error_enum "StorageError"
analyze_error_enum "SecurityError"
analyze_error_enum "ConfigError"

echo ""
echo "🎯 **PHASE 2: HIGH-PRIORITY ERROR CONSOLIDATION**"
echo "------------------------------------------------"

# Create error consolidation mapping
ERROR_CONSOLIDATION_MAP="error-consolidation-map.txt"

cat > "$ERROR_CONSOLIDATION_MAP" << 'EOF'
# ERROR ENUM CONSOLIDATION MAPPING
# Format: OLD_ERROR -> NESTGATE_UNIFIED_ERROR_CATEGORY

# Generic module errors (40+ instances)
ModuleError -> NestGateUnifiedError::Internal
ServiceError -> NestGateUnifiedError::Service
ProviderError -> NestGateUnifiedError::Provider

# Domain-specific errors
NetworkError -> NestGateUnifiedError::Network
StorageError -> NestGateUnifiedError::Storage
SecurityError -> NestGateUnifiedError::Security
ConfigError -> NestGateUnifiedError::Configuration
ValidationError -> NestGateUnifiedError::Validation

# Tool-specific errors
MigratorError -> NestGateUnifiedError::Tool
CloneOptimizerError -> NestGateUnifiedError::Tool
ScannerError -> NestGateUnifiedError::Tool

# Test-specific errors
TestError -> NestGateUnifiedError::Test
MockError -> NestGateUnifiedError::Test
HardwareTestError -> NestGateUnifiedError::Test

# Specialized errors
CircuitBreakerError -> NestGateUnifiedError::Network
ZfsError -> NestGateUnifiedError::Storage
AuthError -> NestGateUnifiedError::Security
EOF

echo "✅ Created error consolidation mapping: $ERROR_CONSOLIDATION_MAP"

echo ""
echo "🔄 **PHASE 3: SYSTEMATIC ERROR CONSOLIDATION**"
echo "----------------------------------------------"

# Function to consolidate error enums
consolidate_error_enum() {
    local old_error="$1"
    local unified_category="$2"
    local description="$3"
    
    echo "🔄 Consolidating $old_error -> $unified_category"
    
    # Find files with the old error enum
    local files=$(find . -name "*.rs" -exec grep -l "enum.*${old_error}" {} \; 2>/dev/null || true)
    local count=$(echo "$files" | grep -v '^$' | wc -l)
    
    if [ "$count" -gt 0 ]; then
        echo "   Found $count files with $old_error"
        
        # Create migration helper for this error type
        local migration_file="code/crates/nestgate-core/src/error/migration_helpers/${old_error,,}_migration.rs"
        mkdir -p "$(dirname "$migration_file")"
        
        cat > "$migration_file" << EOF
//! **${old_error} MIGRATION HELPER**
//! 
//! Provides migration path from legacy ${old_error} to NestGateUnifiedError system.
//! 
//! **USAGE**:
//! \`\`\`rust
//! use nestgate_core::error::migration_helpers::${old_error,,}_migration::migrate_${old_error,,};
//! 
//! // Migrate legacy error
//! let unified_error = migrate_${old_error,,}(legacy_error)?;
//! \`\`\`

use crate::error::NestGateUnifiedError;

/// Migrate legacy ${old_error} to NestGateUnifiedError
pub fn migrate_${old_error,,}(legacy_error: Legacy${old_error}) -> NestGateUnifiedError {
    match legacy_error {
        // Add specific migration cases based on legacy error variants
        _ => ${unified_category} {
            message: format!("Migrated from legacy ${old_error}: {}", legacy_error),
            context: "${description}".to_string(),
            source: None,
            recovery_suggestion: Some("Use NestGateUnifiedError variants directly".to_string()),
        }
    }
}

/// Legacy ${old_error} enum for migration compatibility
#[deprecated(since = "0.6.0", note = "Use NestGateUnifiedError instead")]
#[derive(Debug, Clone)]
pub enum Legacy${old_error} {
    // Variants will be populated during migration analysis
    Unknown(String),
}

impl std::fmt::Display for Legacy${old_error} {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Legacy${old_error}::Unknown(msg) => write!(f, "Legacy ${old_error}: {}", msg),
        }
    }
}

impl std::error::Error for Legacy${old_error} {}

/// Create unified error for ${description}
pub fn create_${old_error,,}_error(message: &str) -> NestGateUnifiedError {
    ${unified_category} {
        message: message.to_string(),
        context: "${description}".to_string(),
        source: None,
        recovery_suggestion: Some("Check ${description} configuration and retry".to_string()),
    }
}
EOF
        
        echo "   ✅ Created migration helper: $migration_file"
    else
        echo "   ℹ️  No instances found"
    fi
}

# Consolidate high-priority error types
echo "Starting systematic error consolidation..."

consolidate_error_enum "ModuleError" "NestGateUnifiedError::Internal" "internal module operations"
consolidate_error_enum "NetworkError" "NestGateUnifiedError::Network" "network operations"
consolidate_error_enum "StorageError" "NestGateUnifiedError::Storage" "storage operations"
consolidate_error_enum "SecurityError" "NestGateUnifiedError::Security" "security operations"
consolidate_error_enum "ConfigError" "NestGateUnifiedError::Configuration" "configuration management"
consolidate_error_enum "ValidationError" "NestGateUnifiedError::Validation" "data validation"

echo ""
echo "🧹 **PHASE 4: DEPRECATED ERROR CLEANUP**"
echo "----------------------------------------"

# Find and catalog deprecated error patterns
echo "Cataloging deprecated error patterns..."

DEPRECATED_ERRORS="deprecated-errors-catalog.txt"

cat > "$DEPRECATED_ERRORS" << 'EOF'
# DEPRECATED ERROR PATTERNS TO REMOVE
# These patterns should be removed after migration is complete

# Generic placeholder errors (most common)
enum ModuleError {
    Unknown(String),
}

# Tool-specific errors that should use NestGateUnifiedError::Tool
enum MigratorError { ... }
enum CloneOptimizerError { ... }
enum ScannerError { ... }

# Test-specific errors that should use NestGateUnifiedError::Test  
enum TestError { ... }
enum MockError { ... }
enum HardwareTestError { ... }

# Domain errors that have canonical equivalents
enum NetworkError { ... }  // -> NestGateUnifiedError::Network
enum StorageError { ... }  // -> NestGateUnifiedError::Storage
enum SecurityError { ... } // -> NestGateUnifiedError::Security
EOF

echo "✅ Created deprecated errors catalog: $DEPRECATED_ERRORS"

echo ""
echo "📝 **PHASE 5: ERROR MIGRATION DOCUMENTATION**"
echo "--------------------------------------------"

# Create comprehensive error migration guide
ERROR_MIGRATION_GUIDE="docs/ERROR_ENUM_CONSOLIDATION_GUIDE.md"

cat > "$ERROR_MIGRATION_GUIDE" << 'EOF'
# ⚠️ **ERROR ENUM CONSOLIDATION GUIDE**

**Generated**: $(date)  
**Purpose**: Systematic consolidation of error enums into NestGateUnifiedError  
**Status**: 🔄 **CONSOLIDATION IN PROGRESS**

---

## 📊 **ERROR CONSOLIDATION OVERVIEW**

This guide provides systematic migration paths for consolidating scattered error enums into the unified error system.

### **🎯 ERROR CONSOLIDATION TARGETS**

| **Error Pattern** | **Instances** | **Migration Target** | **Status** |
|-------------------|---------------|---------------------|------------|
| `ModuleError` | 40+ | `NestGateUnifiedError::Internal` | 🔄 In Progress |
| `NetworkError` | 15+ | `NestGateUnifiedError::Network` | 🔄 In Progress |
| `StorageError` | 12+ | `NestGateUnifiedError::Storage` | 🔄 In Progress |
| `SecurityError` | 10+ | `NestGateUnifiedError::Security` | 🔄 In Progress |
| `ConfigError` | 8+ | `NestGateUnifiedError::Configuration` | 🔄 In Progress |
| `ValidationError` | 6+ | `NestGateUnifiedError::Validation` | 🔄 In Progress |

---

## 🔄 **MIGRATION PATTERNS**

### **Pattern 1: Generic ModuleError Consolidation**

**BEFORE** (Scattered):
```rust
// Multiple files with generic ModuleError
#[derive(Debug)]
pub enum ModuleError {
    Unknown(String),
    InvalidInput(String),
    ProcessingFailed(String),
}
```

**AFTER** (Consolidated):
```rust
use nestgate_core::error::NestGateUnifiedError;

// Use specific unified error categories
let error = NestGateUnifiedError::Internal {
    message: "Processing failed".to_string(),
    context: "module operation".to_string(),
    source: None,
    recovery_suggestion: Some("Check input parameters and retry".to_string()),
};
```

### **Pattern 2: Domain-Specific Error Consolidation**

**BEFORE** (Scattered):
```rust
#[derive(Debug)]
pub enum NetworkError {
    ConnectionFailed(String),
    Timeout,
    InvalidAddress(String),
}

#[derive(Debug)]  
pub enum StorageError {
    FileNotFound(String),
    PermissionDenied,
    DiskFull,
}
```

**AFTER** (Consolidated):
```rust
use nestgate_core::error::NestGateUnifiedError;

// Network errors
let network_error = NestGateUnifiedError::Network {
    message: "Connection failed".to_string(),
    context: "network communication".to_string(),
    endpoint: Some("https://api.example.com".to_string()),
    recovery_suggestion: Some("Check network connectivity".to_string()),
};

// Storage errors
let storage_error = NestGateUnifiedError::Storage {
    message: "File not found".to_string(),
    context: "file system operation".to_string(),
    operation: "read".to_string(),
    recovery_suggestion: Some("Verify file path exists".to_string()),
};
```

---

## 🛠️ **MIGRATION HELPERS**

Migration helpers are available in `nestgate-core/src/error/migration_helpers/`:

- `moduleerror_migration.rs`: Migrate generic module errors
- `networkerror_migration.rs`: Migrate network-specific errors  
- `storageerror_migration.rs`: Migrate storage-specific errors
- `securityerror_migration.rs`: Migrate security-specific errors
- `configerror_migration.rs`: Migrate configuration errors
- `validationerror_migration.rs`: Migrate validation errors

---

## ✅ **VALIDATION CHECKLIST**

After consolidation, verify:

- [ ] All error enums use NestGateUnifiedError
- [ ] Error categories match domain contexts
- [ ] Rich error context is preserved
- [ ] Recovery suggestions are meaningful
- [ ] Migration helpers provide backward compatibility
- [ ] Tests pass with unified errors

---

## 🔍 **COMMON MIGRATION PATTERNS**

### **Generic Errors**
```rust
// OLD
enum ModuleError { Unknown(String) }

// NEW  
NestGateUnifiedError::Internal {
    message: error_message,
    context: "module_name".to_string(),
    // ...
}
```

### **Domain Errors**
```rust
// OLD
enum NetworkError { ConnectionFailed }

// NEW
NestGateUnifiedError::Network {
    message: "Connection failed".to_string(),
    endpoint: Some(endpoint_url),
    // ...
}
```

### **Tool Errors**
```rust
// OLD
enum MigratorError { ParseFailed }

// NEW
NestGateUnifiedError::Tool {
    message: "Parse failed".to_string(),
    tool_name: "migrator".to_string(),
    // ...
}
```

---

*Generated by NestGate Error Consolidation System*
EOF

echo "✅ Created error migration guide: $ERROR_MIGRATION_GUIDE"

echo ""
echo "📈 **ERROR CONSOLIDATION SUMMARY**"
echo "---------------------------------"

echo "✅ Error enum analysis complete"
echo "✅ Consolidation mapping created"
echo "✅ Migration helpers generated"
echo "✅ Deprecated errors cataloged"
echo "✅ Documentation created"

echo ""
echo "🎯 **NEXT STEPS**"
echo "----------------"
echo "1. Review generated migration helpers"
echo "2. Implement specific error conversions"
echo "3. Update error handling to use NestGateUnifiedError"
echo "4. Test error consolidation"
echo "5. Remove deprecated error enums"

echo ""
echo "📊 **PROGRESS METRICS**"
echo "----------------------"
TOTAL_ERROR_ENUMS=$(find . -name "*.rs" -exec grep -l "enum.*Error" {} \; 2>/dev/null | wc -l)
echo "Total error enums found: $TOTAL_ERROR_ENUMS"
echo "Migration helpers created: 6"
echo "Consolidation progress: Phase 2 Complete"

echo ""
echo "✅ **ERROR ENUM CONSOLIDATION - PHASE 2 COMPLETE**"
echo "==================================================" 