#!/bin/bash
echo "🎯 COMPREHENSIVE UTILS.RS PATTERN FIX"

# Fix all malformed Io patterns by removing extra fields
sed -i 's/})), resource: None, retryable: true }/})/' code/crates/nestgate-core/src/utils.rs

# Fix all malformed Validation patterns by removing extra fields  
sed -i 's/})), current_value: [^}]*, expected: [^}]*, user_error: [^}]* }/})/' code/crates/nestgate-core/src/utils.rs

# Fix all malformed System patterns by removing extra fields
sed -i 's/})), resource: [^}]*, utilization: [^}]*, recovery: [^}]* }/})/' code/crates/nestgate-core/src/utils.rs

# Fix all malformed Network patterns by removing extra fields
sed -i 's/})), retry_count: [^}]*, last_attempt: [^}]* }, context: [^}]* }/}, context: None })/' code/crates/nestgate-core/src/utils.rs

# Generic fix for any remaining malformed patterns
sed -i 's/})), [^}]*}/})/' code/crates/nestgate-core/src/utils.rs

# Fix closing parenthesis issues
sed -i 's/message: [^}]*}) }/message: e.to_string() })/g' code/crates/nestgate-core/src/utils.rs

echo "✅ Applied comprehensive fixes to all patterns!"

# Check for any remaining malformed patterns
remaining=$(grep -c "})), " code/crates/nestgate-core/src/utils.rs 2>/dev/null || echo "0")
echo "Remaining malformed patterns: $remaining"
