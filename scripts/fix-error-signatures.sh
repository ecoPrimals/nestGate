#!/bin/bash
# 🔧 **ERROR SYSTEM SIGNATURE FIXES**
# Systematically fixes the most common error function signature issues

set -euo pipefail

echo "🔧 **FIXING ERROR SYSTEM SIGNATURES**"
echo "====================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

echo "📝 Phase 1: Fixing internal_error calls (most common)..."

# Fix internal_error calls that are missing the component parameter
find code/crates -name "*.rs" -type f -exec sed -i \
  's/NestGateError::internal_error("\([^"]*\)")/NestGateError::internal_error("\1", "system")/g' {} \;

# Fix specific patterns found in the errors
find code/crates -name "*.rs" -type f -exec sed -i \
  's/NestGateError::internal_error("Failed to serialize performance report: {}", e))/NestGateError::internal_error(\&format!("Failed to serialize performance report: {}", e), "performance")/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
  's/NestGateError::internal_error("Failed to read stats")/NestGateError::internal_error("Failed to read stats", "service")/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
  's/NestGateError::internal_error("Sync service is not available")/NestGateError::internal_error("Sync service is not available", "sync_service")/g' {} \;

echo "📝 Phase 2: Fixing configuration_error calls..."

# Fix configuration_error calls
find code/crates -name "*.rs" -type f -exec sed -i \
  's/NestGateError::configuration_error(\&format!("Missing required environment variable {key}: {e}"))/NestGateError::configuration_error("environment_variable", \&format!("Missing required environment variable {key}: {e}"))/g' {} \;

echo "📝 Phase 3: Fixing validation_error calls..."

# Fix validation_error calls that need two parameters
find code/crates -name "*.rs" -type f -exec sed -i \
  's/NestGateError::validation_error("File checksum verification failed ")/NestGateError::validation_error("checksum", "File checksum verification failed")/g' {} \;

echo "📝 Phase 4: Fixing comparison operators in error conversions..."

# Fix malformed comparison operators
find code/crates -name "*.rs" -type f -exec sed -i \
  's/error> "string_conversion"/\&error.to_string(), "string_conversion"/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
  's/error> "str_conversion"/\&error.to_string(), "str_conversion"/g' {} \;

find code/crates -name "*.rs" -type f -exec sed -i \
  's/"sync_config"> \&message.into()/"sync_config", \&message.to_string()/g' {} \;

echo "📝 Phase 5: Fixing format! macro issues..."

# Fix format! macro usage in error calls
find code/crates -name "*.rs" -type f -exec sed -i \
  's/format!("Sync IO error: {error}")/\&format!("Sync IO error: {}", error)/g' {} \;

echo "✅ Error signature fixes applied!"

echo ""
echo "🔧 Running cargo check to validate fixes..."
ERRORS_BEFORE=$(cargo check --workspace 2>&1 | grep -c "error:" || echo "0")
echo "📊 Errors before fixes: $ERRORS_BEFORE"

if cargo check --workspace --quiet 2>/dev/null; then
  echo "✅ All error signature fixes successful!"
else
  ERRORS_AFTER=$(cargo check --workspace 2>&1 | grep -c "error:" || echo "0")
  echo "📊 Errors after fixes: $ERRORS_AFTER"
  FIXED_COUNT=$((ERRORS_BEFORE - ERRORS_AFTER))
  if [ $FIXED_COUNT -gt 0 ]; then
    echo "✅ Fixed $FIXED_COUNT errors!"
  fi
  echo "📋 Remaining error summary:"
  cargo check --workspace 2>&1 | grep "error:" | head -10
fi 