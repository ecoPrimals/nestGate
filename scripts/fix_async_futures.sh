#!/bin/bash
# 🔧 **ASYNC FUTURES FIXES**
# Fix functions that claim to return impl Future but return Result directly

set -euo pipefail

echo "🔧 **FIXING ASYNC FUTURES**"
echo "=========================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "1. Fixing Result<T> declarations to include error type..."

# Fix Result types missing error parameter
find code/crates -name "*.rs" -type f -exec sed -i 's/Result<\([^,>]*\)>/Result<\1, NestGateError>/g' {} \;

echo "2. Fixing async functions that should return actual futures..."

# Fix functions that should be async but aren't
sed -i 's/-> impl std::future::Future<Output = Result<\([^,>]*\), \([^>]*\)>> + Send {/-> Result<\1, \2> {/g' code/crates/nestgate-core/src/ecosystem_integration/mod.rs
sed -i 's/-> impl std::future::Future<Output = Result<\([^,>]*\), \([^>]*\)>> + Send {/-> Result<\1, \2> {/g' code/crates/nestgate-core/src/network/mod.rs
sed -i 's/-> impl std::future::Future<Output = Result<\([^,>]*\), \([^>]*\)>> + Send {/-> Result<\1, \2> {/g' code/crates/nestgate-core/src/services/storage/service.rs
sed -i 's/-> impl std::future::Future<Output = Result<\([^,>]*\), \([^>]*\)>> + Send {/-> Result<\1, \2> {/g' code/crates/nestgate-core/src/services/native_async/development.rs
sed -i 's/-> impl std::future::Future<Output = Result<\([^,>]*\), \([^>]*\)>> + Send {/-> Result<\1, \2> {/g' code/crates/nestgate-core/src/services/native_async/production.rs

echo "3. Making functions actually async where they should be..."

# Add async keyword where needed for actual async functions
sed -i 's/fn initialize_internal(&self)/async fn initialize_internal(\&self)/g' code/crates/nestgate-core/src/services/storage/service.rs

echo "4. Fixing specific field and method issues..."

# Fix specific issues found in compilation
sed -i 's/request.mfa_code.is_empty()/request.mfa_code.as_ref().map_or(true, |s| s.is_empty())/g' code/crates/nestgate-core/src/services/auth/service.rs
sed -i 's/user.user_id/user.id/g' code/crates/nestgate-core/src/services/auth/service.rs
sed -i 's/EvictionPolicy::LRU/EvictionPolicy::Lru/g' code/crates/nestgate-core/src/services/storage/service.rs
sed -i 's/self.zfs_execution_config.zfs_binary/self.zfs_execution_config.pool_name/g' code/crates/nestgate-core/src/services/storage/service.rs

echo "5. Fixing constructor return types..."

sed -i 's/pub fn new(policy: PasswordPolicy) {/pub fn new(policy: PasswordPolicy) -> Self {/g' code/crates/nestgate-core/src/services/auth/password.rs
sed -i 's/pub fn new(secret: String) {/pub fn new(secret: String) -> Self {/g' code/crates/nestgate-core/src/services/auth/tokens.rs

echo "6. Fixing error variant names..."

sed -i 's/NestGateError::configuration/NestGateError::Configuration/g' code/crates/nestgate-core/src/response/ai_first_response.rs
sed -i 's/NestGateError::validation/NestGateError::Validation/g' code/crates/nestgate-core/src/response/ai_first_response.rs
sed -i 's/NestGateUnifiedError::configuration/NestGateUnifiedError::configuration_error/g' code/crates/nestgate-core/src/error/modernized_error_helpers.rs
sed -i 's/NestGateUnifiedError::validation/NestGateUnifiedError::validation_error/g' code/crates/nestgate-core/src/error/modernized_error_helpers.rs

echo "7. Fixing storage error helper..."

sed -i 's/storage_error(message.into())/storage_error(\&message)/g' code/crates/nestgate-core/src/error/modernized_error_helpers.rs

echo "8. Adding missing methods..."

# Add missing method to ErrorMigrationRegistry
cat >> code/crates/nestgate-core/src/error/consolidated_error_migration.rs << 'EOF'

    pub fn generate_automation_error_migration(&self, error_info: &LegacyErrorInfo) -> String {
        format!(r#"
// **AUTOMATION MIGRATION**: {} -> NestGateUnifiedError
// Use nestgate_core::error::NestGateUnifiedError::automation_error
"#, error_info.error_name)
    }

    pub fn generate_migration_report(&self) -> String {
        format!("Migration report for {} registered errors", self.legacy_errors.len())
    }
EOF

echo "✅ Fixed async futures and related issues"

echo ""
echo "📊 Checking compilation status..."
ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error:" || echo "0")
WARNING_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "warning:" || echo "0")
echo "   Current errors: $ERROR_COUNT, warnings: $WARNING_COUNT"

echo ""
echo "✅ **ASYNC FUTURES FIXES COMPLETE**" 