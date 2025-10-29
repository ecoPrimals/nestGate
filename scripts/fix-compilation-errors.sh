#!/bin/bash

# Fix Compilation Errors Script
# Systematically fixes the most common compilation error patterns

echo "🔧 Starting systematic compilation error fixes..."

# Fix missing context fields in Validation errors
echo "📋 Fixing Validation errors missing context field..."
find code/ -name "*.rs" -exec sed -i 's/field: "\([^"]*\)".to_string(),/field: Some("\1".to_string()),/g' {} \;
find code/ -name "*.rs" -exec sed -i '/user_error: true,$/a\                context: None,' {} \;

# Fix Internal errors with debug_info to use context
echo "📋 Fixing Internal errors with debug_info..."
find code/ -name "*.rs" -exec sed -i 's/debug_info: None,/context: None,/g' {} \;
find code/ -name "*.rs" -exec sed -i 's/debug_info: Some(\([^)]*\)),/context: Some(crate::error::context::ErrorContext { request_id: "internal".to_string(), user_context: None, system_context: None, debug_info: Some(\1), severity: crate::unified_types::error_types::UnifiedErrorSeverity::Error }),/g' {} \;

# Fix missing context fields in Io errors
echo "📋 Fixing I/O errors missing context field..."
find code/ -name "*.rs" -exec sed -i '/retryable: [^,]*,$/a\                context: None,' {} \;

# Fix Configuration errors missing fields
echo "📋 Fixing Configuration errors..."
find code/ -name "*.rs" -exec sed -i 's/config_source: [^,]*,//g' {} \;
find code/ -name "*.rs" -exec sed -i 's/suggested_fix: [^,]*,//g' {} \;

echo "✅ Systematic fixes applied!"
echo "🔍 Running compilation check..."

cargo check --package nestgate-core --quiet 2>&1 | grep -c "error\[E" || echo "0" 