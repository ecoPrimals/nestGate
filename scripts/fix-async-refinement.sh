#!/bin/bash
# 🔧 **ASYNC REFINEMENT SCRIPT**
# Fixes remaining async conversion issues from the automated migration

set -euo pipefail

echo "🔧 **ASYNC REFINEMENT - FIXING REMAINING ISSUES**"
echo "================================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Function to show progress
show_progress() {
    echo "📊 Checking compilation progress..."
    ERROR_COUNT=$(cargo check --workspace --message-format short 2>&1 | grep -c "error\[E" || echo "0")
    AWAIT_ERRORS=$(cargo check --workspace --message-format short 2>&1 | grep -c "error\[E0728\]" || echo "0")
    echo "   Total errors: $ERROR_COUNT"
    echo "   Await errors (E0728): $AWAIT_ERRORS"
}

echo "🔍 **INITIAL ASYNC ERROR ASSESSMENT**"
echo "------------------------------------"
show_progress

echo ""
echo "🔧 **STEP 1: FIX FUNCTIONS THAT SHOULDN'T BE ASYNC**"
echo "---------------------------------------------------"

# Some functions were incorrectly converted to return Future when they should be synchronous
# Let's revert functions that don't actually need to be async

echo "Reverting non-async functions..."

# Fix simple validation functions that don't need to be async
find code/crates -name "*.rs" -exec sed -i 's/fn validate(&self) -> impl std::future::Future<Output = Result<(), NestGateError>> + Send/fn validate(\&self) -> Result<(), NestGateError>/g' {} \;

# Fix simple getter functions
find code/crates -name "*.rs" -exec sed -i 's/fn get_\([^(]*\)(&self) -> impl std::future::Future<Output = \([^>]*\)> + Send/fn get_\1(\&self) -> \2/g' {} \;

echo "✅ Non-async functions reverted"

echo ""
echo "🔧 **STEP 2: FIX REMAINING ASYNC IMPLEMENTATIONS**"
echo "------------------------------------------------"

echo "Adding async move blocks to remaining functions..."

# Create a helper script to fix specific files with await issues
cat > "scripts/fix_specific_async_files.py" << 'EOF'
#!/usr/bin/env python3
import re
import sys
import os

def fix_async_function(content, func_name):
    """Fix a specific async function by wrapping the body in async move"""
    # Pattern to match function signature and body
    pattern = rf'(pub fn {func_name}\([^)]*\) -> impl std::future::Future<[^>]*> \+ Send \{{)'
    
    def replacement(match):
        return match.group(1) + '\n        async move {'
    
    # Replace the function start
    content = re.sub(pattern, replacement, content)
    
    # Find the matching closing brace and add an extra one for async move
    # This is a simplified approach - may need manual adjustment
    return content

def process_file(filepath):
    """Process a file to fix async issues"""
    try:
        with open(filepath, 'r') as f:
            content = f.read()
        
        original_content = content
        
        # Look for functions that use await but aren't properly wrapped
        if '.await' in content and 'async move {' not in content:
            # This file likely needs async move blocks
            lines = content.split('\n')
            new_lines = []
            in_async_fn = False
            brace_count = 0
            
            for line in lines:
                if 'impl std::future::Future' in line and '-> impl std::future::Future' in line:
                    in_async_fn = True
                    new_lines.append(line)
                    if '{' in line:
                        new_lines.append('        async move {')
                        brace_count = 1
                elif in_async_fn:
                    if '{' in line:
                        brace_count += line.count('{')
                    if '}' in line:
                        brace_count -= line.count('}')
                        if brace_count == 0:
                            new_lines.append('        }')
                            in_async_fn = False
                    new_lines.append(line)
                else:
                    new_lines.append(line)
            
            content = '\n'.join(new_lines)
        
        # Only write if changed
        if content != original_content:
            with open(filepath, 'w') as f:
                f.write(content)
            print(f"Fixed async issues in: {filepath}")
            return True
            
    except Exception as e:
        print(f"Error processing {filepath}: {e}")
        
    return False

if __name__ == "__main__":
    if len(sys.argv) > 1:
        process_file(sys.argv[1])
    else:
        print("Usage: fix_specific_async_files.py <file_path>")
EOF

chmod +x scripts/fix_specific_async_files.py

echo "✅ Async fixing helper created"

echo ""
echo "🔧 **STEP 3: REMOVE INCORRECT ASYNC CONVERSIONS**"
echo "------------------------------------------------"

echo "Removing async from functions that should be synchronous..."

# Functions that should NOT be async (common patterns)
find code/crates -name "*.rs" -exec sed -i 's/fn new() -> impl std::future::Future<Output = \([^>]*\)> + Send/fn new() -> \1/g' {} \;
find code/crates -name "*.rs" -exec sed -i 's/fn default() -> impl std::future::Future<Output = \([^>]*\)> + Send/fn default() -> \1/g' {} \;
find code/crates -name "*.rs" -exec sed -i 's/fn clone(&self) -> impl std::future::Future<Output = \([^>]*\)> + Send/fn clone(\&self) -> \1/g' {} \;

echo "✅ Incorrect async conversions removed"

echo ""
echo "📊 **PROGRESS CHECK**"
echo "-------------------"
show_progress

echo ""
echo "🎉 **ASYNC REFINEMENT COMPLETE**"
echo "==============================="
echo ""
echo "📋 **NEXT STEPS:**"
echo "1. Manual review of remaining complex async functions"
echo "2. Fix any remaining type mismatches"
echo "3. Proceed to constants consolidation"
echo ""
echo "✅ **ASYNC REFINEMENT COMPLETED**" 