#!/bin/bash

# Script to fix error format issues in NestGate codebase
# Converts old error format to new unified format

echo "🔧 Fixing error format issues across NestGate codebase..."

# Find all files with the old error format
files=$(find code/crates/nestgate-core/src -name "*.rs" -exec grep -l "location.*Some.*file!" {} \;)

for file in $files; do
    echo "📝 Processing: $file"
    
    # Create a temporary file for processing
    temp_file=$(mktemp)
    
    # Process the file with sed to fix common error patterns
    sed -E '
        # Remove location and is_bug fields
        /location: Some\(format!\("{}\:{}", file!\(\), line!\(\)\)\),/d
        /is_bug: (true|false),/d
        
        # Fix ErrorContext structure - add missing fields
        s/context: Some\(crate::error::ErrorContext \{/context: Some(crate::error::ErrorContext {\
            error_id: uuid::Uuid::new_v4().to_string(),/
        
        # Add missing fields before the closing brace
        s/recovery_suggestions: vec!\[([^\]]+)\],/recovery_suggestions: vec![\1],\
            performance_metrics: None,\
            environment: None,/
        
        # Fix timestamp placement
        s/timestamp: std::time::SystemTime::now\(\),/timestamp: std::time::SystemTime::now(),\
            stack_trace: None,\
            related_errors: vec![],/
        
        # Add component field for Internal errors missing it
        /NestGateError::Internal \{/,/\}/ {
            /message: format!\(/a\
        component: "internal".to_string(),
        }
    ' "$file" > "$temp_file"
    
    # Replace the original file if changes were made
    if ! cmp -s "$file" "$temp_file"; then
        mv "$temp_file" "$file"
        echo "✅ Fixed: $file"
    else
        rm "$temp_file"
        echo "⏭️  No changes needed: $file"
    fi
done

echo "🎉 Error format fixes completed!" 