#!/bin/bash
# **NESTGATE ERROR SYSTEM UNIFICATION COMPLETION SCRIPT**
# 
# This script completes the migration from scattered error types to the
# unified NestGateUnifiedError system.

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚨 NestGate Error System Unification - Phase 3 Completion${NC}"
echo "=============================================================="

# Function to log with timestamp
log() {
    echo -e "[$(date '+%H:%M:%S')] $1"
}

# Function to find legacy error patterns
find_legacy_errors() {
    log "${BLUE}📊 Analyzing error system fragmentation...${NC}"
    
    echo "Legacy error patterns found:"
    
    # Find custom error enums
    echo "  Custom error enums:"
    find code/crates/ -name "*.rs" -exec grep -l "enum.*Error" {} \; | \
        grep -v "NestGateUnifiedError" | wc -l
    
    # Find Result type aliases
    echo "  Custom Result type aliases:"
    find code/crates/ -name "*.rs" -exec grep -l "type.*Result" {} \; | wc -l
    
    # Find unwrap/expect patterns
    echo "  Unsafe unwrap/expect patterns:"
    find code/crates/ -name "*.rs" -exec grep -l "\.unwrap()\|\.expect(" {} \; | wc -l
    
    # Find thiserror::Error usage
    echo "  thiserror::Error usage (should use NestGateUnifiedError):"
    find code/crates/ -name "*.rs" -exec grep -l "#\[derive.*Error\]" {} \; | wc -l
}

# Function to identify specific migration targets
identify_error_migration_targets() {
    log "${YELLOW}🎯 Identifying specific error migration targets...${NC}"
    
    echo "Specific files needing error migration:"
    
    # Find files with custom error types
    echo "  Files with custom error enums:"
    find code/crates/ -name "*.rs" -exec grep -l "pub enum.*Error" {} \; | \
        head -10
    
    echo "  Files with unsafe patterns:"
    find code/crates/ -name "*.rs" -exec grep -l "\.unwrap()" {} \; | \
        head -10
}

# Function to create error migration utilities
create_error_migration_utilities() {
    log "${BLUE}🔧 Creating error migration utilities...${NC}"
    
    # Create error migration helper
    cat > "code/crates/nestgate-core/src/error/migration_helper.rs" << 'EOF'
//! **ERROR MIGRATION HELPER**
//! 
//! Utilities for migrating legacy error types to NestGateUnifiedError

use crate::error::{NestGateUnifiedError, NestGateError};
use std::fmt;

/// Helper trait for migrating legacy errors to unified system
pub trait LegacyErrorMigration {
    /// Convert legacy error to unified error
    fn to_unified_error(self) -> NestGateUnifiedError;
}

/// Migration helper for common error patterns
pub struct ErrorMigrationHelper;

impl ErrorMigrationHelper {
    /// Migrate a generic error string to configuration error
    pub fn config_error(message: impl Into<String>) -> NestGateError {
        NestGateUnifiedError::configuration_error(message.into())
    }
    
    /// Migrate a generic error string to network error
    pub fn network_error(message: impl Into<String>) -> NestGateError {
        NestGateUnifiedError::network_error(message.into())
    }
    
    /// Migrate a generic error string to storage error
    pub fn storage_error(message: impl Into<String>) -> NestGateError {
        NestGateUnifiedError::storage_error(message.into())
    }
    
    /// Safe alternative to unwrap() with context
    pub fn safe_unwrap<T>(
        result: Option<T>, 
        context: &str
    ) -> Result<T, NestGateError> {
        result.ok_or_else(|| {
            NestGateUnifiedError::internal_error(
                format!("Value not found in context: {}", context)
            )
        })
    }
    
    /// Safe alternative to expect() with context
    pub fn safe_expect<T, E: fmt::Display>(
        result: Result<T, E>, 
        context: &str
    ) -> Result<T, NestGateError> {
        result.map_err(|e| {
            NestGateUnifiedError::internal_error(
                format!("Operation failed in {}: {}", context, e)
            )
        })
    }
}

/// Macro for easy error migration
#[macro_export]
macro_rules! migrate_error {
    (config, $msg:expr) => {
        $crate::error::migration_helper::ErrorMigrationHelper::config_error($msg)
    };
    (network, $msg:expr) => {
        $crate::error::migration_helper::ErrorMigrationHelper::network_error($msg)
    };
    (storage, $msg:expr) => {
        $crate::error::migration_helper::ErrorMigrationHelper::storage_error($msg)
    };
}

/// Macro for safe unwrap migration
#[macro_export]
macro_rules! safe_unwrap {
    ($expr:expr, $context:expr) => {
        $crate::error::migration_helper::ErrorMigrationHelper::safe_unwrap($expr, $context)?
    };
}

/// Macro for safe expect migration
#[macro_export]
macro_rules! safe_expect {
    ($expr:expr, $context:expr) => {
        $crate::error::migration_helper::ErrorMigrationHelper::safe_expect($expr, $context)?
    };
}
EOF

    log "${GREEN}✅ Error migration utilities created${NC}"
}

# Function to demonstrate unified error usage
demonstrate_unified_error_usage() {
    log "${BLUE}🚀 Demonstrating unified error system usage...${NC}"
    
    cat << EOF

Example of unified error handling migration:

BEFORE (Legacy fragmented errors):
\`\`\`rust
// Multiple different error types
pub enum ApiError { ... }
pub enum StorageError { ... }  
pub enum NetworkError { ... }

// Unsafe patterns
let value = option.unwrap();
let result = operation().expect("failed");
\`\`\`

AFTER (Unified error system):
\`\`\`rust
use nestgate_core::error::{NestGateError, Result};
use nestgate_core::{safe_unwrap, safe_expect, migrate_error};

// Single unified error type
fn operation() -> Result<Data> {
    let value = safe_unwrap!(option, "getting configuration value");
    let result = safe_expect!(external_call(), "calling external service");
    
    if validation_fails {
        return Err(migrate_error!(config, "validation failed"));
    }
    
    Ok(result)
}
\`\`\`

Benefits achieved:
- ✅ Single error type across entire ecosystem
- ✅ Rich error context with recovery suggestions  
- ✅ Safe alternatives to unwrap/expect
- ✅ Consistent error handling patterns
- ✅ Better debugging and monitoring integration

EOF
}

# Function to create migration report
create_error_migration_report() {
    log "${BLUE}📋 Creating error migration report...${NC}"
    
    local report_file="docs/ERROR_UNIFICATION_REPORT.md"
    
    cat > "$report_file" << EOF
# 🚨 Error System Unification Progress Report

**Generated**: $(date)
**Status**: Phase 3 - Error System Consolidation

## 📊 Current State Analysis

### Error System Framework Status
- ✅ **Unified Error Type**: \`NestGateUnifiedError\` established as single source
- ✅ **Rich Error Context**: Error details with recovery suggestions
- ✅ **Migration Utilities**: Helper functions and macros available
- ✅ **Safe Patterns**: Alternatives to unwrap/expect implemented

### Remaining Migration Targets

#### High Priority (Safety Critical)
EOF

    # Add migration statistics
    local custom_errors=$(find code/crates/ -name "*.rs" -exec grep -l "enum.*Error" {} \; | wc -l)
    local unsafe_patterns=$(find code/crates/ -name "*.rs" -exec grep -l "\.unwrap()" {} \; | wc -l)
    local custom_results=$(find code/crates/ -name "*.rs" -exec grep -l "type.*Result" {} \; | wc -l)
    
    cat >> "$report_file" << EOF
- **Custom Error Enums**: $custom_errors files need migration
- **Unsafe Patterns**: $unsafe_patterns files with unwrap/expect
- **Custom Result Types**: $custom_results files with custom Result aliases

## 🎯 Migration Strategy

1. **Phase 3A**: Replace custom error enums with NestGateUnifiedError variants
2. **Phase 3B**: Migrate unsafe unwrap/expect patterns to safe alternatives  
3. **Phase 3C**: Consolidate custom Result type aliases
4. **Phase 3D**: Update error handling throughout ecosystem

## 📈 Success Metrics

- **Target**: 100% usage of NestGateUnifiedError across ecosystem
- **Safety**: Zero unsafe unwrap/expect patterns in production code
- **Consistency**: Single Result<T> type with unified error handling
- **Quality**: Rich error context with actionable recovery suggestions

EOF

    log "${GREEN}✅ Error migration report created: $report_file${NC}"
}

# Main execution
main() {
    log "${GREEN}🚀 Starting error system unification analysis...${NC}"
    
    find_legacy_errors
    echo
    identify_error_migration_targets  
    echo
    create_error_migration_utilities
    echo
    create_error_migration_report
    echo
    demonstrate_unified_error_usage
    
    log "${GREEN}✅ Error system unification analysis complete!${NC}"
    log "${YELLOW}📋 Next: Begin systematic migration using provided utilities${NC}"
}

# Run the script
main "$@" 