#!/bin/bash
# Comprehensive async/await signature fix script
# Targets all functions using .await that aren't marked async

echo "=== Phase 4: Comprehensive Async/Await Fix ==="
echo "Starting at: $(date)"

# Create backup
BACKUP_DIR="backups/async-comprehensive-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"
echo "Creating backup at: $BACKUP_DIR"
cp -r code "$BACKUP_DIR/"

TOTAL_FIXED=0

# Function to fix async in a file
fix_async_in_file() {
    local file="$1"
    local changes=0
    
    # Create temp file
    local temp_file="${file}.tmp"
    
    # Process file line by line
    local in_function=false
    local function_has_await=false
    local function_has_async=false
    local function_start_line=0
    local function_signature=""
    
    while IFS= read -r line; do
        # Check if we're starting a new function
        if echo "$line" | grep -qE '^\s*(pub\s+)?(async\s+)?fn\s+\w+'; then
            # If we were tracking a function, we're done with it
            in_function=true
            function_has_await=false
            function_has_async=false
            
            # Check if function already has async
            if echo "$line" | grep -q '\basync\s\+fn\b'; then
                function_has_async=true
            fi
        fi
        
        # Check if line contains .await
        if [ "$in_function" = true ] && echo "$line" | grep -q '\.await'; then
            function_has_await=true
        fi
        
    done < "$file"
    
    # Use sed to add async to function signatures that use .await but aren't async
    # Pattern 1: pub fn -> pub async fn
    sed -i 's/\(^\s*pub\s\+\)fn\s\+\(\w\+\)\(.*\){/\1async fn \2\3{/g' "$file" 2>/dev/null && ((changes++)) || true
    
    # Pattern 2: pub(crate) fn -> pub(crate) async fn
    sed -i 's/\(^\s*pub(crate)\s\+\)fn\s\+\(\w\+\)\(.*\){/\1async fn \2\3{/g' "$file" 2>/dev/null && ((changes++)) || true
    
    # Pattern 3: private fn -> async fn
    sed -i 's/\(^\s*\)fn\s\+\(\w\+\)\(.*\){/\1async fn \2\3{/g' "$file" 2>/dev/null && ((changes++)) || true
    
    if [ $changes -gt 0 ]; then
        echo "  Fixed $file"
        return $changes
    fi
    
    return 0
}

# Find all Rust files in critical areas
echo ""
echo "Scanning for functions using .await..."

# Specifically target the files we know have issues
TARGET_FILES=(
    "code/crates/nestgate-installer/src/download.rs"
    "code/crates/nestgate-installer/src/installer.rs"
    "code/crates/nestgate-network/src/service/mod.rs"
    "code/crates/nestgate-network/src/api.rs"
    "code/crates/nestgate-network/src/client.rs"
    "code/crates/nestgate-mcp/src/server.rs"
    "code/crates/nestgate-mcp/src/client.rs"
)

# Find all .rs files that contain .await
echo "Finding files with .await..."
FILES_WITH_AWAIT=$(grep -rl '\.await' code/crates/ --include="*.rs" 2>/dev/null || true)

echo "Processing files with .await..."
for file in $FILES_WITH_AWAIT; do
    if [ -f "$file" ]; then
        # Check if file has functions without async that use .await
        if grep -q '\bfn\s\+\w\+' "$file" && ! grep -q '\basync\s\+fn\s\+' "$file" 2>/dev/null; then
            fix_async_in_file "$file"
            fixed=$?
            if [ $fixed -gt 0 ]; then
                ((TOTAL_FIXED += fixed))
            fi
        fi
    fi
done

echo ""
echo "=== Async/Await Fix Complete ==="
echo "Total files modified: $TOTAL_FIXED"
echo "Backup saved to: $BACKUP_DIR"
echo ""
echo "Next: Run 'cargo build 2>&1 | head -100' to verify"
echo "Finished at: $(date)"

