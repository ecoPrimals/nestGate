#!/bin/bash
# Fix all malformed error patterns in utils.rs

echo "🔧 Fixing malformed error patterns in utils.rs..."

# Fix the Io error patterns - remove the extra fields that were incorrectly added
sed -i 's/NestGateError::Io { operation: "filesystem"\.to_string(), source: std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get file size: {e}"))), retry_count: 0 }/NestGateError::Io { operation: "filesystem".to_string(), source: std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to get file size: {e}")) })/' code/crates/nestgate-core/src/utils.rs

sed -i 's/NestGateError::Io { operation: "filesystem"\.to_string(), source: std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create directory: {e}"))), retry_count: 0 }/NestGateError::Io { operation: "filesystem".to_string(), source: std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to create directory: {e}")) })/' code/crates/nestgate-core/src/utils.rs

# Check for any other similar patterns and fix them
grep -n ")), retry_count: 0" code/crates/nestgate-core/src/utils.rs | while read line; do
    line_num=$(echo $line | cut -d: -f1)
    echo "Found malformed pattern on line $line_num, fixing..."
    sed -i "${line_num}s/)), retry_count: 0 }/})/" code/crates/nestgate-core/src/utils.rs
done

echo "✅ utils.rs error patterns fixed!"
