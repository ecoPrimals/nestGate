#!/bin/bash
# Add SafeUnwrap imports to files that need them

for file in $(find code/crates/nestgate-core/src -name "*.rs" -type f -exec grep -l "\.safe_unwrap(" {} \;); do
    if ! grep -q "use.*ErrorCategory" "$file"; then
        # Determine the correct import path
        if [[ "$file" == *"/error/"* ]]; then
            import="use crate::error::helpers::{ErrorCategory, SafeUnwrap, SafeUnwrapOption};"
        else
            import="use crate::error::{ErrorCategory, SafeUnwrap, SafeUnwrapOption};"
        fi
        
        # Find the last use statement
        last_use=$(grep -n "^use " "$file" | grep -v "use super" | tail -1 | cut -d: -f1)
        
        if [ -n "$last_use" ]; then
            # Insert after last use statement
            sed -i "${last_use}a\\${import}" "$file"
            echo "✓ Added import to $file"
        else
            echo "⚠ No use statements found in $file"
        fi
    fi
done

