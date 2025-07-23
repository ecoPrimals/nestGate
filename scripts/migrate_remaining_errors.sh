#!/bin/bash
echo "🎯 SYSTEMATIC ERROR PATTERN MIGRATION"

# 1. Replace Parse errors with Validation errors
echo "1. Migrating Parse errors to Validation..."
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/NestGateError::Parse(/NestGateError::Validation { field: "parsing".to_string(), message: /g' {} \;

# 2. Replace SystemError with System errors
echo "2. Migrating SystemError to System..."  
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/NestGateError::SystemError(/NestGateError::System { component: "system".to_string(), message: /g' {} \;

# 3. Replace InvalidInput with Validation errors
echo "3. Migrating InvalidInput to Validation..."
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/NestGateError::InvalidInput(/NestGateError::Validation { field: "input".to_string(), message: /g' {} \;

# 4. Fix Internal error patterns (from tuple to struct)
echo "4. Migrating Internal error patterns..."
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/NestGateError::Internal(/NestGateError::Internal { message: /g' {} \;

# 5. Fix Network error patterns (from tuple to struct)  
echo "5. Migrating Network error patterns..."
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/NestGateError::Network(/NestGateError::Network { error: NetworkError::Connection { endpoint: "unknown".to_string(), message: /g' {} \;

echo "✅ Basic pattern migration complete!"

# 6. Fix closing patterns for migrated errors
echo "6. Fixing closing patterns..."
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/message: \([^}]*\)\))/message: \1, current_value: None, expected: None, user_error: false }/g' {} \;
find code/crates/nestgate-core/src -name "*.rs" -exec sed -i 's/message: \([^}]*\), current_value: None, expected: None, user_error: false })/message: \1, location: Some(file!().to_string()), debug_info: None, is_bug: false }/g' {} \;

echo "🎉 Migration script complete!"
