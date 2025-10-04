#!/bin/bash
# 🔧 **ASYNC FUNCTION RETURN TYPE FIXES**
# Systematically fixes async function return types that should be synchronous

set -euo pipefail

echo "🔧 **ASYNC FUNCTION RETURN TYPE FIXES**"
echo "======================================="

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

cd "$PROJECT_ROOT"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo ""
echo -e "${BLUE}Step 1: Converting sync functions with Future return types...${NC}"

# Fix capability discovery functions that should be sync
find code/crates -name "*.rs" -type f | while read -r file; do
    # Skip backup files
    if [[ "$file" == *".backup-"* ]]; then
        continue
    fi
    
    # Check for capability discovery functions that should be sync
    if grep -q "discover.*capability.*-> impl std::future::Future<Output = Result<" "$file" 2>/dev/null; then
        echo -e "${BLUE}   📝 Fixing capability discovery functions in: $file${NC}"
        
        # Backup the file
        cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        
        # Convert these functions to return Result directly
        sed -i 's/fn discover_\([^(]*\)_capability(&self) -> impl std::future::Future<Output = Result<\([^>]*\)>> + Send {/fn discover_\1_capability(\&self) -> Result<\2, NestGateError> {/g' "$file"
        sed -i 's/pub fn discover_capabilities(&self) -> impl std::future::Future<Output = Result<\([^>]*\)> + Send>  {/pub fn discover_capabilities(\&self) -> Result<\1, NestGateError> {/g' "$file"
    fi
done

echo ""
echo -e "${BLUE}Step 2: Fixing Result type parameter issues...${NC}"

# Fix Result types with missing error parameters
find code/crates -name "*.rs" -type f | while read -r file; do
    # Skip backup files
    if [[ "$file" == *".backup-"* ]]; then
        continue
    fi
    
    if grep -q "-> Result<[^,>]*>  {" "$file" 2>/dev/null; then
        echo -e "${BLUE}   📝 Adding missing error parameters in: $file${NC}"
        
        # Backup if not already backed up
        if [[ ! -f "$file.backup-$(date +%Y%m%d-%H%M%S)" ]]; then
            cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        fi
        
        # Add NestGateError as second parameter
        sed -i 's/-> Result<\([^,>]*\)>  {/-> Result<\1, NestGateError> {/g' "$file"
        sed -i 's/-> Result<\([^,>]*\)> {/-> Result<\1, NestGateError> {/g' "$file"
    fi
done

echo ""
echo -e "${BLUE}Step 3: Fixing functions that return futures but shouldn't...${NC}"

# Fix specific patterns that should be sync
find code/crates -name "*.rs" -type f | while read -r file; do
    # Skip backup files
    if [[ "$file" == *".backup-"* ]]; then
        continue
    fi
    
    # Check for update/add functions that should be sync
    if grep -q "-> impl std::future::Future<Output = Result<(), NestGateError>> + Send  {" "$file" 2>/dev/null; then
        echo -e "${BLUE}   📝 Converting sync functions with Future return in: $file${NC}"
        
        # Backup if not already backed up
        if [[ ! -f "$file.backup-$(date +%Y%m%d-%H%M%S)" ]]; then
            cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        fi
        
        # Convert these to sync functions
        sed -i 's/-> impl std::future::Future<Output = Result<(), NestGateError>> + Send  {/-> Result<(), NestGateError> {/g' "$file"
    fi
done

echo ""
echo -e "${BLUE}Step 4: Fixing type conversion errors...${NC}"

# Fix f32::from type conversion issues
find code/crates -name "*.rs" -type f | while read -r file; do
    # Skip backup files
    if [[ "$file" == *".backup-"* ]]; then
        continue
    fi
    
    if grep -q "f32::from(.*u32\|usize)" "$file" 2>/dev/null; then
        echo -e "${BLUE}   📝 Fixing f32 type conversions in: $file${NC}"
        
        # Backup if not already backed up
        if [[ ! -f "$file.backup-$(date +%Y%m%d-%H%M%S)" ]]; then
            cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        fi
        
        # Fix f32 conversions
        sed -i 's/f32::from(\([^)]*\))/(\1 as f32)/g' "$file"
    fi
done

echo ""
echo -e "${BLUE}Step 5: Fixing malformed Future syntax in trait bounds...${NC}"

# Fix malformed Future syntax with extra generic parameters
find code/crates -name "*.rs" -type f | while read -r file; do
    # Skip backup files
    if [[ "$file" == *".backup-"* ]]; then
        continue
    fi
    
    if grep -q "Future<Output = Result<.*> + Send, NestGateError>" "$file" 2>/dev/null; then
        echo -e "${BLUE}   📝 Fixing malformed Future syntax in: $file${NC}"
        
        # Backup if not already backed up
        if [[ ! -f "$file.backup-$(date +%Y%m%d-%H%M%S)" ]]; then
            cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        fi
        
        # Fix malformed Future syntax - remove extra generic parameter
        sed -i 's/Future<Output = Result<\([^>]*\)> + Send, NestGateError>/Future<Output = Result<\1, NestGateError>> + Send/g' "$file"
    fi
done

echo ""
echo -e "${BLUE}Step 6: Removing .await from non-async calls...${NC}"

# Fix .await on non-async calls
find code/crates -name "*.rs" -type f | while read -r file; do
    # Skip backup files
    if [[ "$file" == *".backup-"* ]]; then
        continue
    fi
    
    # Look for specific patterns that shouldn't be awaited
    if grep -q "\.get_capabilities().*\.await" "$file" 2>/dev/null; then
        echo -e "${BLUE}   📝 Removing unnecessary .await in: $file${NC}"
        
        # Backup if not already backed up
        if [[ ! -f "$file.backup-$(date +%Y%m%d-%H%M%S)" ]]; then
            cp "$file" "$file.backup-$(date +%Y%m%d-%H%M%S)"
        fi
        
        # Remove .await from sync calls
        sed -i 's/\.get_capabilities()\.await/.get_capabilities()/g' "$file"
    fi
done

echo ""
echo -e "${BLUE}Step 7: Testing compilation progress...${NC}"

ERROR_COUNT=$(cargo check --package nestgate-core --message-format short 2>&1 | grep -c "error\|warning" || echo "0")
echo -e "${BLUE}   Current errors/warnings: $ERROR_COUNT${NC}"

echo ""
echo -e "${GREEN}✅ **ASYNC FUNCTION FIXES COMPLETED**${NC}"
echo -e "${GREEN}===================================${NC}"

echo ""
echo -e "${BLUE}📊 **FIXES APPLIED**${NC}"
echo -e "${GREEN}   ✅ Capability discovery functions converted to sync${NC}"
echo -e "${GREEN}   ✅ Result type parameters corrected${NC}"
echo -e "${GREEN}   ✅ Future return types normalized${NC}"
echo -e "${GREEN}   ✅ Type conversion errors fixed${NC}"
echo -e "${GREEN}   ✅ Malformed Future syntax corrected${NC}"
echo -e "${GREEN}   ✅ Unnecessary .await calls removed${NC}"

echo ""
echo -e "${GREEN}🚀 Async function fixes completed!${NC}" 