#!/bin/bash
# 🔄 **MODULEERROR MIGRATION IMPLEMENTATION**
# Systematically migrate ModuleError to NestGateUnifiedError across 151+ files

set -euo pipefail

echo "🔄 **NESTGATE MODULEERROR MIGRATION IMPLEMENTATION**"
echo "==================================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📊 **PHASE 1: MIGRATION ANALYSIS AND PREPARATION**"
echo "------------------------------------------------"

# Create migration implementation helper
MIGRATION_IMPL="code/crates/nestgate-core/src/error/migration_helpers/moduleerror_implementation.rs"

cat > "$MIGRATION_IMPL" << 'EOF'
//! **MODULEERROR MIGRATION IMPLEMENTATION**
//! 
//! Provides complete implementation for migrating ModuleError patterns to NestGateUnifiedError.
//! This handles the systematic conversion of 151+ ModuleError instances.

use crate::error::NestGateUnifiedError;
use crate::error::variants::core_errors::{
    InternalErrorDetails, ConfigurationErrorDetails, NetworkErrorDetails,
    StorageErrorDetails, ValidationErrorDetails, SystemErrorDetails
};

/// Migrate generic ModuleError patterns to appropriate NestGateUnifiedError variants
pub fn migrate_module_error(
    error_message: &str,
    module_context: &str,
    error_category: ModuleErrorCategory,
) -> NestGateUnifiedError {
    match error_category {
        ModuleErrorCategory::Configuration => {
            NestGateUnifiedError::Configuration(Box::new(ConfigurationErrorDetails {
                message: error_message.to_string(),
                context: format!("Module: {}", module_context),
                config_key: None,
                suggested_value: None,
                recovery_suggestion: Some("Check module configuration and retry".to_string()),
            }))
        }
        
        ModuleErrorCategory::Network => {
            NestGateUnifiedError::Network(Box::new(NetworkErrorDetails {
                message: error_message.to_string(),
                context: format!("Module: {}", module_context),
                endpoint: None,
                status_code: None,
                retry_after: None,
                recovery_suggestion: Some("Check network connectivity and retry".to_string()),
            }))
        }
        
        ModuleErrorCategory::Storage => {
            NestGateUnifiedError::Storage(Box::new(StorageErrorDetails {
                message: error_message.to_string(),
                context: format!("Module: {}", module_context),
                operation: "module_operation".to_string(),
                path: None,
                recovery_suggestion: Some("Check storage configuration and retry".to_string()),
            }))
        }
        
        ModuleErrorCategory::Validation => {
            NestGateUnifiedError::Validation(Box::new(ValidationErrorDetails {
                message: error_message.to_string(),
                context: format!("Module: {}", module_context),
                field: None,
                expected: None,
                actual: None,
                recovery_suggestion: Some("Validate input parameters and retry".to_string()),
            }))
        }
        
        ModuleErrorCategory::System => {
            NestGateUnifiedError::System(Box::new(SystemErrorDetails {
                message: error_message.to_string(),
                context: format!("Module: {}", module_context),
                system_info: None,
                recovery_suggestion: Some("Check system resources and retry".to_string()),
            }))
        }
        
        ModuleErrorCategory::Internal | ModuleErrorCategory::Unknown => {
            NestGateUnifiedError::Internal(Box::new(InternalErrorDetails {
                message: error_message.to_string(),
                context: format!("Module: {} (migrated from ModuleError)", module_context),
                operation: "module_operation".to_string(),
                recovery_suggestion: Some("Check module implementation and retry".to_string()),
            }))
        }
    }
}

/// Categories for ModuleError migration
#[derive(Debug, Clone)]
pub enum ModuleErrorCategory {
    Configuration,
    Network,
    Storage,
    Validation,
    System,
    Internal,
    Unknown,
}

impl ModuleErrorCategory {
    /// Determine category from module path
    pub fn from_module_path(module_path: &str) -> Self {
        if module_path.contains("config") {
            Self::Configuration
        } else if module_path.contains("network") {
            Self::Network
        } else if module_path.contains("storage") || module_path.contains("zfs") {
            Self::Storage
        } else if module_path.contains("validation") {
            Self::Validation
        } else if module_path.contains("system") {
            Self::System
        } else {
            Self::Internal
        }
    }
    
    /// Determine category from error message
    pub fn from_error_message(message: &str) -> Self {
        let msg_lower = message.to_lowercase();
        if msg_lower.contains("config") {
            Self::Configuration
        } else if msg_lower.contains("network") || msg_lower.contains("connection") {
            Self::Network
        } else if msg_lower.contains("storage") || msg_lower.contains("file") || msg_lower.contains("zfs") {
            Self::Storage
        } else if msg_lower.contains("validation") || msg_lower.contains("invalid") {
            Self::Validation
        } else if msg_lower.contains("system") || msg_lower.contains("resource") {
            Self::System
        } else {
            Self::Internal
        }
    }
}

/// Helper macros for common ModuleError migration patterns
#[macro_export]
macro_rules! migrate_module_error {
    ($msg:expr, $module:expr) => {
        $crate::error::migration_helpers::moduleerror_implementation::migrate_module_error(
            $msg,
            $module,
            $crate::error::migration_helpers::moduleerror_implementation::ModuleErrorCategory::from_module_path($module)
        )
    };
    
    ($msg:expr, $module:expr, $category:expr) => {
        $crate::error::migration_helpers::moduleerror_implementation::migrate_module_error(
            $msg,
            $module,
            $category
        )
    };
}

/// Convert legacy ModuleError enum variants to unified errors
pub fn convert_legacy_module_error(legacy_error: &str, module_context: &str) -> NestGateUnifiedError {
    // Parse legacy error patterns and convert
    if legacy_error.contains("Configuration error:") {
        let message = legacy_error.replace("Configuration error: ", "");
        migrate_module_error(&message, module_context, ModuleErrorCategory::Configuration)
    } else if legacy_error.contains("Operation failed:") {
        let message = legacy_error.replace("Operation failed: ", "");
        migrate_module_error(&message, module_context, ModuleErrorCategory::Internal)
    } else if legacy_error.contains("Resource unavailable:") {
        let message = legacy_error.replace("Resource unavailable: ", "");
        migrate_module_error(&message, module_context, ModuleErrorCategory::System)
    } else {
        migrate_module_error(legacy_error, module_context, ModuleErrorCategory::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_error_migration() {
        let error = migrate_module_error(
            "Test error message",
            "test_module",
            ModuleErrorCategory::Configuration
        );
        
        match error {
            NestGateUnifiedError::Configuration(_) => (),
            _ => panic!("Expected Configuration error variant"),
        }
    }
    
    #[test]
    fn test_category_detection() {
        assert!(matches!(
            ModuleErrorCategory::from_module_path("network/retry"),
            ModuleErrorCategory::Network
        ));
        
        assert!(matches!(
            ModuleErrorCategory::from_error_message("Invalid configuration"),
            ModuleErrorCategory::Configuration
        ));
    }
}
EOF

echo "✅ Created migration implementation: $MIGRATION_IMPL"

echo ""
echo "🔄 **PHASE 2: SYSTEMATIC FILE MIGRATION**"
echo "----------------------------------------"

# Function to migrate a specific file
migrate_file() {
    local file_path="$1"
    local backup_path="${file_path}.backup"
    
    echo "🔄 Migrating: $file_path"
    
    # Create backup
    cp "$file_path" "$backup_path"
    
    # Determine module context from file path
    local module_context=$(echo "$file_path" | sed 's|.*/code/crates/nestgate-core/src/||' | sed 's|\.rs$||')
    
    # Apply migration transformations
    sed -i \
        -e 's|pub enum ModuleError|#[deprecated(since = "0.6.0", note = "Use NestGateUnifiedError instead")]\npub enum LegacyModuleError|g' \
        -e 's|ModuleError::|LegacyModuleError::|g' \
        -e 's|-> Result<\(.*\), ModuleError>|-> Result<\1, NestGateUnifiedError>|g' \
        -e 's|Result<\(.*\), ModuleError>|Result<\1, NestGateUnifiedError>|g' \
        "$file_path"
    
    # Add unified error import if not present
    if ! grep -q "use.*NestGateUnifiedError" "$file_path"; then
        # Add import after existing use statements
        sed -i '/^use /a use crate::error::NestGateUnifiedError;' "$file_path"
    fi
    
    # Add migration helper import
    if ! grep -q "migrate_module_error" "$file_path"; then
        sed -i '/^use.*NestGateUnifiedError/a use crate::migrate_module_error;' "$file_path"
    fi
    
    echo "   ✅ Migrated: $file_path (backup: $backup_path)"
}

# Get list of files with ModuleError
echo "Finding files with ModuleError patterns..."
FILES_TO_MIGRATE=$(find code/crates/nestgate-core -name "*.rs" -exec grep -l "pub enum ModuleError" {} \;)
TOTAL_FILES=$(echo "$FILES_TO_MIGRATE" | wc -l)

echo "Found $TOTAL_FILES files to migrate"

# Migrate files in batches
BATCH_SIZE=10
CURRENT_BATCH=0

for file in $FILES_TO_MIGRATE; do
    migrate_file "$file"
    CURRENT_BATCH=$((CURRENT_BATCH + 1))
    
    if [ $((CURRENT_BATCH % BATCH_SIZE)) -eq 0 ]; then
        echo "   📊 Migrated $CURRENT_BATCH / $TOTAL_FILES files..."
    fi
done

echo ""
echo "🔧 **PHASE 3: UPDATE MODULE EXPORTS**"
echo "-----------------------------------"

# Update mod.rs files to export migration helpers
MOD_FILES=$(find code/crates/nestgate-core/src -name "mod.rs")

for mod_file in $MOD_FILES; do
    if ! grep -q "migration_helpers" "$mod_file" 2>/dev/null; then
        echo "pub mod migration_helpers;" >> "$mod_file"
        echo "   ✅ Updated: $mod_file"
    fi
done

# Update main error mod.rs
ERROR_MOD="code/crates/nestgate-core/src/error/mod.rs"
if ! grep -q "migration_helpers" "$ERROR_MOD"; then
    echo "pub mod migration_helpers;" >> "$ERROR_MOD"
    echo "   ✅ Updated: $ERROR_MOD"
fi

echo ""
echo "📝 **PHASE 4: CREATE MIGRATION VALIDATION**"
echo "------------------------------------------"

# Create validation script
VALIDATION_SCRIPT="scripts/validate-moduleerror-migration.sh"

cat > "$VALIDATION_SCRIPT" << 'EOF'
#!/bin/bash
# Validate ModuleError migration results

echo "🔍 VALIDATING MODULEERROR MIGRATION"
echo "=================================="

ERRORS=0

# Check for remaining non-deprecated ModuleError
REMAINING_MODULE_ERRORS=$(find code/crates/nestgate-core -name "*.rs" -exec grep -l "pub enum ModuleError" {} \; | wc -l)
if [ "$REMAINING_MODULE_ERRORS" -gt 0 ]; then
    echo "❌ Found $REMAINING_MODULE_ERRORS files with non-deprecated ModuleError"
    ERRORS=$((ERRORS + 1))
else
    echo "✅ No non-deprecated ModuleError found"
fi

# Check for NestGateUnifiedError imports
UNIFIED_IMPORTS=$(find code/crates/nestgate-core -name "*.rs" -exec grep -l "use.*NestGateUnifiedError" {} \; | wc -l)
echo "✅ Found $UNIFIED_IMPORTS files with NestGateUnifiedError imports"

# Check compilation
echo "🔧 Testing compilation..."
if cargo check --quiet 2>/dev/null; then
    echo "✅ Compilation successful"
else
    echo "❌ Compilation failed - migration needs adjustment"
    ERRORS=$((ERRORS + 1))
fi

if [ "$ERRORS" -eq 0 ]; then
    echo "✅ MIGRATION VALIDATION SUCCESSFUL"
    exit 0
else
    echo "❌ MIGRATION VALIDATION FAILED ($ERRORS errors)"
    exit 1
fi
EOF

chmod +x "$VALIDATION_SCRIPT"
echo "✅ Created validation script: $VALIDATION_SCRIPT"

echo ""
echo "📈 **MIGRATION IMPLEMENTATION SUMMARY**"
echo "--------------------------------------"

echo "✅ Migration implementation complete"
echo "✅ $TOTAL_FILES files processed"
echo "✅ Backup files created for rollback"
echo "✅ Module exports updated"
echo "✅ Validation script created"

echo ""
echo "🎯 **NEXT STEPS**"
echo "----------------"
echo "1. Run validation: ./scripts/validate-moduleerror-migration.sh"
echo "2. Test compilation: cargo check"
echo "3. Run tests: cargo test"
echo "4. Review migration results"
echo "5. Remove backup files after validation"

echo ""
echo "📊 **MIGRATION PROGRESS**"
echo "------------------------"
echo "Files migrated: $TOTAL_FILES"
echo "Backup files created: $TOTAL_FILES"
echo "Migration helpers: 1 implementation module"
echo "Validation script: 1 created"

echo ""
echo "✅ **MODULEERROR MIGRATION IMPLEMENTATION COMPLETE**"
echo "===================================================" 